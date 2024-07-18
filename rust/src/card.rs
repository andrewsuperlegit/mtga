use serde::Deserialize;
use strum_macros::{EnumString, VariantArray, VariantNames};
use crate::card_db::{CardDB, get_card_db};
use crate::colors::Color;
use crate::cost::Cost;

#[derive(
	Debug, PartialEq, EnumString, Eq, VariantNames,
	VariantArray, Deserialize
)]
#[strum(serialize_all="lowercase")]
pub enum CardType {
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


// weird types that arent really types but i guess are types according to mtgjson:
	Dungeon,
	Legend,
	Scheme,
	Plane,
	Conspiracy,
	Vanguard,
	Summon,
	Kindred,
	Hero,
	Rat,
	Rogue,
	Jaguar,
	Phenomenon,
	Dragon,
	Goblin,
	Knights
}


#[derive(Debug, Deserialize)]
enum TapPurpose{
	Mana,
	Action,
	None
}

#[derive(Debug, Default, PartialEq)]
pub enum CardLocation {
	Exile,
	Graveyard,
	Hand,
	Library,
	// TemporaryView, // <- for scrying and surveiling?
	#[default]
	None
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


/// Where is the card, and whether opponents can see it.
#[derive(Debug, Default)]
pub struct VisibilityBehavior {
	pub current_location: CardLocation,
	revealed: bool
}
impl VisibilityBehavior{
	pub fn set_location(&mut self, location: CardLocation) {
		self.current_location = location;
	}
	pub fn set_revealed(&mut self, should_reveal: bool){
		self.revealed = should_reveal;
	}
	pub fn is_revealed(&self) -> bool{
		self.revealed
	}
}


/// How the card enters the battlefield
#[derive(Debug, Default)]
pub struct EntranceBehavior{
	can_have_summoning_sickness: bool,
	enters_on_instant_stack: bool,
	enters_battlefield_tapped: bool
}

impl EntranceBehavior{
	fn new(card: &Card) -> EntranceBehavior {
		let can_have_summoning_sickness = card.card_types.contains(&CardType::Creature);
		let enters_on_instant_stack = card.card_types.contains(&CardType::Instant) ||
			card.keywords.contains(&"Flash".to_string());
		// todo add checks for the word "unless" and ensure conditions are met
		let enters_battlefield_tapped = card.description.contains("enters the battlefield tapped");

		EntranceBehavior {
			can_have_summoning_sickness,
			enters_on_instant_stack,
			enters_battlefield_tapped,
		}
	}
}
/// How the card behaves on the battlefield
#[derive(Debug, Default)]
pub struct BattlefieldBehavior {
	/// does it have the ability to attack?
	can_attack: bool,
	/// does it have the ability to block?
	can_block: bool,
	/// Can it be tapped for a purpose?
	can_tap: bool,
	/// can it be turned facedown/faceup?
	can_turn_face_up: bool,
	/// is the card face down (disguised?)?
	is_face_down: bool,
	/// does this card have summoning sickness?
	is_summon_sick: bool,
	/// is it tapped?
	is_tapped: bool,
	/// when it is tapped, what does tapping the card do?
	tap_purpose: Vec<TapPurpose>,
	/// when it attacks, how much damage does it do? (defaults to 0 because some cards cant attack)
	power: i32,
	/// when it blocks how much damage can it take before it dies? (defaults to 0 because some cards
	/// cant block)
	toughness: i32
}

/// searches a string slice for keywords. keywords in mtgjson data are of the format: {_char_}
/// examples: {T} (tap), {C} (colorless), etc
fn get_unbracketed_keywords(description:&str) -> Vec<char>{
	let mut vec = vec![' '];
	let mut chars = description.chars().enumerate();
	while let Some(enumerated) = chars.next(){
		let (i, c) = enumerated;
		if c == '{' {
			let closed = match description.chars().nth(i+2){
				Some(rb) => rb == '}',
				None => panic!("Error Parsing {}", description)
			};
			if closed {
				let special_text = description.chars().nth(i+1).unwrap();
				vec.push(special_text);
			}
			chars.next();
			chars.next();
			continue;
		}
	}
	vec
}

/// given a string slice, returns whether it has a tap keyword in it.
fn can_tap(description:&str)->bool{
	let unbracketed_keywords = get_unbracketed_keywords(description);
	unbracketed_keywords.contains(&'T')
}

fn get_tap_purpose(card: &Card, can_tap: bool) -> Vec<TapPurpose> {
	if can_tap == false {
		return vec![TapPurpose::None];
	}
	//todo fix this so it's doing this better. probably need regex.
	if card.description.contains("Add {") {
		return vec![TapPurpose::Mana];
	}
	// todo more clearly specify which action type.
	return vec![TapPurpose::Action];
}

impl BattlefieldBehavior{
	fn new(card: &Card) -> BattlefieldBehavior {
		let can_attack = card.card_types.contains(&CardType::Creature) &&
			card.keywords.contains(&"Defender".to_string()) == false;
		let can_block = card.card_types.contains(&CardType::Creature);
		let can_tap = card.card_types.contains(&CardType::Land) || can_tap(&card.description);
		let can_turn_face_up = card.keywords.contains(&"Disguise".to_string());
		let is_face_down = false;
		let is_summon_sick = false;
		let is_tapped = false;
		let tap_purpose = get_tap_purpose(card, can_tap);

		let mut power = card.power.clone();
		let mut toughness = card.toughness.clone();

		if power == "*" {
			power = "0".to_string();
		}
		if toughness == "*"{
			toughness = "0".to_string();
		}

		let power = power.parse().unwrap_or_else(|e| panic!("Card {} had invalid power: {}", card.name, card.power));
		let toughness = toughness.parse().unwrap_or_else(|e| panic!("Card {} had invalid toughness: {}", card.name, card.toughness));

		BattlefieldBehavior{
			can_attack,
			can_block,
			can_tap,
			can_turn_face_up,
			is_face_down,
			is_summon_sick,
			is_tapped,
			tap_purpose,
			power,
			toughness
		}
	}
}


/// When the card dies, what does it do?
#[derive(Debug, Default)]
pub struct ExitBehavior {
	hits_graveyard_on_death: bool,
	hits_exile_on_death: bool,
	location_on_death: CardLocation,
}

impl ExitBehavior{
	fn update_hits_graveyard_on_death(&mut self, should_go_to_graveyard: bool){
		self.hits_graveyard_on_death = should_go_to_graveyard;
	}
	fn update_hits_exile_on_death(&mut self, should_hit_exile:bool){
		self.hits_exile_on_death = should_hit_exile;
	}
	fn update_location_on_death(&mut self, death_location: CardLocation) {
		self.location_on_death = death_location;
	}
}

fn zero() -> String{
	"0".to_string()
}


/// A Card is more or less a direct mapping from
/// [mtgjson](https://mtgjson.com/data-models/card/card-atomic/#card-atomic)
/// with a bunch of the excess properties removed.
///
/// This is mostly used just to deserialize the data in the json files so we can use it in rust.
#[derive(Debug, Deserialize, Default)]
pub struct Card {
	#[serde(rename(deserialize = "type"))]
	pub card_type: String, // because stuff like Artifact - Equipment
	#[serde(rename(deserialize = "types"))]
	pub card_types: Vec<CardType>,
	pub colors: Vec<Color>,
	#[serde(rename(deserialize = "colorIdentity"))]
	pub color_identity: Vec<Color>,
	#[serde(rename(deserialize = "convertedManaCost"), default)]
	pub converted_mana_cost: u8,
	#[serde(rename(deserialize = "text"), default)]
	pub description: String,
	#[serde(default)]
	pub keywords: Vec<String>,
	pub layout: String,
	#[serde(rename(deserialize = "manaCost"), default)]
	pub mana_cost: Cost,
	#[serde(rename(deserialize = "manaValue"), default)]
	pub mana_value: u8,
	pub name: String,
	/// power and toughness default to zero as strings; the strings are parsed into numbers in
	/// RealCard.battlefield_behavior
	#[serde(default="zero")]
	pub power: String,
	pub subtypes: Vec<String>,
	pub supertypes: Vec<String>,
	#[serde(default="zero")]
	pub toughness: String
}

#[derive(PartialEq, Debug)]
pub enum RealCardError{
	CardNotFound(String),
	InvalidQuantity,
}


#[derive(Debug)]
/// RealCards are cards that we actually interact with; They are created via
/// ```rust
/// let card_name = "Forest".to_string();
/// let quantity = 4;
/// RealCard::new(card_name, quantity);
/// ```
/// the ::new searches the card database for that Card, and then creates the various behaviors and
/// properties we need to do stuff in the game.
pub struct RealCard <'a>{
	/// the Card in CardDb
	pub card: &'a Card,
	/// the card name. case and punctuation matters.
	pub name: &'a str,
	/// how many of this card are in your deck.
	pub quantity: u8,
	pub visibility_behavior: VisibilityBehavior,
	pub entrance_behavior: EntranceBehavior,
	pub battlefield_behavior: BattlefieldBehavior,
	pub exit_behavior: ExitBehavior,
}

/// determines if a card is a basic land by searching it's card_types and supertypes
fn card_is_basic_land(card_types: &Vec<CardType>, supertypes: &Vec<String>) -> bool{
	(card_types.contains(&CardType::Land) && supertypes.contains(&"Basic".to_string()))
}

impl RealCard<'_>{
	pub fn new(name: &str, quantity: u8)-> Result<RealCard, RealCardError> {
		let db: &CardDB = get_card_db();
		let card_result: Result<&Card, RealCardError> = match db.get_card(name){
			Ok(card) => Ok(card),
			Err(e) => Err(RealCardError::CardNotFound(name.to_string()))
		};
		let card = card_result?;
		let is_basic_land =  card_is_basic_land(&card.card_types, &card.supertypes);

		// can only have up to 4 of the same card in a deck unless its a basic land.
		if !is_basic_land && (quantity > 4 || quantity < 1) {
			return Err(RealCardError::InvalidQuantity);
		}

		let visibility_behavior = VisibilityBehavior {
			current_location: CardLocation::Library,
			revealed: false,
		};
		let entrance_behavior = EntranceBehavior::new(card);
		let battlefield_behavior = BattlefieldBehavior::new(card);
		let exit_behavior = ExitBehavior {
			hits_graveyard_on_death: true,
			hits_exile_on_death: false,
			location_on_death: CardLocation::Graveyard
		};
		Ok(RealCard{
			name,
			card,
			quantity,
			visibility_behavior,
			entrance_behavior,
			battlefield_behavior,
			exit_behavior
		})
	}

	/// move card from its current location to a new location.
	pub fn change_current_location(&mut self, new_location: CardLocation){
		self.visibility_behavior.set_location(new_location);
	}
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn real_card_searches_carddb_for_card(){
		let card = RealCard::new("Forest", 20).unwrap();
		assert_eq!(card.quantity, 20);
	}

	#[test]
	fn real_card_searches_carddb_for_card_and_the_search_is_case_sensitive(){
		let card = RealCard::new("forest", 20);
		assert!(card.is_err_and(|e| e == RealCardError::CardNotFound("forest".to_string())));
	}

	#[test]
	fn can_tap_returns_true_the_only_behavior_in_desc_is_a_tap_behavior(){
		let str = "{T}: Target creature gets +X/+X until end of turn, where X is Auriok Bladewarden's power.";
		let res = can_tap(str);
		assert_eq!(res, true);
	}

	#[test]
	fn can_tap_returns_true_if_there_are_multiple_behaviors_and_tap_isnt_the_first_one(){
		let str = "{W}, {T}: Tap target artifact.";
		let res = can_tap(str);
		assert_eq!(res, true);
	}

	#[test]
	fn can_tap_returns_false_if_theres_no_tap_behavior_specified(){
		let str = "{B}: Mill a card.\nDelirium — At the beginning of your end step, \
		if there are four or more card types among cards in your graveyard, \
		transform Autumnal Gloom.";
		let res = can_tap(str);
		assert_eq!(res, false);
	}


	#[test]
	fn valid_card_can_be_made() {
		let vis_b = VisibilityBehavior {
			current_location: CardLocation::Library,
			revealed: false
		};
		let entrance_b = EntranceBehavior {
			can_have_summoning_sickness: false,
			enters_on_instant_stack: false,
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
			power: 0,
			toughness: 0,
		};
		let exit_b = ExitBehavior {
			hits_graveyard_on_death: true,
			hits_exile_on_death: false,
			location_on_death: CardLocation::Graveyard,
		};
		let card = Card {
			card_type: "Land".to_string(),
			card_types: vec![CardType::Land],
			color_identity: vec![Color::G],
			colors: vec![Color::G],
			converted_mana_cost: 0,
			description: "derp".to_string(),
			layout: "normal".to_string(),
			keywords: vec![],
			mana_cost: crate::cost::parse_costs(""),
			mana_value: 1,
			name: "Forest".to_string(),
			power: "0".to_string(),
			subtypes: vec![],
			supertypes: vec![],
			toughness: "0".to_string(),
		};
		assert_eq!(card.name, "Forest");
	}
}