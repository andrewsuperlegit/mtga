use strum_macros::{EnumString, VariantNames};
use serde::Deserialize;
use std::str::FromStr;



/// Colors are the different possible colors a MTG card can have. There are more than you would think!
/// B = Black
/// U = Blue
/// C = Colorless
/// G = Green
/// R = Red
/// W = White
/// X = Variable of colorless
/// S = Snow
/// P = Phyrexian (aka pay with life)
/// MultiColor = MultiColor (see struct def)
/// None = Doesn't have a Color.
///
/// You can use from_str to convert a word to a color for example:
///
/// ```rust
///		let g = Color::from_str("green");
/// 	println!("{}", g);
/// 	// prints Color::G
/// ```
#[derive(
	Debug, PartialEq, EnumString, Clone,
	Hash, Eq, VariantNames,
	Deserialize
)]
#[strum(serialize_all="lowercase")]
pub enum Color {
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




///
///	Colors is the ordered vector of colors
///	multicolor_cost is the ordered vector of costs
///	so for example:
///	MultiColor{
///		colors: [Color::C, Color::G],
///		multicolor_cost: [2, 1]
///	}
///	means that for this MultiColor you either have to pay
///	2 Colorless, or 1 Green.
pub struct MultiColor{
	colors: Vec<Color>,
	multicolor_cost: Vec<u8>,
}


#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn color_accepts_lowercase() {
		let _green = Color::G;
		let green = Color::from_str("green");
		assert_eq!(_green, green.unwrap());
	}

	#[test]
	fn color_accepts_single_letter() {
		let green = Color::G;
		let g = Color::from_str("g");
		assert_eq!(green, g.unwrap());
	}

	#[test]
	fn color_accepts_single_letter_uppercase() {
		let green = Color::G;
		let g = Color::from_str("G");
		assert_eq!(green, g.unwrap());
	}

	#[test]
	fn color_accepts_brackets() {
		let green = Color::G;
		let g = Color::from_str("{green}");
		assert_eq!(green, g.unwrap());
	}

	#[test]
	fn color_accepts_case_insensitive_brackets() {
		let green = Color::G;
		let g = Color::from_str("{GReen}");
		assert_eq!(green, g.unwrap());
	}
}