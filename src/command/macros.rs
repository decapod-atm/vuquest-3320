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
