use casper_types::bytesrepr::FromBytes;

use casper_event_toolkit::fetcher::Fetcher;
use casper_event_toolkit::metadata::CesMetadataRef;
use casper_event_toolkit::rpc::client::CasperClient;

mod cep78;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("*====================================*");
    println!("| Casper Event Toolkit - CEP-78 demo |");
    println!("*====================================*");
    println!("\n");
    std::thread::sleep(std::time::Duration::from_secs(1));

    let client = CasperClient::default_mainnet();

    println!("Fetching metadata of contract fe03..a7ff:");
    std::thread::sleep(std::time::Duration::from_secs(1));

    let metadata = CesMetadataRef::fetch_metadata(
        &client,
        "fe03021407879ce6fc5e035b70ff6a90941afdbea325a9164c7a497827efa7ff",
    )
    .await?;

    println!("-> events schema uref: {}", metadata.events_schema);
    println!("-> events count uref: {}", metadata.events_length);
    println!("-> events data uref: {}", metadata.events_data);
    println!("\n");
    std::thread::sleep(std::time::Duration::from_secs(1));

    let fetcher = Fetcher {
        client: CasperClient::default_mainnet(),
        ces_metadata: metadata,
    };

    println!("Extracting schema:");
    std::thread::sleep(std::time::Duration::from_secs(1));

    let schemas = fetcher.fetch_schema().await?;
    // Alternatively - user locally defined schemas.
    //let schemas = cep78::schemas::get_local_schemas();

    println!("-> {:?}", schemas);
    println!("\n");
    std::thread::sleep(std::time::Duration::from_secs(1));

    println!("Fetching events count:");
    std::thread::sleep(std::time::Duration::from_secs(1));

    let num_events = fetcher.fetch_events_count().await?;

    println!("-> {}", num_events);
    println!("\n");
    std::thread::sleep(std::time::Duration::from_secs(1));

    // Fetch events from particular deploy.
    // let events = fetcher
    //     .fetch_events_from_deploy(
    //         "657ff0cd295bf398988c12f913d3da62946039aab592da0eb5e577627e9bcaf5",
    //         &schemas,
    //     )
    //     .await?;

    // Fetch some event from storage.
    let event_id = 3;

    println!("Fetching event {}:", event_id);
    std::thread::sleep(std::time::Duration::from_secs(1));

    let dynamic_event = fetcher.fetch_event(event_id, &schemas).await?;

    println!("-> {:?}", dynamic_event);
    std::thread::sleep(std::time::Duration::from_secs(1));
    println!("\n");

    println!("Parsing event {}:", event_id);
    std::thread::sleep(std::time::Duration::from_secs(1));

    match dynamic_event.name.as_str() {
        "Mint" => {
            let data = dynamic_event.to_ces_bytes()?;
            let (parsed_further, rem) = cep78::events::Mint::from_bytes(&data).unwrap(); // TODO
            assert!(rem.len() == 0);
            println!("-> {:?}", parsed_further);
        }
        other => {
            println!("Unknown event type: {}", other)
        }
    }

    Ok(())
}
