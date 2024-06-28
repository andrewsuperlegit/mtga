use serde::Deserialize;
use strum_macros::{EnumString, VariantArray, VariantNames};
use crate::colors::Color;
use crate::cost::Cost;
use crate::TapPurpose;

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
pub enum CardLocation {
	Exile,
	Graveyard,
	Hand,
	Library
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
pub struct VisibilityBehavior {
	current_location: CardLocation,
	is_revealed: bool
}

#[derive(Debug, Deserialize)]
pub struct EntranceBehavior{
	can_have_summoning_sickness: bool,
	enters_battlefield_on_instant_stack: bool,
	enters_battlefield_tapped: bool
}

#[derive(Debug, Deserialize)]
pub struct BattlefieldBehavior {
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
pub struct ExitBehavior {
	hits_graveyard_on_death: bool,
	hits_exile_on_death: bool,
	location_on_death: CardLocation,
}

#[derive(Debug, Deserialize)]
pub struct PhysicalBehavior {
	visibility_behavior: VisibilityBehavior,
	entrance_behavior: EntranceBehavior,
	battlefield_behavior: BattlefieldBehavior,
	exit_behavior: ExitBehavior,
}


#[derive(Debug, Deserialize)]
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
	// todo customize deserialization/parsing to do this
	// physical_behavior: PhysicalBehavior,
	pub subtypes: Vec<String>,
	pub supertypes: Vec<String>,
}





#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn valid_card_can_be_made() {
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
			subtypes: vec![],
			supertypes: vec![],
		};
		assert_eq!(card.name, "Forest");
	}
}