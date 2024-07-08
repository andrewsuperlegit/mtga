#![warn(missing_docs)]
//! My very own implementation of Magic the Gathering as an exercise in (futility and)
//! learning Rust! More documentation to follow!
#![allow(unused)]
mod colors;
mod cost;
mod card;
mod card_db;
mod library;
mod r#state_manager;

use serde::Deserialize;
use strum::VariantNames;
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;
use redux_rs::{Store};
use card_db::get_card_db;
use crate::state_manager::{Action, GameState, reducer};


#[tokio::main]
async fn main() {
	let store = Store::new(reducer);
	store.subscribe(|state: &GameState| println!("New state: {:?}", state)).await;
}

