mod abi;
#[allow(unused)]
mod pb;

use hex_literal::hex;
use pb::contract::v1 as contract;
use substreams::prelude::*;
use substreams::store::{StoreGetInt64, StoreSetInt64, StoreAppend};
use substreams::Hex;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::Event;
use hex as _hex;

#[allow(unused_imports)]
use {num_traits::cast::ToPrimitive, std::str::FromStr, substreams::scalar::BigDecimal};

substreams_ethereum::init!();

const SECONDARYPOOL_FACTORY_TRACKED_CONTRACT: [u8; 20] =
    hex!("4519148cc4030c2e3573f1f886ed4071fa35d62b");

    

fn pool_exists(addr: &Vec<u8>, ordinal: u64, store: &StoreGetInt64) -> bool {
    let key = Hex(addr).to_string();
    store.get_at(ordinal, key).is_some()
}

fn map_secondarypool_factory_events(blk: &eth::Block, events: &mut contract::Events) {
    events.secondarypool_factory_pool_createds.append(
        &mut blk
            .receipts()
            .flat_map(|rcpt| {
                rcpt.receipt.logs.iter()
                    .filter(|log| log.address == SECONDARYPOOL_FACTORY_TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::secondarypool_factory_contract::events::PoolCreated::match_and_decode(log) {
                            Some(contract::SecondarypoolFactoryPoolCreated {
                                evt_tx_hash: Hex(&rcpt.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                pool: event.pool,
                            })
                        } else {
                            None
                        }
                    })
            })
            .collect::<Vec<_>>(),
    );
}


fn map_secondary_pool_events(
    blk: &eth::Block,
    pool_store: &StoreGetInt64,
    events: &mut contract::Events,
) {
    events.secondary_pool_trade_reports.append(
        &mut blk
            .receipts()
            .flat_map(|rcpt| {
                rcpt.receipt.logs.iter()
                    .filter(|log| pool_exists(&log.address, log.ordinal, pool_store))
                    .filter_map(|log| {
                        if let Some(event) = abi::secondary_pool_contract::events::TradeReport::match_and_decode(log) {
                            Some(contract::SecondaryPoolTradeReport {
                                evt_tx_hash: Hex(&rcpt.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                evt_address: Hex(&log.address).to_string(),
                                amount: event.amount.to_string(),
                                counterparty: event.counterparty,
                                currency: event.currency,
                                execution_date: event.execution_date.to_string(),
                                order_ref: Vec::from(event.order_ref),
                                order_type: Vec::from(event.order_type),
                                party: event.party,
                                price: event.price.to_string(),
                                security: event.security,
                            })
                        } else {
                            None
                        }
                    })
            })
            .collect::<Vec<_>>(),
    );
}

#[substreams::handlers::store]
fn store_secondary_pool(blk: eth::Block, store: StoreSetInt64) {
    for rcpt in blk.receipts() {
        for log in rcpt.receipt.logs.iter() {
            // Track pool creation
            if log.address == SECONDARYPOOL_FACTORY_TRACKED_CONTRACT {
                if let Some(event) =
                    abi::secondarypool_factory_contract::events::PoolCreated::match_and_decode(log)
                {
                    let key = Hex(event.pool).to_string();
                    store.set(log.ordinal, key, &1);
                }
            }

            // Track secondary pool trades
            if let Some(_event) =
                abi::secondary_pool_contract::events::TradeReport::match_and_decode(log)
            {
                let key = Hex(&log.address).to_string();
                store.set(log.ordinal, key, &1);
            }
        }
    }
}

#[substreams::handlers::store]
fn store_secondary_pool_trades_per_pool(
    blk: eth::Block,
    pool_store: StoreGetInt64,
    store: StoreAppend<String>,
) {
    for rcpt in blk.receipts() {
        for log in rcpt.receipt.logs.iter() {
            let pool_key = Hex(&log.address).to_string();

            // Include trades only if pool exists
            if pool_exists(&log.address, log.ordinal, &pool_store) {
                if let Some(event) = abi::secondary_pool_contract::events::TradeReport::match_and_decode(log) {
                    let trade = contract::SecondaryPoolTradeReport {
                        evt_tx_hash: Hex(&rcpt.transaction.hash).to_string(),
                        evt_index: log.block_index,
                        evt_block_time: Some(blk.timestamp().to_owned()),
                        evt_block_number: blk.number,
                        evt_address: pool_key.clone(),
                        amount: event.amount.to_string(),
                        counterparty: event.counterparty,
                        currency: event.currency,
                        execution_date: event.execution_date.to_string(),
                        order_ref: Vec::from(event.order_ref),
                        order_type: Vec::from(event.order_type),
                        party: event.party,
                        price: event.price.to_string(),
                        security: event.security,
                    };

                    let bytes = substreams::proto::encode(&trade).expect("failed to encode trade proto");
                    store.append(log.ordinal, pool_key, _hex::encode(bytes));
                }
            }
        }
    }
}

#[substreams::handlers::map]
fn map_trades_per_pool(
    blk: eth::Block,
    pool_store: StoreGetInt64,
) -> Result<contract::SecondaryPoolTradeReportsList, substreams::errors::Error> {
    use std::collections::HashMap;

    let mut per_pool: HashMap<String, contract::SecondaryPoolTradeReports> = HashMap::new();

    for rcpt in blk.receipts() {
        for log in rcpt.receipt.logs.iter() {
            let pool_key = Hex(&log.address).to_string();

            if pool_exists(&log.address, log.ordinal, &pool_store) {
                if let Some(event) = abi::secondary_pool_contract::events::TradeReport::match_and_decode(log) {
                    let entry = per_pool.entry(pool_key.clone()).or_insert_with(|| contract::SecondaryPoolTradeReports {
                        pool_address: pool_key.clone(),
                        items: vec![],
                    });

                    entry.items.push(contract::SecondaryPoolTradeReport {
                        evt_tx_hash: Hex(&rcpt.transaction.hash).to_string(),
                        evt_index: log.block_index,
                        evt_block_time: Some(blk.timestamp().to_owned()),
                        evt_block_number: blk.number,
                        evt_address: pool_key.clone(),
                        amount: event.amount.to_string(),
                        counterparty: event.counterparty,
                        currency: event.currency,
                        execution_date: event.execution_date.to_string(),
                        order_ref: Vec::from(event.order_ref),
                        order_type: Vec::from(event.order_type),
                        party: event.party,
                        price: event.price.to_string(),
                        security: event.security,
                    });
                }
            }
        }
    }

    Ok(contract::SecondaryPoolTradeReportsList {
        items: per_pool.into_values().collect(),
    })
}

#[substreams::handlers::map]
fn map_events(
    blk: eth::Block,
    pool_store: StoreGetInt64,
) -> Result<contract::Events, substreams::errors::Error> {
    let mut events = contract::Events::default();
    map_secondarypool_factory_events(&blk, &mut events);
    map_secondary_pool_events(&blk, &pool_store, &mut events);
    Ok(events)
}
