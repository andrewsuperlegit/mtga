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
use crate::state_manager::{Action, GameState, reducer, SelectPlayerCount, SelectPlayerNames, SelectFirstPlayer, CurrentEvent, Player, EventSource};


#[tokio::main]
async fn main() {
	let gs = GameState::new();
	let store = Store::new_with_state(reducer, gs);

	store.subscribe(|state: &GameState | println!("New state: {:#?}", state)).await;

	println!("There are {} players", store.select(SelectPlayerCount).await);
	println!("Players are named: {:?} ", store.select(SelectPlayerNames).await);
	println!("Players are named: {:?} ", store.select(SelectFirstPlayer).await);
	sleep(Duration::new(2,0));
	store.dispatch(Action::AddPlayer("Andrew".to_string())).await;
	println!("There are {} players", store.select(SelectPlayerCount).await);
	println!("Players are named: {:?} ", store.select(SelectPlayerNames).await);
	// todo implement the front end so we can start playing the game!
}

