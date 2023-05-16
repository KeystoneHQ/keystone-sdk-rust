use alloc::format;
#[macro_export]
macro_rules! impl_ur_try_from_cbor_bytes {
    ($name: ident) => {
        impl TryFrom<Bytes> for $name {
            type Error = URError;
            fn try_from(value: Bytes) -> URResult<Self> {
                minicbor::decode(&value).map_err(|e| URError::CborDecodeError(e.to_string()))
            }
        }
    };
}

#[macro_export]
macro_rules! impl_ur_try_into_cbor_bytes {
    ($name: ident) => {
        impl TryInto<Bytes> for $name {
            type Error = URError;

            fn try_into(self) -> URResult<Bytes> {
                minicbor::to_vec(self.clone()).map_err(|e| URError::CborDecodeError(e.to_string()))
            }
        }
    };
}

#[macro_export]
macro_rules! impl_from_into_cbor_bytes {
    ($name: ident) => {
        impl_ur_try_from_cbor_bytes!($name);
        impl_ur_try_into_cbor_bytes!($name);
    };
}

use paste::paste;

#[macro_export]
macro_rules! impl_template_struct {
    ($name: ident { $($field: ident: $t: ty), *}) => {
        #[derive(Debug, Clone, Default)]
        pub struct $name {
            $(
              $field: $t,
            )*
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
