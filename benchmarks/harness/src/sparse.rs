//! Exact dyadic five-diagonal fixtures. Setup and scalar oracles are untimed.
use crate::{Tensor, measure};
use serde_json::{Value, json};

fn dimensions(name: &str) -> (usize, usize, bool, bool) {
    match name {
        "sparse_matvec_2048_f64" => (2048, 1, false, false),
        "sparse_matmul_512x32_f64" | "sparse_matmul_512x32_f32" => (512, 32, false, false),
        "sparse_product_256_f64" => (256, 256, true, false),
        "sparse_auto_512_f64" => (512, 512, false, true),
        _ => panic!("unknown sparse fixture"),
    }
}
fn coefficients(n: usize) -> Vec<(usize, usize, f64)> {
    let mut entries = Vec::with_capacity(5 * n);
    for i in 0..n {
        let mut row: Vec<_> = (0..5)
            .map(|d| (i, (i + n + d - 2) % n, (d + 1) as f64 / 16.))
            .collect();
        row.sort_unstable_by_key(|v| v.1);
        entries.extend(row);
    }
    entries
}
fn check(
    n: usize,
    cols: usize,
    product: bool,
    construction: bool,
    get: impl Fn(usize, usize) -> f64,
) -> Value {
    let mut expected = vec![0.; n * cols];
    if construction {
        for (i, k, v) in coefficients(n) {
            expected[i * n + k] = v;
        }
    } else if product {
        // Independent coordinate generation; no call to any measured matrix kernel.
        for i in 0..n {
            for d in 0..5 {
                let k = (i + n + d - 2) % n;
                for e in 0..5 {
                    let j = (k + n + e - 2) % n;
                    expected[i * n + j] += (d + 1) as f64 * (e + 1) as f64 / 256.;
                }
            }
        }
    } else {
        for i in 0..n {
            for d in 0..5 {
                let k = (i + n + d - 2) % n;
                for j in 0..cols {
                    expected[i * cols + j] +=
                        (d + 1) as f64 / 16. * (((k * cols + j) % 17) as f64 - 8.) / 8.;
                }
            }
        }
    }
    let mut error = 0.0f64;
    for i in 0..n {
        for j in 0..cols {
            error = error.max((get(i, j) - expected[i * cols + j]).abs());
        }
    }
    assert_eq!(error, 0., "dyadic fixture must remain exact");
    json!({"coordinate_max_absolute_error":error,"budget":0.,"left_nnz":5*n,"pattern":"cyclic five diagonals; no pruning"})
}
macro_rules! run_typed {
    ($name:ident,$t:ty) => {
        fn $name(name:&str,library:&str,mode:&str)->Value {
            let (n,cols,product,construction)=dimensions(name);
            let triples:Vec<_>=coefficients(n).into_iter().map(|(i,j,v)|(i,j,v as $t)).collect();
            let mut dense=vec![0. as $t;n*n];
            for &(i,j,v) in &triples {dense[i*n+j]=v;}
            let rhs:Vec<$t>=if product {dense.clone()} else {(0..n*cols).map(|p|((p%17) as $t-8.)/8.).collect()};
            let verify=|get:&dyn Fn(usize,usize)->f64|check(n,cols,product,construction,get);
            let mut result=match library {
                "kernix-dense" => {
                    let a=Tensor::from_values([n,n],dense.clone()).unwrap();
                    let b=if cols==1 {Tensor::from_values([n],rhs.clone()).unwrap()}else{Tensor::from_values([n,cols],rhs.clone()).unwrap()};
                    if construction {measure(mode,||Tensor::from_values([n,n],dense.clone()).unwrap(),|o|verify(&|i,j|o.storage_slice()[i*n+j] as f64))}
                    else {measure(mode,||kernix::matmul(&a,&b).unwrap(),|o|verify(&|i,j|o.storage_slice()[i*cols+j] as f64))}
                }
                #[cfg(not(feature="legacy-sparse"))]
                "kernix" => {
                    if construction {measure(mode,||kernix::Matrix::from_values([n,n],dense.clone()).unwrap(),|o|{assert_eq!(o.storage_kind(),kernix::MatrixStorage::Csr);verify(&|i,j|o.as_sparse().unwrap().get(i,j).unwrap().copied().unwrap_or(0.) as f64)})}
                    else {
                        let a=kernix::Matrix::from_values([n,n],dense).unwrap();
                        assert_eq!(a.storage_kind(),kernix::MatrixStorage::Csr);
                        if cols==1 {
                            let b=Tensor::from_values([n],rhs).unwrap();
                            measure(mode,||a.matvec(&b).unwrap(),|o|verify(&|i,j|o.storage_slice()[i*cols+j] as f64))
                        }else{
                            let b=if product {kernix::Matrix::from_values([n,n],rhs).unwrap()}else{kernix::Matrix::dense(Tensor::from_values([n,cols],rhs).unwrap()).unwrap()};
                            measure(mode,||a.matmul(&b).unwrap(),|o|verify(&|i,j|if let Some(d)=o.as_dense(){d.storage_slice()[i*cols+j] as f64}else{o.as_sparse().unwrap().get(i,j).unwrap().copied().unwrap_or(0.) as f64}))
                        }
                    }
                }
                "faer" => {
                    assert!(!construction && !product);
                    let triplets:Vec<_>=triples.into_iter().map(|(i,j,v)|faer::sparse::Triplet::new(i,j,v)).collect();
                    let a=faer::sparse::SparseRowMat::<usize,$t>::try_new_from_triplets(n,n,&triplets).unwrap();
                    let b=faer::MatRef::from_row_major_slice(&rhs,n,cols);
                    measure(mode,||{
                        let mut out=faer::Mat::<$t>::zeros(n,cols);
                        faer::sparse::linalg::matmul::sparse_dense_matmul(out.as_mut(),faer::Accum::Replace,a.as_ref(),b,1.,faer::Par::Seq);
                        out
                    },|o|verify(&|i,j|o[(i,j)] as f64))
                }
                _=>panic!("unknown sparse library"),
            };
            let sparse=library=="kernix"||library=="faer";
            result["storage"]=json!({"left_nnz":5*n,"left_payload_bytes":if sparse{(n+1)*std::mem::size_of::<usize>()+5*n*(std::mem::size_of::<usize>()+std::mem::size_of::<$t>())}else{n*n*std::mem::size_of::<$t>()},"dense_left_bytes":n*n*std::mem::size_of::<$t>(),"index_bytes":if sparse{std::mem::size_of::<usize>()}else{0},"pattern":"cyclic five diagonals","setup_excluded":!construction,"finite_validation":"Kernix Matrix: finite constructor and checked results; raw dense and peers have different nonfinite contracts"});
            result
        }
    }
}
run_typed!(run_f64, f64);
run_typed!(run_f32, f32);
pub fn run(name: &str, library: &str, mode: &str) -> Value {
    if name.ends_with("f32") {
        run_f32(name, library, mode)
    } else {
        run_f64(name, library, mode)
    }
}
#[test]
fn dyadic_sparse_fixtures_match_all_measured_rust_paths() {
    let mode = if cfg!(feature = "allocation-counting") {
        "resources"
    } else {
        "first"
    };
    for name in [
        "sparse_matvec_2048_f64",
        "sparse_matmul_512x32_f64",
        "sparse_matmul_512x32_f32",
        "sparse_product_256_f64",
        "sparse_auto_512_f64",
    ] {
        for library in ["kernix-dense", "faer"] {
            if library == "faer"
                && (name.starts_with("sparse_product_") || name.starts_with("sparse_auto_"))
            {
                continue;
            }
            run(name, library, mode);
        }
        #[cfg(not(feature = "legacy-sparse"))]
        run(name, "kernix", mode);
    }
}
