use crate::{modifier_command, modifier_field};

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

modifier_command! {
    /// Configure all barcode `Image Ship` encodings.
    ImageShip: "IMGSHP" {
        infinity_filter: InfinityFilter,
        compensation: Compensation,
        pixel_depth: PixelDepth,
        edge_sharpen: EdgeSharpen,
        histogram_stretch: HistogramStretch,
        invert_image: InvertImage,
        noise_reduction: NoiseReduction,
        image_rotate: ImageRotate,
        jpeg_image_quality: JpegImageQuality,
        gamma_correction: GammaCorrection,
        protocol: Protocol,
        pixel_ship: PixelShip,
        document_filter: DocumentFilter,
        blur_image: BlurImage,
        histogram_ship: HistogramShip,
    }
}

modifier_field! {
    ImageShip,
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

modifier_field! {
    ImageShip,
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

modifier_field! {
    ImageShip,
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

modifier_field! {
    ImageShip,
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

modifier_field! {
    ImageShip,
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

modifier_field! {
    ImageShip,
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

modifier_field! {
    ImageShip,
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

modifier_field! {
    ImageShip,
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

modifier_field! {
    ImageShip,
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

modifier_field! {
    ImageShip,
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

modifier_field! {
    ImageShip,
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

modifier_field! {
    ImageShip,
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

modifier_field! {
    ImageShip,
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

modifier_field! {
    ImageShip,
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

modifier_field! {
    ImageShip,
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
        let prefix = ImageShip::prefix();

        [
            "", "0A", "0C", "8D", "0H", "1ix", "0if", "0ir", "50J", "0K", "0P", "1S", "0U", "0V",
            "0W",
        ]
        .into_iter()
        .map(|s| format!("{prefix}{s}"))
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
