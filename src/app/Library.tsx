import Card from './Card';

import { useSelector, useDispatch } from 'react-redux';
import { confirmLibrary } from '@/Actions';
import {RootState} from "@reduxjs/toolkit/query";
import {StandardCard} from "@/CardModel";

interface LibraryProp{
	whos: string;
	deck: StandardCard[]
}

export default function Library({whos, deck}: LibraryProp) {
	// const game = useSelector((state: RootState) => state.game);
	const dispatch = useDispatch();

	deck = (deck) ? deck : []

	// todo: redux is gonna have an issue with this because decks aren't serializable.
	function dispatchConfirmLibrary(whos, deck) {
		dispatch(confirmLibrary([whos, deck]));
	}

	function unconfirmLibrary(whos, deck) {

	}

	return (
		<>
			<div>
				<p>Are you cool with this library?</p>
				<button onClick={() => dispatchConfirmLibrary(whos, deck)}>Yes</button>
				<button>No</button>
			</div>

			<section>
				<h1>{whos} Library</h1>
				<h2>{whos} deck has {deck.length} cards in it</h2>

				<div className="library">
					{
						deck.map((card: StandardCard, idx: number) => {
							return <Card key={idx} keyName={`${idx}-name`} card={card}/>
						})
					}
				</div>
			</section>
		</>
	)
};
