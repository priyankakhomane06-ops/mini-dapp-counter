#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn test_increment() {
        let env = Env::default();
        let result = CounterContract::increment(env.clone(), 5);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_multiple_increment() {
        let env = Env::default();
        CounterContract::increment(env.clone(), 5);
        let result = CounterContract::increment(env.clone(), 3);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_get() {
        let env = Env::default();
        CounterContract::increment(env.clone(), 2);
        let result = CounterContract::get(env.clone());
        assert_eq!(result, 2);
    }
}