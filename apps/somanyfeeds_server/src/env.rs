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
