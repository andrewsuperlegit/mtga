use redux_rs::{Selector, Store};
use crate::reducers::CurrentEvent;
use crate::state_manager::MetaGamePhase::*;

#[derive(Debug, Default, Clone)]
/// The different phases of the ACT of playing an MTG game.
pub enum MetaGamePhase{
	#[default]
	/// Choose number of players and name them.
	ChoosePlayerCount,
	/// Allow the custom input each player's library if a player doesnt have a deck;
	/// any sideboard substitutions if they already do.
	ConfirmLibrary,
	/// Players will roll a d20 to determine who goes first or play some other minigame of some kind.
	ChooseFirstTurn,
	/// The drawing of the first hand.
	InitialDraw,
	/// The choice of whether to keep the first hand.
	MulliganChoice,
	/// An active game of MTG!
	ActiveGame,
	/// The game ending - cleaning up the decks and stuff.
	EndGame,
	/// The winner declared
	DeclareWinner,
	/// Error state.
	None
}

#[derive(Debug, Default, Clone, PartialEq)]
/// Turn Phases are the standard phases of a Magic the Gathering turn phase. Once we implement the
/// instant/flash stack/mechanics this is gonna be super useful.
pub enum TurnPhase{
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

#[derive(Debug, Default, Clone, PartialEq)]
/// A player is someone playing the game, identified in the ChoosePlayerCount step of MetaGamePhase
/// In the future we'll have like profile pics and avatars and stuff but right now it's just a name
pub struct Player{
	pub name: String
}


fn queue_events(events: Vec<CurrentEvent>){
	todo!("implement this so you can queue a bunch of events to run one after another");
}


#[derive(Debug, Default, Clone)]
pub struct PlayerTurn{
	pub players: Vec<Player>,
	pub current_turn: Player,
}

impl PlayerTurn{
	pub fn new(players:Vec<Player>) -> PlayerTurn{
		let first_player = &players.clone()[0];
		PlayerTurn{
			players,
			current_turn: first_player.clone(),
		}
	}

	pub fn add_player(&mut self, name: String) -> Self{
		self.players.push(Player{name});
		Self{
			players: self.players.clone(),
			current_turn: self.current_turn.clone()
		}
	}

	pub fn change_turn(&mut self) -> Self{
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
	pub current_event: CurrentEvent,
	pub meta_game_phase: MetaGamePhase,
	pub player_turn: PlayerTurn,
	pub turn_phase: TurnPhase,
	// todo implement instant stack.
}


impl GameState{
	pub fn new() -> Self{
		Self{
			current_event: CurrentEvent::NewGame,
			meta_game_phase: MetaGamePhase::ChoosePlayerCount,
			player_turn: PlayerTurn{
				players: vec![]
			,	current_turn: Player{name: "default".to_string()}
			},
			turn_phase: TurnPhase::Untap
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::reducers::{Action, reducer};
	use crate::selectors::{SelectCurrentEvent, SelectFirstPlayer, SelectPlayerCount, SelectPlayerNames, SelectTurnPhase};
	use super::*;

	#[tokio::test]
	async fn initial_game_state_redux(){
		let gs = GameState::new();
		let store = Store::new_with_state(reducer, gs);

		store.subscribe(|state: &GameState | {
			assert_eq!(state.player_turn.players[0].name, "Andrew".to_string());
		}).await;

		assert_eq!(store.select(SelectCurrentEvent).await, CurrentEvent::NewGame);
		assert_eq!(store.select(SelectPlayerCount).await, 0);
		assert_eq!(store.select(SelectPlayerNames).await.len(), 0);
		assert_eq!(store.select(SelectFirstPlayer).await.name, "No Players Added Yet");
		store.dispatch(Action::AddPlayer("Andrew".to_string())).await;
		assert_eq!(store.select(SelectPlayerCount).await, 1);
		assert_eq!(store.select(SelectPlayerNames).await.len(), 1);
		assert_eq!(store.select(SelectFirstPlayer).await.name, "Andrew");
		store.dispatch(Action::AddPlayer("Adam".to_string())).await;
		assert_eq!(store.select(SelectPlayerCount).await, 2);
		store.dispatch(Action::ProgressTurnPhase).await;
		assert_eq!(store.select(SelectTurnPhase).await, TurnPhase::Upkeep);


	}

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

