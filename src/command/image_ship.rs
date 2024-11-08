use alloc::string::{String, ToString};

use crate::result::{Error, Result};

mod compensation;
mod edge_sharpen;
mod infinity_filter;
mod pixel_depth;

pub use compensation::*;
pub use edge_sharpen::*;
pub use infinity_filter::*;
pub use pixel_depth::*;

const IMAGE_SHIP: &str = "IMGSHP";

/// Configure all barcode `Image Ship` encodings.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ImageShip {
    infinity_filter: Option<InfinityFilter>,
    compensation: Option<Compensation>,
    pixel_depth: Option<PixelDepth>,
    edge_sharpen: Option<EdgeSharpen>,
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

impl ImageShip {
    /// Creates a new [ImageShip].
    pub const fn new() -> Self {
        Self {
            infinity_filter: None,
            compensation: None,
            pixel_depth: None,
            edge_sharpen: None,
        }
    }

    /// Builder function that sets the [InfinityFilter].
    pub const fn with_infinity_filter(self, val: InfinityFilter) -> Self {
        Self {
            infinity_filter: Some(val),
            compensation: self.compensation,
            pixel_depth: self.pixel_depth,
            edge_sharpen: self.edge_sharpen,
        }
    }

    /// Builder function that sets the [Compensation].
    pub const fn with_compensation(self, val: Compensation) -> Self {
        Self {
            infinity_filter: self.infinity_filter,
            compensation: Some(val),
            pixel_depth: self.pixel_depth,
            edge_sharpen: self.edge_sharpen,
        }
    }

    /// Builder function that sets the [PixelDepth].
    pub const fn with_pixel_depth(self, val: PixelDepth) -> Self {
        Self {
            infinity_filter: self.infinity_filter,
            compensation: self.compensation,
            pixel_depth: Some(val),
            edge_sharpen: self.edge_sharpen,
        }
    }

    /// Builder function that sets the [EdgeSharpen].
    pub const fn with_edge_sharpen(self, val: EdgeSharpen) -> Self {
        Self {
            infinity_filter: self.infinity_filter,
            compensation: self.compensation,
            pixel_depth: self.pixel_depth,
            edge_sharpen: Some(val),
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

        format!("{IMAGE_SHIP}{infinity}{comp}{depth}{edge}")
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

        Ok(Self {
            infinity_filter,
            compensation,
            pixel_depth,
            edge_sharpen,
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

        ["", "0A", "0C", "8D"]
            .into_iter()
            .map(|s| format!("{IMAGE_SHIP}{s}"))
            .zip([
                ImageShip::new(),
                ImageShip::new().with_infinity_filter(exp_infinity_filter),
                ImageShip::new().with_compensation(exp_compensation),
                ImageShip::new().with_pixel_depth(exp_pixel_depth),
            ])
            .for_each(|(img_str, exp_img_ship)| {
                assert_eq!(ImageShip::try_from(img_str.as_str()), Ok(exp_img_ship));
                assert_eq!(exp_img_ship.command(), img_str);
            });

        let mut img = ImageShip::new();

        test_image_ship_field!(img, infinity_filter, exp_infinity_filter);
        test_image_ship_field!(img, compensation, exp_compensation);
    }
}
