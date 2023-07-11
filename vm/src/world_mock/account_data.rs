use num_bigint::BigUint;

use super::AccountEsdt;
use crate::{display_util::key_hex, types::VMAddress};
use std::{collections::HashMap, fmt, fmt::Write};

pub type AccountStorage = HashMap<Vec<u8>, Vec<u8>>;

#[derive(Clone, Debug)]
pub struct AccountData {
    pub address: VMAddress,
    pub nonce: u64,
    pub egld_balance: BigUint,
    pub esdt: AccountEsdt,
    pub storage: AccountStorage,
    pub username: Vec<u8>,
    pub contract_path: Option<Vec<u8>>,
    pub contract_owner: Option<VMAddress>,
    pub developer_rewards: BigUint,
}

impl fmt::Display for AccountData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut storage_buf = String::new();
        let mut storage_keys: Vec<Vec<u8>> = self.storage.keys().cloned().collect();
        storage_keys.sort();

        for key in &storage_keys {
            let value = self.storage.get(key).unwrap();
            write!(
                storage_buf,
                "\n\t\t\t{} -> 0x{}",
                key_hex(key.as_slice()),
                hex::encode(value.as_slice())
            )
            .unwrap();
        }

        write!(
            f,
            "AccountData {{
		nonce: {},
		balance: {},
		esdt: [{} ],
		username: {},
		storage: [{} ],
		developerRewards: {},
	}}",
            self.nonce,
            self.egld_balance,
            self.esdt,
            String::from_utf8(self.username.clone()).unwrap(),
            storage_buf,
            self.developer_rewards
        )
    }
}
