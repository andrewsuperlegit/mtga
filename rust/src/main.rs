#![warn(missing_docs)]
//! My very own implementation of Magic the Gathering as an exercise in (futility and)
//! learning Rust! More documentation to follow!
#![allow(unused)]
mod colors;
mod cost;
mod card;
mod card_db;
mod library;

use serde::Deserialize;
use strum::VariantNames;
use std::error::Error;

enum MetaGamePhase{
	ChoosePlayerCount,
	ConfirmLibrary,
	ChooseFirstTurn,
	InitialDraw,
	MulliganChoice,
	ActiveGame,
	EndGame,
	DeclareWinner
}

// in the future, based on the player count,
// we should have to populate this
// probably with a macro?
enum Turn{
	Player1,
	Player2
}

enum TurnPhase{
	Untap,
	Upkeep,
	Draw,
	PreCombatMainPhase,
	CombatPhase,
	DeclareAttackers,
	DeclareBlockers,
	CombatResolution,
	EndCombat,
	PostCombatMainPhase,
	End,
	Cleanup
}

struct Player{
	name: String
}

// these arrays will have to grow/shrink depending on the
// number of players
struct GameState<'a> {
	libraries: [Library<'a>; 2],
	graveyards: [Vec<Card>; 2],
	hands: [Vec<Card>; 2],
	exiles: [Vec<Card>; 2],
	meta_game_phase: MetaGamePhase,
	turn: Turn,
	turn_phase: TurnPhase,
	player_count: u8,
	player_names: [String; 2],
	first_player: Player,
}


use card::Card;
use card_db::get_card_db;
use library::{CardListItem, Library};

/// just testing that this works.
pub fn example(){
	// Use a list of cards to populate library
	// let vec = vec![
	// 	CardListItem("Artificer's Dragon".to_string(), 1),
	// 	CardListItem("First Response".to_string(), 1),
	// 	CardListItem("Gaea's Gift".to_string(), 1),
	// 	CardListItem("Jeskai Banner".to_string(), 1),
	// 	CardListItem("Mind's Eye".to_string(), 3)
	// ];
	// let vec_b= vec![];
	// let lib = Library::new(&vec, &vec_b).unwrap();
	// println!("{:#?}", lib);

}

fn main() {
	let db = get_card_db();
	example();
	// println!("{:#?}", cards.get("Advice from the Fae"));
	// println!("\n{:#?}", cards.get("Reaper King"));
	// println!("{}", cards.library.len());

	// let card = cards.get("Returned Pastcaller");
	// println!("{:#?}", card.unwrap().mana_cost.cost);

}

