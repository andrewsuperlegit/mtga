import Card from './Card';
import {deck} from './../CardModel';


export default function Library({name, color, cost, description}){
	return(
		<>
		<section>
			<h1>Library</h1>
			<h2>deck has {deck.length} cards in it</h2>
			<div className="library">
			{
				deck.map((card, idx)=>{
					return <Card key={idx} keyName={`${idx}-name`} name={card.name} color={card.color} cost={card.cost} description={card.description}/>
				})
			}
			</div>
		</section>
		</>
	)
}
