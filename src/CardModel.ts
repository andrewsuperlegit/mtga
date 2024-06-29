import {CardLocation, Color, ColorKeywords, ColorMap, LogicalQuantifiers, TapPurpose, Type} from './Enums';
import {
	CardBattlefieldBehavior,
	CardBehavior,
	CardEntranceBehavior,
	CardExitBehavior,
	CardVisibilityBehavior,
	shouldEnterTapped
} from './CardBehavior';
import dataset from './noForeignModernAtomic.json' with {type: 'json'};
import {identifyManaToTap, identifyKeywords} from "@/CardHelpers";

const deck = [];
const lands = [];

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
	toString():string{
		let str = "";
		for (let [color, amount] of Object.entries(this.costs)){
			str += `${color}: ${amount}`;
		}
		return str;
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

	constructor(type: Type, color: Color[], cost: Cost, name: string, description: string,
							behavior: CardBehavior, rawData: Object, location: CardLocation = CardLocation.library) {
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
	manaTapAmount = {
		[Color.B]: 0
	,	[Color.W]: 0
	,	[Color.G]: 0
	,	[Color.R]: 0
	,	[Color.U]: 0
	, [Color.C]: 0
	};
	manaAmountIsMutuallyExclusive: boolean = false;

	constructor(color:Color[], name:string, description:string, rawData:Object){
		let cost = new Cost();
		let visibilityBehavior = new CardVisibilityBehavior(CardLocation.library, false);
		let entranceBehavior = new CardEntranceBehavior(false, false, shouldEnterTapped(description));
		let battlefieldBehavior = new CardBattlefieldBehavior(false, false, true, false, false, false, false, TapPurpose.mana)
		let exitBehavior = new CardExitBehavior(true, false, CardLocation.graveyard);
		let behavior = new CardBehavior(visibilityBehavior, entranceBehavior, battlefieldBehavior, exitBehavior);
		super(Type.land, color, cost, name, description, behavior, rawData)
		this.setManaTapAmount(identifyManaToTap(description));
	}

	setManaTapAmount(manaValueArray: string[]){
		manaValueArray.forEach((keyword)=>{
			if(this.manaTapAmount.hasOwnProperty(ColorKeywords[keyword])){
				this.manaTapAmount[ColorKeywords[keyword]]++;
			}
		});

		if(manaValueArray.includes(LogicalQuantifiers.OR)){
			this.manaAmountIsMutuallyExclusive = true;
		} else {
			this.manaAmountIsMutuallyExclusive = false;
		}
	}
}

class NonBasicLand extends BasicLand{
	constructor(color:Color[], name:string, description:string, rawData:Object){
		super(color, name, description, rawData);
		this.identifyOtherTappingBehaviors(description);
		// this.log()

	}

	identifyOtherTappingBehaviors(description:string){
		// console.log('\n\n', description);

		// {T}: Add {C}.
		// {G/W}, {T}: Add {G}{G}, {G}{W}, or {W}{W}.' oh no
		// ^^^^ and also the comma. super oh no.
		let keywords = identifyKeywords(description);
		// console.log(description, keywords);
	}

}

// man the rust implementation of this is so much better.
for (let cardname in dataset){
	let card = dataset[cardname][0];
	if(card.types.includes(Type.land)){
		let color = (card.colorIdentity.length > 0) ? card.colorIdentity[0] : ColorMap.colorless;

		if(card.supertypes.includes(Type.basic)){
			deck.push(new BasicLand(color, card.name, card.text, card))
		} else {
			if(card.name === "Wooded Bastion"){
				lands.push(new NonBasicLand(color, card.name, card.text, card));
			}
		}
	}
}

deck.forEach(c=>{
	// console.log(c);
})

lands.forEach(l=>{
	// console.log('\n', l.name, l)
	if(l.name === "Wooded Bastion"){
		// console.log(l);
	}
});

export type {Payment};
export {lands, deck, Cost, StandardCard, BasicLand, NonBasicLand};
