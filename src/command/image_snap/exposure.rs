use alloc::string::String;

use crate::result::{Error, Result};

const EXP_MIN: u32 = 1;
const EXP_MIN_MS: u32 = 127;
const EXP_MAX_GEN6: u32 = 7874;
const EXP_MAX_GEN7: u32 = 10_000_000;
const MS_PER_UNIT: u32 = 127;
const EXP_MAX_MS_GEN6: u32 = EXP_MAX_GEN6 * MS_PER_UNIT;
const EXP_MAX_MS_GEN7: u32 = EXP_MAX_GEN7 * MS_PER_UNIT;
const EXP_SUFFIX: &str = "E";

/// Represents the image snap exposure settings (in units of 127 microseconds).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Exposure {
    units: u32,
}

impl Exposure {
    /// Creates a new [Exposure].
    pub const fn new() -> Self {
        Self {
            units: EXP_MAX_GEN6,
        }
    }

    /// Creates a new [Exposure] (GEN 7).
    pub const fn new_gen7() -> Self {
        Self {
            units: EXP_MAX_GEN7,
        }
    }

    /// Gets the number of millisecond units (127 ms / unit) for the [Exposure].
    pub const fn units(&self) -> u32 {
        self.units
    }

    /// Gets the number of milliseconds  for the [Exposure].
    pub const fn ms(&self) -> u32 {
        self.units * MS_PER_UNIT
    }

    /// Gets the maximum number of milliseconds for an [Exposure] (GEN 6).
    pub const fn max_ms_gen6() -> u32 {
        EXP_MAX_MS_GEN6
    }

    /// Gets the maximum number of milliseconds for an [Exposure] (GEN 7).
    pub const fn max_ms_gen7() -> u32 {
        EXP_MAX_MS_GEN7
    }

    /// Attempts to create an [Exposure] from milliseconds parameter.
    pub const fn try_from_ms(ms: u32) -> Result<Self> {
        match ms {
            m if m >= EXP_MIN_MS && m <= EXP_MAX_MS_GEN7 => Ok(Self {
                units: m.saturating_div(MS_PER_UNIT),
            }),
            _ => Err(Error::InvalidVariant),
        }
    }

    /// Attempts to create an [Exposure] from units parameter (127ms per unit).
    pub const fn try_from_unit(units: u32) -> Result<Self> {
        match units {
            u if u >= EXP_MIN && u <= EXP_MAX_GEN7 => Ok(Self { units }),
            _ => Err(Error::InvalidVariant),
        }
    }

    /// Gets the ASCII serial command code for [Exposure].
    pub fn command(&self) -> String {
        let units = self.units;
        format!("{units}{EXP_SUFFIX}")
    }
}

impl Default for Exposure {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<u32> for Exposure {
    type Error = Error;

    fn try_from(val: u32) -> Result<Self> {
        Self::try_from_unit(val)
    }
}

impl TryFrom<&str> for Exposure {
    type Error = Error;

    fn try_from(val: &str) -> Result<Self> {
        let pos = val.find(EXP_SUFFIX).ok_or(Error::InvalidVariant)?;
        let exp_start = val[..pos]
            .rfind(|c: char| c.is_ascii_uppercase() || c == '%')
            .map(|s| s + 1)
            .unwrap_or(0);

        val[exp_start..pos]
            .parse::<u32>()
            .map_err(|_| Error::InvalidVariant)
            .and_then(|u| Self::try_from_unit(u))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        (EXP_MIN..=EXP_MAX_GEN7)
            .zip((EXP_MIN_MS..=EXP_MAX_MS_GEN7).step_by(MS_PER_UNIT as usize))
            .for_each(|(units, ms)| {
                let exp_exposure = Exposure { units };

                assert_eq!(Exposure::try_from_unit(units), Ok(exp_exposure));
                assert_eq!(Exposure::try_from_ms(ms), Ok(exp_exposure));
                assert_eq!(exp_exposure.units(), units);
                assert_eq!(exp_exposure.ms(), ms);
            });
    }

    #[test]
    fn test_invalid() {
        [0].into_iter()
            .chain((EXP_MAX_GEN7 + 1)..=(EXP_MAX_GEN7 + 1024))
            .chain([u32::MAX])
            .for_each(|units| {
                assert_eq!(Exposure::try_from_unit(units), Err(Error::InvalidVariant));
                assert_eq!(
                    Exposure::try_from_ms(units.saturating_mul(MS_PER_UNIT)),
                    Err(Error::InvalidVariant)
                );
            });
    }
}
