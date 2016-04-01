pub use std::marker::PhantomData;

pub struct CustomVisitor<T>(pub PhantomData<T>);

/// takes a type `T`, a `FnOnce(&T) -> String` and a `FnOnce(&str) -> Option<T>` and implements
/// serde `Serialize` and `Deserialize` for `T`. `CustomVisitor` must be in scope for invocation.
#[macro_export]
macro_rules! custom_string_serde {
    ($t:ty, $ser:expr, $de:expr) => {
        impl serde::Serialize for $t {
            fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
                where S: serde::Serializer
            {
                serializer.serialize_str(&$ser(self))
            }
        }

        impl serde::de::Visitor for CustomVisitor<$t> {
            type Value = $t;

            fn visit_str<E>(&mut self, v: &str) -> Result<Self::Value, E>
                where E: serde::de::Error
            {
                $de(v)
            }
        }

        impl serde::Deserialize for $t {
            fn deserialize<D>(deserializer: &mut D) -> Result<$t, D::Error>
                where D: serde::Deserializer
            {
                let visitor: CustomVisitor<$t> = CustomVisitor(PhantomData);
                deserializer.deserialize(visitor)
            }
        }
    };
}

macro_rules! simple_serde {
    ($t:ty, $ser:expr, $de:expr) => {
        impl serde::Serialize for $t {
            fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
                where S: serde::Serializer
            {
                serializer.serialize_str(&$ser(self))
            }
        }

        impl serde::de::Visitor for CustomVisitor<$t> {
            type Value = $t;

            fn visit_str<E>(&mut self, v: &str) -> Result<Self::Value, E>
                where E: serde::de::Error
            {
                Ok($de(v))
            }
        }

        impl serde::Deserialize for $t {
            fn deserialize<D>(deserializer: &mut D) -> Result<$t, D::Error>
                where D: serde::Deserializer
            {
                let visitor: CustomVisitor<$t> = CustomVisitor(PhantomData);
                deserializer.deserialize(visitor)
            }
        }
    }
}
