/// The map collection type to output for Protobuf `map` fields.
#[non_exhaustive]
#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub(crate) enum MapType {
    /// The [`std::collections::HashMap`] type.
    #[default]
    HashMap,
    /// The [`std::collections::BTreeMap`] type.
    BTreeMap,
}

/// The bytes collection type to output for Protobuf `bytes` fields.
#[non_exhaustive]
#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub(crate) enum BytesType {
    /// The [`prost::alloc::vec::Vec<u8>`] type.
    #[default]
    Vec,
    /// The [`bytes::Bytes`](prost::bytes::Bytes) type.
    Bytes,
}

impl MapType {
    /// The `prost-derive` annotation type corresponding to the map type.
    pub fn annotation(&self) -> &'static str {
        match self {
            MapType::HashMap => "map",
            MapType::BTreeMap => "btree_map",
        }
    }

    /// The fully-qualified Rust type corresponding to the map type.
    pub fn rust_type(&self, prost_path: &str) -> String {
        match self {
            MapType::HashMap => "::std::collections::HashMap".to_string(),
            MapType::BTreeMap => format!("{}::alloc::collections::BTreeMap", prost_path),
        }
    }
}

impl BytesType {
    /// The `prost-derive` annotation type corresponding to the bytes type.
    pub fn annotation(&self) -> &'static str {
        match self {
            BytesType::Vec => "vec",
            BytesType::Bytes => "bytes",
        }
    }

    /// The fully-qualified Rust type corresponding to the bytes type.
    pub fn rust_type(&self, prost_path: &str) -> String {
        match self {
            BytesType::Vec => format!("{}::alloc::vec::Vec<u8>", prost_path),
            BytesType::Bytes => format!("{}::bytes::Bytes", prost_path),
        }
    }
}
