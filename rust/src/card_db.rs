use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::error::Error;
use std::io::BufReader;
use crate::card::Card;
use std::sync::OnceLock;
use std::env::consts::{OS};

fn get_mtg_library_data_filename_based_on_os()-> String {
	if OS == "windows"{
		return "src\\data\\Atomic.json".to_string();
	}
	"src/data/Atomic.json".to_string()
}

#[derive(Debug, Deserialize)]

/// A database of all the cards. I successfully resisted the urge to call this CardiB. Personal win.
/// library and data_filename are both private; to get cards / card info use the get_card method
/// and to get the carddb, don't call or construct this directly (if you do, each time you're going to
/// parse whatever huge json file you give it over and over), use the public get_card_db method-
/// it returns an immutable singleton db that you can call get_card on.
pub struct CardDB {
	library: HashMap<String, Vec<Card>>,
	#[serde(skip)]
	data_filename: String,
}

fn get_file(filename: &str) -> File{
	match File::open(filename){
		Ok(file) => file,
		Err(error) => panic!("Something went wrong opening the mtg \
			card data: {}", error)
	}
}

impl CardDB {
	fn new(filename: String) -> CardDB {
		let file = get_file(&filename);
		let reader = BufReader::new(file);
		let serde_result = serde_json::from_reader(reader);

		let mut card_db:CardDB = match serde_result{
			Ok(db) => db,
			Err(error) => panic!("Something went wrong trying to create \
			CardDB: {}", error)
		};
		card_db.data_filename = filename;
		card_db
	}

// todo make the search case insensitive; will almost certainly involve something with serde rename
	pub fn get_card(&self, card_name: &str) -> Result<&Card, Box<dyn Error>>{
		match self.library.get(card_name){
			Some(card) => Ok(&card[0]),
			None => Err(format!("card {} not found ", card_name).into()),
		}
	}
}

/// returns a singleton CardDB instance that you can use to look up cards on without having to reparse
/// that massive json file of 13,000 cards.
pub fn get_card_db() -> &'static CardDB{
	let mtg_library_data_filename = get_mtg_library_data_filename_based_on_os();
	static COMPUTATION: OnceLock<CardDB> = OnceLock::new();
	COMPUTATION.get_or_init(|| CardDB::new(mtg_library_data_filename))
}

/// this is pretty much just for test purposes to prove that the singleton is a singleton and that
/// its way tf faster than instantiating a new carddb each time.
pub fn get_card_db_slow() -> CardDB{
	let mtg_library_data_filename = get_mtg_library_data_filename_based_on_os();
	CardDB::new(mtg_library_data_filename)
}

#[cfg(test)]
mod tests {
	use std::time::Instant;
	use super::*;

	#[test]
	fn cardsdb_has_all_the_cards(){
		let filename = get_mtg_library_data_filename_based_on_os();
		let cards = CardDB::new(filename);
		assert_eq!(cards.library.len(), 28925);
	}

	#[test]
	fn get_card_db_is_fast_af(){
		let now1 = Instant::now();
		for n in 1..100{
			let db = get_card_db();
			db.get_card("Advice from the Fae");
		}
		let elapsed1 = now1.elapsed();

		let now2 = Instant::now();
		for n in 1..3{
			let slowdb = get_card_db_slow();
			slowdb.get_card("Advice from the Fae");
		}
		let elapsed2 = now2.elapsed();
		assert!(elapsed1 < elapsed2);
	}
}