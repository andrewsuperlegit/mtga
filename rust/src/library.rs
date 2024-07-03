use std::collections::HashMap;
use std::rc::Rc;
use crate::card::{RealCard, RealCardError};

/// a tuple with the card_name and the number of that card you want in your library/sideboard.
/// ("Forest", 20)
pub struct CardListItem(String, u8);


/// Library is a players' library of cards they can play.
/// cards is a hashmap of cards so that when someone searches
/// for cards, we can give them the option to type the card theyre
/// looking for and look it up in constant time.
/// sideboard is optional
///
#[derive(Debug)]
pub struct Library<'a> {
	/// library is the actual library people will draw from; it can be shuffled and milled, etc.
	/// if a RealCard in cards has a quantity of 4, there will be 4 copies of the RealCard in library.
	/// order matters. the RealCards in library are REFERENCES to the RealCards in cards.
	library: Vec<Rc<RealCard<'a>>>,
	///
	cards: HashMap<(String, u8), Rc<RealCard<'a>>>,
	sideboard: HashMap<String, Rc<RealCard<'a>>>,
}

impl<'a> Library<'a>{
	/// Accepts 2 vectors of cardnames and quantities like:
	/// Library::new([("Forest", 20),("Swamp", 15), ("Insidious Roots", 4)], [/* optional sideboard */])
	/// and converts them into a Library.
	fn new(card_list: &'a Vec<CardListItem>, sideboard_list: &'a Vec<CardListItem>) -> Result<Library<'a>, RealCardError>{
		let mut cards = HashMap::new();
		let mut library = vec![];
		let mut sideboard= HashMap::new();

		for card in card_list.iter(){
			let (card_name, qty) = (&card.0, card.1);
			for i in (0..qty){
				let real_card = Rc::new(RealCard::new(card_name, qty)?);
				let real_card_ref = Rc::clone(&real_card);
				library.push(real_card_ref);
				cards.insert((real_card.name.to_string(), i), real_card);
			}

		}
		for card in sideboard_list.iter(){
			let real_card = Rc::new(RealCard::new(&card.0, card.1)?);
			sideboard.insert(real_card.name.to_string(), real_card);
		}

		Ok(Library{
			library,
			cards,
			sideboard
		})
	}

	fn get_card(&self, card_name: &String, card_key: u8) -> Option<&Rc<RealCard<'a>>> {
		let card_name = card_name.clone();
		self.cards.get(&(card_name, card_key))
	}

	fn search_cards(&self, card_name: String) -> Vec<&Rc<RealCard<'a>>> {
		let card0 = self.get_card(&card_name, 0).unwrap();
		let mut vec = vec![];
		for i in (0..card0.quantity){
			vec.push(self.get_card(&card_name, i).unwrap());
		}
		println!("{:#?}", vec);
		// Ok(vec)
		vec
	}
}



#[cfg(test)]
mod tests {
	use crate::card::CardLocation;
	use super::*;
	#[test]
	fn library_cards_has_hashmap_of_correct_number_of_cards() {
		let vec = vec![
			CardListItem("Mind's Eye".to_string(), 3)
		];
		let vec_b= vec![];
		let lib = Library::new(&vec, &vec_b).unwrap();
		let me0 = &lib.cards[&("Mind's Eye".to_string(), 0)];
		let me1 = &lib.cards[&("Mind's Eye".to_string(), 1)];
		let me2 = &lib.cards[&("Mind's Eye".to_string(), 2)];
		assert_eq!(me0.name, "Mind's Eye".to_string());
		assert_eq!(me1.name, "Mind's Eye".to_string());
		assert_eq!(me2.name, "Mind's Eye".to_string());
		assert_eq!(lib.library.len(), 3);
	}

	#[test]
	fn search_cards_returns_vec_of_cards(){
		let vec = vec![
			CardListItem("Mind's Eye".to_string(), 3)
		];
		let vec_b= vec![];
		let lib = Library::new(&vec, &vec_b).unwrap();
		let result = lib.search_cards("Mind's Eye".to_string());
		assert_eq!(result.len(), 3);
	}

	#[test]
	fn sanity(){
		let vec = vec![
			CardListItem("Mind's Eye".to_string(), 3)
		];
		let vec_b = vec![];
		let lib = Library::new(&vec, &vec_b).unwrap();
		let mut card = lib.get_card(&"Mind's Eye".to_string(), 0).unwrap();

		dbg!("{:#?}", Rc::strong_count(card));
		card.change_current_location(CardLocation::Graveyard);
		let card2 = lib.get_card(&"Mind's Eye".to_string(), 1).unwrap();
		assert_ne!(card.visibility_behavior.current_location, card2.visibility_behavior.current_location);
	}
}