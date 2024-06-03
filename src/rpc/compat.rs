use casper_event_standard::casper_types;
use casper_hashing::Digest;
use casper_types::CLValue;
use casper_types::ExecutionResult;
use casper_types::Key;
use casper_types::URef;

use crate::error::ToolkitError;

/// Recreate `Digest` to avoid leaking incompatible `casper-client` types.
pub fn digest_to_client_types(digest: Digest) -> casper_client_hashing::Digest {
    let raw_bytes: [u8; Digest::LENGTH] = digest.into();
    let digest = casper_client_hashing::Digest::from(raw_bytes);

    digest
}

/// Recreate incompatible `Digest` from `casper-client` types.
pub fn digest_from_client_types(digest: casper_client_hashing::Digest) -> Digest {
    let raw_bytes: [u8; Digest::LENGTH] = digest.into();
    let digest = Digest::from(raw_bytes);

    digest
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
