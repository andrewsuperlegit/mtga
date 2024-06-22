#![allow(warnings)]
enum CardType {
	Artifact,
	Battle,
	Commander,
	Creature,
	Enchantment,
	Equipment,
	Instant,
	Land,
	Planeswalker,
	Sorcery,
	Basic,
	Legendary,
	Snow,
}

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
	#[strum(serialize="colorless", serialize="C", serialize="{colorless}", ascii_case_insensitive)]
	Colorless,
	#[strum(serialize="green", serialize="G", serialize="{green}", ascii_case_insensitive)]
	Green,
	#[strum(serialize="red", serialize="R", serialize="{red}", ascii_case_insensitive)]
	Red,
	#[strum(serialize="white", serialize="W", serialize="{white}", ascii_case_insensitive)]
	White,
	None
}

enum LandTypes{
	Swamp,
	Plain,
	Forest,
	Mountain,
	Island,
	Waste
}
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


struct Library{}
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

struct CardBehavior{}

struct Card{
	behavior: CardBehavior,
	card_type: CardType,
	color: Vec<Color>,
	cost: Cost,
	description: String,
	location: CardLocation,
	name: String,
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
	fn cost_accepts_none_payments(){
		let cost = Cost::new(vec![]);
		assert_eq!(*cost.cost.get(&Color::None).unwrap(), 0);
		assert_eq!(cost.cost.contains_key(&Color::Green), false);
		assert_eq!(cost.cost.contains_key(&Color::None), true);
	}
}
