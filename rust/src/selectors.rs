use redux_rs::Selector;
use crate::reducers::CurrentEvent;
use crate::state_manager::{GameState, MetaGamePhase, Player, PlayerTurn, TurnPhase};

pub struct SelectPlayerCount;

impl Selector<GameState> for SelectPlayerCount{
	type Result = usize;

	fn select(&self, state: &GameState) -> Self::Result {
		state.player_turn.players.iter().count()
	}
}

pub struct SelectPlayerNames;

impl Selector<GameState> for SelectPlayerNames{
	type Result = Vec<String>;
	fn select(&self, state: &GameState) -> Self::Result {
		state.player_turn.players.iter().map(|player| player.name.clone()).collect()
	}
}

pub struct SelectFirstPlayer;

impl Selector<GameState> for SelectFirstPlayer{
	type Result = Player;
	fn select(&self, state: &GameState) -> Self::Result {
		if state.player_turn.players.len() > 0 {
			return state.player_turn.players[0].clone();
		}
		Player{name: "No Players Added Yet".to_string() }
	}
}

// We don't NEED these selectors that don't contain logic really... but I feel like it's better to
// do this by cloning stuff than it would be to have to worry about adding lifetime specifiers
// in the future- which if we didn't use selectors that clone the state i feel like we'd run into.
// Plus it's consistent; so doing anything with state will always look like:
// store.select(_Selector_).await
pub struct SelectCurrentEvent;

impl Selector<GameState> for SelectCurrentEvent {
	type Result = CurrentEvent;
	fn select(&self, state: &GameState) -> Self::Result {
		state.current_event.clone()
	}
}

pub struct SelectMetaGamePhase;

impl Selector<GameState> for SelectMetaGamePhase {
	type Result = MetaGamePhase;
	fn select(&self, state: &GameState) -> Self::Result {
		state.meta_game_phase.clone()
	}
}

pub struct SelectPlayerTurn;

impl Selector<GameState> for SelectPlayerTurn {
	type Result = PlayerTurn;
	fn select(&self, state: &GameState) -> Self::Result {
		state.player_turn.clone()
	}
}

pub struct SelectTurnPhase;

impl Selector<GameState> for SelectTurnPhase {
	type Result = TurnPhase;
	fn select(&self, state: &GameState) -> Self::Result {
		state.turn_phase.clone()
	}
}
