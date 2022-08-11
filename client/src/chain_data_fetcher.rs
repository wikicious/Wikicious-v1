use std::sync::{Arc, RwLock};
use std::thread;
use std::time::{Duration, Instant};

use crate::chain_data::*;

use anchor_lang::Discriminator;

use mango_v4::accounts_zerocopy::LoadZeroCopy;
use mango_v4::state::{MangoAccount, MangoAccountValue};

use anyhow::Context;

use solana_client::rpc_client::RpcClient;
use solana_sdk::account::{AccountSharedData, ReadableAccount};
use solana_sdk::clock::Slot;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;

pub struct AccountFetcher {
    pub chain_data: Arc<RwLock<ChainData>>,
    pub rpc: RpcClient,
}

impl AccountFetcher {
    // loads from ChainData
    pub fn fetch<T: anchor_lang::ZeroCopy + anchor_lang::Owner>(
        &self,
        address: &Pubkey,
    ) -> anyhow::Result<T> {
        Ok(self
            .fetch_raw(address)?
            .load::<T>()
            .with_context(|| format!("loading account {}", address))?
            .clone())
    }

    pub fn fetch_mango_account(&self, address: &Pubkey) -> anyhow::Result<MangoAccountValue> {
        let acc = self.fetch_raw(address)?;

        let data = acc.data();
        let disc_bytes = &data[0..8];
        if disc_bytes != &MangoAccount::discriminator() {
            anyhow::bail!("not a mango account at {}", address);
        }

        Ok(MangoAccountValue::from_bytes(&data[8..])
            .with_context(|| format!("loading mango account {}", address))?)
    }

    // fetches via RPC, stores in ChainData, returns new version
    pub fn fetch_fresh<T: anchor_lang::ZeroCopy + anchor_lang::Owner>(
        &self,
        address: &Pubkey,
    ) -> anyhow::Result<T> {
        self.refresh_account_via_rpc(address)?;
        self.fetch(address)
    }

    pub fn fetch_fresh_mango_account(&self, address: &Pubkey) -> anyhow::Result<MangoAccountValue> {
        self.refresh_account_via_rpc(address)?;
        self.fetch_mango_account(address)
    }

    pub fn fetch_raw(&self, address: &Pubkey) -> anyhow::Result<AccountSharedData> {
        let chain_data = self.chain_data.read().unwrap();
        Ok(chain_data
            .account(address)
            .with_context(|| format!("fetch account {} via chain_data", address))?
            .clone())
    }

    pub fn refresh_account_via_rpc(&self, address: &Pubkey) -> anyhow::Result<Slot> {
        let response = self
            .rpc
            .get_account_with_commitment(&address, self.rpc.commitment())
            .with_context(|| format!("refresh account {} via rpc", address))?;
        let slot = response.context.slot;
        let account = response
            .value
            .ok_or(anchor_client::ClientError::AccountNotFound)
            .with_context(|| format!("refresh account {} via rpc", address))?;

        let mut chain_data = self.chain_data.write().unwrap();
        chain_data.update_from_rpc(
            address,
            AccountAndSlot {
                slot: response.context.slot,
                account: account.into(),
            },
        );

        Ok(slot)
    }

    /// Return the maximum slot reported for the processing of the signatures
    pub fn transaction_max_slot(&self, signatures: &[Signature]) -> anyhow::Result<Slot> {
        let statuses = self.rpc.get_signature_statuses(signatures)?.value;
        Ok(statuses
            .iter()
            .map(|status_opt| status_opt.as_ref().map(|status| status.slot).unwrap_or(0))
            .max()
            .unwrap_or(0))
    }

    /// Return success once all addresses have data >= min_slot
    pub fn refresh_accounts_via_rpc_until_slot(
        &self,
        addresses: &[Pubkey],
        min_slot: Slot,
        timeout: Duration,
    ) -> anyhow::Result<()> {
        let start = Instant::now();
        for address in addresses {
            loop {
                if start.elapsed() > timeout {
                    anyhow::bail!(
                        "timeout while waiting for data for {} that's newer than slot {}",
                        address,
                        min_slot
                    );
                }
                let data_slot = self.refresh_account_via_rpc(address)?;
                if data_slot >= min_slot {
                    break;
                }
                thread::sleep(Duration::from_millis(500));
            }
        }
        Ok(())
    }
}

impl crate::AccountFetcher for AccountFetcher {
    fn fetch_raw_account(&self, address: Pubkey) -> anyhow::Result<solana_sdk::account::Account> {
        self.fetch_raw(&address).map(|a| a.into())
    }
}