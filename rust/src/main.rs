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

/// a tuple with the card_name and the number of that card you want in your library/sideboard.
/// ("Forest", 20)
struct CardListItem(String, u8);

#[derive(PartialEq, Debug)]
enum LibraryError{

}

/// Library is a players' library of cards they can play.
/// cards is just a hashmap of cards so that when someone searches
/// for cards, we can give them the option to type the card theyre
/// looking for and look it up in constant time.
/// sideboard is optional
///
#[derive(Debug)]
struct Library<'a> {
	/// library is the actual library people will draw from; it can be shuffled and milled, etc.
	/// if a RealCard in cards has a quantity of 4, there will be 4 copies of the RealCard in library.
	/// order matters.
	library: Vec<RealCard<'a>>,
	/// cards is a way to search through your library in constant time and for holding the actual cards
	/// I think library is just going to be smart pointers.
	cards: HashMap<String, RealCard<'a>>,

	sideboard: Vec<Option<RealCard<'a>>>,
}

impl<'a> Library<'a>{
	fn new(card_list:Vec<CardListItem>, optional_list:Vec<CardListItem>) -> Result<Library<'a>, LibraryError>{
		let mut cards = HashMap::new();
		let mut library = vec![];
		let mut sideboard= vec![];

		for card in card_list.iter(){
			let real_card = RealCard::new(&card.0, card.1);
			// todo i need to make sure that the cards in library stay in sync with the cards in cards.
			// so i think the cards in library should be smart pointers.
			// cards.insert(real_card.name, real_card); // this is gonna be a lifetime problem i'm calling it now.
		}

		Ok(Library{
			library,
			cards,
			sideboard
		})
		// todo!("Write the parser that accepts a vector of card names \
		// and searches the CardDB for those cards, and adds them to \
		// a players library");
	}
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
use card::RealCard;
use card_db::{get_card_db, get_card_db_slow};


/// just testing that this works.
pub fn example(){
	// Use a list of cards to populate library
	let vec = vec!["Artificer's Dragon", "First Response", "Gaea's Gift",
		"Jeskai Banner", "Mind's Eye"];

	let res:Vec<RealCard> = vec.iter().map(|name| RealCard::new(name, 2).unwrap()).collect();

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

