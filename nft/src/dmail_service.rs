use ic_kit::macros::*;

use crate::{types::{TokenIndex, into_token_index, TokenIdentifier}, utils::ledger};

#[update]
fn bind(token_identifier: TokenIdentifier) -> bool{
    let token_index:TokenIndex = into_token_index(&token_identifier);
    ledger().bind_alias_for_using_dmail(token_index)
}
