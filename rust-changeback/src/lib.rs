//! This contract implements simple Change backed by storage on blockchain.
//!
//! The contract provides methods to [add] / [change] Change and
//! [get it's current value][get_num] or [reset].
//!
//! [add]: struct.Change.html#method.add
//! [change]: struct.Change.html#method.change
//! [get_num]: struct.Change.html#method.get_num
//! [reset]: struct.Change.html#method.reset

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};

near_sdk::setup_alloc!();

// add the following attributes to prepare your code for serialization and invocation on the blockchain
// More built-in Rust attributes here: https://doc.rust-lang.org/reference/attributes.html#built-in-attributes-index
#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Change {
    // See more data types at https://doc.rust-lang.org/book/ch03-02-data-types.html
    val: i32, // i32 is signed. unsigned integers are also available: u8, i32, u32, u64, u128
}

#[near_bindgen]
impl Change {
    /// Returns 32-bit signed integer of the Change value.
    ///
    /// This must match the type from our struct's 'val' defined above.
    ///
    /// Note, the parameter is `&self` (without being mutable) meaning it doesn't modify state.
    /// In the frontend (/src/main.js) this is added to the "viewMethods" array
    /// using near-cli we can call this by:
    ///
    /// ```bash
    /// near view Change.YOU.testnet get_num
    /// ```
    pub fn get_num(&self) -> i32 {
        return self.val;
    }

    /// add the Change.
    ///
    /// Note, the parameter is "&mut self" as this function modifies state.
    /// In the frontend (/src/main.js) this is added to the "changeMethods" array
    /// using near-cli we can call this by:
    ///
    /// ```bash
    /// near call Change.YOU.testnet add --accountId donation.YOU.testnet
    /// ```
    pub fn add(&mut self) {
        // note: adding one like this is an easy way to accidentally overflow
        // real smart contracts will want to have safety checks
        // e.g. self.val = i32::wrapping_add(self.val, 1);
        // https://doc.rust-lang.org/std/primitive.i32.html#method.wrapping_add
        self.val += 1000;
        let log_message = format!("Added money to {}", self.val);
        env::log(log_message.as_bytes());
        after_counter_change();
    }

    /// change (subtract from) the Change.
    ///
    /// In (/src/main.js) this is also added to the "changeMethods" array
    /// using near-cli we can call this by:
    ///
    /// ```bash
    /// near call Change.YOU.testnet change --accountId donation.YOU.testnet
    /// ```
    pub fn change(&mut self) {
        // note: subtracting one like this is an easy way to accidentally overflow
        // real smart contracts will want to have safety checks
        // e.g. self.val = i32::wrapping_sub(self.val, 1);
        // https://doc.rust-lang.org/std/primitive.i32.html#method.wrapping_sub
        self.val -= 10;
        let log_message = format!("Value after change {}", self.val);
        env::log(log_message.as_bytes());
        after_counter_change();
    }

    /// Reset to zero.
    pub fn reset(&mut self) {
        self.val = 0;
        // Another way to log is to cast a string into bytes, hence "b" below:
        env::log(b"Reset Change to zero");
    }
}

// unlike the struct's functions above, this function cannot use attributes #[derive(…)] or #[near_bindgen]
// any attempts will throw helpful warnings upon 'cargo build'
// while this function cannot be invoked directly on the blockchain, it can be called from an invoked function
fn after_counter_change() {
    // show helpful warning that i32 (8-bit signed integer) will overflow above 127 or below -128
    env::log("Make sure you don't overflow, my friend.".as_bytes());
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-Change-tutorial -- --nocapture
 * Note: 'rust-Change-tutorial' comes from cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    // part of writing unit tests is setting up a mock context
    // in this example, this is only needed for env::log in the contract
    // this is also a useful list to peek at when wondering what's available in env::*
    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "robert.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "jane.testnet".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    // mark individual unit tests with #[test] for them to be registered and fired
    #[test]
    fn add() {
        // set up the mock context into the testing environment
        let context = get_context(vec![], false);
        testing_env!(context);
        // instantiate a contract variable with the Change at zero
        let mut contract = Change { val: 0 };
        contract.add();
        println!("Value after add: {}", contract.get_num());
        // confirm that we received 1 when calling get_num
        assert_eq!(1000, contract.get_num());
    }

    #[test]
    fn change() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Change { val: 0 };
        contract.change();
        println!("Value after change: {}", contract.get_num());
        // confirm that we received -1 when calling get_num
        assert_eq!(-10, contract.get_num());
    }

    #[test]
    fn add_and_reset() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Change { val: 0 };
        contract.add();
        contract.reset();
        println!("Value after reset: {}", contract.get_num());
        // confirm that we received -1 when calling get_num
        assert_eq!(0, contract.get_num());
    }
}