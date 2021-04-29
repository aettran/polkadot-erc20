#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod polkadot_erc20 {

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct PolkadotErc20 {
        /// The total supply of tokens
        total_supply: Balance,
        /// Store the balance for each user
        balances: ink_storage::collections::HashMap<AccountId, Balance>,
    }

    impl PolkadotErc20 {
        /// Constructor that initializes the contract with initial supply.
        #[ink(constructor)]
        pub fn new(initial_supply: Balance) -> Self {
            let mut balances = ink_storage::collections::HashMap::new();
            balances.insert(Self::env().caller(), initial_supply);
            let instance = Self {
                total_supply: initial_supply,
                balances
            };
            instance
        }

        /// A message that can be called on instantiated contracts.
        /// This will retreive the total supply
        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
          self.total_supply
        }

        /// A message that can be called on instantiated contracts.
        /// This will retrieve the balance of an account/user
        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
          self.balance_of_or_zero(&owner)
        }

        /// A message that can be called on instantiated contracts.
        /// This will tranfer the tokens to account/user
        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> bool {
            let from = self.env().caller();
            self.transfer_from_to(from, to, value)
        }

        /// A private function to transfer the tokens from and to account/user
        fn transfer_from_to(&mut self, from: AccountId, to: AccountId, value: Balance) -> bool {
            let from_balance = self.balance_of(from);
            if from_balance < value {
                return false;
            }
            self.balances.insert(from, from_balance - value);
            let to_balance = self.balance_of(to);
            self.balances.insert(to, to_balance + value);
            return true;

            // ACTION: Get the balance for `from` and `to`
            //   HINT: Use the `balance_of_or_zero` function to do this
            // ACTION: If `from_balance` is less than `value`, return `false`
            // ACTION: Insert new values for `from` and `to`
            //         * from_balance - value
            //         * to_balance + value
            // ACTION: Return `true`
        }

        /// A private function to retreive balance from an account
        fn balance_of_or_zero(&self, owner: &AccountId) -> Balance {
            *self.balances.get(owner).unwrap_or(&0)
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        use super::*;

        use ink_lang as ink;

        #[ink::test]
        fn new_works() {
            let contract = PolkadotErc20::new(999);
            assert_eq!(contract.total_supply(), 999);
        }

        #[ink::test]
        fn balance_works() {
            let contract = PolkadotErc20::new(100);
            assert_eq!(contract.total_supply(), 100);
            assert_eq!(contract.balance_of(AccountId::from([0x1; 32])), 100);
            assert_eq!(contract.balance_of(AccountId::from([0x0; 32])), 0);
        }

        #[ink::test]
        fn transfer_works() {
            let mut contract = PolkadotErc20::new(100);
            assert_eq!(contract.balance_of(AccountId::from([0x1; 32])), 100);
            assert!(contract.transfer(AccountId::from([0x0; 32]), 10));
            assert_eq!(contract.balance_of(AccountId::from([0x0; 32])), 10);
            assert!(!contract.transfer(AccountId::from([0x0; 32]), 100));
        }
    }
}
