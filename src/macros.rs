#[macro_export]
macro_rules! boilerplate_flags {
    ($name:ident) => {
        impl From<$name> for u64 {
            fn from(value: $name) -> u64 {
                value.bits()
            }
        }
        
        impl serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_u64(self.bits())
            }
        }

        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let value = u64::deserialize(deserializer)?;
                Ok($name::from_bits_truncate(value))
            }
        }
    };
}

#[macro_export]
macro_rules! boilerplate_flags_as_u8 {
    ($name:ident) => {
        impl From<$name> for u8 {
            fn from(value: $name) -> u8 {
                value.bits()
            }
        }

        impl serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_u8(self.bits())
            }
        }

        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let value = u8::deserialize(deserializer)?;
                Ok($name::from_bits_truncate(value))
            }
        }
    };
}
