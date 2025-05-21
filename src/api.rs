//! Electrum APIs

use bp::{BlockHeader, ConsensusDecode, ConsensusEncode, ScriptPubkey, Tx, Txid};
use std::borrow::Borrow;
use std::convert::TryInto;
use std::ops::Deref;

use crate::batch::Batch;
use crate::types::*;

impl<E: Deref> ElectrumApi for E
where
    E::Target: ElectrumApi,
{
    fn raw_call(
        &self,
        method_name: &str,
        params: impl IntoIterator<Item = Param>,
    ) -> Result<serde_json::Value, Error> {
        (**self).raw_call(method_name, params)
    }

    fn batch_call(&self, batch: &Batch) -> Result<Vec<serde_json::Value>, Error> {
        (**self).batch_call(batch)
    }

    fn block_headers_subscribe_raw(&self) -> Result<RawHeaderNotification, Error> {
        (**self).block_headers_subscribe_raw()
    }

    fn block_headers_pop_raw(&self) -> Result<Option<RawHeaderNotification>, Error> {
        (**self).block_headers_pop_raw()
    }

    fn block_header_raw(&self, height: usize) -> Result<Vec<u8>, Error> {
        (**self).block_header_raw(height)
    }

    fn block_headers(&self, start_height: usize, count: usize) -> Result<GetHeadersRes, Error> {
        (**self).block_headers(start_height, count)
    }

    fn estimate_fee(&self, number: usize) -> Result<f64, Error> {
        (**self).estimate_fee(number)
    }

    fn relay_fee(&self) -> Result<f64, Error> {
        (**self).relay_fee()
    }

    fn script_subscribe(&self, script: &ScriptPubkey) -> Result<Option<ScriptStatus>, Error> {
        (**self).script_subscribe(script)
    }

    fn batch_script_subscribe<'s, I>(&self, scripts: I) -> Result<Vec<Option<ScriptStatus>>, Error>
    where
        I: IntoIterator + Clone,
        I::Item: Borrow<&'s ScriptPubkey>,
    {
        (**self).batch_script_subscribe(scripts)
    }

    fn script_unsubscribe(&self, script: &ScriptPubkey) -> Result<bool, Error> {
        (**self).script_unsubscribe(script)
    }

    fn script_pop(&self, script: &ScriptPubkey) -> Result<Option<ScriptStatus>, Error> {
        (**self).script_pop(script)
    }

    fn script_get_balance(&self, script: &ScriptPubkey) -> Result<GetBalanceRes, Error> {
        (**self).script_get_balance(script)
    }

    fn batch_script_get_balance<'s, I>(&self, scripts: I) -> Result<Vec<GetBalanceRes>, Error>
    where
        I: IntoIterator + Clone,
        I::Item: Borrow<&'s ScriptPubkey>,
    {
        (**self).batch_script_get_balance(scripts)
    }

    fn script_get_history(&self, script: &ScriptPubkey) -> Result<Vec<GetHistoryRes>, Error> {
        (**self).script_get_history(script)
    }

    fn batch_script_get_history<'s, I>(&self, scripts: I) -> Result<Vec<Vec<GetHistoryRes>>, Error>
    where
        I: IntoIterator + Clone,
        I::Item: Borrow<&'s ScriptPubkey>,
    {
        (**self).batch_script_get_history(scripts)
    }

    fn script_list_unspent(&self, script: &ScriptPubkey) -> Result<Vec<ListUnspentRes>, Error> {
        (**self).script_list_unspent(script)
    }

    fn batch_script_list_unspent<'s, I>(
        &self,
        scripts: I,
    ) -> Result<Vec<Vec<ListUnspentRes>>, Error>
    where
        I: IntoIterator + Clone,
        I::Item: Borrow<&'s ScriptPubkey>,
    {
        (**self).batch_script_list_unspent(scripts)
    }

    fn script_get_mempool(&self, script: &ScriptPubkey) -> Result<Vec<GetMempoolRes>, Error> {
        (**self).script_get_mempool(script)
    }

    fn transaction_get_verbose(&self, txid: &Txid) -> Result<Option<TxRes>, Error> {
        (**self).transaction_get_verbose(txid)
    }

    fn transaction_get_raw(&self, txid: &Txid) -> Result<Option<Vec<u8>>, Error> {
        (**self).transaction_get_raw(txid)
    }

    fn batch_transaction_get_raw<'t, I>(&self, txids: I) -> Result<Vec<Vec<u8>>, Error>
    where
        I: IntoIterator + Clone,
        I::Item: Borrow<&'t Txid>,
    {
        (**self).batch_transaction_get_raw(txids)
    }

    fn batch_block_header_raw<I>(&self, heights: I) -> Result<Vec<Vec<u8>>, Error>
    where
        I: IntoIterator + Clone,
        I::Item: Borrow<u32>,
    {
        (**self).batch_block_header_raw(heights)
    }

    fn batch_estimate_fee<I>(&self, numbers: I) -> Result<Vec<f64>, Error>
    where
        I: IntoIterator + Clone,
        I::Item: Borrow<usize>,
    {
        (**self).batch_estimate_fee(numbers)
    }

    fn transaction_broadcast_raw(&self, raw_tx: &[u8]) -> Result<Txid, Error> {
        (**self).transaction_broadcast_raw(raw_tx)
    }

    fn transaction_get_merkle(&self, txid: &Txid, height: usize) -> Result<GetMerkleRes, Error> {
        (**self).transaction_get_merkle(txid, height)
    }

    fn txid_from_pos(&self, height: usize, tx_pos: usize) -> Result<Txid, Error> {
        (**self).txid_from_pos(height, tx_pos)
    }

    fn txid_from_pos_with_merkle(
        &self,
        height: usize,
        tx_pos: usize,
    ) -> Result<TxidFromPosRes, Error> {
        (**self).txid_from_pos_with_merkle(height, tx_pos)
    }

    fn server_features(&self) -> Result<ServerFeaturesRes, Error> {
        (**self).server_features()
    }

    fn ping(&self) -> Result<(), Error> {
        (**self).ping()
    }

    #[cfg(feature = "debug-calls")]
    fn calls_made(&self) -> Result<usize, Error> {
        (**self).calls_made()
    }
}

/// API calls exposed by an Electrum client
pub trait ElectrumApi {
    /// Gets the block header for height `height`.
    fn block_header(&self, height: usize) -> Result<BlockHeader, Error> {
        Ok(BlockHeader::consensus_deserialize(
            &self.block_header_raw(height)?,
        )?)
    }

    /// Subscribes to notifications for new block headers, by sending a `blockchain.headers.subscribe` call.
    fn block_headers_subscribe(&self) -> Result<HeaderNotification, Error> {
        self.block_headers_subscribe_raw()?.try_into()
    }

    /// Tries to pop one queued notification for a new block header that we might have received.
    /// Returns `None` if there are no items in the queue.
    fn block_headers_pop(&self) -> Result<Option<HeaderNotification>, Error> {
        self.block_headers_pop_raw()?
            .map(|raw| raw.try_into())
            .transpose()
    }

    /// Gets the transaction with `txid`. Returns an error if not found.
    fn transaction_get(&self, txid: &Txid) -> Result<Option<Tx>, Error> {
        Ok(self
            .transaction_get_raw(txid)?
            .map(Tx::consensus_deserialize)
            .transpose()?)
    }

    /// Gets the transaction with `txid`, including additional verbose details returned by Butcoin Core. Returns an error if not found.
    fn transaction_get_verbose(&self, txid: &Txid) -> Result<Option<TxRes>, Error>;

    /// Batch version of [`transaction_get`](#method.transaction_get).
    ///
    /// Takes a list of `txids` and returns a list of transactions.
    fn batch_transaction_get<'t, I>(&self, txids: I) -> Result<Vec<Tx>, Error>
    where
        I: IntoIterator + Clone,
        I::Item: Borrow<&'t Txid>,
    {
        self.batch_transaction_get_raw(txids)?
            .iter()
            .map(|s| Ok(Tx::consensus_deserialize(s)?))
            .collect()
    }

    /// Batch version of [`block_header`](#method.block_header).
    ///
    /// Takes a list of `heights` of blocks and returns a list of headers.
    fn batch_block_header<I>(&self, heights: I) -> Result<Vec<BlockHeader>, Error>
    where
        I: IntoIterator + Clone,
        I::Item: Borrow<u32>,
    {
        self.batch_block_header_raw(heights)?
            .iter()
            .map(|s| Ok(BlockHeader::consensus_deserialize(s)?))
            .collect()
    }

    /// Broadcasts a transaction to the network.
    fn transaction_broadcast(&self, tx: &Tx) -> Result<Txid, Error> {
        let buffer: Vec<u8> = tx.consensus_serialize();
        self.transaction_broadcast_raw(&buffer)
    }

    /// Executes the requested API call returning the raw answer.
    fn raw_call(
        &self,
        method_name: &str,
        params: impl IntoIterator<Item = Param>,
    ) -> Result<serde_json::Value, Error>;

    /// Execute a queue of calls stored in a [`Batch`](../batch/struct.Batch.html) struct. Returns
    /// `Ok()` **only if** all of the calls are successful. The order of the JSON `Value`s returned
    /// reflects the order in which the calls were made on the `Batch` struct.
    fn batch_call(&self, batch: &Batch) -> Result<Vec<serde_json::Value>, Error>;

    /// Subscribes to notifications for new block headers, by sending a `blockchain.headers.subscribe` call and
    /// returns the current tip as raw bytes instead of deserializing them.
    fn block_headers_subscribe_raw(&self) -> Result<RawHeaderNotification, Error>;

    /// Tries to pop one queued notification for a new block header that we might have received.
    /// Returns a the header in raw bytes if a notification is found in the queue, None otherwise.
    fn block_headers_pop_raw(&self) -> Result<Option<RawHeaderNotification>, Error>;

    /// Gets the raw bytes of block header for height `height`.
    fn block_header_raw(&self, height: usize) -> Result<Vec<u8>, Error>;

    /// Tries to fetch `count` block headers starting from `start_height`.
    fn block_headers(&self, start_height: usize, count: usize) -> Result<GetHeadersRes, Error>;

    /// Estimates the fee required in **Bitcoin per kilobyte** to confirm a transaction in `number` blocks.
    fn estimate_fee(&self, number: usize) -> Result<f64, Error>;

    /// Returns the minimum accepted fee by the server's node in **Bitcoin, not Satoshi**.
    fn relay_fee(&self) -> Result<f64, Error>;

    /// Subscribes to notifications for activity on a specific *scriptPubKey*.
    ///
    /// Returns a [`ScriptStatus`](../types/type.ScriptStatus.html) when successful that represents
    /// the current status for the requested script.
    ///
    /// Returns [`Error::AlreadySubscribed`](../types/enum.Error.html#variant.AlreadySubscribed) if
    /// already subscribed to the script.
    fn script_subscribe(&self, script: &ScriptPubkey) -> Result<Option<ScriptStatus>, Error>;

    /// Batch version of [`script_subscribe`](#method.script_subscribe).
    ///
    /// Takes a list of scripts and returns a list of script status responses.
    ///
    /// Note you should pass a reference to a collection because otherwise an expensive clone is made
    fn batch_script_subscribe<'s, I>(&self, scripts: I) -> Result<Vec<Option<ScriptStatus>>, Error>
    where
        I: IntoIterator + Clone,
        I::Item: Borrow<&'s ScriptPubkey>;

    /// Subscribes to notifications for activity on a specific *scriptPubKey*.
    ///
    /// Returns a `bool` with the server response when successful.
    ///
    /// Returns [`Error::NotSubscribed`](../types/enum.Error.html#variant.NotSubscribed) if
    /// not subscribed to the script.
    fn script_unsubscribe(&self, script: &ScriptPubkey) -> Result<bool, Error>;

    /// Tries to pop one queued notification for a requested script. Returns `None` if there are no items in the queue.
    fn script_pop(&self, script: &ScriptPubkey) -> Result<Option<ScriptStatus>, Error>;

    /// Returns the balance for a *scriptPubKey*.
    fn script_get_balance(&self, script: &ScriptPubkey) -> Result<GetBalanceRes, Error>;

    /// Batch version of [`script_get_balance`](#method.script_get_balance).
    ///
    /// Takes a list of scripts and returns a list of balance responses.
    fn batch_script_get_balance<'s, I>(&self, scripts: I) -> Result<Vec<GetBalanceRes>, Error>
    where
        I: IntoIterator + Clone,
        I::Item: Borrow<&'s ScriptPubkey>;

    /// Returns the history for a *scriptPubKey*
    fn script_get_history(&self, script: &ScriptPubkey) -> Result<Vec<GetHistoryRes>, Error>;

    /// Batch version of [`script_get_history`](#method.script_get_history).
    ///
    /// Takes a list of scripts and returns a list of history responses.
    fn batch_script_get_history<'s, I>(&self, scripts: I) -> Result<Vec<Vec<GetHistoryRes>>, Error>
    where
        I: IntoIterator + Clone,
        I::Item: Borrow<&'s ScriptPubkey>;

    /// Returns the list of unspent outputs for a *scriptPubKey*
    fn script_list_unspent(&self, script: &ScriptPubkey) -> Result<Vec<ListUnspentRes>, Error>;

    /// Batch version of [`script_list_unspent`](#method.script_list_unspent).
    ///
    /// Takes a list of scripts and returns a list of a list of utxos.
    fn batch_script_list_unspent<'s, I>(
        &self,
        scripts: I,
    ) -> Result<Vec<Vec<ListUnspentRes>>, Error>
    where
        I: IntoIterator + Clone,
        I::Item: Borrow<&'s ScriptPubkey>;

    /// Return the unconfirmed transactions of a script hash.
    ///
    /// Added in version 1.1.
    fn script_get_mempool(&self, script: &ScriptPubkey) -> Result<Vec<GetMempoolRes>, Error>;

    /// Gets the raw bytes of a transaction with `txid`. Returns an error if not found.
    fn transaction_get_raw(&self, txid: &Txid) -> Result<Option<Vec<u8>>, Error>;

    /// Batch version of [`transaction_get_raw`](#method.transaction_get_raw).
    ///
    /// Takes a list of `txids` and returns a list of transactions raw bytes.
    fn batch_transaction_get_raw<'t, I>(&self, txids: I) -> Result<Vec<Vec<u8>>, Error>
    where
        I: IntoIterator + Clone,
        I::Item: Borrow<&'t Txid>;

    /// Batch version of [`block_header_raw`](#method.block_header_raw).
    ///
    /// Takes a list of `heights` of blocks and returns a list of block header raw bytes.
    fn batch_block_header_raw<I>(&self, heights: I) -> Result<Vec<Vec<u8>>, Error>
    where
        I: IntoIterator + Clone,
        I::Item: Borrow<u32>;

    /// Batch version of [`estimate_fee`](#method.estimate_fee).
    ///
    /// Takes a list of `numbers` of blocks and returns a list of fee required in
    /// **Satoshis per kilobyte** to confirm a transaction in the given number of blocks.
    fn batch_estimate_fee<I>(&self, numbers: I) -> Result<Vec<f64>, Error>
    where
        I: IntoIterator + Clone,
        I::Item: Borrow<usize>;

    /// Broadcasts the raw bytes of a transaction to the network.
    fn transaction_broadcast_raw(&self, raw_tx: &[u8]) -> Result<Txid, Error>;

    /// Returns the merkle path for the transaction `txid` confirmed in the block at `height`.
    fn transaction_get_merkle(&self, txid: &Txid, height: usize) -> Result<GetMerkleRes, Error>;

    /// Returns a transaction hash, given a block `height` and a `tx_pos` in the block.
    fn txid_from_pos(&self, height: usize, tx_pos: usize) -> Result<Txid, Error>;

    /// Returns a transaction hash and a merkle path, given a block `height` and a `tx_pos` in the
    /// block.
    fn txid_from_pos_with_merkle(
        &self,
        height: usize,
        tx_pos: usize,
    ) -> Result<TxidFromPosRes, Error>;

    /// Returns the capabilities of the server.
    fn server_features(&self) -> Result<ServerFeaturesRes, Error>;

    /// Pings the server. This method can also be used as a "dummy" call to trigger the processing
    /// of incoming block header or script notifications.
    fn ping(&self) -> Result<(), Error>;

    #[cfg(feature = "debug-calls")]
    /// Returns the number of network calls made since the creation of the client.
    fn calls_made(&self) -> Result<usize, Error>;
}

#[cfg(test)]
mod test {
    use super::ElectrumApi;
    use crate::{
        Batch, Error, GetBalanceRes, GetHeadersRes, GetHistoryRes, GetMempoolRes, GetMerkleRes,
        ListUnspentRes, Param, RawHeaderNotification, ScriptStatus, ServerFeaturesRes, TxRes,
        TxidFromPosRes,
    };
    use bp::{ScriptPubkey, Txid};
    use serde_json::Value;
    use std::borrow::Borrow;
    use std::{borrow::Cow, sync::Arc};

    #[derive(Debug, Clone)]
    struct FakeApi;

    #[allow(unused_variables)]
    impl ElectrumApi for FakeApi {
        fn transaction_get_verbose(&self, txid: &Txid) -> Result<Option<TxRes>, Error> {
            unreachable!()
        }

        fn raw_call(
            &self,
            method_name: &str,
            params: impl IntoIterator<Item = Param>,
        ) -> Result<Value, Error> {
            unreachable!()
        }

        fn batch_call(&self, batch: &Batch) -> Result<Vec<Value>, Error> {
            unreachable!()
        }

        fn block_headers_subscribe_raw(&self) -> Result<RawHeaderNotification, Error> {
            unreachable!()
        }

        fn block_headers_pop_raw(&self) -> Result<Option<RawHeaderNotification>, Error> {
            unreachable!()
        }

        fn block_header_raw(&self, height: usize) -> Result<Vec<u8>, Error> {
            unreachable!()
        }

        fn block_headers(&self, start_height: usize, count: usize) -> Result<GetHeadersRes, Error> {
            unreachable!()
        }

        fn estimate_fee(&self, number: usize) -> Result<f64, Error> {
            unreachable!()
        }

        fn relay_fee(&self) -> Result<f64, Error> {
            unreachable!()
        }

        fn script_subscribe(&self, script: &ScriptPubkey) -> Result<Option<ScriptStatus>, Error> {
            unreachable!()
        }

        fn batch_script_subscribe<'s, I>(
            &self,
            scripts: I,
        ) -> Result<Vec<Option<ScriptStatus>>, Error>
        where
            I: IntoIterator + Clone,
            I::Item: Borrow<&'s ScriptPubkey>,
        {
            unreachable!()
        }

        fn script_unsubscribe(&self, script: &ScriptPubkey) -> Result<bool, Error> {
            unreachable!()
        }

        fn script_pop(&self, script: &ScriptPubkey) -> Result<Option<ScriptStatus>, Error> {
            unreachable!()
        }

        fn script_get_balance(&self, script: &ScriptPubkey) -> Result<GetBalanceRes, Error> {
            unreachable!()
        }

        fn batch_script_get_balance<'s, I>(&self, scripts: I) -> Result<Vec<GetBalanceRes>, Error>
        where
            I: IntoIterator + Clone,
            I::Item: Borrow<&'s ScriptPubkey>,
        {
            unreachable!()
        }

        fn script_get_history(&self, script: &ScriptPubkey) -> Result<Vec<GetHistoryRes>, Error> {
            unreachable!()
        }

        fn batch_script_get_history<'s, I>(
            &self,
            scripts: I,
        ) -> Result<Vec<Vec<GetHistoryRes>>, Error>
        where
            I: IntoIterator + Clone,
            I::Item: Borrow<&'s ScriptPubkey>,
        {
            unreachable!()
        }

        fn script_list_unspent(&self, script: &ScriptPubkey) -> Result<Vec<ListUnspentRes>, Error> {
            unreachable!()
        }

        fn batch_script_list_unspent<'s, I>(
            &self,
            scripts: I,
        ) -> Result<Vec<Vec<ListUnspentRes>>, Error>
        where
            I: IntoIterator + Clone,
            I::Item: Borrow<&'s ScriptPubkey>,
        {
            unreachable!()
        }

        fn script_get_mempool(&self, script: &ScriptPubkey) -> Result<Vec<GetMempoolRes>, Error> {
            unreachable!()
        }

        fn transaction_get_raw(&self, txid: &Txid) -> Result<Option<Vec<u8>>, Error> {
            unreachable!()
        }

        fn batch_transaction_get_raw<'t, I>(&self, txids: I) -> Result<Vec<Vec<u8>>, Error>
        where
            I: IntoIterator + Clone,
            I::Item: Borrow<&'t Txid>,
        {
            unreachable!()
        }

        fn batch_block_header_raw<I>(&self, heights: I) -> Result<Vec<Vec<u8>>, Error>
        where
            I: IntoIterator + Clone,
            I::Item: Borrow<u32>,
        {
            unreachable!()
        }

        fn batch_estimate_fee<I>(&self, numbers: I) -> Result<Vec<f64>, Error>
        where
            I: IntoIterator + Clone,
            I::Item: Borrow<usize>,
        {
            unreachable!()
        }

        fn transaction_broadcast_raw(&self, raw_tx: &[u8]) -> Result<Txid, Error> {
            unreachable!()
        }

        fn transaction_get_merkle(
            &self,
            txid: &Txid,
            height: usize,
        ) -> Result<GetMerkleRes, Error> {
            unreachable!()
        }

        fn txid_from_pos(&self, height: usize, tx_pos: usize) -> Result<Txid, Error> {
            unreachable!()
        }

        fn txid_from_pos_with_merkle(
            &self,
            height: usize,
            tx_pos: usize,
        ) -> Result<TxidFromPosRes, Error> {
            unreachable!()
        }

        fn server_features(&self) -> Result<ServerFeaturesRes, Error> {
            unreachable!()
        }

        fn ping(&self) -> Result<(), Error> {
            unreachable!()
        }

        #[cfg(feature = "debug-calls")]
        /// Returns the number of network calls made since the creation of the client.
        fn calls_made(&self) -> Result<usize, Error> {
            unreachable!()
        }
    }

    fn is_impl<A: ElectrumApi>() {}

    #[test]
    fn deref() {
        is_impl::<FakeApi>();
        is_impl::<&FakeApi>();
        is_impl::<Arc<FakeApi>>();
        is_impl::<Box<FakeApi>>();
        is_impl::<Cow<FakeApi>>();
    }
}
