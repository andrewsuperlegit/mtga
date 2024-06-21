import Card from './Card';

import { useSelector, useDispatch } from 'react-redux';
import { confirmLibrary } from './../Actions';

export default function Library({whos, deck}){
	const game = useSelector((state: RootState) => state.game);
	const dispatch = useDispatch();
	console.log(game);

	deck = (deck) ? deck : []

	function confirmLibrary(whos, deck){
		dispatch(confirmLibrary([whos, deck]));
	}
	function unconfirmLibrary(whos, deck){

	}

	return(
		<>
			<div>
				<p>Are you cool with this library?</p>
				<button onClick={()=>confirmLibrary(whos, deck)}>Yes</button>
				<button>No</button>
			</div>

			<section>
				<h1>{whos} Library</h1>
				<h2>{whos} deck has {deck.length} cards in it</h2>

				<div className="library">
				{
					deck.map((card, idx)=>{
						return <Card key={idx} keyName={`${idx}-name`} card={card}/>
					})
				}
				</div>
			</section>
		</>
	)
}
