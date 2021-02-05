use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::ValidAccountId;
use near_sdk::{env, log, near_bindgen, Promise};

#[global_allocator]
static ALLOC: near_sdk::wee_alloc::WeeAlloc<'_> = near_sdk::wee_alloc::WeeAlloc::INIT;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct PromiseA {

}

#[near_bindgen]
impl PromiseA {
    pub fn call_b(&mut self, account_id: ValidAccountId) -> Promise {
        log!("Calling B at @{} from A @{}", account_id.as_ref(), env::current_account_id());
        // callback_receiver is expected to be called by a callback so error happens when calling directly
        // Error: Accessed invalid promise result index: 0
        let _another_call = Promise::new(env::current_account_id()).function_call(
            b"callback_receiver".to_vec(),
            b"{}".to_vec(),
            0,
            5_000_000_000_000
        );
        // Defined a callback to execute after promises are resolved
        let callback_a = Promise::new(env::current_account_id()).function_call(
            b"callback_receiver".to_vec(),
            b"{}".to_vec(),
            0,
            5_000_000_000_000
        );
        // Create 2 crosscontract call promises to execute in //
        let promise_a = Promise::new(account_id.into()).function_call(
            b"return_ok".to_vec(),
            b"{}".to_vec(),
            0,
            5_000_000_000_000
        );
        let promise_a_bis = Promise::new(env::current_account_id()).function_call(
            b"return_ok".to_vec(),
            b"{}".to_vec(),
            0,
            5_000_000_000_000
        );
        // Join the promises to execute a callback after both have executed
        let joined_promises = promise_a.and(promise_a_bis);
        // defines the order of cross contract calls and use the promise_a result in callback_a
        joined_promises.then(callback_a.clone());
        // function returns the result of the callback
        callback_a
    }

    pub fn callback_receiver(&mut self, #[callback] promise_a_result: String, #[callback] promise_a_bis_result: String ) -> String {
        log!(
            "I'm A at @{}, called back by B @{}, promise_a_result: {}/{}",
            env::predecessor_account_id(),
            env::current_account_id(),
            promise_a_result,
            promise_a_bis_result
        );
        "Callback".to_string()
    }

    pub fn return_ok(&mut self) -> String {
        log!(
            "return ok I'm B at @{}, called by A @{}",
            env::current_account_id(),
            env::predecessor_account_id()
        );
        "OK".to_string()
    }

    pub fn return_ok2(&mut self) -> String {
        log!(
            "return ok2 I'm B at @{}, called by A @{}",
            env::current_account_id(),
            env::predecessor_account_id()
        );
        "OK2".to_string()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
