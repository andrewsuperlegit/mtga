use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::rc::Rc;
use crate::card::{CardLocation, RealCard, RealCardError};
use rand::seq::SliceRandom;
use rand::thread_rng;
use crate::state_manager::{Player};

/// a tuple with the card_name and the number of that card you want in your library/sideboard.
/// ("Forest", 20)
pub struct CardListItem(pub String, pub u8);

/// Deck is a players' library of cards they can play. It is its' own state manager.
/// cards is a hashmap of cards so that when someone searches
/// for cards, we can give them the option to type the card theyre
/// looking for and look it up in constant time.
/// sideboard is optional
#[derive(Debug)]
pub struct Deck<'a> {
	/// library is the actual library people will draw from; it can be shuffled and milled, etc.
	/// if a RealCard in cards has a quantity of 4, there will be 4 copies of the RealCard in library.
	/// order matters.
	library: Vec<Rc<RefCell<RealCard<'a>>>>,
	/// cards is a hashmap that contains all the cards in your library. Used to get cards in
	/// constant time.
	cards: HashMap<(String, u8), Rc<RefCell<RealCard<'a>>>>,
	/// sideboard is a hashmap of cards in your sideboard.
	// TODO add a function that moves cards from your library to your sideboard and vise-versa.
	sideboard: HashMap<String, Rc<RefCell<RealCard<'a>>>>,
	graveyard: Vec<Rc<RefCell<RealCard<'a>>>>,
	exile: Vec<Rc<RefCell<RealCard<'a>>>>,
	player: Player,
}

impl<'a> Deck<'a>{
	/// Accepts 2 vectors of cardnames and quantities like:
	/// Library::new([("Forest", 20),("Swamp", 15), ("Insidious Roots", 4)], [/* optional sideboard */])
	/// and converts them into a Library.
	pub fn new(player: Player, card_list: &'a Vec<CardListItem>, sideboard_list: &'a Vec<CardListItem>) -> Result<Deck<'a>, RealCardError>{
		let mut cards = HashMap::new();
		let mut library = vec![];
		let mut sideboard= HashMap::new();
		let exile = vec![];
		let graveyard = vec![];

		for card in card_list.iter(){
			let (card_name, qty) = (&card.0, card.1);
			for i in (0..qty){
				let real_card = Rc::new(RefCell::new(RealCard::new(card_name, qty)?));
				let real_card_ref = Rc::clone(&real_card);

				library.push(real_card_ref); // todo: problem... i don't know which card (i) this is in library.
				cards.insert((card_name.to_string(), i), real_card);
			}

		}
		for card in sideboard_list.iter(){
			let (card_name, qty) = (&card.0, card.1);
			let real_card = Rc::new(RefCell::new(RealCard::new(&card.0, card.1)?));
			sideboard.insert(card_name.to_string(), real_card);
		}

		Ok(Deck {
			library,
			cards,
			sideboard,
			player,
			exile,
			graveyard
		})
	}

	/// gets an immutable reference to a card.
	pub fn get_card_immut(&self, card_name: &String, card_key: u8) -> Option<Ref<RealCard<'a>>> {
		let card_name = card_name.clone();
		let card = self.cards.get(&(card_name, card_key))?.borrow();
		Some(card)
	}

	/// gets a mutable reference to a card.
	pub fn get_card(&self, card_name: &String, card_key:u8)-> Option<RefMut<RealCard<'a>>> {
		let card_name = card_name.clone();
		let card = self.cards.get(&(card_name, card_key))?.borrow_mut();
		Some(card)
	}

	/// gets all cards of a given card name.
	pub fn search_cards(&self, card_name: String) -> Vec<Ref<RealCard<'a>>> {
		let card0 = self.get_card_immut(&card_name, 0).unwrap();
		let mut vec = vec![];
		for i in (0..card0.quantity){
			vec.push(self.get_card_immut(&card_name, i).unwrap());
		}
		println!("{:#?}", vec);
		vec
	}

	/// shuffles the library
	pub fn shuffle_library(&mut self)->(){
		let mut rng = thread_rng();
		&self.library.shuffle(&mut rng);
	}

	/// draw a card from your deck's library to your hand.
	pub fn draw_card(&mut self) -> Option<Rc<RefCell<RealCard<'a>>>> {
		let card_opt = self.send_card_from_library_to_place(CardLocation::Hand);
		match card_opt {
			Some(card_opt) => Some(card_opt),
			None => None
		}
	}

	pub fn mill_card(&mut self) -> Option<Rc<RefCell<RealCard<'a>>>> {
		self.send_card_from_library_to_place(CardLocation::Graveyard)
	}

	/// todo add method for scrying/surveiling a card
	// this is hard because we have to prompt the user for input...aka wait for them to finish
	// looking at it before we put it back on library.
	// fn scry_card(&mut self) ->

	/// todo add method for revealing a card on top of library
	pub fn reveal_top_card(&mut self, should_reveal: bool) -> Option<()>{
		let card = match self.library.pop(){
			Some(card) => card,
			None => return None
		};
		card.borrow_mut().visibility_behavior.set_revealed(should_reveal);
		self.library.push(card);
		Some(())
	}

	fn send_card_from_library_to_place(&mut self, place: CardLocation)
		-> Option<Rc<RefCell<RealCard<'a>>>> {
		let card = match self.library.pop(){
			Some(card) => card,
			None => return None
		};
		card.borrow_mut().visibility_behavior.set_location(place);
		Some(card)
	}
}

#[cfg(test)]
mod tests {
	use crate::card::CardLocation;
	use super::*;

	#[test]
	/// I think that TECHNICALLY this function has the capability of failing randomly.
	///  because if you truely randomly shuffle something, it's POSSIBLE that it shuffles
	/// into the same order as before you shuffled. So if this test fails once in a while...
	/// rerun the test.
	fn library_shuffle(){
		let vec = vec![
			CardListItem("Mind's Eye".to_string(), 1),
			CardListItem("Forest".to_string(), 1),
			CardListItem("Swamp".to_string(), 1),
			CardListItem("Insidious Roots".to_string(), 1),
			CardListItem("Murder".to_string(), 1),
			CardListItem("Lightning Storm".to_string(), 1),
			CardListItem("Island".to_string(), 1),
			CardListItem("Mountain".to_string(), 1),
			CardListItem("Reject".to_string(), 1),
			CardListItem("Opt".to_string(), 1),
		];
		let vec_b = vec![];
		let mut lib = Deck::new(Player{name: "Me".to_string() } ,&vec, &vec_b).unwrap();
		let before0 = lib.library[0].borrow().name;
		let before1 = lib.library[1].borrow().name;
		let before2 = lib.library[2].borrow().name;

		lib.shuffle_library();

		let after0 = lib.library[0].borrow().name;
		let after1 = lib.library[1].borrow().name;
		let after2 = lib.library[2].borrow().name;

		assert_ne!([before0, before1, before2], [after0, after1, after2]);

	}

	#[test]
	fn library_cards_has_hashmap_of_correct_number_of_cards() {
		let vec = vec![
			CardListItem("Mind's Eye".to_string(), 3)
		];
		let vec_b= vec![];
		let lib = Deck::new(Player{name: "Me".to_string() }, &vec, &vec_b).unwrap();
		let me0 = &lib.cards[&("Mind's Eye".to_string(), 0)];
		let me1 = &lib.cards[&("Mind's Eye".to_string(), 1)];
		let me2 = &lib.cards[&("Mind's Eye".to_string(), 2)];
		assert_eq!(me0.borrow().name, "Mind's Eye".to_string());
		assert_eq!(me1.borrow().name, "Mind's Eye".to_string());
		assert_eq!(me2.borrow().name, "Mind's Eye".to_string());
		assert_eq!(lib.library.len(), 3);
	}

	#[test]
	fn search_cards_returns_vec_of_cards(){
		let vec = vec![
			CardListItem("Mind's Eye".to_string(), 3)
		];
		let vec_b= vec![];
		let lib = Deck::new(Player{name: "Me".to_string() }, &vec, &vec_b).unwrap();
		let result = lib.search_cards("Mind's Eye".to_string());
		assert_eq!(result.len(), 3);
	}

	#[test]
	fn get_card_allows_you_to_mutate_a_card(){
		let vec = vec![
			CardListItem("Mind's Eye".to_string(), 3)
		];
		let vec_b = vec![];
		let lib = Deck::new(Player{name: "Me".to_string() }, &vec, &vec_b).unwrap();
		let mut card = lib.get_card(&"Mind's Eye".to_string(), 0).unwrap();
		card.change_current_location(CardLocation::Graveyard);
		let card2 = lib.get_card_immut(&"Mind's Eye".to_string(), 1).unwrap();
		println!("{:#?}\n\n\n{:#?}", card, card2);
		assert_ne!(card.visibility_behavior.current_location, card2.visibility_behavior.current_location);

	}
}