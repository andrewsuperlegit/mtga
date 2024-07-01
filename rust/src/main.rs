#![warn(missing_docs)]
//! My very own implementation of Magic the Gathering as an exercise in (futility and)
//! learning Rust! More documentation to follow!
#![allow(unused)]
mod colors;
mod cost;
mod card;
mod card_db;

use serde::Deserialize;
use std::collections::HashMap;
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


/// Library is the LIBRARY which can be like... shuffled and stuff
/// cards is just a hashmap of cards so that when someone searches
/// for cards, we can give them the option to type the card theyre
/// looking for and look it up in constant time.
/// sideboard is optional
struct Library<'a> {
	library: Vec<RealCard<'a>>,
	cards: HashMap<String, Card>,
	sideboard: Vec<Option<Card>>,

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


fn populate_library(){
	todo!("Write the parser that accepts a vector of card names \
	and searches the CardDB for those cards, and adds them to \
	a players library");
}


use std::time::Instant;
use card::Card;
use crate::card::RealCard;
use crate::card_db::{get_card_db, get_card_db_slow};


/// just testing that this works.
pub fn example(){
	// Use a list of cards to populate library
	let vec = vec!["Artificer's Dragon", "First Response", "Gaea's Gift",
		"Jeskai Banner", "Mind's Eye"];

	let res:Vec<RealCard> = vec.iter().map(|name| RealCard::new(name)).collect();

	println!("{:#?}", res[2]);
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

