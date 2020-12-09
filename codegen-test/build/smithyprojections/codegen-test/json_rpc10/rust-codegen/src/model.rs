// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use smithy_types::Blob;
use smithy_types::Instant;
#[non_exhaustive]
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    ::std::clone::Clone,
    ::std::cmp::PartialEq,
    ::std::fmt::Debug,
)]
pub enum MyUnion {
    #[serde(rename = "blobValue")]
    #[serde(serialize_with = "crate::serde_util::blob_ser")]
    #[serde(deserialize_with = "crate::serde_util::blob_deser")]
    BlobValue(Blob),
    #[serde(rename = "booleanValue")]
    BooleanValue(bool),
    #[serde(rename = "enumValue")]
    EnumValue(FooEnum),
    #[serde(rename = "listValue")]
    ListValue(::std::vec::Vec<::std::string::String>),
    #[serde(rename = "mapValue")]
    MapValue(::std::collections::HashMap<::std::string::String, ::std::string::String>),
    #[serde(rename = "numberValue")]
    NumberValue(i32),
    #[serde(rename = "stringValue")]
    StringValue(::std::string::String),
    #[serde(rename = "structureValue")]
    StructureValue(GreetingStruct),
    #[serde(rename = "timestampValue")]
    #[serde(serialize_with = "crate::serde_util::instant_epoch_seconds_ser")]
    #[serde(deserialize_with = "crate::serde_util::instant_epoch_seconds_deser")]
    TimestampValue(Instant),
}

#[non_exhaustive]
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    ::std::clone::Clone,
    ::std::cmp::PartialEq,
    ::std::fmt::Debug,
)]
pub struct GreetingStruct {
    #[serde(rename = "hi")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hi: ::std::option::Option<::std::string::String>,
}
impl GreetingStruct {
    /// Creates a new builder-style object to manufacture [`GreetingStruct`](crate::model::GreetingStruct)
    pub fn builder() -> crate::model::greeting_struct::Builder {
        crate::model::greeting_struct::Builder::default()
    }
}
/// See [`GreetingStruct`](crate::model::GreetingStruct)
pub mod greeting_struct {

    use crate::model::GreetingStruct;
    /// A builder for [`GreetingStruct`](crate::model::GreetingStruct)
    #[non_exhaustive]
    #[derive(Debug, Clone, Default)]
    pub struct Builder {
        hi: ::std::option::Option<::std::string::String>,
    }
    impl Builder {
        pub fn hi(mut self, inp: impl Into<::std::string::String>) -> Self {
            self.hi = Some(inp.into());
            self
        }
        /// Consumes the builder and constructs a [`GreetingStruct`](crate::model::GreetingStruct)
        pub fn build(self) -> GreetingStruct {
            GreetingStruct { hi: self.hi }
        }
    }
}

#[non_exhaustive]
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    ::std::clone::Clone,
    ::std::cmp::Eq,
    ::std::cmp::Ord,
    ::std::cmp::PartialEq,
    ::std::cmp::PartialOrd,
    ::std::fmt::Debug,
    ::std::hash::Hash,
)]
pub enum FooEnum {
    Zero,
    One,
    Bar,
    Baz,
    Foo,
    Unknown(String),
}
impl<T> ::std::convert::From<T> for FooEnum
where
    T: ::std::convert::AsRef<str>,
{
    fn from(s: T) -> Self {
        match s.as_ref() {
            "0" => FooEnum::Zero,
            "1" => FooEnum::One,
            "Bar" => FooEnum::Bar,
            "Baz" => FooEnum::Baz,
            "Foo" => FooEnum::Foo,
            other => FooEnum::Unknown(other.to_owned()),
        }
    }
}
impl FooEnum {
    pub fn as_str(&self) -> &str {
        match self {
            FooEnum::Zero => "0",
            FooEnum::One => "1",
            FooEnum::Bar => "Bar",
            FooEnum::Baz => "Baz",
            FooEnum::Foo => "Foo",
            FooEnum::Unknown(s) => s.as_ref(),
        }
    }
}

#[non_exhaustive]
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    ::std::clone::Clone,
    ::std::cmp::PartialEq,
    ::std::fmt::Debug,
)]
pub struct ComplexNestedErrorData {
    #[serde(rename = "Fooooo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foo: ::std::option::Option<::std::string::String>,
}
impl ComplexNestedErrorData {
    /// Creates a new builder-style object to manufacture [`ComplexNestedErrorData`](crate::model::ComplexNestedErrorData)
    pub fn builder() -> crate::model::complex_nested_error_data::Builder {
        crate::model::complex_nested_error_data::Builder::default()
    }
}
/// See [`ComplexNestedErrorData`](crate::model::ComplexNestedErrorData)
pub mod complex_nested_error_data {

    use crate::model::ComplexNestedErrorData;
    /// A builder for [`ComplexNestedErrorData`](crate::model::ComplexNestedErrorData)
    #[non_exhaustive]
    #[derive(Debug, Clone, Default)]
    pub struct Builder {
        foo: ::std::option::Option<::std::string::String>,
    }
    impl Builder {
        pub fn foo(mut self, inp: impl Into<::std::string::String>) -> Self {
            self.foo = Some(inp.into());
            self
        }
        /// Consumes the builder and constructs a [`ComplexNestedErrorData`](crate::model::ComplexNestedErrorData)
        pub fn build(self) -> ComplexNestedErrorData {
            ComplexNestedErrorData { foo: self.foo }
        }
    }
}
