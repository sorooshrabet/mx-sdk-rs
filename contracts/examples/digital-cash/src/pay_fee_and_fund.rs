multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::{constants::*, helpers, storage};

#[multiversx_sc::module]
pub trait PayFeeAndFund: storage::StorageModule + helpers::HelpersModule {
    #[endpoint]
    #[payable("*")]
    fn pay_fee_and_fund_esdt(&self, address: ManagedAddress, valability: u64) {
        let mut payments = self.call_value().all_esdt_transfers().clone_value();
        let fee = EgldOrEsdtTokenPayment::from(payments.get(0));
        let caller_address = self.blockchain().get_caller();
        self.update_fees(caller_address, &address, fee);

        payments.remove(0);

        self.make_fund(0u64.into(), payments, address, valability)
    }
    #[endpoint]
    #[payable("EGLD")]
    fn pay_fee_and_fund_egld(&self, address: ManagedAddress, valability: u64) {
        let mut fund = self.call_value().egld_value().clone_value();
        let fee_value = self.fee().get();
        require!(fund > fee_value, "payment not covering fees");

        fund -= fee_value.clone();
        let fee = EgldOrEsdtTokenPayment::new(EgldOrEsdtTokenIdentifier::egld(), 0, fee_value);
        let caller_address = self.blockchain().get_caller();
        self.update_fees(caller_address, &address, fee);

        self.make_fund(fund, ManagedVec::new(), address, valability);
    }

    #[endpoint]
    #[payable("*")]
    fn fund(&self, address: ManagedAddress, valability: u64) {
        let deposit_mapper = self.deposit(&address);
        require!(!deposit_mapper.is_empty(), FEES_NOT_COVERED_ERR_MSG);
        let depositor = deposit_mapper.get().depositor_address;
        require!(
            self.blockchain().get_caller() == depositor,
            "invalid depositor"
        );
        let egld_payment = self.call_value().egld_value().clone_value();
        let esdt_payment = self.call_value().all_esdt_transfers().clone_value();
        self.make_fund(egld_payment, esdt_payment, address, valability)
    }

    #[endpoint(depositFees)]
    #[payable("EGLD")]
    fn deposit_fees(&self, address: &ManagedAddress) {
        let payment = self.call_value().egld_or_single_esdt();
        let caller_address = self.blockchain().get_caller();
        self.update_fees(caller_address, address, payment);
    }
}
