use std::collections::HashMap;
use std::sync::OnceLock;
use crate::card::Card;
use crate::library::Deck;
use redux_rs::{Selector, Store};
use crate::card_db::CardDB;
use crate::state_manager::MetaGamePhase::*;

#[derive(Debug, Default)]
enum MetaGamePhase{
	#[default]
	ChoosePlayerCount,
	ConfirmLibrary,
	ChooseFirstTurn,
	InitialDraw,
	MulliganChoice,
	ActiveGame,
	EndGame,
	DeclareWinner,
	None
}

#[derive(Debug, Default)]
enum TurnPhase{
	#[default]
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
	Cleanup,
	PassTurn,
}

#[derive(Debug, Default, Clone)]
pub struct Player{
	pub name: String
}

#[derive(Debug)]
pub enum EventSource{
	/// player_name, card_name
	Card(String, String),
	NaturalProgression,
	/// vector of card names
	Damage(Vec<String>),
	/// playerName
	Player(String),
}

/// some cards will have to subscribe to current events because theyll have actions that happen
/// when certain events take place. like some cards deal damage when a card enters the graveyard, etc.
#[derive(Debug, Default)]
pub enum CurrentEvent{
	CardRevealedOnLibrary(EventSource),
	CardPlacedFaceDown(EventSource),
	CardTurnedFaceUp(EventSource),

	CardDrawn(EventSource),
	CardPlacedInGraveyard(EventSource),
	CardRemovedFromGraveyard(EventSource),

	CardPlacedInExile(EventSource),
	CardRemovedFromExile(EventSource),

	CardCasted(EventSource),
	CardAbilityActivated(EventSource),

	CardPlacedOnBattlefield(EventSource),
	CardRemovedFromBattlefield(EventSource),

	CardTapped(EventSource),
	CardUntapped(EventSource),

	CardDeclaredAsAttacker(EventSource),
	CardDeclaredAsBlocker(EventSource),

	CardDealtDamage(EventSource),
	PlayerDealtDamage(EventSource),
	PlayerKilled(EventSource),
	CardRegeneratedLife(EventSource),
	PlayerGainedLife(EventSource),

	LibraryShuffled(EventSource),
	GraveyardShuffledIntoLibrary(EventSource),
	GraveyardExiled(EventSource),
	#[default]
	None
}


fn queue_events(events: Vec<CurrentEvent>){
	todo!("implement this so you can queue a bunch of events to run one after another");
}
#[derive(Debug)]
pub enum Action {
	UpdateCurrentEvent{event: CurrentEvent},
	ProgressMetaGamePhase,
	RegressMetaGamePhase,
	ProgressTurn,
	ProgressTurnPhase,
}

pub fn reducer(mut state: GameState, action: Action) -> GameState {
	match action{
		Action::UpdateCurrentEvent {event} => GameState {
			current_event: event,
			..state
		},
		Action::ProgressMetaGamePhase => GameState{
			 meta_game_phase: {
				 use MetaGamePhase::*;
				 match state.meta_game_phase {
					 ChoosePlayerCount => ConfirmLibrary,
					 ConfirmLibrary => ChooseFirstTurn,
					 ChooseFirstTurn => InitialDraw,
					 InitialDraw => MulliganChoice,
					 MulliganChoice => ActiveGame,
					 ActiveGame => EndGame,
					 EndGame => DeclareWinner,
					 DeclareWinner => ChoosePlayerCount,
					 None => ChoosePlayerCount
				 }
			 },
			..state
		},
		Action::RegressMetaGamePhase => GameState{
			meta_game_phase: {
				use MetaGamePhase::*;
				match state.meta_game_phase {
					ChoosePlayerCount => None,
					ConfirmLibrary => ChoosePlayerCount,
					ChooseFirstTurn => ConfirmLibrary,
					InitialDraw => ChooseFirstTurn,
					MulliganChoice => InitialDraw,
					ActiveGame => MulliganChoice,
					EndGame => ActiveGame,
					DeclareWinner => EndGame,
					None => ChoosePlayerCount
				}
			},
			..state
		},
		Action::ProgressTurn => GameState{
			player_turn: {
				state.player_turn.change_turn()
			},
			..state
		},
		Action::ProgressTurnPhase => GameState {
			turn_phase: {
				use TurnPhase::*;
				match state.turn_phase {
					Untap => Upkeep,
					Upkeep => Draw,
					Draw => PreCombatMainPhase,
					PreCombatMainPhase => CombatPhase,
					CombatPhase => DeclareAttackers,
					DeclareAttackers => DeclareBlockers,
					DeclareBlockers => CombatResolution,
					CombatResolution => EndCombat,
					EndCombat => PostCombatMainPhase,
					PostCombatMainPhase => End,
					End => Cleanup,
					Cleanup => PassTurn,
					PassTurn => Untap,
				}
			},
			..state
		},
	}
}


#[derive(Debug, Default)]
struct PlayerTurn{
	players: Vec<Player>,
	current_turn: Player,
}

impl PlayerTurn{
	fn new(players:Vec<Player>) -> PlayerTurn{
		let first_player = &players.clone()[0];
		PlayerTurn{
			players,
			current_turn: first_player.clone(),
		}
	}

	fn add_player(&mut self, name: String){
		self.players.push(Player{name});
	}

	fn change_turn(&mut self) -> Self{
		let new_players_turn =  {
			// find which index in the vector the player whose turn it is currently is.
			let index = &self.players.iter().position(|n| {
				n.name == self.current_turn.name
			}).unwrap();

			// if that index + 1 is the length of the vector, return first player in the vector
			if index + 1 >= self.players.len() {
				self.players[0].clone()
				// otherwise return the next player in the vector
			} else {
				self.players[index + 1].clone()
			}
		};
		self.current_turn = new_players_turn;
		Self{
			players: self.players.clone(),
			current_turn: self.current_turn.clone(),
		}
	}
}

#[derive(Debug, Default)]
pub struct GameState{
	current_event: CurrentEvent,
	meta_game_phase: MetaGamePhase,
	player_turn: PlayerTurn,
	turn_phase: TurnPhase,
	// todo implement instant stack.
}

struct SelectPlayerCount;
impl Selector<GameState> for SelectPlayerCount{
	type Result = usize;

	fn select(&self, state: &GameState) -> Self::Result {
		state.player_turn.players.iter().count()
	}
}

struct SelectPlayerNames;
impl Selector<GameState> for SelectPlayerNames{
	type Result = Vec<String>;

	fn select(&self, state: &GameState) -> Self::Result {
		state.player_turn.players.iter().map(|player| player.name.clone()).collect()
	}
}

struct SelectFirstPlayer;
impl Selector<GameState> for SelectFirstPlayer{
	type Result = Player;

	fn select(&self, state: &GameState) -> Self::Result {
		state.player_turn.players[0].clone()
	}
}


impl GameState{
	fn new() -> Self{
		Self{
			current_event: CurrentEvent::None,
			meta_game_phase: MetaGamePhase::ChoosePlayerCount,
			player_turn: PlayerTurn{ // probably should use new here but we don't know this info yet.
				players: vec![]
			,	current_turn: Player{name: "default".to_string()}
			},
			turn_phase: TurnPhase::Untap
		}
	}
	// fn handle_player_cant_draw(player:Player){
	//
	// }
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn player_turn_change_turn_moves_to_next_player(){
		let andrew = Player{name: "Andrew".to_string() };
		let rory = Player{name: "Rory".to_string() };
		let mut player_turn = PlayerTurn::new(vec![
			andrew.clone()
		,	rory.clone()
		]);
		assert_eq!(player_turn.current_turn.name, andrew.name);
		player_turn.change_turn();
		assert_eq!(player_turn.current_turn.name, rory.name);
		player_turn.change_turn();
		assert_eq!(player_turn.current_turn.name, andrew.name);
	}

	#[test]
	fn player_turn_add_player_adds_player(){
		let andrew = Player{name: "Andrew".to_string() };
		let rory = Player{name: "Rory".to_string() };
		let rosemary = Player{name: "Rosemary".to_string() };
		let mut player_turn = PlayerTurn::new(vec![
			andrew.clone()
		,	rory.clone()
		]);
		assert_eq!(player_turn.current_turn.name, andrew.name);
		player_turn.change_turn();
		assert_eq!(player_turn.current_turn.name, rory.name);
		player_turn.add_player(rosemary.name.clone());
		player_turn.change_turn();
		assert_eq!(player_turn.current_turn.name, rosemary.name);
	}
}

