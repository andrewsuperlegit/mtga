import Card from './Card';

export default function Library({whos, deck}){
	deck = deck || []
	return(
		<>
			<section>
				<h1>{whos} Library</h1>
				<h2>{whos} deck has {deck.length} cards in it</h2>
				<div className="library">
				{
					deck.map((card, idx)=>{
						return <Card key={idx} keyName={`${idx}-name`} name={card.name} color={card.color} cost={card.cost} description={card.description} behavior={card.behavior}/>
					})
				}
				</div>
			</section>
		</>
	)
}
