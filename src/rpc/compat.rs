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

/// Recreate URef by serialization roundtrip.
pub fn uref_to_client_types(uref: &URef) -> Result<casper_client_types::URef, ToolkitError> {
    let addr: casper_client_types::URefAddr = uref.addr();

    let access_rights_bytes = bincode::serialize(&uref.access_rights())
        .map_err(|_e| ToolkitError::SerializationError { context: "URef" })?;
    let access_rights: casper_client_types::AccessRights =
        bincode::deserialize(&access_rights_bytes).map_err(|_e| {
            ToolkitError::DeserializationError {
                context: "client URef",
            }
        })?;

    let uref = casper_client_types::URef::new(addr, access_rights);

    Ok(uref)
}

/// Recreate CLValue by serialization roundtrip.
pub fn clvalue_from_client_types(
    clvalue: &casper_client_types::CLValue,
) -> Result<CLValue, ToolkitError> {
    let clvalue_bytes =
        bincode::serialize(&clvalue).map_err(|_e| ToolkitError::SerializationError {
            context: "client CLValue",
        })?;
    let clvalue: CLValue = bincode::deserialize(&clvalue_bytes)
        .map_err(|_e| ToolkitError::DeserializationError { context: "CLValue" })?;

    Ok(clvalue)
}

/// Recreate ExecutionResults by serialization roundtrip.
pub fn execution_result_from_client_types(
    execution_result: &casper_client_types::ExecutionResult,
) -> Result<ExecutionResult, ToolkitError> {
    let execution_result_bytes =
        bincode::serialize(&execution_result).map_err(|_e| ToolkitError::SerializationError {
            context: "client ExecutionResult",
        })?;
    let execution_result: ExecutionResult =
        bincode::deserialize(&execution_result_bytes).map_err(|_e| {
            ToolkitError::DeserializationError {
                context: "ExecutionResult",
            }
        })?;

    Ok(execution_result)
}

/// Recreate Key by serialization roundtrip.
pub fn key_from_client_types(key: &casper_client_types::Key) -> Result<Key, ToolkitError> {
    let key_bytes = bincode::serialize(&key).map_err(|_e| ToolkitError::SerializationError {
        context: "client Key",
    })?;
    let key: Key = bincode::deserialize(&key_bytes)
        .map_err(|_e| ToolkitError::DeserializationError { context: "Key" })?;

    Ok(key)
}
