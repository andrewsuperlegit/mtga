#![warn(missing_docs)]
//! My very own implementation of Magic the Gathering as an exercise in (futility and)
//! learning Rust! More documentation to follow!
#![allow(unused)]
mod colors;
mod cost;
mod card;
mod card_db;
mod deck;
mod state_manager;
mod selectors;
mod reducers;
mod example_decks;


use std::thread::sleep;
use std::time::Duration;
use redux_rs::Store;
use reducers::{Action, reducer};
use selectors::{SelectFirstPlayer, SelectPlayerCount, SelectPlayerNames};
use crate::example_decks::build_blakes_example_deck;
use crate::deck::Deck;
use crate::state_manager::{GameState, Player};


#[tokio::main]
async fn main() {
	let gs = GameState::new();
	let store = Store::new_with_state(reducer, gs);
	let blakes_cards = build_blakes_example_deck();
	let blakes_sideboard = &vec![];
	let blakes_deck = Deck::new(Player{name: "Blake".to_string()}, &blakes_cards, &blakes_sideboard);

	store.subscribe(|state: &GameState | println!("New state: {:#?}", state)).await;

	println!("{:#?}", blakes_deck);

	println!("There are {} players", store.select(SelectPlayerCount).await);
	println!("Players are named: {:?} ", store.select(SelectPlayerNames).await);
	println!("Players are named: {:?} ", store.select(SelectFirstPlayer).await);
	sleep(Duration::new(2,0));
	store.dispatch(Action::AddPlayer("Andrew".to_string())).await;
	println!("There are {} players", store.select(SelectPlayerCount).await);
	println!("Players are named: {:?} ", store.select(SelectPlayerNames).await);
	// todo implement the front end so we can start playing the game!
}

