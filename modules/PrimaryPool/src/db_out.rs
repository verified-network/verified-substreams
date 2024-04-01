// src/db_out.rs

use substreams::proto;
use substreams::store::{self, DeltaProto};
use substreams_database_change::pb::database::DatabaseChanges;
// use substreams_sink_kv::pb::kv::KvOperations;
use crate::pb;
use crate::pb::verified::primary::v1::{Pool,Subscription, Subscriptions};
use substreams_database_change::pb::database::table_change::Operation;
use substreams::{
    errors::Error,
    log,
    // pb::substreams::module::input::store,
    store::{StoreGet, StoreGetProto, StoreNew, StoreSet, StoreSetProto},
    Hex,
};

// use crate::pb::eth::block_meta::v1::BlockMeta;

pub fn push_create(
    changes: &mut DatabaseChanges,
    key: &str,
    ordinal: u64,
    value: Pool,
) {
    changes.push_change("pool", key, ordinal, Operation::Create)
        .change("pool_address", (None, Hex(value.pool_address)));
}

pub fn push_update(
    changes: &mut DatabaseChanges,
    key: &str,
    ordinal: u64,
    old_value: Pool,
    new_value: Pool,
) {
    changes.push_change("pool", key, ordinal, Operation::Update)
        .change("pool_address", (Hex(old_value.pool_address), Hex(new_value.pool_address)));
}

// pub fn push_delete(changes: &mut DatabaseChanges, key: &str, ordinal: u64, value: Subscription) {
//     changes
//         .push_change("subscription", key, ordinal, Operation::Delete)
//         .change("assetIn_address", (Hex(value.asset_in_address), None))
//         .change("assetOut_address", (Hex(value.asset_out_address), None))
//         .change("subscription_amount", (value.subscription_amount, None))
//         .change("investor_address", (Hex(value.investor_address), None))
//         .change("price", (value.price, None))
//         .change("execution_date", (value.execution_date, None));
// }