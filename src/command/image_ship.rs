use alloc::string::{String, ToString};

use crate::result::{Error, Result};

mod compensation;
mod infinity_filter;

pub use compensation::*;
pub use infinity_filter::*;

const IMAGE_SHIP: &str = "IMGSHP";

/// Configure all barcode `Image Ship` encodings.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ImageShip {
    infinity_filter: Option<InfinityFilter>,
    compensation: Option<Compensation>,
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

impl ImageShip {
    /// Creates a new [ImageShip].
    pub const fn new() -> Self {
        Self {
            infinity_filter: None,
            compensation: None,
        }
    }

    /// Builder function that sets the [InfinityFilter].
    pub const fn with_infinity_filter(self, val: InfinityFilter) -> Self {
        Self {
            infinity_filter: Some(val),
            compensation: self.compensation,
        }
    }

    /// Builder function that sets the [Compensation].
    pub const fn with_compensation(self, val: Compensation) -> Self {
        Self {
            infinity_filter: self.infinity_filter,
            compensation: Some(val),
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

        format!("{IMAGE_SHIP}{infinity}{comp}")
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

        Ok(Self {
            infinity_filter,
            compensation,
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

        ["", "0A", "0C"]
            .into_iter()
            .map(|s| format!("{IMAGE_SHIP}{s}"))
            .zip([
                ImageShip::new(),
                ImageShip::new().with_infinity_filter(exp_infinity_filter),
                ImageShip::new().with_compensation(exp_compensation),
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
