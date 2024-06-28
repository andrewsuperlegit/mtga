use serde::{Deserialize, Deserializer};
use std::borrow::Cow;
use std::collections::HashMap;
use std::str::FromStr;
use crate::colors::Color;

#[derive(Debug, Deserialize)]
pub struct Payment{
	color: Color,
	quantity: u8
}

#[derive(Debug, PartialEq, Default)]
pub struct Cost {
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

use regex::Regex;
pub fn parse_costs(mana_cost: &str) -> Cost{
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


use strum_macros::{EnumString, VariantArray, VariantNames};
use strum;

pub fn parse_color(mana_cost: &str, c: char) -> Color {
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

pub fn get_color_and_quantity(mana_cost: &str, c: char) -> (Color, u8){
	if c.is_numeric(){
		return (parse_color(mana_cost, c), c.to_string().parse().unwrap());
	}
	(parse_color(mana_cost, c), 1)
}

pub fn parse_costs_better(mana_cost: &str) -> Cost{
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



#[cfg(test)]
mod tests {
	use super::*;
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
	fn cost_accepts_payment() {
		let cost = Cost::new(vec![Payment { color: Color::G, quantity: 2 }]);
		assert_eq!(cost.cost.contains_key(&Color::G), true);
		assert_eq!(cost.cost.contains_key(&Color::B), false);
		assert_eq!(*cost.cost.get(&Color::G).unwrap(), 2);
	}

	#[test]
	fn cost_accepts_multiple_payments() {
		let cost = Cost::new(vec![
			Payment { color: Color::G, quantity: 2 },
			Payment { color: Color::R, quantity: 3 }
		]);
		assert_eq!(cost.cost.contains_key(&Color::G), true);
		assert_eq!(cost.cost.contains_key(&Color::R), true);
		assert_eq!(*cost.cost.get(&Color::G).unwrap(), 2);
		assert_eq!(*cost.cost.get(&Color::R).unwrap(), 3);
	}

	#[test]
	fn cost_accepts_multiple_payments_of_same_value() {
		let cost = Cost::new(vec![
			Payment { color: Color::G, quantity: 1 },
			Payment { color: Color::G, quantity: 3 }
		]);
		assert_eq!(cost.cost.contains_key(&Color::G), true);
		assert_eq!(*cost.cost.get(&Color::G).unwrap(), 4);
	}

	#[test]
	fn cost_accepts_none_payments() {
		let cost = Cost::new(vec![]);
		assert_eq!(*cost.cost.get(&Color::None).unwrap(), 0);
		assert_eq!(cost.cost.contains_key(&Color::G), false);
		assert_eq!(cost.cost.contains_key(&Color::None), true);
	}
}