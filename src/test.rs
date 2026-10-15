#![cfg(test)]
use super::*;
use soroban_sdk::{testutils::Address as _, Env};

fn setup() -> (Env, AcuerdoClaroClient<'static>, Address, Address) {
    let env = Env::default();
    env.mock_all_auths();
    let id = env.register(AcuerdoClaro, ());
    let client = AcuerdoClaroClient::new(&env, &id);
    (env.clone(), client, Address::generate(&env), Address::generate(&env))
}

#[test]
fn create_and_accept() {
    let (env, c, a, b) = setup();
    let hash = BytesN::from_array(&env, &[7u8; 32]);
    let id = c.create(&a, &b, &String::from_str(&env, "Logo"), &hash);
    assert_eq!(id, 1);
    assert_eq!(c.get(&id).status, Status::Pending);
    c.accept(&b, &id);
    assert_eq!(c.get(&id).status, Status::Accepted);
    assert!(c.verify(&id, &hash));
}

#[test]
fn only_counterparty_accepts() {
    let (env, c, a, b) = setup();
    let hash = BytesN::from_array(&env, &[1u8; 32]);
    let id = c.create(&a, &b, &String::from_str(&env, "Demo"), &hash);
    assert!(c.try_accept(&a, &id).is_err());
}

#[test]
fn cancel_blocks_accept() {
    let (env, c, a, b) = setup();
    let hash = BytesN::from_array(&env, &[2u8; 32]);
    let id = c.create(&a, &b, &String::from_str(&env, "Show"), &hash);
    c.cancel(&a, &id);
    assert!(c.try_accept(&b, &id).is_err());
}
