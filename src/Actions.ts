import {createSlice} from '@reduxjs/toolkit';
import type { PayloadAction } from '@reduxjs/toolkit'
import {deck, lands} from './CardModel';

export interface GameState {
  libraries: object[]
}


enum MetaGamePhase{
	ChoosePlayerCount
,	ConfirmLibrary
,	ChooseFirstTurn
,	InitialDraw
,	MulliganChoice
,	ActiveGame
,	EndGame
,	DeclareWinner
}

enum GamePhase{
	PreGame
}

const initialState: GameState = {
	libraries: [deck, lands]
,	graveyards: []
,	hands: []
,	exiles: []
, metaGamePhase: MetaGamePhase.ChoosePlayerCount
, gamePhase: GamePhase.PreGame
, playerCount: 0
, confirmedLibraries: []
, playerNames: []
, firstPlayer: -1
, currentPlayersTurn: -1
}


export const gameSlice = createSlice({
	name: 'game'
,	initialState
,	reducers: {
		tap: (state, action: PayloadAction<object>) => {
			state.value = action.payload;
		}
	,	progressMetaGamePhase: (state, action:PayloadAction<number>)=>{
			state.metaGamePhase += 1;
		}
	,	confirmLibrary: (state, action:PayloadAction<object>)=>{
			console.log(action);
			// if(state.confirmedLibraries.length === state.playerCount){

			// }
		}
  }
})


export const { tap } = gameSlice.actions;

const gameReducer = gameSlice.reducer

export {gameReducer};
