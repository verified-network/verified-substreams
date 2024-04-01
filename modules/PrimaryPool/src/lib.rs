mod abi;
#[path = "kv_out.rs"]
mod kv;
#[path = "db_out.rs"]
mod db;
mod pb;

use hex_literal::hex;

use pb::verified::primary::v1::{Pool, Pools, Subscription, Subscriptions};
use substreams::store::{self, DeltaProto};
use substreams::{
    errors::Error,
    log,
    // pb::substreams::module::input::store,
    store::{StoreGet, StoreGetProto, StoreNew, StoreSet, StoreSetProto},
    Hex,
};
use substreams::proto;
use substreams_ethereum::{pb::eth::v2 as eth, Event};
use substreams_sink_kv::pb::sf::substreams::sink::kv::v1::KvOperations;
use substreams_database_change::pb::database::DatabaseChanges;

const FACTORY_CONTRACT: [u8; 20] = hex!("4823be69546f9e1Ab8a87f315108c19dDC8E48b4");
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
    log::info!("Storing Pools from Primary pools");
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
            // use the pool information from the store
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

#[substreams::handlers::map]
pub fn db_out(deltas: store::Deltas<DeltaProto<Pool>>) -> Result<DatabaseChanges, Error> {
    use substreams::pb::substreams::store_delta::Operation;
    let mut database_changes: DatabaseChanges = Default::default();
    for delta in deltas.deltas {
        match delta.operation {
            Operation::Create => {
                db::push_create(&mut database_changes, &delta.key, delta.ordinal, delta.new_value);
            },
            Operation::Update => {
                db::push_update(&mut database_changes, &delta.key, delta.ordinal, delta.old_value, delta.new_value);
            },
            // Operation::Delete => {
            //     db::push_delete(&mut database_changes, &delta.key, delta.ordinal, delta.old_value);
            // },
            x => panic!("unsupported opeation {:?}", x),
        }
    }

    Ok(database_changes)
}

// #[substreams::handlers::map]
// fn db_out(block_meta_start: store::Deltas<DeltaProto<BlockMeta>>) -> Result<DatabaseChanges, substreams::errors::Error> {
//     let mut database_changes: DatabaseChanges = Default::default();
//     transform_block_meta_to_database_changes(&mut database_changes, block_meta_start);
//     Ok(database_changes)
// }