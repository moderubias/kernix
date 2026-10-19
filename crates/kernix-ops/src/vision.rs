//! Borrowed RGB frame views and fused, portable CPU preprocessing.
//!
//! ```
//! use kernix_ops::vision::RgbFrame;
//! let bytes = vec![255u8,0,0, 0,255,0, 0,0,255, 255,255,255];
//! let frame = RgbFrame::new(&bytes,2,2)?;
//! let input = frame.resize_nchw([1,1], [0.0;3], [1.0;3])?;
//! assert_eq!(input.shape().dims(), &[1,3,1,1]);
//! assert_eq!(input.storage_slice(), &[0.5,0.5,0.5]);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
use crate::OpsError;
use kernix_core::{Order, shape::Kahen, tensor::Tensor};
fn invalid(parameter: &'static str) -> OpsError {
    OpsError::InvalidParameter {
        operation: "RGB preprocessing",
        parameter,
    }
}

/// Checked, borrowed packed RGB8 frame with optional padding between rows.
///
/// This type performs no capture or decoding. Supply bytes from any platform or
/// media library. Views/crops allocate nothing and retain the source lifetime.
#[derive(Debug, Clone, Copy)]
pub struct RgbFrame<'a> {
    bytes: &'a [u8],
    width: usize,
    height: usize,
    row_stride: usize,
}
impl<'a> RgbFrame<'a> {
    /// Bind exactly width*height*3 RGB bytes; reject empty frames or overflow.
    pub fn new(bytes: &'a [u8], width: usize, height: usize) -> Result<Self, OpsError> {
        let row = width
            .checked_mul(3)
            .ok_or_else(|| invalid("frame extent overflow"))?;
        if row.checked_mul(height) != Some(bytes.len()) {
            return Err(invalid("byte length must equal width*height*3"));
        }
        Self::from_rows(bytes, width, height, row)
    }
    /// Bind RGB rows with an explicit byte pitch, allowing trailing backing bytes.
    /// Validate positive dimensions, pitch >= width*3 and sufficient reachable bytes.
    pub fn from_rows(
        bytes: &'a [u8],
        width: usize,
        height: usize,
        row_stride: usize,
    ) -> Result<Self, OpsError> {
        if width == 0 || height == 0 {
            return Err(invalid("frame dimensions must be positive"));
        }
        let row = width
            .checked_mul(3)
            .ok_or_else(|| invalid("frame row overflow"))?;
        let required = (height - 1)
            .checked_mul(row_stride)
            .and_then(|x| x.checked_add(row))
            .ok_or_else(|| invalid("frame extent overflow"))?;
        if row_stride < row || required > bytes.len() {
            return Err(invalid("insufficient row pitch or backing bytes"));
        }
        Ok(Self {
            bytes,
            width,
            height,
            row_stride,
        })
    }
    /// Frame width in pixels, excluding row padding.
    pub const fn width(&self) -> usize {
        self.width
    }
    /// Frame height in pixels.
    pub const fn height(&self) -> usize {
        self.height
    }
    /// Return a borrowed rectangular crop without copying pixels. Coordinates use
    /// top-left origin and positive width/height. Out-of-bounds/empty crops fail.
    pub fn crop(&self, x: usize, y: usize, width: usize, height: usize) -> Result<Self, OpsError> {
        if width == 0
            || height == 0
            || x.checked_add(width).is_none_or(|end| end > self.width)
            || y.checked_add(height).is_none_or(|end| end > self.height)
        {
            return Err(invalid("crop outside frame or empty"));
        }
        let offset = y * self.row_stride + x * 3;
        Self::from_rows(&self.bytes[offset..], width, height, self.row_stride)
    }
    /// Fused bilinear resize, RGB8/255 conversion and per-channel (x-mean)/std.
    ///
    /// Output is f32 `[1,3,height,width]`, C-contiguous. Half-pixel coordinates are
    /// clamped to edge pixels; interpolation is computed in f64 before conversion.
    /// No full-resolution floating intermediate is allocated. Cost O(output pixels),
    /// memory O(3*output pixels). Reject zero output dimensions, invalid normalization
    /// or nonfinite normalized results. See the [module example](self) for exact values.
    pub fn resize_nchw(
        &self,
        size: [usize; 2],
        mean: [f32; 3],
        std: [f32; 3],
    ) -> Result<Tensor<f32>, OpsError> {
        let [height, width] = size;
        if height == 0 || width == 0 {
            return Err(invalid("output dimensions must be positive"));
        }
        if mean.iter().any(|v| !v.is_finite()) || std.iter().any(|v| !v.is_finite() || *v <= 0.0) {
            return Err(invalid("mean must be finite and std positive finite"));
        }
        let mut output = Tensor::filled(Kahen::new([1, 3, height, width])?, Order::C, 4, 0.0)?;
        let out = output.storage_slice_mut();
        let plane = height * width;
        for oy in 0..height {
            let sy = ((oy as f64 + 0.5) * self.height as f64 / height as f64 - 0.5)
                .clamp(0.0, (self.height - 1) as f64);
            let y0 = sy as usize;
            let y1 = (y0 + 1).min(self.height - 1);
            let fy = sy - y0 as f64;
            for ox in 0..width {
                let sx = ((ox as f64 + 0.5) * self.width as f64 / width as f64 - 0.5)
                    .clamp(0.0, (self.width - 1) as f64);
                let x0 = sx as usize;
                let x1 = (x0 + 1).min(self.width - 1);
                let fx = sx - x0 as f64;
                for c in 0..3 {
                    let at = |y, x| self.bytes[y * self.row_stride + x * 3 + c] as f64;
                    let upper = at(y0, x0) * (1.0 - fx) + at(y0, x1) * fx;
                    let lower = at(y1, x0) * (1.0 - fx) + at(y1, x1) * fx;
                    let value =
                        (((upper * (1.0 - fy) + lower * fy) / 255.0) as f32 - mean[c]) / std[c];
                    if !value.is_finite() {
                        return Err(invalid("normalized value outside f32 range"));
                    }
                    out[c * plane + oy * width + ox] = value;
                }
            }
        }
        Ok(output)
    }
}
