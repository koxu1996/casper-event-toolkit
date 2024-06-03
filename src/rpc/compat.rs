use casper_event_standard::casper_types;
use casper_hashing::Digest;
use casper_types::CLValue;
use casper_types::ExecutionResult;
use casper_types::Key;
use casper_types::URef;

use crate::error::ToolkitError;

pub fn digest_to_client_types(
    input: &Digest,
) -> Result<casper_client_hashing::Digest, ToolkitError> {
    convert_types(input, "Digest")
}

pub fn digest_from_client_types(
    input: &casper_client_hashing::Digest,
) -> Result<Digest, ToolkitError> {
    convert_types(input, "client Digest")
}

pub fn uref_to_client_types(input: &URef) -> Result<casper_client_types::URef, ToolkitError> {
    convert_types(input, "URef")
}

pub fn clvalue_from_client_types(
    input: &casper_client_types::CLValue,
) -> Result<CLValue, ToolkitError> {
    convert_types(input, "client CLValue")
}

pub fn execution_result_from_client_types(
    input: &casper_client_types::ExecutionResult,
) -> Result<ExecutionResult, ToolkitError> {
    convert_types(input, "client ExecutionResult")
}

pub fn key_from_client_types(input: &casper_client_types::Key) -> Result<Key, ToolkitError> {
    convert_types(input, "client Key")
}

/// Performs serialization rountrip over 2 different types.
fn convert_types<T, U>(input: &T, type_context: &'static str) -> Result<U, ToolkitError>
where
    T: serde::Serialize,
    U: serde::de::DeserializeOwned,
{
    let serialized_bytes =
        bincode::serialize(input).map_err(|_e| ToolkitError::SerializationError {
            context: type_context,
        })?;

    let deserialized: U = bincode::deserialize(&serialized_bytes).map_err(|_e| {
        ToolkitError::DeserializationError {
            context: type_context,
        }
    })?;

    Ok(deserialized)
}
