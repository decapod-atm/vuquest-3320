#[macro_export]
macro_rules! modifier_command {
    (
        $(#[$doc:meta])+
        $cmd:ident: $prefix:literal { $($field:ident: $field_ty:ident$(,)?)+ }$(,)?) => {
        paste::paste! {
            $(#[$doc])+
            #[derive(Clone, Copy, Debug, Eq, PartialEq)]
            pub struct $cmd {
                $($field: Option<$field_ty>,)+
            }

            impl $cmd {
                #[doc = "Creates a new [" $cmd "]."]
                pub const fn new() -> Self {
                    Self {
                        $($field: None,)+
                    }
                }

                #[doc = "Gets the ASCII serial command code for [" $cmd "]."]
                pub fn command(&self) -> alloc::string::String {
                    format!("{self}")
                }

                #[doc = "Gets the command ASCII prefix for [" $cmd "]."]
                pub const fn prefix() -> &'static str {
                    $prefix
                }
            }

            impl Default for $cmd {
                fn default() -> Self {
                    Self::new()
                }
            }

            impl TryFrom<&str> for $cmd {
                type Error = $crate::result::Error;

                fn try_from(val: &str) -> $crate::result::Result<Self> {
                    let rem = val.strip_prefix($prefix).ok_or($crate::result::Error::InvalidVariant)?;

                    Ok(Self {
                        $($field: $field_ty::try_from(rem).ok(),)+
                    })
                }
            }

            impl core::fmt::Display for $cmd {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    write!(f, "{}", $prefix)?;
                    $(
                        write!(f, "{}", self.$field.map(|c| alloc::string::String::from(c.command())).unwrap_or_default())?;
                    )+
                    Ok(())
                }
            }
        }
    }
}

/// Helper macro to define command modifier field access.
#[macro_export]
macro_rules! modifier_field {
    ($cmd:ident, $field:ident: $field_ty:ty, [$($not_field:ident$(,)?)+]$(,)?) => {
        paste::paste! {
            impl $cmd {
                #[doc = "Gets the [" $field_ty "] for [" $cmd "]."]
                pub const fn $field(&self) -> Option<$field_ty> {
                    self.$field
                }

                #[doc = "Sets the [" $field_ty "] for [" $cmd "]."]
                pub fn [<set_ $field>](&mut self, val: $field_ty) {
                    self.$field = Some(val);
                }

                #[doc = "Unsets the [" $field_ty "] for [" $cmd "]."]
                pub fn [<unset_ $field>](&mut self) {
                    self.$field = None;
                }

                #[doc = "Builder function that sets the [" $field_ty "] for [" $cmd "]."]
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
