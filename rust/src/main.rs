#![allow(warnings)]
mod colors;
mod cost;
mod card;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use card::CardType::Land;
use std::str::FromStr;
use strum::VariantNames;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Deserialize)]
enum TapPurpose{
	Mana,
	Action
}
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

#[derive(Debug, Deserialize)]
struct CardDB {
	library: HashMap<String, Vec<Card>>,
}

fn get_file(filename: &str) -> File{
	match File::open(filename){
		Ok(file) => file,
		Err(error) => panic!("Something went wrong opening the mtg\
			card data: {}", error)
	}
}

impl CardDB {
	fn new() -> CardDB{
		let filename = "src\\noForeignModernAtomic-rust.json";
		let file = get_file(filename);
		let reader = BufReader::new(file);
		let serde_result = serde_json::from_reader(reader);

		let card_db = match serde_result{
			Ok(db) => db,
			Err(error) => panic!("Something went wrong trying to create\
			CardDB: {}", error)
		};
		card_db
	}

	fn get(&self, card_name: &str) -> Result<&Card, Box<dyn Error>>{
		match self.library.get(card_name){
			Some(card) => Ok(&card[0]),
			None => Err(format!("card {} not found ", card_name).into()),
		}
	}
}


// Library is the LIBRARY which can be like... shuffled and stuff
// cards is just a hashmap of cards so that when someone searches
// for cards, we can give them the option to type the card theyre
// looking for and look it up in constant time.
// sideboard is optional
struct Library {
	library: Vec<Card>,
	cards: HashMap<String, Card>,
	sideboard: Vec<Option<Card>>
}

struct Player{
	name: String
}

// these arrays will have to grow/shrink depending on the
// number of players
struct GameState {
	libraries: [Library; 2],
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

use serde::Deserializer;

fn populate_library(){
	todo!("Write the parser that accepts a vector of card names \
	and searches the CardDB for those cards, and adds them to \
	a players library");
}


use std::time::Instant;
use card::Card;
use colors::Color;

fn main() {
	let now = Instant::now();
	println!("starting");
	println!("{:.2?}", now);
	println!("in progress");
	let cards = CardDB::new();

	let elapsed = now.elapsed();
	println!("Elapsed: {:.2?}", elapsed);
	println!("{:#?}", cards.get("Advice from the Fae"));
	println!("\n{:#?}", cards.get("Reaper King"));
	println!("{}", cards.library.len());
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn cardsdb_has_all_the_cards(){
		let cards = CardDB::new();
		assert_eq!(cards.library.len(), 18432);
	}

}
