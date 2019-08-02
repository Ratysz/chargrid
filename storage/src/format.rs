#[allow(unused_imports)]
use crate::StorageFormat;
#[allow(unused_imports)]
use serde::de::DeserializeOwned;
#[allow(unused_imports)]
use serde::ser::Serialize;

#[cfg(feature = "bincode")]
pub struct Bincode;

#[cfg(feature = "bincode")]
impl StorageFormat for Bincode {
    type SerializeError = bincode::Error;
    type DeserializeError = bincode::Error;

    fn to_vec<T: ?Sized>(value: &T) -> Result<Vec<u8>, Self::SerializeError>
    where
        T: Serialize,
    {
        bincode::serialize(value)
    }

    fn from_slice<T>(bytes: &[u8]) -> Result<T, Self::DeserializeError>
    where
        T: DeserializeOwned,
    {
        bincode::deserialize(bytes)
    }
}

#[cfg(feature = "json")]
pub struct Json;

#[cfg(feature = "json")]
impl StorageFormat for Json {
    type SerializeError = serde_json::error::Error;
    type DeserializeError = serde_json::error::Error;

    fn to_vec<T: ?Sized>(value: &T) -> Result<Vec<u8>, Self::SerializeError>
    where
        T: Serialize,
    {
        serde_json::to_vec(value)
    }

    fn from_slice<T>(bytes: &[u8]) -> Result<T, Self::DeserializeError>
    where
        T: DeserializeOwned,
    {
        serde_json::from_slice(bytes)
    }
}

#[cfg(feature = "yaml")]
pub struct Yaml;

#[cfg(feature = "yaml")]
impl StorageFormat for Yaml {
    type SerializeError = serde_yaml::Error;
    type DeserializeError = serde_yaml::Error;

    fn to_vec<T: ?Sized>(value: &T) -> Result<Vec<u8>, Self::SerializeError>
    where
        T: Serialize,
    {
        serde_yaml::to_vec(value)
    }

    fn from_slice<T>(bytes: &[u8]) -> Result<T, Self::DeserializeError>
    where
        T: DeserializeOwned,
    {
        serde_yaml::from_slice(bytes)
    }
}

#[cfg(feature = "toml")]
pub struct Toml;

#[cfg(feature = "toml")]
impl StorageFormat for Toml {
    type SerializeError = toml::ser::Error;
    type DeserializeError = toml::de::Error;

    fn to_vec<T: ?Sized>(value: &T) -> Result<Vec<u8>, Self::SerializeError>
    where
        T: Serialize,
    {
        toml::ser::to_vec(value)
    }

    fn from_slice<T>(bytes: &[u8]) -> Result<T, Self::DeserializeError>
    where
        T: DeserializeOwned,
    {
        toml::de::from_slice(bytes)
    }
}
