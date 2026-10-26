//! Automatic storage, sparse assembly and an existing dense activation compose.
use kernix::{CsrMatrix, Matrix, MatrixStorage, SparseError, Tensor};
fn main() -> Result<(), SparseError> {
    let weights = Matrix::from_rows([
        [2.0f32, 0., 0., 0., 0., 0., 0., 0.],
        [0., 3., 0., 0., 0., 0., 0., 0.],
    ])?;
    assert_eq!(weights.storage_kind(), MatrixStorage::Csr);
    let input = Matrix::from_rows([[1.0f32], [2.], [3.], [4.], [5.], [6.], [7.], [8.]])?;
    let scores: Tensor<f32> = weights.matmul(&input)?.into_dense()?;
    assert_eq!(kernix::relu(&scores)?.storage_slice(), &[2., 6.]);
    let graph = CsrMatrix::from_triplets([4, 4], vec![(0, 1, 1.0f32), (1, 2, 1.), (2, 3, 1.)])?;
    let paths = graph.matmul_with_limit(&graph, 4)?;
    assert_eq!(paths.nnz(), 2);
    let back = graph.transpose();
    assert!(core::ptr::eq(
        back.values().as_ptr(),
        graph.values().as_ptr()
    ));
    Ok(())
}
