use anyhow::Result;
use serde::{Serialize, Deserialize};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::account::Account;

use super::Network;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct AccountSchema {
    pub pubkey: Pubkey,
    pub network: Network,
    pub lamports: u64,
    pub data: Vec<u8>,
    pub owner: Pubkey,
    pub executable: bool,
    pub rent_epoch: u64
}

impl AccountSchema {

    pub fn from_account(account: &Account, pubkey: &Pubkey, network: &Network) -> Result<Self> {
        Ok(Self {
            pubkey: *pubkey,
            network: network.clone(),
            lamports: account.lamports,
            data: account.data.clone(),
            owner: account.owner,
            executable: account.executable,
            rent_epoch: account.rent_epoch,
        })
    }

    pub fn to_account(self) -> Result<Account> {
        Ok(Account {
            lamports: self.lamports,
            data: self.data,
            owner: self.owner,
            executable: self.executable,
            rent_epoch: self.rent_epoch,
        })
    }

    pub fn get_program_executable_data_address(&self) -> Result<Pubkey> {
        let mut executable_data_bytes = [0u8;32];
        executable_data_bytes.copy_from_slice(&self.data[4..36]);
        Ok(Pubkey::new_from_array(executable_data_bytes))
    }

    pub fn get_pubkey(&self) -> Pubkey {
        self.pubkey
    }

    pub fn get_network(&self) -> Network {
        self.network.clone()
    }
}