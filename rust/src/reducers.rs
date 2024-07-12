use crate::state_manager::{GameState, Player};

#[derive(Debug, Clone, PartialEq)]
/// Event Sources are things that cause an event to take place. For Example, if a creature gets killed
/// by an instant like murder, that'd be EventSource::Card("player_1", "Murder")
pub enum EventSource{
	/// A card that caused the event to take place. Args are: (player_name, card_name)
	Card(String, String),
	/// a thing that happens as a natural result of the game like CardRegeneratedLife (which happens
	/// in the upkeep phase).
	NaturalProgression,
	/// vector of tuples of (player_names, card_names). Like if multiple creatures are blocking
	Damage(Vec<(String, String)>),
	/// playerName
	Player(Player),
}

/// some cards will have to subscribe to current events because theyll have actions that happen
/// when certain events take place. like some cards deal damage when a card enters the graveyard, etc.
#[derive(Debug, Default, Clone, PartialEq)]
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
	None,
	#[default]
	NewGame
}

#[derive(Debug)]
pub enum Action {
	UpdateCurrentEvent{event: CurrentEvent},
	ProgressMetaGamePhase,
	RegressMetaGamePhase,
	ProgressTurn,
	ProgressTurnPhase,
	AddPlayer(String)
}

pub fn reducer(mut state: GameState, action: Action) -> GameState {
	match action{
		Action::UpdateCurrentEvent {event} => GameState {
			current_event: event,
			..state
		},
		Action::ProgressMetaGamePhase => GameState{
			 meta_game_phase: {
				 use crate::state_manager::MetaGamePhase::*;
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
				use crate::state_manager::MetaGamePhase::*;
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
				use crate::state_manager::TurnPhase::*;
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
		Action::AddPlayer(name) => GameState{
			player_turn: {
				state.player_turn.add_player(name)
			},
			..state
		},
	}
}

