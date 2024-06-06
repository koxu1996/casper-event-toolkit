use casper_event_standard::casper_types;

use crate::error::ToolkitError;

type CompatResult<T> = Result<T, ToolkitError>;

/// Performs serialization rountrip over 2 different types.
fn convert_types<T, U>(input: &T, type_context: &'static str) -> CompatResult<U>
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

pub fn clvalue_from_client_types(
    input: &casper_client_types::CLValue,
) -> CompatResult<casper_types::CLValue> {
    convert_types(input, "client CLValue")
}

pub fn digest_from_client_types(
    input: &casper_client_hashing::Digest,
) -> CompatResult<casper_hashing::Digest> {
    convert_types(input, "client Digest")
}

pub fn digest_to_client_types(
    input: &casper_hashing::Digest,
) -> CompatResult<casper_client_hashing::Digest> {
    convert_types(input, "Digest")
}

pub fn execution_result_from_client_types(
    input: &casper_client_types::ExecutionResult,
) -> CompatResult<casper_types::ExecutionResult> {
    convert_types(input, "client ExecutionResult")
}

pub fn key_from_client_types(input: &casper_client_types::Key) -> CompatResult<casper_types::Key> {
    convert_types(input, "client Key")
}

// NOTE: This method is used in Kairos.
#[allow(unused)]
pub fn key_to_client_types(input: &casper_types::Key) -> CompatResult<casper_client_types::Key> {
    convert_types(input, "Key")
}

pub fn uref_to_client_types(input: &casper_types::URef) -> CompatResult<casper_client_types::URef> {
    convert_types(input, "URef")
}
