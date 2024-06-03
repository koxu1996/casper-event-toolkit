use std::collections::BTreeMap;

use casper_client::types::Contract;
use casper_event_standard::casper_types;
use casper_types::Key;

use crate::error::ToolkitError;

use super::compat;

/// Transforms a contract's named keys into a proper `NamedKeys` (from `casper_types`).
///
pub fn extract_named_keys(
    contract: Contract,
) -> Result<casper_types::contracts::NamedKeys, ToolkitError> {
    let named_keys_result: Result<BTreeMap<String, Key>, ToolkitError> = contract
        .named_keys()
        .map(|named_key| {
            named_key
                .key()
                .map_err(|e| ToolkitError::UnexpectedError {
                    context: format!("invalid named key '{}'", e),
                })
                .and_then(|key| {
                    let key = compat::key_from_client_types(&key)?;
                    Ok((named_key.name().to_owned(), key))
                })
        })
        .collect();
    let named_keys = named_keys_result?;

    Ok(named_keys)
}
