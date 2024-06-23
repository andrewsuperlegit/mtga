#![allow(warnings)]

#[derive(Debug, PartialEq, EnumString, Eq, VariantNames, VariantArray)]
#[strum(serialize_all="lowercase")]
enum CardType {
	Artifact,
	Battle,
	Commander,
	Creature,
	Enchantment,
	Equipment,
	Instant,
	Land,
	#[strum(ascii_case_insensitive)]
	Planeswalker,
	Sorcery,
	Basic,
	Legendary,
	Snow,
}
#[derive(Debug)]
enum CardLocation {
	Exile,
	Graveyard,
	Hand,
	Library
}
use std::str::FromStr;
use strum_macros::{EnumString, VariantNames, VariantArray};
use strum::VariantNames;

#[derive(Debug, PartialEq, EnumString, Clone, Hash, Eq, VariantNames, VariantArray)]
#[strum(serialize_all="lowercase")]
enum Color {
	#[strum(serialize="black", serialize="B", serialize="{black}", ascii_case_insensitive)]
	Black,
	#[strum(serialize="blue", serialize="U", serialize="{blue}", ascii_case_insensitive)]
	Blue,
	#[strum(serialize="colorless", serialize="C", serialize="{colorless}",
		ascii_case_insensitive)]
	Colorless,
	#[strum(serialize="green", serialize="G", serialize="{green}", ascii_case_insensitive)]
	Green,
	#[strum(serialize="red", serialize="R", serialize="{red}", ascii_case_insensitive)]
	Red,
	#[strum(serialize="white", serialize="W", serialize="{white}", ascii_case_insensitive)]
	White,
	None
}
#[derive(Debug)]
enum LandTypes{
	Swamp,
	Plain,
	Forest,
	Mountain,
	Island,
	Waste
}
#[derive(Debug)]
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

// this is gonna be the master list of cards for when we deserialize
// that big json file, and will allow us to accept a list of card
// names as input and insert those cards into peoples' libraries.
struct CardDB {
	library: HashMap<String, Card>,
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

// these arrays will have to grow/shrink depending on the  // number of players
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
#[derive(Debug)]
struct Payment{
	color: Color,
	quantity: u8
}
use std::collections::HashMap;
use crate::CardType::Land;

#[derive(Debug)]
struct Cost {
	cost: HashMap<Color, u8>,
}

impl Cost {
	fn new(payments: Vec<Payment>) -> Self{
		let mut cost =  HashMap::new();
		if payments.len() == 0 {
			cost.insert(Color::None, 0);
		}
		payments.iter().for_each(|payment|{
			cost.insert(payment.color.clone(), payment.quantity);
		});
		return Self {cost}
	}
}
#[derive(Debug)]
struct VisibilityBehavior {
	current_location: CardLocation,
	is_revealed: bool
}
#[derive(Debug)]
struct EntranceBehavior{
	can_have_summoning_sickness: bool,
	enters_battlefield_on_instant_stack: bool,
	enters_battlefield_tapped: bool
}
#[derive(Debug)]
struct BattlefieldBehavior {
	can_attack: bool,
	can_block: bool,
	can_tap: bool,
	can_turn_face_up: bool,
	is_face_down: bool,
	is_summon_sick: bool,
	is_tapped: bool,
	tap_purpose: Vec<TapPurpose>,
}
#[derive(Debug)]
struct ExitBehavior {
	hits_graveyard_on_death: bool,
	hits_exile_on_death: bool,
	location_on_death: CardLocation,
}

#[derive(Debug)]
struct PhysicalBehavior {
	visibility_behavior: VisibilityBehavior,
	entrance_behavior: EntranceBehavior,
	battlefield_behavior: BattlefieldBehavior,
	exit_behavior: ExitBehavior,
}

#[derive(Debug)]
struct Card{
	physical_behavior: PhysicalBehavior,
	card_type: Vec<CardType>,
	color: Vec<Color>,
	cost: Cost,
	description: String,
	//location: &'a CardLocation, // not having this because it makes life complicated;
	// will instead use a get method as per
	// https://shorturl.at/3qeyh
	name: String,

}

fn populate_db(){
	todo!("Write the CardDB parser that will take all the cards \
	from the json file, and populate the CardDB with them.");
}

fn populate_library(){
	todo!("Write the parser that accepts a vector of card names \
	and searches the CardDB for those cards, and adds them to \
	a players library");
}

fn main() {

}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn color_accepts_lowercase(){
		let _green = Color::Green;
		let green = Color::from_str("green");
		assert_eq!(_green, green.unwrap());
	}
	#[test]
	fn color_accepts_single_letter(){
		let green = Color::Green;
		let g = Color::from_str("g");
		assert_eq!(green, g.unwrap());
	}
	#[test]
	fn color_accepts_single_letter_uppercase(){
		let green = Color::Green;
		let G = Color::from_str("G");
		assert_eq!(green, G.unwrap());
	}
	#[test]
	fn color_accepts_brackets(){
		let green = Color::Green;
		let G = Color::from_str("{green}");
		assert_eq!(green, G.unwrap());
	}
	#[test]
	fn color_accepts_case_insensitive_brackets(){
		let green = Color::Green;
		let G = Color::from_str("{GReen}");
		assert_eq!(green, G.unwrap());
	}

	#[test]
	fn cost_accepts_payment(){
		let cost = Cost::new(vec![Payment{color: Color::Green, quantity: 2}]);
		assert_eq!(cost.cost.contains_key(&Color::Green), true);
		assert_eq!(cost.cost.contains_key(&Color::Blue), false);
		assert_eq!(*cost.cost.get(&Color::Green).unwrap(), 2);
	}

	#[test]
	fn cost_accepts_multiple_payments(){
		let cost = Cost::new(vec![
			Payment{color: Color::Green, quantity: 2},
			Payment{color: Color::Red, quantity: 3}
		]);
		assert_eq!(cost.cost.contains_key(&Color::Green), true);
		assert_eq!(cost.cost.contains_key(&Color::Red), true);
		assert_eq!(*cost.cost.get(&Color::Green).unwrap(), 2);
		assert_eq!(*cost.cost.get(&Color::Red).unwrap(), 3);
	}

	#[test]
	fn cost_accepts_none_payments(){
		let cost = Cost::new(vec![]);
		assert_eq!(*cost.cost.get(&Color::None).unwrap(), 0);
		assert_eq!(cost.cost.contains_key(&Color::Green), false);
		assert_eq!(cost.cost.contains_key(&Color::None), true);
	}

	#[test]
	fn valid_card_can_be_made(){
		let vis_b = VisibilityBehavior {
			current_location: CardLocation::Library,
			is_revealed: false
		};
		let entrance_b = EntranceBehavior {
			can_have_summoning_sickness: false,
			enters_battlefield_on_instant_stack: false,
			enters_battlefield_tapped: false
		};
		let battle_b = BattlefieldBehavior {
			can_attack: false,
			can_block: false,
			can_tap: true,
			can_turn_face_up: false,
			is_tapped: false,
			is_face_down: false,
			is_summon_sick: false,
			tap_purpose: vec![TapPurpose::Mana],
		};
		let exit_b = ExitBehavior {
			hits_graveyard_on_death: true,
			hits_exile_on_death: false,
			location_on_death: CardLocation::Graveyard,
		};
		let card_behavior = PhysicalBehavior {
			visibility_behavior: vis_b,
			entrance_behavior: entrance_b,
			battlefield_behavior: battle_b,
			exit_behavior: exit_b,
		};
		let card = Card {
			physical_behavior: card_behavior,
			card_type: vec![Land],
			color: vec![Color::Green],
			cost: Cost::new(vec![]),
			description: "derp".to_string(),
			name: "Forest".to_string(),
		};
		assert_eq!(card.name, "Forest");
	}
}
