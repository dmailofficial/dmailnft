use ic_kit::{candid::CandidType, ic, Principal};
use serde::Deserialize;

#[derive(CandidType, Deserialize)]
pub struct Fleek(pub Vec<Principal>);

impl Default for Fleek {
    fn default() -> Self {
        panic!()
    }
}

pub fn is_fleek(account: &Principal) -> bool {
    let controller_list = ic::get::<Fleek>()
        .0
        .iter()
        .cloned()
        .map(|p| p.to_text())
        .collect::<Vec<_>>()
        .join(",");

    ic_cdk::println!("controller_list------:{:?}", controller_list);
    ic_cdk::println!("caller----:{:?}", account.to_text());
    ic::get::<Fleek>().0.contains(account)
}
