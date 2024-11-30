mod abi;
#[path = "kv_out.rs"]
mod kv;
mod pb;

use hex_literal::hex;

use env_logger::Env;
use pb::verified::primary::v1::{Pool, Pools, Subscription, Subscriptions};
use substreams::store::{self, DeltaProto};
use substreams::{
    errors::Error,
    log,
    // pb::substreams::module::input::store,
    store::{StoreGet, StoreGetProto, StoreNew, StoreSet, StoreSetProto},
    Hex,
};
use substreams_ethereum::{pb::eth::v2 as eth, Event};
use substreams_sink_kv::pb::sf::substreams::sink::kv::v1::KvOperations;
use crate::pb::sf::substreams::sink::files::v1::Lines;
use serde::Serialize;
use serde_json;



const FACTORY_CONTRACT: [u8; 20] = hex!("DA13BC71FEe08FfD523f10458B0e2c2D8427BBD5"); //sepolia
// const FACTORY_CONTRACT: [u8; 20] = hex!("2081d59917ee6B58D92A19174c158354359187BC"); 

substreams_ethereum::init!();

#[substreams::handlers::map]
fn map_pools(blk: eth::Block) -> Result<Pools, substreams::errors::Error> {
    log::info!("Detecting Pools from Primary pools");
    Ok(Pools {
        pools: blk
            .events::<abi::factory::events::PoolCreated>(&[&FACTORY_CONTRACT])
            .map(|(pool_created, _log)| {
                log::info!("PoolCreated event seen");

                Pool {
                    pool_address: (pool_created.pool),
                }
            })
            .collect(),
    })
}

#[substreams::handlers::store]
pub fn store_pools_created(pools: Pools, store: StoreSetProto<Pool>) {
    let mut ordinal = 0;
    for pool in pools.pools {
        ordinal = ordinal + 1;
        let pool_address = &pool.pool_address;
        // store.set(ordinal, bytes_to_hex(pool_address), &pool);
        store.set(ordinal, &Hex::encode(pool_address), &pool);
        // store.set(ordinal, format!("pool:{}", Hex::encode(pool_address)), &pool);
        // store.set(ordinal, format!("pool:{:?}", pool_address), &pool);
    }
}

#[substreams::handlers::map]
fn map_subscriptions(
    blk: eth::Block,
    pools_store: StoreGetProto<Pool>,
) -> Result<Subscription, Error> {
    log::info!("Detecting subscriptions from Primary module");
    let mut pool_events = Subscription::default();
    for trx in blk.transactions() {
        for (log, call_view) in trx.logs_with_calls() {
            let pool_address = &Hex(&log.address).to_string();
            let pool_opt = pools_store.get_last(&pool_address);
            if pool_opt.is_none() {
                continue;
            }
            if let Some(subscription) = abi::pool::events::Subscription::match_and_decode(log) {
                log::info!("Subscription event seen");
                pool_events.asset_in_address = subscription.asset_in;
                pool_events.asset_out_address = subscription.asset_out;
                pool_events.subscription_amount = subscription.subscription.to_u64();
                pool_events.investor_address = subscription.investor;
                pool_events.price = subscription.price.to_u64();
                pool_events.execution_date = subscription.execution_date.to_u64();
            }
        }
    }

    Ok(pool_events)
}

#[substreams::handlers::store]
pub fn store_subscription_created(subscriptions: Subscription, store: StoreSetProto<Subscription>) {
    let ordinal = 1;
    store.set(ordinal, &Hex::encode("s"), &subscriptions);
}

#[substreams::handlers::map]
fn map_subscriptions_check(
    blk: eth::Block,
    pools_store: StoreGetProto<Subscription>,
) -> Result<Subscription, Error> {
    log::info!("Detecting subscriptions from Primary module");
    let mut pool_events = Subscription::default();

    Ok(pool_events)
}
#[substreams::handlers::map]
pub fn kv_out(deltas: store::Deltas<DeltaProto<Subscription>>) -> Result<KvOperations, Error> {
    // Create an empty 'KvOperations' structure
    let mut kv_ops: KvOperations = Default::default();
    kv::process_deltas(&mut kv_ops, deltas);
    // Here, we could add more operations to the kv_ops
    // kv_ops.push_new("assetInAddress", order_created.asset_in_address, 1);
    // kv_ops.push_new("assetOutAddress", order_created.asset_out_address, 2);
    // kv_ops.push_new(
    //     "subscriptionAmount",
    //     order_created.subscription_amount.to_be_bytes(),
    //     3,
    // );
    // kv_ops.push_new("investorAddress", order_created.investor_address, 4);
    // kv_ops.push_new("price", order_created.price.to_be_bytes(), 5);
    // kv_ops.push_new(
    //     "executionDate",
    //     order_created.execution_date.to_be_bytes(),
    //     6,
    // );

    Ok(kv_ops)
}

fn init_logger() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .init();
}

// JSON output structure
#[derive(Serialize)]
struct JsonOutput {
    #[serde(rename = "type")]
    event_type: String,
    pool: Option<String>,
    asset_in: Option<String>,
    asset_out: Option<String>,
    subscription: Option<String>,
    investor: Option<String>,
    price: Option<String>,
    execution_date: Option<String>,
    block_number: Option<u64>,
    timestamp: Option<u64>,
}

// Event collection function
fn get_events(
    blk: &eth::Block,
    pools_store: StoreGetProto<Pool>,
    subscription_store: StoreGetProto<Subscription>
) -> impl Iterator<Item = JsonOutput> + '_ {
    // Add debug logging
    log::info!("Processing block {} for events", blk.number);

    // Pool events (from factory)
    let pool_events = blk.events::<abi::factory::events::PoolCreated>(&[&FACTORY_CONTRACT])
        .map(|(event, _log)| {
            log::info!("Found PoolCreated event");
            JsonOutput {
                event_type: "PoolCreated".to_string(),
                pool: Some(format!("0x{}", hex::encode(&event.pool))),
                asset_in: None,
                asset_out: None,
                subscription: None,
                investor: None,
                price: None,
                execution_date: None,
                block_number: Some(blk.number as u64),
                timestamp: Some(blk.timestamp().seconds as u64),
            }
        })
        .collect::<Vec<_>>();

    // Subscription events - Modified to better match map_subscriptions logic
    let subscription_events: Vec<JsonOutput> = blk
        .transactions()
        .flat_map(|trx| {
            trx.logs_with_calls().filter_map(|(log, _call_view)| {
                let pool_address = Hex(&log.address).to_string();
                
                // Check if this is a known pool
                if pools_store.get_last(&pool_address).is_some() {
                    // Try to decode as Subscription event
                    if let Some(event) = abi::pool::events::Subscription::match_and_decode(log) {
                        log::info!("Found Subscription event in pool {}", pool_address);
                        Some(JsonOutput {
                            event_type: "Subscription".to_string(),
                            pool: Some(format!("0x{}", pool_address)),
                            asset_in: Some(format!("0x{}", hex::encode(&event.asset_in))),
                            asset_out: Some(format!("0x{}", hex::encode(&event.asset_out))),
                            subscription: Some(event.subscription.to_string()),
                            investor: Some(format!("0x{}", hex::encode(&event.investor))),
                            price: Some(event.price.to_string()),
                            execution_date: Some(event.execution_date.to_string()),
                            block_number: Some(blk.number),
                            timestamp: Some(blk.timestamp().seconds as u64),
                        })
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
        })
        .collect();

    // Add debug logging for event counts
    log::info!(
        "Found {} pool events and {} subscription events",
        pool_events.len(),
        subscription_events.len()
    );

    pool_events.into_iter().chain(subscription_events.into_iter())
}

// Map handler for JSON output
#[substreams::handlers::map]
fn jsonl_out(
    blk: eth::Block,
    pools_store: StoreGetProto<Pool>,
    subscription_store: StoreGetProto<Subscription>
) -> Result<Lines, Error> {
    log::info!("Processing block {} for JSON output", blk.number);
    
    let events = get_events(&blk, pools_store, subscription_store);
    let lines: Vec<String> = events
        .map(|event| {
            match serde_json::to_string(&event) {
                Ok(json) => {
                    log::info!("Successfully serialized event: {}", json);
                    json
                },
                Err(e) => {
                    log::info!("Failed to serialize event: {}", e);
                    String::new()
                }
            }
        })
        .filter(|line| !line.is_empty())
        .collect();

    log::info!("Generated {} JSON lines", lines.len());
    
    Ok(Lines { lines })
}