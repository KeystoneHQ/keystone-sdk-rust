#[macro_export]
macro_rules! impl_ur_try_from_cbor_bytes {
    ($name: ident) => {
        impl TryFrom<Vec<u8>> for $name {
            type Error = URError;
            fn try_from(value: Vec<u8>) -> URResult<Self> {
                minicbor::decode(&value).map_err(|e| URError::CborDecodeError(e.to_string()))
            }
        }
    };
}

#[macro_export]
macro_rules! impl_ur_try_into_cbor_bytes {
    ($name: ident) => {
        impl TryInto<Vec<u8>> for $name {
            type Error = URError;

            fn try_into(self) -> URResult<Vec<u8>> {
                minicbor::to_vec(self.clone()).map_err(|e| URError::CborDecodeError(e.to_string()))
            }
        }
    };
}

#[macro_export]
macro_rules! impl_cbor_bytes {
    ($($name: ident,) *) => {
        $(
         impl_ur_try_from_cbor_bytes!($name);
         impl_ur_try_into_cbor_bytes!($name);
        )*
    };
}

#[macro_export]
macro_rules! impl_template_struct {
    ($name: ident { $($field: ident: $t: ty), *}) => {
        #[derive(Debug, Clone, Default)]
        pub struct $name {
            $(
              $field: $t,
            )*
        }

        impl $name {
            pub fn new($($field: $t), *) -> Self {
                Self {
                    $(
                        $field
                    ), *
                }
            }
        }

        paste::item! {
            impl $name {
                $(
                    pub fn [<get_ $field>](&self) -> $t {
                        self.$field.clone()
                    }
                    pub fn [<set_ $field>](&mut self, $field: $t) {
                        self.$field = $field
                    }
                )*
            }
        }
    }
}
