//! Types and algorithms related to `Image Snap` configuration.

use alloc::string::{String, ToString};

use crate::result::{Error, Result};
use crate::modifier_field;

mod beeper;
mod delta_for_acceptance;
mod exposure;
mod gain;
mod imaging_style;
mod led;
mod target_set_point;
mod target_white_value;
mod update_tries;
mod wait_for_trigger;

pub use beeper::*;
pub use delta_for_acceptance::*;
pub use exposure::*;
pub use gain::*;
pub use imaging_style::*;
pub use led::*;
pub use target_set_point::*;
pub use target_white_value::*;
pub use update_tries::*;
pub use wait_for_trigger::*;

const IMAGE_SNAP: &str = "IMGSNP";

/// Configure all barcode `Image Snap` encodings.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ImageSnap {
    imaging_style: Option<ImagingStyle>,
    beeper: Option<Beeper>,
    wait_for_trigger: Option<WaitForTrigger>,
    led: Option<LED>,
    exposure: Option<Exposure>,
    gain: Option<Gain>,
    target_white_value: Option<TargetWhiteValue>,
    delta_for_acceptance: Option<DeltaForAcceptance>,
    update_tries: Option<UpdateTries>,
    target_set_point: Option<TargetSetPoint>,
}

modifier_field! {
    ImageSnap,
    imaging_style: ImagingStyle,
    [
        beeper,
        wait_for_trigger,
        led,
        exposure,
        gain,
        target_white_value,
        delta_for_acceptance,
        update_tries,
        target_set_point,
    ],
}

modifier_field! {
    ImageSnap,
    beeper: Beeper,
    [
        imaging_style,
        wait_for_trigger,
        led,
        exposure,
        gain,
        target_white_value,
        delta_for_acceptance,
        update_tries,
        target_set_point,
    ],
}

modifier_field! {
    ImageSnap,
    wait_for_trigger: WaitForTrigger,
    [
        imaging_style,
        beeper,
        led,
        exposure,
        gain,
        target_white_value,
        delta_for_acceptance,
        update_tries,
        target_set_point,
    ],
}

modifier_field! {
    ImageSnap,
    led: LED,
    [
        imaging_style,
        beeper,
        wait_for_trigger,
        exposure,
        gain,
        target_white_value,
        delta_for_acceptance,
        update_tries,
        target_set_point,
    ],
}

modifier_field! {
    ImageSnap,
    exposure: Exposure,
    [
        imaging_style,
        beeper,
        wait_for_trigger,
        led,
        gain,
        target_white_value,
        delta_for_acceptance,
        update_tries,
        target_set_point,
    ],
}

modifier_field! {
    ImageSnap,
    gain: Gain,
    [
        imaging_style,
        beeper,
        wait_for_trigger,
        led,
        exposure,
        target_white_value,
        delta_for_acceptance,
        update_tries,
        target_set_point,
    ],
}

modifier_field! {
    ImageSnap,
    target_white_value: TargetWhiteValue,
    [
        imaging_style,
        beeper,
        wait_for_trigger,
        led,
        exposure,
        gain,
        delta_for_acceptance,
        update_tries,
        target_set_point,
    ],
}

modifier_field! {
    ImageSnap,
    delta_for_acceptance: DeltaForAcceptance,
    [
        imaging_style,
        beeper,
        wait_for_trigger,
        led,
        exposure,
        gain,
        target_white_value,
        update_tries,
        target_set_point,
    ],
}

modifier_field! {
    ImageSnap,
    update_tries: UpdateTries,
    [
        imaging_style,
        beeper,
        wait_for_trigger,
        led,
        exposure,
        gain,
        target_white_value,
        delta_for_acceptance,
        target_set_point,
    ],
}

modifier_field! {
    ImageSnap,
    target_set_point: TargetSetPoint,
    [
        imaging_style,
        beeper,
        wait_for_trigger,
        led,
        exposure,
        gain,
        target_white_value,
        delta_for_acceptance,
        update_tries,
    ],
}

impl ImageSnap {
    /// Creates a new [ImageSnap].
    pub const fn new() -> Self {
        Self {
            imaging_style: None,
            beeper: None,
            wait_for_trigger: None,
            led: None,
            exposure: None,
            gain: None,
            target_white_value: None,
            delta_for_acceptance: None,
            update_tries: None,
            target_set_point: None,
        }
    }

    /// Gets the ASCII serial command code for [ImageSnap].
    pub fn command(&self) -> String {
        let imaging = self
            .imaging_style
            .map(|v| v.command().to_string())
            .unwrap_or_default();

        let beeper = self
            .beeper
            .map(|v| v.command().to_string())
            .unwrap_or_default();

        let wait_for_trigger = self
            .wait_for_trigger
            .map(|v| v.command().to_string())
            .unwrap_or_default();

        let led = self
            .led
            .map(|v| v.command().to_string())
            .unwrap_or_default();

        let exposure = self
            .exposure
            .map(|v| v.command().to_string())
            .unwrap_or_default();

        let gain = self
            .gain
            .map(|v| v.command().to_string())
            .unwrap_or_default();

        let twv = self
            .target_white_value
            .map(|v| v.command().to_string())
            .unwrap_or_default();

        let delta = self
            .delta_for_acceptance
            .map(|v| v.command().to_string())
            .unwrap_or_default();

        let update = self
            .update_tries
            .map(|v| v.command().to_string())
            .unwrap_or_default();

        let tsp = self
            .target_set_point
            .map(|v| v.command().to_string())
            .unwrap_or_default();

        format!("{IMAGE_SNAP}{imaging}{beeper}{wait_for_trigger}{led}{exposure}{gain}{twv}{delta}{update}{tsp}")
    }
}

impl Default for ImageSnap {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<&str> for ImageSnap {
    type Error = Error;

    fn try_from(val: &str) -> Result<Self> {
        let rem = val.strip_prefix(IMAGE_SNAP).ok_or(Error::InvalidVariant)?;
        let imaging_style = ImagingStyle::try_from(rem).ok();
        let beeper = Beeper::try_from(rem).ok();
        let wait_for_trigger = WaitForTrigger::try_from(rem).ok();
        let led = LED::try_from(rem).ok();
        let exposure = Exposure::try_from(rem).ok();
        let gain = Gain::try_from(rem).ok();
        let target_white_value = TargetWhiteValue::try_from(rem).ok();
        let delta_for_acceptance = DeltaForAcceptance::try_from(rem).ok();
        let update_tries = UpdateTries::try_from(rem).ok();
        let target_set_point = TargetSetPoint::try_from(rem).ok();

        Ok(Self {
            imaging_style,
            beeper,
            wait_for_trigger,
            led,
            exposure,
            gain,
            target_white_value,
            delta_for_acceptance,
            update_tries,
            target_set_point,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_image_snap_field {
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
        let exp_imaging_style = ImagingStyle::new();
        let exp_beeper = Beeper::new();
        let exp_wait_for_trigger = WaitForTrigger::new();
        let exp_led = LED::new();
        let exp_exposure = Exposure::new();
        let exp_gain = Gain::new();
        let exp_target_white_value = TargetWhiteValue::new();
        let exp_delta_for_acceptance = DeltaForAcceptance::new();
        let exp_update_tries = UpdateTries::new();
        let exp_target_set_point = TargetSetPoint::new();

        [
            "",
            "1P",
            "0B",
            "0T",
            "0L",
            "7874E",
            "1G",
            "125W",
            "25D",
            "6U",
            "50%",
            "1P0B0T0L7874E1G125W25D6U50%",
        ]
        .into_iter()
        .map(|s| format!("{IMAGE_SNAP}{s}"))
        .zip([
            ImageSnap::new(),
            ImageSnap::new().with_imaging_style(exp_imaging_style),
            ImageSnap::new().with_beeper(exp_beeper),
            ImageSnap::new().with_wait_for_trigger(exp_wait_for_trigger),
            ImageSnap::new().with_led(exp_led),
            ImageSnap::new().with_exposure(exp_exposure),
            ImageSnap::new().with_gain(exp_gain),
            ImageSnap::new().with_target_white_value(exp_target_white_value),
            ImageSnap::new().with_delta_for_acceptance(exp_delta_for_acceptance),
            ImageSnap::new().with_update_tries(exp_update_tries),
            ImageSnap::new().with_target_set_point(exp_target_set_point),
            ImageSnap::new()
                .with_imaging_style(exp_imaging_style)
                .with_beeper(exp_beeper)
                .with_wait_for_trigger(exp_wait_for_trigger)
                .with_led(exp_led)
                .with_exposure(exp_exposure)
                .with_gain(exp_gain)
                .with_target_white_value(exp_target_white_value)
                .with_delta_for_acceptance(exp_delta_for_acceptance)
                .with_update_tries(exp_update_tries)
                .with_target_set_point(exp_target_set_point),
        ])
        .for_each(|(img_str, exp_img_snap)| {
            assert_eq!(ImageSnap::try_from(img_str.as_str()), Ok(exp_img_snap));
            assert_eq!(exp_img_snap.command(), img_str);
        });

        let mut img = ImageSnap::new();

        test_image_snap_field!(img, imaging_style, exp_imaging_style);
        test_image_snap_field!(img, beeper, exp_beeper);
        test_image_snap_field!(img, wait_for_trigger, exp_wait_for_trigger);
        test_image_snap_field!(img, led, exp_led);
        test_image_snap_field!(img, exposure, exp_exposure);
        test_image_snap_field!(img, gain, exp_gain);
        test_image_snap_field!(img, target_white_value, exp_target_white_value);
        test_image_snap_field!(img, delta_for_acceptance, exp_delta_for_acceptance);
        test_image_snap_field!(img, update_tries, exp_update_tries);
        test_image_snap_field!(img, target_set_point, exp_target_set_point);
    }
}
