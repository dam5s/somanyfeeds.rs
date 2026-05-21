use std::fmt::Debug;
use std::str::FromStr;

pub fn load_env_num<T>(key: &str, default: T) -> T
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    match std::env::var(key) {
        Ok(val) => val
            .parse()
            .unwrap_or_else(|_| panic!("{} must be a valid number", key)),
        Err(_) => default,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_load_env_num_default() {
        let key = "TEST_ENV_VAR_DEFAULT";
        unsafe { env::remove_var(key); }
        assert_eq!(load_env_num(key, 42), 42);
    }

    #[test]
    fn test_load_env_num_present() {
        let key = "TEST_ENV_VAR_PRESENT";
        unsafe { env::set_var(key, "123"); }
        assert_eq!(load_env_num(key, 42), 123);
        unsafe { env::remove_var(key); }
    }

    #[test]
    #[should_panic(expected = "TEST_ENV_VAR_INVALID must be a valid number")]
    fn test_load_env_num_invalid() {
        let key = "TEST_ENV_VAR_INVALID";
        unsafe { env::set_var(key, "abc"); }
        load_env_num::<i32>(key, 42);
    }
}
