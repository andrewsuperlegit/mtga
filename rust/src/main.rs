#![warn(missing_docs)]
//! My very own implementation of Magic the Gathering as an exercise in (futility and)
//! learning Rust! More documentation to follow!
#![allow(unused)]
mod colors;
mod cost;
mod card;
mod card_db;
mod library;
mod state_manager;


use std::thread::sleep;
use std::time::Duration;
use redux_rs::{Store};
use crate::state_manager::{Action, GameState, reducer};


#[tokio::main]
async fn main() {
	let store = Store::new(reducer);
	store.subscribe(|state: &GameState| println!("New state: {:?}", state)).await;
	// todo implement the front end so we can start playing the game!
}

