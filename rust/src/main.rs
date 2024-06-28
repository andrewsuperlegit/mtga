#![allow(warnings)]
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::CardType::Land;
use std::str::FromStr;
use strum_macros::{EnumString, VariantNames, VariantArray};
use strum::VariantNames;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

#[derive(
	Debug, PartialEq, EnumString, Eq, VariantNames,
	VariantArray, Deserialize
)]
#[strum(serialize_all="lowercase")]
enum CardType {
	Artifact,
	Battle,
	Commander,
	Creature,
	Dungeon,
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
	Tribal,
}
#[derive(Debug, Deserialize)]
enum CardLocation {
	Exile,
	Graveyard,
	Hand,
	Library
}

#[derive(
	Debug, PartialEq, EnumString, Clone,
	Hash, Eq, VariantNames,
	Deserialize
)]
#[strum(serialize_all="lowercase")]
enum Color {
	#[strum(
		serialize="black", serialize="b",
		serialize="{black}", ascii_case_insensitive
	)]
	B,
	#[strum(
		serialize="blue",serialize="u",
		serialize="{blue}", ascii_case_insensitive
	)]
	U,
	#[strum(
		serialize="colorless",serialize="c",
		serialize="{colorless}", ascii_case_insensitive
	)]
	C,
	#[strum(
		serialize="green",serialize="g",
		serialize="{green}", ascii_case_insensitive
	)]
	G,
	#[strum(
		serialize="red",serialize="r",
		serialize="{red}", ascii_case_insensitive
	)]
	R,
	#[strum(
		serialize="white",serialize="w",
		serialize="{white}", ascii_case_insensitive
	)]
	W,
	#[strum(
		serialize="variable", serialize="x",
		serialize="{x}", ascii_case_insensitive
	)]
	X,
	#[strum(
		serialize="snow", serialize="s",
		serialize="{s}", ascii_case_insensitive
	)]
	S,
	#[strum(
		serialize="phyrexian", serialize="p",
		serialize="{p}", ascii_case_insensitive
	)]
	P,
	MultiColor { colors: Vec<Color>, multicolor_cost: Vec<u8> },
	None
}

/*
Colors is the ordered vector of colors
multicolor_cost is the ordered vector of costs
so for example:
MultiColor{
	colors: [Color::C, Color::G],
	multicolor_cost: [2, 1]
}
means that for this MultiColor you either have to pay
2 Colorless, or 1 Green.
 */
struct MultiColor{
	colors: Vec<Color>,
	multicolor_cost: Vec<u8>,
}


#[derive(Debug, Deserialize)]
enum LandTypes{
	Swamp,
	Plain,
	Forest,
	Mountain,
	Island,
	Waste
}
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

// this is gonna be the master list of cards for when we deserialize
// that big json file, and will allow us to accept a list of card
// names as input and insert those cards into peoples' libraries.
#[derive(Debug, Deserialize)]
struct CardDB {
	library: HashMap<String, Vec<Card>>,
}

// todo!("Write the CardDB parser that will take all the cards \
// from the json file, and populate the CardDB with them.");

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

#[derive(Debug, Deserialize)]
struct Payment{
	color: Color,
	quantity: u8
}

#[derive(Debug, PartialEq, Default)]
struct Cost {
	cost: HashMap<Color, u8>,
	// has_variable_cost: bool
}

impl Cost {
	fn new(payments: Vec<Payment>) -> Self{
		let mut cost =  HashMap::new();
		if payments.len() == 0 {
			cost.insert(Color::None, 0);
		}
		payments.iter().for_each(|payment|{
			let key = &payment.color;
			if cost.contains_key(key) {
				let mut val = cost.get_mut(key).unwrap();
				*val += &payment.quantity;
			} else {
				cost.insert(payment.color.clone(), payment.quantity);
			}
		});
		return Self {cost}
	}
}
use std::borrow::Cow;
use serde::Deserializer;

impl<'de> Deserialize<'de> for Cost {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
	{
		// `&str` can't deserialize JSON strings with escapes, and `String`
		// is not optimally efficient when there are no escapes, so we use
		// `Cow`. `Cow`'s deserialization uses `str` when it can, otherwise
		// it falls back to `String`.
		let cow = Cow::<str>::deserialize(deserializer)?;
		let s: &str = cow.as_ref();
		Ok(parse_costs_better(s))
		// Ok(parse_costs(s))
	}
}


#[derive(Debug, Deserialize)]
struct VisibilityBehavior {
	current_location: CardLocation,
	is_revealed: bool
}
#[derive(Debug, Deserialize)]
struct EntranceBehavior{
	can_have_summoning_sickness: bool,
	enters_battlefield_on_instant_stack: bool,
	enters_battlefield_tapped: bool
}
#[derive(Debug, Deserialize)]
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
#[derive(Debug, Deserialize)]
struct ExitBehavior {
	hits_graveyard_on_death: bool,
	hits_exile_on_death: bool,
	location_on_death: CardLocation,
}

#[derive(Debug, Deserialize)]
struct PhysicalBehavior {
	visibility_behavior: VisibilityBehavior,
	entrance_behavior: EntranceBehavior,
	battlefield_behavior: BattlefieldBehavior,
	exit_behavior: ExitBehavior,
}


#[derive(Debug, Deserialize)]
struct Card {
	#[serde(rename(deserialize = "type"))]
	card_type: String, // because stuff like Artifact - Equipment
	#[serde(rename(deserialize = "types"))]
	card_types: Vec<CardType>,
	colors: Vec<Color>,
	#[serde(rename(deserialize = "colorIdentity"))]
	color_identity: Vec<Color>,
	#[serde(rename(deserialize = "convertedManaCost"), default)]
	converted_mana_cost: u8,
	// todo customize deserialization/parsing to do this
	// cost: Cost,
	#[serde(rename(deserialize = "text"), default)]
	description: String,
	#[serde(default)]
	keywords: Vec<String>,
	layout: String,
	#[serde(rename(deserialize = "manaCost"), default)]
	mana_cost: Cost,
	#[serde(rename(deserialize = "manaValue"), default)]
	mana_value: u8,
	name: String,
	// todo customize deserialization/parsing to do this
	// physical_behavior: PhysicalBehavior,
	subtypes: Vec<String>,
	supertypes: Vec<String>,
}

fn populate_library(){
	todo!("Write the parser that accepts a vector of card names \
	and searches the CardDB for those cards, and adds them to \
	a players library");
}
use regex::Regex;
fn parse_costs(mana_cost: &str) -> Cost{
	let re = Regex::new(r"\{(\w+)}").unwrap();
	let haystack = mana_cost;
	let mut payments_vec:Vec<Payment> = vec!();

	for (_, [color]) in re.captures_iter(haystack).map(|c| c.extract()){
		if color.parse::<u8>().is_ok(){
			payments_vec.push(Payment{ color: Color::C, quantity: color.parse().unwrap() })
		} else {
			let color = Color::from_str(color).or_else(|err|{
				println!("ERROR IS: {:?}, COLOR WAS: {}, MANA COST WAS:{}", err, color, mana_cost);
				Err(Color::None)
			}).unwrap();
			payments_vec.push(Payment { color: color , quantity: 1 });
		}
	}
	Cost::new(payments_vec)
}



fn parse_color(mana_cost: &str, c: char) -> Color {
	if c.is_numeric(){
		return Color::C;
	}
	let c = &c.to_string()[..];
	let color = Color::from_str(c).or_else(|err| {
		println!("ERROR IS: {:?}, COLOR WAS: {}, MANA COST WAS:{}", err, c, mana_cost);
		Err(Color::None)
	}).unwrap();
	color
}

fn get_color_and_quantity(mana_cost: &str, c: char) -> (Color, u8){
	if c.is_numeric(){
		return (parse_color(mana_cost, c), c.to_string().parse().unwrap());
	}
	(parse_color(mana_cost, c), 1)
}

fn parse_costs_better(mana_cost: &str) -> Cost{
	let mut payments_vec:Vec<Payment> = vec!();
	let mut chars = mana_cost.chars().enumerate();

	while let Some(enumerated) = chars.next(){
		let (i, c) = enumerated;
		if c == '/'{
			payments_vec.pop();
			let prev_color = &mana_cost.chars().nth(i - 1).unwrap();
			let (prev_color, prev_color_quantity) = get_color_and_quantity(mana_cost, *prev_color);
			let next_color = &mana_cost.chars().nth(i + 1).unwrap();
			let (next_color, next_color_quantity) = get_color_and_quantity(mana_cost, *next_color);
			payments_vec.push(Payment {
				quantity: 1,
				color: Color::MultiColor {
					colors: vec![
						prev_color,
						next_color,
					],
					multicolor_cost: vec![prev_color_quantity, next_color_quantity],
				},
			});
			chars.next();
			continue
		}

		if c.is_numeric() {
			let (color, quantity) = get_color_and_quantity(mana_cost, c);
			payments_vec.push(Payment {color, quantity});
		} else if c.is_alphabetic() {
			let (color, quantity) = get_color_and_quantity(mana_cost, c);
			payments_vec.push(Payment {color , quantity: 1 });
		}
	}

	Cost::new(payments_vec)
}


use std::time::Instant;
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

	#[test]
	fn parse_costs_better_works(){
		let str = "{G}";
		let cost = parse_costs_better(&str);
		assert_eq!(cost.cost.get_key_value(&Color::G), Some((&Color::G, &1)));
	}
	#[test]
	fn parse_costs_better_doesnt_duplicate_multicolors(){
		let str = "{G/W}{G/W}";
		let cost = parse_costs_better(&str);
		assert_eq!(cost.cost
			.get_key_value(&Color::MultiColor{
				colors: vec![Color::G, Color::W],
				multicolor_cost: vec![1,1],
			}), Some((&Color::MultiColor{
				colors: vec![Color::G, Color::W],
				multicolor_cost: vec![1,1],
		}, &2)));

		// this should fail.
		assert_eq!(cost.cost.get_key_value(&Color::W), None);
	}
	#[test]
	fn parse_costs_better_accepts_colorless_multicolors(){
		let str = "{2/U}{2/U}";
		let cost = parse_costs_better(&str);
		assert_eq!(cost.cost
			.get_key_value(&Color::MultiColor{
				colors: vec![Color::C, Color::U],
				multicolor_cost: vec![2,1],
			}), Some((&Color::MultiColor{
			colors: vec![Color::C, Color::U],
			multicolor_cost: vec![2,1],
		}, &2)));
		println!("{:?}", cost);

		// this should fail.
		assert_eq!(cost.cost.get_key_value(&Color::C), None);
	}

	#[test]
	#[should_panic]
	fn parse_costs_throws_error_on_unexpected(){
		let str = "{z}";
		let cost = parse_costs(&str);
	}


	#[test]
	fn parse_costs_maps_variable_costs(){
		let str = "{X}{X}{G}";
		let cost = parse_costs(&str);
		assert_eq!(cost.cost.get_key_value(&Color::X), Some((&Color::X, &2)));
		assert_eq!(cost.cost.get_key_value(&Color::G), Some((&Color::G, &1)));
	}

	#[test]
	fn parse_costs_maps_colorless(){
		let str = "{4}";
		let cost = parse_costs(&str);
		assert_eq!(cost.cost.get_key_value(&Color::C), Some((&Color::C, &4)));
	}
	#[test]
	fn parse_costs_maps_colors(){
		let str = "{G}{G}";
		let cost = parse_costs(&str);
		assert_eq!(cost.cost.get_key_value(&Color::G), Some((&Color::G, &2)));
	}

	#[test]
	fn parse_costs_maps_everything(){
		let str = "{G}{G}{3}{R}{r}{blue}{w}{B}";
		let cost = parse_costs(&str);
		assert_eq!(cost.cost.get_key_value(&Color::G), Some((&Color::G, &2)));
		assert_eq!(cost.cost.get_key_value(&Color::C), Some((&Color::C, &3)));
		assert_eq!(cost.cost.get_key_value(&Color::R), Some((&Color::R, &2)));
		assert_eq!(cost.cost.get_key_value(&Color::U), Some((&Color::U, &1)));
		assert_eq!(cost.cost.get_key_value(&Color::W), Some((&Color::W, &1)));
		assert_eq!(cost.cost.get_key_value(&Color::B), Some((&Color::B, &1)));
	}

	#[test]
	fn parse_costs_maps_empty(){
		let str = "";
		let cost = parse_costs(&str);
		assert_eq!(cost.cost.get_key_value(&Color::None), Some((&Color::None, &0)));
	}

	#[test]
	fn color_accepts_lowercase(){
		let _green = Color::G;
		let green = Color::from_str("green");
		assert_eq!(_green, green.unwrap());
	}
	#[test]
	fn color_accepts_single_letter(){
		let green = Color::G;
		let g = Color::from_str("g");
		assert_eq!(green, g.unwrap());
	}
	#[test]
	fn color_accepts_single_letter_uppercase(){
		let green = Color::G;
		let G = Color::from_str("G");
		assert_eq!(green, G.unwrap());
	}
	#[test]
	fn color_accepts_brackets(){
		let green = Color::G;
		let G = Color::from_str("{green}");
		assert_eq!(green, G.unwrap());
	}
	#[test]
	fn color_accepts_case_insensitive_brackets(){
		let green = Color::G;
		let G = Color::from_str("{GReen}");
		assert_eq!(green, G.unwrap());
	}

	#[test]
	fn cost_accepts_payment(){
		let cost = Cost::new(vec![Payment{color: Color::G, quantity: 2}]);
		assert_eq!(cost.cost.contains_key(&Color::G), true);
		assert_eq!(cost.cost.contains_key(&Color::B), false);
		assert_eq!(*cost.cost.get(&Color::G).unwrap(), 2);
	}

	#[test]
	fn cost_accepts_multiple_payments(){
		let cost = Cost::new(vec![
			Payment{color: Color::G, quantity: 2},
			Payment{color: Color::R, quantity: 3}
		]);
		assert_eq!(cost.cost.contains_key(&Color::G), true);
		assert_eq!(cost.cost.contains_key(&Color::R), true);
		assert_eq!(*cost.cost.get(&Color::G).unwrap(), 2);
		assert_eq!(*cost.cost.get(&Color::R).unwrap(), 3);
	}

	#[test]
	fn cost_accepts_multiple_payments_of_same_value(){
		let cost = Cost::new(vec![
			Payment{color: Color::G, quantity: 1},
			Payment{color: Color::G, quantity: 3}
		]);
		assert_eq!(cost.cost.contains_key(&Color::G), true);
		assert_eq!(*cost.cost.get(&Color::G).unwrap(), 4);
	}

	#[test]
	fn cost_accepts_none_payments(){
		let cost = Cost::new(vec![]);
		assert_eq!(*cost.cost.get(&Color::None).unwrap(), 0);
		assert_eq!(cost.cost.contains_key(&Color::G), false);
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
			// physical_behavior: card_behavior,
			card_type: "Land".to_string(),
			card_types: vec![Land],
			color_identity: vec![Color::G],
			colors: vec![Color::G],
			converted_mana_cost: 0,
			// cost: Cost::new(vec![]),
			description: "derp".to_string(),
			layout: "normal".to_string(),
			keywords: vec![],
			mana_cost: parse_costs(""),
			mana_value: 1,
			name: "Forest".to_string(),
			subtypes: vec![],
			supertypes: vec![],
		};
		assert_eq!(card.name, "Forest");
	}
}
