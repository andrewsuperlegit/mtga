use std::collections::HashMap;
use crate::card::Card;
use crate::library::Deck;
use redux_rs::{Selector, Store};
use crate::state_manager::MetaGamePhase::{ActiveGame, ChooseFirstTurn, ChoosePlayerCount, ConfirmLibrary, DeclareWinner, EndGame, InitialDraw, MulliganChoice};

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

// in the future, based on the player count,
// we should have to populate this
// probably with a macro?
#[derive(Debug, Default)]
enum PlayerTurn {
	#[default]
	Player1,
	Player2
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

#[derive(Debug, Default)]
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

/// some cards will have to subscribe to current events (lol) because theyll have actions that happen
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
				use PlayerTurn::*;
				match state.player_turn {
					Player1 => Player2,
					Player2 => Player1,
				}
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
pub struct GameState{
	current_event: CurrentEvent,
	meta_game_phase: MetaGamePhase,
	player_turn: PlayerTurn,
	turn_phase: TurnPhase,
	// todo implement instant stack.
	player_count: u8,
	player_names: [String; 2],
	first_player: Player,
}

impl GameState{
	fn handle_player_cant_draw(player:Player){

	}

}
