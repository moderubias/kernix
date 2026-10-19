//! Numerical oracles, finite differences and vision geometry regression tests.
use kernix_core::{Order, shape::Kahen, tensor::Tensor};
use kernix_ops::{neural::*, vision::RgbFrame};
fn tensor(dims: &[usize], data: Vec<f64>) -> Tensor<f64> {
    Tensor::from_vec(data, Kahen::new(dims).unwrap(), Order::C).unwrap()
}
fn generated(dims: &[usize]) -> Tensor<f64> {
    let n = dims.iter().product();
    tensor(
        dims,
        (0..n).map(|i| ((i * 7 % 19) as f64 - 9.0) / 10.0).collect(),
    )
}
fn near(a: f64, b: f64) {
    assert!(
        (a - b).abs() <= 2e-7 * (1.0 + a.abs() + b.abs()),
        "{a} != {b}"
    );
}
fn objective(x: &Tensor<f64>, w: &Tensor<f64>, o: Conv2dOptions) -> f64 {
    conv2d(x, w, o)
        .unwrap()
        .storage_slice()
        .iter()
        .map(|v| v * v / 2.0)
        .sum()
}
#[test]
fn convolution_hand_oracle_and_strides() {
    let x = tensor(&[1, 1, 3, 3], (1..=9).map(|v| v as f64).collect());
    let w = tensor(&[1, 1, 2, 2], vec![1.0, 0.0, 0.0, -1.0]);
    assert_eq!(
        conv2d(&x, &w, Conv2dOptions::default())
            .unwrap()
            .storage_slice(),
        &[-4.0; 4]
    );
    let reversed = x.reverse_axis(3).unwrap();
    assert_eq!(
        conv2d(&reversed, &w, Conv2dOptions::default())
            .unwrap()
            .storage_slice(),
        &[-2.0; 4]
    );
}
#[test]
fn grouped_dilated_padded_convolution_gradients() {
    let x = generated(&[1, 2, 4, 5]);
    let w = generated(&[2, 1, 2, 2]);
    let o = Conv2dOptions {
        groups: 2,
        padding: [1, 1],
        stride: [2, 1],
        dilation: [2, 1],
    };
    let y = conv2d(&x, &w, o).unwrap();
    let (dx, dw) = conv2d_backward(&x, &w, &y, o).unwrap();
    let h = 1e-5;
    for i in 0..x.numel() {
        let mut a = x.clone();
        let mut b = x.clone();
        a.storage_slice_mut()[i] += h;
        b.storage_slice_mut()[i] -= h;
        near(
            dx.storage_slice()[i],
            (objective(&a, &w, o) - objective(&b, &w, o)) / (2.0 * h),
        );
    }
    for i in 0..w.numel() {
        let mut a = w.clone();
        let mut b = w.clone();
        a.storage_slice_mut()[i] += h;
        b.storage_slice_mut()[i] -= h;
        near(
            dw.storage_slice()[i],
            (objective(&x, &a, o) - objective(&x, &b, o)) / (2.0 * h),
        );
    }
}
#[test]
fn pooling_oracles_ties_and_overlap_gradients() {
    let x = tensor(&[1, 1, 3, 3], (1..=9).map(|v| v as f64).collect());
    assert_eq!(
        avg_pool2d(&x, [2, 2], [1, 1]).unwrap().storage_slice(),
        &[3.0, 4.0, 6.0, 7.0]
    );
    assert_eq!(
        max_pool2d(&x, [2, 2], [1, 1]).unwrap().storage_slice(),
        &[5.0, 6.0, 8.0, 9.0]
    );
    let g = tensor(&[1, 1, 2, 2], vec![1.0; 4]);
    assert_eq!(
        avg_pool2d_backward(x.shape().dims(), &g, [2, 2], [1, 1])
            .unwrap()
            .storage_slice(),
        &[0.25, 0.5, 0.25, 0.5, 1.0, 0.5, 0.25, 0.5, 0.25]
    );
    let tied = tensor(&[1, 1, 2, 2], vec![3.0; 4]);
    let one = tensor(&[1, 1, 1, 1], vec![2.0]);
    assert_eq!(
        max_pool2d_backward(&tied, &one, [2, 2], [1, 1])
            .unwrap()
            .storage_slice(),
        &[2.0, 0.0, 0.0, 0.0]
    );
    let nan = tensor(&[1, 1, 2, 2], vec![3.0, f64::NAN, f64::NAN, 1.0]);
    assert!(max_pool2d(&nan, [2, 2], [1, 1]).unwrap().storage_slice()[0].is_nan());
    assert_eq!(
        max_pool2d_backward(&nan, &one, [2, 2], [1, 1])
            .unwrap()
            .storage_slice(),
        &[0.0, 2.0, 0.0, 0.0]
    );
}
#[test]
fn invalid_geometry_and_empty_windows() {
    let x = generated(&[1, 2, 2, 3]);
    let w = generated(&[2, 2, 3, 3]);
    assert!(
        conv2d(
            &x,
            &w,
            Conv2dOptions {
                stride: [0, 1],
                ..Default::default()
            }
        )
        .is_err()
    );
    assert!(
        conv2d(
            &x,
            &w,
            Conv2dOptions {
                groups: 3,
                ..Default::default()
            }
        )
        .is_err()
    );
    assert!(
        conv2d(
            &x,
            &w,
            Conv2dOptions {
                padding: [usize::MAX, 0],
                ..Default::default()
            }
        )
        .is_err()
    );
    assert_eq!(conv2d(&x, &w, Default::default()).unwrap().numel(), 0);
    assert!(avg_pool2d(&x, [0, 2], [1, 1]).is_err());
    assert_eq!(max_pool2d(&x, [4, 4], [1, 1]).unwrap().numel(), 0);
    let empty = tensor(&[1, 2, 0, 0], vec![]);
    assert_eq!(
        max_pool2d_backward(&x, &empty, [4, 4], [1, 1])
            .unwrap()
            .storage_slice(),
        &[0.0; 12]
    );
}
#[test]
fn stable_losses_and_dense_oracles() {
    let logits = tensor(&[2, 2], vec![1000.0, -1000.0, -1000.0, 1000.0]);
    near(cross_entropy_with_logits(&logits, &[0, 1]).unwrap(), 0.0);
    near(cross_entropy_with_logits(&logits, &[1, 0]).unwrap(), 2000.0);
    assert!(cross_entropy_with_logits(&logits, &[2, 0]).is_err());
    let target = tensor(&[2, 2], vec![1.0, 0.0, 0.0, 1.0]);
    near(
        binary_cross_entropy_with_logits(&logits, &target).unwrap(),
        0.0,
    );
    near(mse_loss(&target, &target).unwrap(), 0.0);
    let x = tensor(&[1, 2], vec![2.0, 3.0]);
    let w = tensor(&[2, 2], vec![1.0, 2.0, 3.0, 4.0]);
    let b = tensor(&[2], vec![1.0, -1.0]);
    assert_eq!(
        linear(&x, &w, Some(&b)).unwrap().storage_slice(),
        &[9.0, 17.0]
    );
}
#[test]
fn layer_normalization_and_invalid_loss_domains() {
    let x = tensor(&[2, 2], vec![1.0, 3.0, 7.0, 7.0]);
    let y = layer_norm(&x, 1, 1e-5).unwrap();
    near(y.storage_slice()[0], -1.0 / (1.00001_f64).sqrt());
    assert_eq!(y.storage_slice()[2], 0.0);
    assert!(layer_norm(&x, 0, 1e-5).is_err());
    assert!(layer_norm(&x, 1, 0.0).is_err());
    assert!(binary_cross_entropy_with_logits(&x, &tensor(&[2, 2], vec![2.0; 4])).is_err());
    assert!(mse_loss(&tensor(&[0], vec![]), &tensor(&[0], vec![])).is_err());
}
#[test]
fn rgb_identity_crop_and_half_pixel_interpolation() {
    let bytes = [255, 0, 0, 0, 255, 0, 0, 0, 255, 255, 255, 255];
    let frame = RgbFrame::new(&bytes, 2, 2).unwrap();
    assert_eq!(
        frame
            .resize_nchw([1, 1], [0.0; 3], [1.0; 3])
            .unwrap()
            .storage_slice(),
        &[0.5; 3]
    );
    let crop = frame.crop(1, 0, 1, 2).unwrap();
    assert_eq!((crop.width(), crop.height()), (1, 2));
    assert_eq!(
        crop.resize_nchw([2, 1], [0.0; 3], [1.0; 3])
            .unwrap()
            .storage_slice(),
        &[0.0, 1.0, 1.0, 1.0, 0.0, 1.0]
    );
    assert_eq!(
        frame
            .resize_nchw([2, 2], [0.0; 3], [1.0; 3])
            .unwrap()
            .storage_slice(),
        &[1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0]
    );
}
#[test]
fn padded_rgb_rows_and_invalid_frames() {
    let bytes = [255, 0, 0, 99, 99, 0, 255, 0];
    let f = RgbFrame::from_rows(&bytes, 1, 2, 5).unwrap();
    assert_eq!(
        f.resize_nchw([1, 1], [0.0; 3], [1.0; 3])
            .unwrap()
            .storage_slice(),
        &[0.5, 0.5, 0.0]
    );
    assert!(f.crop(usize::MAX, 0, 2, 2).is_err());
    assert!(f.resize_nchw([0, 1], [0.0; 3], [1.0; 3]).is_err());
    assert!(f.resize_nchw([1, 1], [0.0; 3], [0.0; 3]).is_err());
    assert!(RgbFrame::new(&[], 0, 1).is_err());
    assert!(RgbFrame::from_rows(&[], usize::MAX, 1, 3).is_err());
}

#[test]
fn finite_mean_cross_entropy_survives_unrepresentable_single_row_loss() {
    let x = tensor(&[2, 2], vec![f64::MAX, -f64::MAX, 0.0, 0.0]);
    let loss = cross_entropy_with_logits(&x, &[1, 0]).unwrap();
    assert!(loss.is_finite());
    near(loss / f64::MAX, 1.0);
}
