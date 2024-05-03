import {createSlice} from '@reduxjs/toolkit';
import type { PayloadAction } from '@reduxjs/toolkit'
import {deck, lands} from './CardModel';

export interface GameState {
  libraries: object[]
}

const initialState: GameState = {
  libraries: [deck, lands]
}
// need to store the both decks here,
// and then search the deck for the card being tapped and then tap it.
export const gameSlice = createSlice({
  name: 'game',
  initialState,
  reducers: {
    tap: (state, action: PayloadAction<object>) => {
			console.log(action);
      state.value = action.payload;
    }
  },
})


export const { tap } = gameSlice.actions;

const gameReducer = gameSlice.reducer

export {gameReducer};
