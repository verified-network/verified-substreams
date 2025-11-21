mod abi;
#[allow(unused)]
mod pb;

use hex_literal::hex;
use pb::contract::v1 as contract;
use substreams::prelude::*;
use substreams::store::{StoreGetInt64, StoreSetInt64, StoreAppend};
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::Event;
use substreams::Hex;
use hex as _hex;

substreams_ethereum::init!();

const MARGINPOOL_FACTORY_TRACKED_CONTRACT: [u8; 20] =
    hex!("b1ae3fc5b16d3736bf0db20606fb9a10b435392c");

fn pool_exists(addr: &Vec<u8>, ordinal: u64, store: &StoreGetInt64) -> bool {
    store.get_at(ordinal, Hex(addr).to_string()).is_some()
}

fn map_marginpool_factory_events(blk: &eth::Block, events: &mut contract::Events) {
    events.marginpool_factory_pool_createds.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == MARGINPOOL_FACTORY_TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::marginpool_factory_contract::events::PoolCreated::match_and_decode(log)
                        {
                            Some(contract::MarginpoolFactoryPoolCreated {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
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

fn map_margin_pool_events(
    blk: &eth::Block,
    pool_store: &StoreGetInt64,
    events: &mut contract::Events,
) {
    events.margin_pool_margin_trade_reports.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| pool_exists(&log.address, log.ordinal, pool_store))
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::margin_pool_contract::events::MarginTradeReport::match_and_decode(log)
                        {
                            Some(contract::MarginPoolMarginTradeReport {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
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
fn store_margin_pool(blk: eth::Block, store: StoreSetInt64) {
    for rcpt in blk.receipts() {
        for log in rcpt.receipt.logs.iter() {
            // Track pool creation
            if log.address == MARGINPOOL_FACTORY_TRACKED_CONTRACT {
                if let Some(event) =
                    abi::marginpool_factory_contract::events::PoolCreated::match_and_decode(log)
                {
                    let key = Hex(event.pool).to_string();
                    store.set(log.ordinal, key, &1);
                }
            }
            // Track margin pool trades (just set, no read)
            if let Some(_event) =
                abi::margin_pool_contract::events::MarginTradeReport::match_and_decode(log)
            {
                let key = Hex(&log.address).to_string();
                store.set(log.ordinal, key, &1);
            }
        }
    }
}

#[substreams::handlers::store]
fn store_margin_pool_trades_per_pool(
    blk: eth::Block,
    pool_store: StoreGetInt64,
    store: StoreAppend<String>,
) {
    for rcpt in blk.receipts() {
        for log in rcpt.receipt.logs.iter() {
            let pool_key = Hex(&log.address).to_string();

            // Include trades only if pool exists in store
            if pool_exists(&log.address, log.ordinal, &pool_store) {
                if let Some(event) = abi::margin_pool_contract::events::MarginTradeReport::match_and_decode(log) {
                    let trade = contract::MarginPoolMarginTradeReport {
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
) -> Result<contract::MarginPoolTradeReportsList, substreams::errors::Error> {
    use std::collections::HashMap;

    let mut per_pool: HashMap<String, contract::MarginPoolTradeReports> = HashMap::new();

    for rcpt in blk.receipts() {
        for log in rcpt.receipt.logs.iter() {
            let pool_key = Hex(&log.address).to_string();

            // Only include trades if pool exists in store
            if pool_exists(&log.address, log.ordinal, &pool_store) {
                if let Some(event) = abi::margin_pool_contract::events::MarginTradeReport::match_and_decode(log) {
                    let entry = per_pool.entry(pool_key.clone()).or_insert_with(|| contract::MarginPoolTradeReports {
                        pool_address: pool_key.clone(),
                        items: vec![],
                    });

                    entry.items.push(contract::MarginPoolMarginTradeReport {
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

    Ok(contract::MarginPoolTradeReportsList {
        items: per_pool.into_values().collect(),
    })
}



#[substreams::handlers::map]
fn map_events(
    blk: eth::Block,
    pool_store: StoreGetInt64,
) -> Result<contract::Events, substreams::errors::Error> {
    let mut events = contract::Events::default();
    map_marginpool_factory_events(&blk, &mut events);
    map_margin_pool_events(&blk, &pool_store, &mut events);
    Ok(events)
}
