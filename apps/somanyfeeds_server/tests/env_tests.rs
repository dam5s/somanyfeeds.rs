use somanyfeeds_server::env::{load_env_num, load_env_str};
use std::env;

#[test]
fn test_load_env_str_default() {
    let key = "TEST_ENV_VAR_STR_DEFAULT";
    unsafe {
        env::remove_var(key);
    }
    assert_eq!(load_env_str(key, "default".to_string()), "default");
}

#[test]
fn test_load_env_str_present() {
    let key = "TEST_ENV_VAR_STR_PRESENT";
    unsafe {
        env::set_var(key, "present");
    }
    assert_eq!(load_env_str(key, "default".to_string()), "present");
    unsafe {
        env::remove_var(key);
    }
}

#[test]
fn test_load_env_num_default() {
    let key = "TEST_ENV_VAR_DEFAULT";
    unsafe {
        env::remove_var(key);
    }
    assert_eq!(load_env_num(key, 42), 42);
}

#[test]
fn test_load_env_num_present() {
    let key = "TEST_ENV_VAR_PRESENT";
    unsafe {
        env::set_var(key, "123");
    }
    assert_eq!(load_env_num(key, 42), 123);
    unsafe {
        env::remove_var(key);
    }
}

#[test]
#[should_panic(expected = "TEST_ENV_VAR_INVALID must be a valid number")]
fn test_load_env_num_invalid() {
    let key = "TEST_ENV_VAR_INVALID";
    unsafe {
        env::set_var(key, "abc");
    }
    load_env_num::<i32>(key, 42);
}
