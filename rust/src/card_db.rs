use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::error::Error;
use std::io::BufReader;
use crate::card::Card;
use std::sync::OnceLock;

#[derive(Debug, Deserialize)]
pub struct CardDB {
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
	fn new(filename: &str) -> CardDB {
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

	pub fn get(&self, card_name: &str) -> Result<&Card, Box<dyn Error>>{
		match self.library.get(card_name){
			Some(card) => Ok(&card[0]),
			None => Err(format!("card {} not found ", card_name).into()),
		}
	}
}
pub fn get_card_db() -> &'static CardDB{
	static COMPUTATION: OnceLock<CardDB> = OnceLock::new();
	COMPUTATION.get_or_init(|| CardDB::new("src\\noForeignModernAtomic-rust.json"))
}

pub fn get_card_db_slow() -> CardDB{
	CardDB::new("src\\noForeignModernAtomic-rust.json")
}

#[cfg(test)]
mod tests {
	use std::time::Instant;
	use super::*;

	#[test]
	fn cardsdb_has_all_the_cards(){
		let filename = "src\\noForeignModernAtomic-rust.json";
		let cards = CardDB::new(filename);
		assert_eq!(cards.library.len(), 18432);
	}
	#[test]
	fn get_card_db_is_fast_af(){
		let now1 = Instant::now();
		for n in 1..100{
			let db = get_card_db();
			db.get("Advice from the Fae");
		}
		let elapsed1 = now1.elapsed();

		let now2 = Instant::now();
		for n in 1..3{
			let slowdb = get_card_db_slow();
			slowdb.get("Advice from the Fae");
		}
		let elapsed2 = now2.elapsed();
		assert!(elapsed1 < elapsed2);
	}
}