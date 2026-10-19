//! Optimizer transactionality, seeded initialization and exact checkpoint resume.
use kernix::{
    Kahen, Order, Tensor,
    autograd::TrainFloat,
    checkpoint::Checkpoint,
    training::{Adam, AdamOptions, Parameter, SeededRng, Sgd, clip_grad_norm},
};
fn tensor(dims: &[usize], v: &[f64]) -> Tensor<f64> {
    Tensor::from_vec(v.to_vec(), Kahen::new(dims).unwrap(), Order::C).unwrap()
}
fn parameter(v: &[f64]) -> Parameter<f64> {
    Parameter::new(tensor(&[v.len()], v)).unwrap()
}
#[test]
fn seeded_initialization_and_invalid_domains() {
    let a = Parameter::<f32>::xavier([3, 2], &mut SeededRng::new(7)).unwrap();
    let b = Parameter::<f32>::xavier([3, 2], &mut SeededRng::new(7)).unwrap();
    assert_eq!(a.value().storage_slice(), b.value().storage_slice());
    assert!(Parameter::<f64>::xavier([2], &mut SeededRng::new(1)).is_err());
    assert!(Sgd::<f64>::new(f64::NAN).is_err());
    assert!(Adam::<f64>::new(-1.0).is_err());
    assert!(
        Adam::from_options(AdamOptions::<f64> {
            beta1: 1.0,
            ..Default::default()
        })
        .is_err()
    );
    let mut p = parameter(&[1.0]);
    assert!(p.replace(tensor(&[2], &[1.0, 2.0])).is_err());
    assert_eq!(p.value().storage_slice(), &[1.0]);
}
#[test]
fn sgd_and_adam_oracles_transactionality_and_identity() {
    let mut p = parameter(&[2.0]);
    let gradient = tensor(&[1], &[4.0]);
    Sgd::new(0.1)
        .unwrap()
        .step(&mut [(&mut p, &gradient)])
        .unwrap();
    assert!((p.value().storage_slice()[0] - 1.6).abs() < 1e-14);
    let mut p = parameter(&[2.0]);
    let mut q = parameter(&[3.0]);
    let mut adam = Adam::new(0.1).unwrap();
    adam.step(&mut [(&mut p, &gradient)]).unwrap();
    assert!((p.value().storage_slice()[0] - 1.9).abs() < 1e-8);
    assert_eq!(adam.state_count(), 1);
    let before = p.value().storage_slice().to_vec();
    let bad = tensor(&[1], &[f64::NAN]);
    assert!(
        adam.step(&mut [(&mut p, &gradient), (&mut q, &bad)])
            .is_err()
    );
    assert_eq!(p.value().storage_slice(), before);
    assert_eq!(q.value().storage_slice(), &[3.0]);
    assert_eq!(adam.state_count(), 1);
    let mut clone = p.clone();
    adam.step(&mut [(&mut clone, &gradient)]).unwrap();
    assert_eq!(adam.state_count(), 2);
    adam.reset();
    assert_eq!(adam.state_count(), 0);
    let mut p = parameter(&[1.0]);
    let mut q = parameter(&[2.0]);
    assert!(
        Sgd::new(0.1)
            .unwrap()
            .step(&mut [(&mut p, &gradient), (&mut q, &bad)])
            .is_err()
    );
    assert_eq!(p.value().storage_slice(), &[1.0]);
}
#[test]
fn adamw_decoupled_weight_decay() {
    let mut p = parameter(&[2.0]);
    let gradient = tensor(&[1], &[0.0]);
    let mut adam = Adam::from_options(AdamOptions {
        learning_rate: 0.1,
        weight_decay: 0.5,
        ..Default::default()
    })
    .unwrap();
    adam.step(&mut [(&mut p, &gradient)]).unwrap();
    assert_eq!(p.value().storage_slice(), &[1.9]);
}
#[test]
fn clipping_handles_huge_and_small_values_without_intermediate_overflow() {
    let mut gradients = [tensor(&[2], &[3.0, 4.0])];
    assert_eq!(clip_grad_norm(&mut gradients, 2.5).unwrap(), 5.0);
    assert_eq!(gradients[0].storage_slice(), &[1.5, 2.0]);
    let mut huge = [tensor(&[2], &[f64::MAX, f64::MAX])];
    assert!(clip_grad_norm(&mut huge, 1e-300).unwrap().is_infinite());
    assert!(
        huge[0]
            .storage_slice()
            .iter()
            .all(|x| x.is_finite() && *x > 0.0)
    );
    let mut bad = [tensor(&[1], &[1.0]), tensor(&[1], &[f64::NAN])];
    assert!(clip_grad_norm(&mut bad, 1.0).is_err());
    assert_eq!(bad[0].storage_slice(), &[1.0]);
}
fn roundtrip<T: TrainFloat + core::fmt::Debug>() {
    let mut p = Parameter::new(
        Tensor::from_vec(
            vec![T::from_f64(1.25), T::from_f64(-0.0)],
            Kahen::new([2]).unwrap(),
            Order::C,
        )
        .unwrap(),
    )
    .unwrap();
    let g = Tensor::from_vec(
        vec![T::from_f64(0.3), T::from_f64(-0.7)],
        Kahen::new([2]).unwrap(),
        Order::C,
    )
    .unwrap();
    let mut adam = Adam::new(T::from_f64(0.01)).unwrap();
    adam.step(&mut [(&mut p, &g)]).unwrap();
    let bytes = Checkpoint::capture(&[("weight", &p)], Some(&adam))
        .unwrap()
        .to_bytes()
        .unwrap();
    let cp = Checkpoint::<T>::from_bytes(&bytes).unwrap();
    assert_eq!(cp.to_bytes().unwrap(), bytes);
    let mut restored = p.clone();
    let mut resumed = Adam::new(T::from_f64(0.9)).unwrap();
    cp.restore(&mut [("weight", &mut restored)], Some(&mut resumed))
        .unwrap();
    adam.step(&mut [(&mut p, &g)]).unwrap();
    resumed.step(&mut [(&mut restored, &g)]).unwrap();
    assert_eq!(p.value().storage_slice(), restored.value().storage_slice());
    assert_eq!(
        Checkpoint::capture(&[("weight", &p)], Some(&adam))
            .unwrap()
            .to_bytes()
            .unwrap(),
        Checkpoint::capture(&[("weight", &restored)], Some(&resumed))
            .unwrap()
            .to_bytes()
            .unwrap()
    );
}
#[test]
fn checkpoint_resumes_adam_exactly_for_both_dtypes() {
    roundtrip::<f32>();
    roundtrip::<f64>();
}
#[test]
fn checkpoint_rejects_corruption_truncation_dtype_and_schema() {
    let p = parameter(&[1.0, 2.0]);
    let bytes = Checkpoint::capture(&[("weight", &p)], None)
        .unwrap()
        .to_bytes()
        .unwrap();
    assert!(Checkpoint::<f32>::from_bytes(&bytes).is_err());
    for i in 0..bytes.len() {
        let mut bad = bytes.clone();
        bad[i] ^= 1;
        assert!(
            Checkpoint::<f64>::from_bytes(&bad).is_err(),
            "corrupt byte {i}"
        );
        assert!(Checkpoint::<f64>::from_bytes(&bytes[..i]).is_err());
    }
    assert!(Checkpoint::capture(&[("same", &p), ("same", &p)], None).is_err());
    let cp = Checkpoint::<f64>::from_bytes(&bytes).unwrap();
    let mut q = parameter(&[7.0]);
    assert!(cp.restore_weights(&mut [("weight", &mut q)]).is_err());
    assert_eq!(q.value().storage_slice(), &[7.0]);
    let mut q = parameter(&[7.0, 8.0]);
    assert!(cp.restore_weights(&mut [("wrong", &mut q)]).is_err());
    assert_eq!(q.value().storage_slice(), &[7.0, 8.0]);
}
#[cfg(feature = "std")]
#[test]
fn filesystem_checkpoint_replaces_file_and_honors_limits() {
    let path =
        std::env::temp_dir().join(format!("kernix-checkpoint-test-{}.bin", std::process::id()));
    let p = parameter(&[1.0]);
    let cp = Checkpoint::capture(&[("p", &p)], None).unwrap();
    cp.save(&path).unwrap();
    cp.save(&path).unwrap();
    assert!(Checkpoint::<f64>::load_with_limit(&path, 1).is_err());
    assert_eq!(
        Checkpoint::<f64>::load(&path).unwrap().parameters().len(),
        1
    );
    std::fs::remove_file(path).unwrap();
}

#[test]
fn checksum_valid_checkpoint_rejects_impossible_adam_bias_power() {
    let mut p = parameter(&[2.0]);
    let mut adam = Adam::new(0.1).unwrap();
    adam.step(&mut [(&mut p, &tensor(&[1], &[4.0]))]).unwrap();
    let mut bytes = Checkpoint::capture(&[("p", &p)], Some(&adam))
        .unwrap()
        .to_bytes()
        .unwrap();
    let payload_len = bytes.len() - 8;
    let offset = bytes[..payload_len]
        .windows(8)
        .rposition(|x| x == 0.9f64.to_le_bytes())
        .unwrap();
    bytes[offset..offset + 8].copy_from_slice(&1.0f64.to_le_bytes());
    let mut hash = 0xcbf29ce484222325u64;
    for &byte in &bytes[..payload_len] {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    bytes[payload_len..].copy_from_slice(&hash.to_le_bytes());
    assert!(Checkpoint::<f64>::from_bytes(&bytes).is_err());
}
