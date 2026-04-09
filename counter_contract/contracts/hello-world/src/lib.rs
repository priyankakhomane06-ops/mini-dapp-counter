#![no_std]

use soroban_sdk::{contract, contractimpl, Env, symbol_short};

#[cfg(test)]
extern crate std;

#[contract]
pub struct CounterContract;

#[contractimpl]
impl CounterContract {

    pub fn increment(env: Env, value: i32) -> i32 {
        let key = symbol_short!("COUNT");

        let mut count: i32 = env.storage().instance().get(&key).unwrap_or(0);
        count += value;

        env.storage().instance().set(&key, &count);
        count
    }

    pub fn get(env: Env) -> i32 {
        let key = symbol_short!("COUNT");
        env.storage().instance().get(&key).unwrap_or(0)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{Env};

    #[test]
    fn test_increment() {
        let env = Env::default();
        let contract_id = env.register(CounterContract, ());
        let client = CounterContractClient::new(&env, &contract_id);

        let result = client.increment(&5);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_multiple_increment() {
        let env = Env::default();
        let contract_id = env.register(CounterContract, ());
        let client = CounterContractClient::new(&env, &contract_id);

        client.increment(&5);
        let result = client.increment(&3);

        assert_eq!(result, 8);
    }

    #[test]
    fn test_get() {
        let env = Env::default();
        let contract_id = env.register(CounterContract, ());
        let client = CounterContractClient::new(&env, &contract_id);

        client.increment(&2);
        let result = client.get();

        assert_eq!(result, 2);
    }
}