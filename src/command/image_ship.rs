use alloc::string::{String, ToString};

use crate::result::{Error, Result};

mod blur_image;
mod compensation;
mod document_filter;
mod edge_sharpen;
mod gamma_correction;
mod histogram_ship;
mod histogram_stretch;
mod image_rotate;
mod infinity_filter;
mod invert_image;
mod jpeg_image_quality;
mod noise_reduction;
mod pixel_depth;
mod pixel_ship;
mod protocol;

pub use blur_image::*;
pub use compensation::*;
pub use document_filter::*;
pub use edge_sharpen::*;
pub use gamma_correction::*;
pub use histogram_ship::*;
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
    document_filter: Option<DocumentFilter>,
    blur_image: Option<BlurImage>,
    histogram_ship: Option<HistogramShip>,
}

macro_rules! image_ship_field {
    ($field:ident: $field_ty:ty, [$($not_field:ident$(,)?)+]$(,)?) => {
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

                #[doc = "Builder function that sets the [" $field_ty "] for [ImageShip]."]
                pub const fn [<with_ $field>](self, val: $field_ty) -> Self {
                    Self {
                        $field: Some(val),
                        $($not_field: self.$not_field,)+
                    }
                }
            }
        }
    };
}

image_ship_field! {
    infinity_filter: InfinityFilter,
    [
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
        document_filter,
        blur_image,
        histogram_ship,
    ],
}

image_ship_field! {
    compensation: Compensation,
    [
        infinity_filter,
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
        document_filter,
        blur_image,
        histogram_ship,
    ],
}

image_ship_field! {
    pixel_depth: PixelDepth,
    [
        infinity_filter,
        compensation,
        edge_sharpen,
        histogram_stretch,
        invert_image,
        noise_reduction,
        image_rotate,
        jpeg_image_quality,
        gamma_correction,
        protocol,
        pixel_ship,
        document_filter,
        blur_image,
        histogram_ship,
    ],
}

image_ship_field! {
    edge_sharpen: EdgeSharpen,
    [
        infinity_filter,
        compensation,
        pixel_depth,
        histogram_stretch,
        invert_image,
        noise_reduction,
        image_rotate,
        jpeg_image_quality,
        gamma_correction,
        protocol,
        pixel_ship,
        document_filter,
        blur_image,
        histogram_ship,
    ],
}

image_ship_field! {
    histogram_stretch: HistogramStretch,
    [
        infinity_filter,
        compensation,
        pixel_depth,
        edge_sharpen,
        invert_image,
        noise_reduction,
        image_rotate,
        jpeg_image_quality,
        gamma_correction,
        protocol,
        pixel_ship,
        document_filter,
        blur_image,
        histogram_ship,
    ],
}

image_ship_field! {
    invert_image: InvertImage,
    [
        infinity_filter,
        compensation,
        pixel_depth,
        edge_sharpen,
        histogram_stretch,
        noise_reduction,
        image_rotate,
        jpeg_image_quality,
        gamma_correction,
        protocol,
        pixel_ship,
        document_filter,
        blur_image,
        histogram_ship,
    ],
}

image_ship_field! {
    noise_reduction: NoiseReduction,
    [
        infinity_filter,
        compensation,
        pixel_depth,
        edge_sharpen,
        histogram_stretch,
        invert_image,
        image_rotate,
        jpeg_image_quality,
        gamma_correction,
        protocol,
        pixel_ship,
        document_filter,
        blur_image,
        histogram_ship,
    ],
}

image_ship_field! {
    image_rotate: ImageRotate,
    [
        infinity_filter,
        compensation,
        pixel_depth,
        edge_sharpen,
        histogram_stretch,
        invert_image,
        noise_reduction,
        jpeg_image_quality,
        gamma_correction,
        protocol,
        pixel_ship,
        document_filter,
        blur_image,
        histogram_ship,
    ],
}

image_ship_field! {
    jpeg_image_quality: JpegImageQuality,
    [
        infinity_filter,
        compensation,
        pixel_depth,
        edge_sharpen,
        histogram_stretch,
        invert_image,
        noise_reduction,
        image_rotate,
        gamma_correction,
        protocol,
        pixel_ship,
        document_filter,
        blur_image,
        histogram_ship,
    ],
}

image_ship_field! {
    gamma_correction: GammaCorrection,
    [
        infinity_filter,
        compensation,
        pixel_depth,
        edge_sharpen,
        histogram_stretch,
        invert_image,
        noise_reduction,
        image_rotate,
        jpeg_image_quality,
        protocol,
        pixel_ship,
        document_filter,
        blur_image,
        histogram_ship,
    ],
}

image_ship_field! {
    protocol: Protocol,
    [
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
        pixel_ship,
        document_filter,
        blur_image,
        histogram_ship,
    ],
}

image_ship_field! {
    pixel_ship: PixelShip,
    [
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
        document_filter,
        blur_image,
        histogram_ship,
    ],
}

image_ship_field! {
    document_filter: DocumentFilter,
    [
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
        blur_image,
        histogram_ship,
    ],
}

image_ship_field! {
    blur_image: BlurImage,
    [
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
        document_filter,
        histogram_ship,
    ],
}

image_ship_field! {
    histogram_ship: HistogramShip,
    [
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
        document_filter,
        blur_image,
    ],
}

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
            document_filter: None,
            blur_image: None,
            histogram_ship: None,
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

        let histo_stretch = self
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

        let doc = self
            .document_filter
            .map(|v| v.command().to_string())
            .unwrap_or_default();

        let blur = self
            .blur_image
            .map(|v| v.command().to_string())
            .unwrap_or_default();

        let histo_ship = self
            .histogram_ship
            .map(|v| v.command().to_string())
            .unwrap_or_default();

        format!(
            "{IMAGE_SHIP}{infinity}{comp}{depth}{edge}{histo_stretch}{invert}{noise}{rotate}{jpeg}{gamma}{proto}{pixel}{doc}{blur}{histo_ship}"
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
        let document_filter = DocumentFilter::try_from(rem).ok();
        let blur_image = BlurImage::try_from(rem).ok();
        let histogram_ship = HistogramShip::try_from(rem).ok();

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
            document_filter,
            blur_image,
            histogram_ship,
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
        let exp_document_filter = DocumentFilter::new();
        let exp_blur_image = BlurImage::new();
        let exp_histogram_ship = HistogramShip::new();

        [
            "", "0A", "0C", "8D", "0H", "1ix", "0if", "0ir", "50J", "0K", "0P", "1S", "0U", "0V",
            "0W",
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
            ImageShip::new().with_document_filter(exp_document_filter),
            ImageShip::new().with_blur_image(exp_blur_image),
            ImageShip::new().with_histogram_ship(exp_histogram_ship),
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
        test_image_ship_field!(img, document_filter, exp_document_filter);
        test_image_ship_field!(img, blur_image, exp_blur_image);
        test_image_ship_field!(img, histogram_ship, exp_histogram_ship);
    }
}
