use alloc::string::{String, ToString};

use crate::result::{Error, Result};

mod compensation;
mod edge_sharpen;
mod gamma_correction;
mod histogram_stretch;
mod image_rotate;
mod infinity_filter;
mod invert_image;
mod jpeg_image_quality;
mod noise_reduction;
mod pixel_depth;
mod pixel_ship;
mod protocol;

pub use compensation::*;
pub use edge_sharpen::*;
pub use gamma_correction::*;
pub use histogram_stretch::*;
pub use image_rotate::*;
pub use infinity_filter::*;
pub use invert_image::*;
pub use jpeg_image_quality::*;
pub use noise_reduction::*;
pub use pixel_depth::*;
pub use pixel_ship::*;
pub use protocol::*;

const IMAGE_SHIP: &str = "IMGSHP";

/// Configure all barcode `Image Ship` encodings.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ImageShip {
    infinity_filter: Option<InfinityFilter>,
    compensation: Option<Compensation>,
    pixel_depth: Option<PixelDepth>,
    edge_sharpen: Option<EdgeSharpen>,
    histogram_stretch: Option<HistogramStretch>,
    invert_image: Option<InvertImage>,
    noise_reduction: Option<NoiseReduction>,
    image_rotate: Option<ImageRotate>,
    jpeg_image_quality: Option<JpegImageQuality>,
    gamma_correction: Option<GammaCorrection>,
    protocol: Option<Protocol>,
    pixel_ship: Option<PixelShip>,
}

macro_rules! image_ship_field {
    ($field:ident: $field_ty:ty) => {
        paste::paste! {
            impl ImageShip {
                #[doc = "Gets the [" $field_ty "] for [ImageShip]."]
                pub const fn $field(&self) -> Option<$field_ty> {
                    self.$field
                }

                #[doc = "Sets the [" $field_ty "] for [ImageShip]."]
                pub fn [<set_ $field>](&mut self, val: $field_ty) {
                    self.$field = Some(val);
                }

                #[doc = "Unsets the [" $field_ty "] for [ImageShip]."]
                pub fn [<unset_ $field>](&mut self) {
                    self.$field = None;
                }
            }
        }
    };
}

image_ship_field!(infinity_filter: InfinityFilter);
image_ship_field!(compensation: Compensation);
image_ship_field!(pixel_depth: PixelDepth);
image_ship_field!(edge_sharpen: EdgeSharpen);
image_ship_field!(histogram_stretch: HistogramStretch);
image_ship_field!(invert_image: InvertImage);
image_ship_field!(noise_reduction: NoiseReduction);
image_ship_field!(image_rotate: ImageRotate);
image_ship_field!(jpeg_image_quality: JpegImageQuality);
image_ship_field!(gamma_correction: GammaCorrection);
image_ship_field!(protocol: Protocol);
image_ship_field!(pixel_ship: PixelShip);

impl ImageShip {
    /// Creates a new [ImageShip].
    pub const fn new() -> Self {
        Self {
            infinity_filter: None,
            compensation: None,
            pixel_depth: None,
            edge_sharpen: None,
            histogram_stretch: None,
            invert_image: None,
            noise_reduction: None,
            image_rotate: None,
            jpeg_image_quality: None,
            gamma_correction: None,
            protocol: None,
            pixel_ship: None,
        }
    }

    /// Builder function that sets the [InfinityFilter].
    pub const fn with_infinity_filter(self, val: InfinityFilter) -> Self {
        Self {
            infinity_filter: Some(val),
            compensation: self.compensation,
            pixel_depth: self.pixel_depth,
            edge_sharpen: self.edge_sharpen,
            histogram_stretch: self.histogram_stretch,
            invert_image: self.invert_image,
            noise_reduction: self.noise_reduction,
            image_rotate: self.image_rotate,
            jpeg_image_quality: self.jpeg_image_quality,
            gamma_correction: self.gamma_correction,
            protocol: self.protocol,
            pixel_ship: self.pixel_ship,
        }
    }

    /// Builder function that sets the [Compensation].
    pub const fn with_compensation(self, val: Compensation) -> Self {
        Self {
            infinity_filter: self.infinity_filter,
            compensation: Some(val),
            pixel_depth: self.pixel_depth,
            edge_sharpen: self.edge_sharpen,
            histogram_stretch: self.histogram_stretch,
            invert_image: self.invert_image,
            noise_reduction: self.noise_reduction,
            image_rotate: self.image_rotate,
            jpeg_image_quality: self.jpeg_image_quality,
            gamma_correction: self.gamma_correction,
            protocol: self.protocol,
            pixel_ship: self.pixel_ship,
        }
    }

    /// Builder function that sets the [PixelDepth].
    pub const fn with_pixel_depth(self, val: PixelDepth) -> Self {
        Self {
            infinity_filter: self.infinity_filter,
            compensation: self.compensation,
            pixel_depth: Some(val),
            edge_sharpen: self.edge_sharpen,
            histogram_stretch: self.histogram_stretch,
            invert_image: self.invert_image,
            noise_reduction: self.noise_reduction,
            image_rotate: self.image_rotate,
            jpeg_image_quality: self.jpeg_image_quality,
            gamma_correction: self.gamma_correction,
            protocol: self.protocol,
            pixel_ship: self.pixel_ship,
        }
    }

    /// Builder function that sets the [EdgeSharpen].
    pub const fn with_edge_sharpen(self, val: EdgeSharpen) -> Self {
        Self {
            infinity_filter: self.infinity_filter,
            compensation: self.compensation,
            pixel_depth: self.pixel_depth,
            edge_sharpen: Some(val),
            histogram_stretch: self.histogram_stretch,
            invert_image: self.invert_image,
            noise_reduction: self.noise_reduction,
            image_rotate: self.image_rotate,
            jpeg_image_quality: self.jpeg_image_quality,
            gamma_correction: self.gamma_correction,
            protocol: self.protocol,
            pixel_ship: self.pixel_ship,
        }
    }

    /// Builder function that sets the [HistogramStretch].
    pub const fn with_histogram_stretch(self, val: HistogramStretch) -> Self {
        Self {
            infinity_filter: self.infinity_filter,
            compensation: self.compensation,
            pixel_depth: self.pixel_depth,
            edge_sharpen: self.edge_sharpen,
            histogram_stretch: Some(val),
            invert_image: self.invert_image,
            noise_reduction: self.noise_reduction,
            image_rotate: self.image_rotate,
            jpeg_image_quality: self.jpeg_image_quality,
            gamma_correction: self.gamma_correction,
            protocol: self.protocol,
            pixel_ship: self.pixel_ship,
        }
    }

    /// Builder function that sets the [InvertImage].
    pub const fn with_invert_image(self, val: InvertImage) -> Self {
        Self {
            infinity_filter: self.infinity_filter,
            compensation: self.compensation,
            pixel_depth: self.pixel_depth,
            edge_sharpen: self.edge_sharpen,
            histogram_stretch: self.histogram_stretch,
            invert_image: Some(val),
            noise_reduction: self.noise_reduction,
            image_rotate: self.image_rotate,
            jpeg_image_quality: self.jpeg_image_quality,
            gamma_correction: self.gamma_correction,
            protocol: self.protocol,
            pixel_ship: self.pixel_ship,
        }
    }

    /// Builder function that sets the [NoiseReduction].
    pub const fn with_noise_reduction(self, val: NoiseReduction) -> Self {
        Self {
            infinity_filter: self.infinity_filter,
            compensation: self.compensation,
            pixel_depth: self.pixel_depth,
            edge_sharpen: self.edge_sharpen,
            histogram_stretch: self.histogram_stretch,
            invert_image: self.invert_image,
            noise_reduction: Some(val),
            image_rotate: self.image_rotate,
            jpeg_image_quality: self.jpeg_image_quality,
            gamma_correction: self.gamma_correction,
            protocol: self.protocol,
            pixel_ship: self.pixel_ship,
        }
    }

    /// Builder function that sets the [ImageRotate].
    pub const fn with_image_rotate(self, val: ImageRotate) -> Self {
        Self {
            infinity_filter: self.infinity_filter,
            compensation: self.compensation,
            pixel_depth: self.pixel_depth,
            edge_sharpen: self.edge_sharpen,
            histogram_stretch: self.histogram_stretch,
            invert_image: self.invert_image,
            noise_reduction: self.noise_reduction,
            image_rotate: Some(val),
            jpeg_image_quality: self.jpeg_image_quality,
            gamma_correction: self.gamma_correction,
            protocol: self.protocol,
            pixel_ship: self.pixel_ship,
        }
    }

    /// Builder function that sets the [JpegImageQuality].
    pub const fn with_jpeg_image_quality(self, val: JpegImageQuality) -> Self {
        Self {
            infinity_filter: self.infinity_filter,
            compensation: self.compensation,
            pixel_depth: self.pixel_depth,
            edge_sharpen: self.edge_sharpen,
            histogram_stretch: self.histogram_stretch,
            invert_image: self.invert_image,
            noise_reduction: self.noise_reduction,
            image_rotate: self.image_rotate,
            jpeg_image_quality: Some(val),
            gamma_correction: self.gamma_correction,
            protocol: self.protocol,
            pixel_ship: self.pixel_ship,
        }
    }

    /// Builder function that sets the [GammaCorrection].
    pub const fn with_gamma_correction(self, val: GammaCorrection) -> Self {
        Self {
            infinity_filter: self.infinity_filter,
            compensation: self.compensation,
            pixel_depth: self.pixel_depth,
            edge_sharpen: self.edge_sharpen,
            histogram_stretch: self.histogram_stretch,
            invert_image: self.invert_image,
            noise_reduction: self.noise_reduction,
            image_rotate: self.image_rotate,
            jpeg_image_quality: self.jpeg_image_quality,
            gamma_correction: Some(val),
            protocol: self.protocol,
            pixel_ship: self.pixel_ship,
        }
    }

    /// Builder function that sets the [Protocol].
    pub const fn with_protocol(self, val: Protocol) -> Self {
        Self {
            infinity_filter: self.infinity_filter,
            compensation: self.compensation,
            pixel_depth: self.pixel_depth,
            edge_sharpen: self.edge_sharpen,
            histogram_stretch: self.histogram_stretch,
            invert_image: self.invert_image,
            noise_reduction: self.noise_reduction,
            image_rotate: self.image_rotate,
            jpeg_image_quality: self.jpeg_image_quality,
            gamma_correction: self.gamma_correction,
            protocol: Some(val),
            pixel_ship: self.pixel_ship,
        }
    }

    /// Builder function that sets the [PixelShip].
    pub const fn with_pixel_ship(self, val: PixelShip) -> Self {
        Self {
            infinity_filter: self.infinity_filter,
            compensation: self.compensation,
            pixel_depth: self.pixel_depth,
            edge_sharpen: self.edge_sharpen,
            histogram_stretch: self.histogram_stretch,
            invert_image: self.invert_image,
            noise_reduction: self.noise_reduction,
            image_rotate: self.image_rotate,
            jpeg_image_quality: self.jpeg_image_quality,
            gamma_correction: self.gamma_correction,
            protocol: self.protocol,
            pixel_ship: Some(val),
        }
    }

    /// Gets the ASCII serial command code for [ImageShip].
    pub fn command(&self) -> String {
        let infinity = self
            .infinity_filter
            .map(|v| v.command().to_string())
            .unwrap_or_default();

        let comp = self
            .compensation
            .map(|v| v.command().to_string())
            .unwrap_or_default();

        let depth = self
            .pixel_depth
            .map(|v| v.command().to_string())
            .unwrap_or_default();

        let edge = self
            .edge_sharpen
            .map(|v| v.command().to_string())
            .unwrap_or_default();

        let histo = self
            .histogram_stretch
            .map(|v| v.command().to_string())
            .unwrap_or_default();

        let invert = self
            .invert_image
            .map(|v| v.command().to_string())
            .unwrap_or_default();

        let noise = self
            .noise_reduction
            .map(|v| v.command().to_string())
            .unwrap_or_default();

        let rotate = self
            .image_rotate
            .map(|v| v.command().to_string())
            .unwrap_or_default();

        let jpeg = self
            .jpeg_image_quality
            .map(|v| v.command().to_string())
            .unwrap_or_default();

        let gamma = self
            .gamma_correction
            .map(|v| v.command().to_string())
            .unwrap_or_default();

        let proto = self
            .protocol
            .map(|v| v.command().to_string())
            .unwrap_or_default();

        let pixel = self
            .pixel_ship
            .map(|v| v.command().to_string())
            .unwrap_or_default();

        format!(
            "{IMAGE_SHIP}{infinity}{comp}{depth}{edge}{histo}{invert}{noise}{rotate}{jpeg}{gamma}{proto}{pixel}"
        )
    }
}

impl Default for ImageShip {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<&str> for ImageShip {
    type Error = Error;

    fn try_from(val: &str) -> Result<Self> {
        let rem = val.strip_prefix(IMAGE_SHIP).ok_or(Error::InvalidVariant)?;
        let infinity_filter = InfinityFilter::try_from(rem).ok();
        let compensation = Compensation::try_from(rem).ok();
        let pixel_depth = PixelDepth::try_from(rem).ok();
        let edge_sharpen = EdgeSharpen::try_from(rem).ok();
        let histogram_stretch = HistogramStretch::try_from(rem).ok();
        let invert_image = InvertImage::try_from(rem).ok();
        let noise_reduction = NoiseReduction::try_from(rem).ok();
        let image_rotate = ImageRotate::try_from(rem).ok();
        let jpeg_image_quality = JpegImageQuality::try_from(rem).ok();
        let gamma_correction = GammaCorrection::try_from(rem).ok();
        let protocol = Protocol::try_from(rem).ok();
        let pixel_ship = PixelShip::try_from(rem).ok();

        Ok(Self {
            infinity_filter,
            compensation,
            pixel_depth,
            edge_sharpen,
            histogram_stretch,
            invert_image,
            noise_reduction,
            image_rotate,
            jpeg_image_quality,
            gamma_correction,
            protocol,
            pixel_ship,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_image_ship_field {
        ($img:ident, $field:ident, $field_val:expr) => {
            paste::paste! {
                assert!($img.$field().is_none());

                $img.[<set_ $field>]($field_val);
                assert_eq!($img.$field(), Some($field_val));

                $img.[<unset_ $field>]();
                assert!($img.$field().is_none());
            }
        };
    }

    #[test]
    fn test_valid() {
        let exp_infinity_filter = InfinityFilter::new();
        let exp_compensation = Compensation::new();
        let exp_pixel_depth = PixelDepth::new();
        let exp_histogram_stretch = HistogramStretch::new();
        let exp_invert_image = InvertImage::new();
        let exp_noise_reduction = NoiseReduction::new();
        let exp_image_rotate = ImageRotate::new();
        let exp_jpeg_image_quality = JpegImageQuality::new();
        let exp_gamma_correction = GammaCorrection::new();
        let exp_protocol = Protocol::new();
        let exp_pixel_ship = PixelShip::new();

        [
            "", "0A", "0C", "8D", "0H", "1ix", "0if", "0ir", "50J", "0K", "0P", "1S",
        ]
        .into_iter()
        .map(|s| format!("{IMAGE_SHIP}{s}"))
        .zip([
            ImageShip::new(),
            ImageShip::new().with_infinity_filter(exp_infinity_filter),
            ImageShip::new().with_compensation(exp_compensation),
            ImageShip::new().with_pixel_depth(exp_pixel_depth),
            ImageShip::new().with_histogram_stretch(exp_histogram_stretch),
            ImageShip::new().with_invert_image(exp_invert_image),
            ImageShip::new().with_noise_reduction(exp_noise_reduction),
            ImageShip::new().with_image_rotate(exp_image_rotate),
            ImageShip::new().with_jpeg_image_quality(exp_jpeg_image_quality),
            ImageShip::new().with_gamma_correction(exp_gamma_correction),
            ImageShip::new().with_protocol(exp_protocol),
            ImageShip::new().with_pixel_ship(exp_pixel_ship),
        ])
        .for_each(|(img_str, exp_img_ship)| {
            assert_eq!(ImageShip::try_from(img_str.as_str()), Ok(exp_img_ship));
            assert_eq!(exp_img_ship.command(), img_str);
        });

        let mut img = ImageShip::new();

        test_image_ship_field!(img, infinity_filter, exp_infinity_filter);
        test_image_ship_field!(img, compensation, exp_compensation);
        test_image_ship_field!(img, pixel_depth, exp_pixel_depth);
        test_image_ship_field!(img, histogram_stretch, exp_histogram_stretch);
        test_image_ship_field!(img, invert_image, exp_invert_image);
        test_image_ship_field!(img, noise_reduction, exp_noise_reduction);
        test_image_ship_field!(img, image_rotate, exp_image_rotate);
        test_image_ship_field!(img, jpeg_image_quality, exp_jpeg_image_quality);
        test_image_ship_field!(img, gamma_correction, exp_gamma_correction);
        test_image_ship_field!(img, protocol, exp_protocol);
        test_image_ship_field!(img, pixel_ship, exp_pixel_ship);
    }
}
