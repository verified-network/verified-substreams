mod abi;
#[allow(unused)]
mod pb;
use hex_literal::hex;
use pb::contract::v1 as contract;
use substreams::prelude::*;
use substreams::store;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::Event;
use hex as _hex;

#[allow(unused_imports)] 
use {num_traits::cast::ToPrimitive, std::str::FromStr, substreams::scalar::BigDecimal};

substreams_ethereum::init!();

const PRIMARYPOOL_FACTORY_TRACKED_CONTRACT: [u8; 20] = hex!("da13bc71fee08ffd523f10458b0e2c2d8427bbd5");

fn map_primarypool_factory_events(blk: &eth::Block, events: &mut contract::Events) {
  events.primarypool_factory_pool_createds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == PRIMARYPOOL_FACTORY_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::primarypool_factory_contract::events::PoolCreated::match_and_decode(log) {
                        return Some(contract::PrimarypoolFactoryPoolCreated {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            pool: event.pool,
                        });
                    }

                    None
                })
        })
        .collect());
}
fn is_declared_dds_address(addr: &Vec<u8>, ordinal: u64, dds_store: &store::StoreGetInt64) -> bool {
    if dds_store.get_at(ordinal, Hex(addr).to_string()).is_some() {
        return true;
    }
    return false;
}
fn map_primary_pool_events(
    blk: &eth::Block,
    dds_store: &store::StoreGetInt64,
    events: &mut contract::Events,
) {
events.primary_pool_subscriptions.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| is_declared_dds_address(&log.address, log.ordinal, dds_store))
                .filter_map(|log| {
                    if let Some(event) = abi::primary_pool_contract::events::Subscription::match_and_decode(log) {
                        return Some(contract::PrimaryPoolSubscription {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            evt_address: Hex(&log.address).to_string(),
                            asset_in: event.asset_in,
                            asset_out: event.asset_out,
                            execution_date: event.execution_date.to_string(),
                            investor: event.investor,
                            price: event.price.to_string(),
                            subscription: event.subscription.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());

   }

#[substreams::handlers::store]
fn store_primary_pool_created(blk: eth::Block, store: StoreSetInt64) {
    for rcpt in blk.receipts() {
        for log in rcpt
            .receipt
            .logs
            .iter()
            .filter(|log| log.address == PRIMARYPOOL_FACTORY_TRACKED_CONTRACT)
        {
            if let Some(event) = abi::primarypool_factory_contract::events::PoolCreated::match_and_decode(log) {
                store.set(log.ordinal, Hex(event.pool).to_string(), &1);
            }
        }
    }
}


#[substreams::handlers::store]
fn store_primary_pool_subscriptions_per_pool(
    blk: eth::Block,
    store_primary_pool: store::StoreGetInt64,
    store: store::StoreAppend<String>, // <-- must be String
) {
    for rcpt in blk.receipts() {
        for log in rcpt.receipt.logs.iter() {
            if is_declared_dds_address(&log.address, log.ordinal, &store_primary_pool) {
                if let Some(event) = abi::primary_pool_contract::events::Subscription::match_and_decode(log) {
                    let subscription = contract::PrimaryPoolSubscription {
                        evt_tx_hash: Hex(&rcpt.transaction.hash).to_string(),
                        evt_index: log.block_index,
                        evt_block_time: Some(blk.timestamp().to_owned()),
                        evt_block_number: blk.number,
                        evt_address: Hex(&log.address).to_string(),
                        asset_in: event.asset_in,
                        asset_out: event.asset_out,
                        execution_date: event.execution_date.to_string(),
                        investor: event.investor,
                        price: event.price.to_string(),
                        subscription: event.subscription.to_string(),
                    };

                    // Serialize to bytes, then hex string
                    let bytes = substreams::proto::encode(&subscription)
                        .expect("failed to encode subscription proto");
                    let serialized = _hex::encode(bytes);

                    let pool_key = Hex(&log.address).to_string();
                    store.append(log.ordinal, pool_key, serialized);
                }
            }
        }
    }
}

#[substreams::handlers::map]
fn map_subscriptions_per_pool(
    blk: eth::Block,
    store_primary_pool_created: store::StoreGetInt64,
) -> Result<contract::PrimaryPoolSubscriptionsList, substreams::errors::Error> {
    use std::collections::HashMap;

    let mut per_pool: HashMap<String, contract::PrimaryPoolSubscriptions> = HashMap::new();

    for rcpt in blk.receipts() {
        for log in rcpt.receipt.logs.iter() {
            if is_declared_dds_address(&log.address, log.ordinal, &store_primary_pool_created) {
                if let Some(event) = abi::primary_pool_contract::events::Subscription::match_and_decode(log) {
                    let pool_address = Hex(&log.address).to_string();

                    let entry = per_pool
                        .entry(pool_address.clone())
                        .or_insert_with(|| contract::PrimaryPoolSubscriptions {
                            pool_address: pool_address.clone(),
                            items: vec![],
                        });

                    entry.items.push(contract::PrimaryPoolSubscription {
                        evt_tx_hash: Hex(&rcpt.transaction.hash).to_string(),
                        evt_index: log.block_index,
                        evt_block_time: Some(blk.timestamp().to_owned()),
                        evt_block_number: blk.number,
                        evt_address: pool_address.clone(),
                        asset_in: event.asset_in,
                        asset_out: event.asset_out,
                        execution_date: event.execution_date.to_string(),
                        investor: event.investor,
                        price: event.price.to_string(),
                        subscription: event.subscription.to_string(),
                    });
                }
            }
        }
    }

    Ok(contract::PrimaryPoolSubscriptionsList {
        items: per_pool.into_values().collect(),
    })
}



#[substreams::handlers::map]
fn map_events(
    blk: eth::Block,
    store_primary_pool: StoreGetInt64,
) -> Result<contract::Events, substreams::errors::Error> {
    let mut events = contract::Events::default();
    map_primarypool_factory_events(&blk, &mut events);
    map_primary_pool_events(&blk, &store_primary_pool, &mut events);
    Ok(events)
}

