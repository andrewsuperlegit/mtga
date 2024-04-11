import {Type, CardLocation, Color, ColorMap, Multicolored, LandTypes, TapPurpose} from './Enums.js';
import {CardBehavior, CardVisibilityBehavior, CardEntranceBehavior, CardBattlefieldBehavior, CardExitBehavior, shouldEnterTapped} from './CardBehavior.js';
import dataset from './noForeignModernAtomic.json' with { type: "json" };


type Payment = [color: Color, quantity: number];

class Cost {
	costs = {
		[Color.B]: 0
	,	[Color.W]: 0
	,	[Color.G]: 0
	,	[Color.R]: 0
	,	[Color.U]: 0
	, [Color.C]: 0
	};
	constructor(...payments:Payment[]){
		for (let payment of payments){
			this.costs[payment[0]] = payment[1];
		}
	}
	log(){
		for(let c in this.costs){
			console.log(`${c} : ${this.costs[c]}`)
		}
	}
}


class StandardCard{
	behavior: CardBehavior;
	location: CardLocation;
	color: Color[];
	cost: Cost | 0;
	description: string;
	name: string;
	type: Type;
	rawData: Object
	constructor(type, color, cost, name, description, behavior, rawData, location=CardLocation.library){
		this.behavior = behavior;
		this.color = color;
		this.cost = cost;
		this.description = description;
		this.location = location;
		this.name = name;
		this.type = type;
		this.rawData = rawData;
	}

	log(){
		console.log(this.name);
		console.log(this);
	}
}




class BasicLand extends StandardCard{
	constructor(color, name, description, rawData){
		let visibilityBehavior = new CardVisibilityBehavior(CardLocation.library, false);
		let entranceBehavior = new CardEntranceBehavior(false, false, shouldEnterTapped(description));
		let battlefieldBehavior = new CardBattlefieldBehavior(false, false, true, false, false, false, false, TapPurpose.mana)
		let exitBehavior = new CardExitBehavior(true, false, CardLocation.graveyard);
		let behavior = new CardBehavior(visibilityBehavior, entranceBehavior, battlefieldBehavior, exitBehavior);
		super(Type.land, color, 0, name, description, behavior, rawData)
	}
}
//
// class NonbasicLand extends BasicLand{
// 	constructor(color, name, description, rawData){
//
// 		super(color, name, description, rawData);
//
// 	}
// }

const deck = [];
const lands = [];

for (let cardname in dataset){
	let card = dataset[cardname][0];
	if(card.types.includes(Type.land)){
		lands.push(card);
		if(card.supertypes.includes(Type.basic)){
			let color = (card.colorIdentity.length > 0) ? card.colorIdentity[0] : ColorMap.colorless;
			deck.push(new BasicLand(color, card.name, card.text, card))
		}

	}
}

deck.forEach(c=>{
	console.log(c);
})

lands.forEach(l=>{
	// console.log(l.types, l.supertypes, l.subtypes, l.text)
});
