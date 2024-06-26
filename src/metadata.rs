use casper_event_standard::casper_types;

pub use casper_types::URef;

use crate::error::ToolkitError;
use crate::rpc::client::CasperClient;
use crate::utils::{self, parse_hash};

const EVENTS_SCHEMA_KEY: &str = "__events_schema";
const EVENTS_LENGTH_KEY: &str = "__events_length";
const EVENTS_DATA_KEY: &str = "__events";

#[derive(Debug)]
pub struct CesMetadataRef {
    pub events_schema: URef,
    pub events_length: URef,
    pub events_data: URef,
}

impl CesMetadataRef {
    pub async fn fetch_metadata(
        client: &CasperClient,
        contract_hash: &str,
    ) -> Result<CesMetadataRef, ToolkitError> {
        // Build contract hash.
        let contract_hash = parse_hash(contract_hash)?;

        // Fetch contract named keys.
        let contract_named_keys = client.get_contract_named_keys(contract_hash).await?;

        // Extract CES metadata from named keys.
        let events_schema_uref =
            utils::extract_uref_from_named_keys(&contract_named_keys, EVENTS_SCHEMA_KEY)?;
        let events_length_uref =
            utils::extract_uref_from_named_keys(&contract_named_keys, EVENTS_LENGTH_KEY)?;
        let events_data_uref =
            utils::extract_uref_from_named_keys(&contract_named_keys, EVENTS_DATA_KEY)?;

        Ok(CesMetadataRef {
            events_data: events_data_uref,
            events_length: events_length_uref,
            events_schema: events_schema_uref,
        })
    }
}
