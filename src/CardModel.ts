import {Type, CardLocation, Color, ColorMap, ColorKeywords, Multicolored, LandTypes, TapPurpose} from './Enums';
import {CardBehavior, CardVisibilityBehavior, CardEntranceBehavior, CardBattlefieldBehavior, CardExitBehavior, shouldEnterTapped} from './CardBehavior';
import dataset from './noForeignModernAtomic.json' with { type: "json" };

const LOGLEVEL = 'NORMAL';


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



// const LOGLEVEL = 'DEBUG';

const manaSearchRegex = /\{\w+\}|(\sor\s)|(\sand\s)|(add)|\./ig;

function identifyKeywords(text:string):string[]{
	let keywords = text.match(manaSearchRegex);
	if(LOGLEVEL === 'DEBUG'){ console.log('\nidentifyKeywords', keywords, text, '\n'); }
	return keywords;
}


function identifyManaToTap(text){
	let keywords = identifyKeywords(text);
	let indexOfTap = keywords.findIndex((text) => text === '{T}')
	let sliced = keywords.slice(indexOfTap);
	let endOfSentenceIndex = sliced.findIndex((text)=> text === '.');
	let manaKeywords = sliced.slice(0, endOfSentenceIndex);


	if(LOGLEVEL === 'DEBUG'){ console.log('\nidentifyManaToTap', manaKeywords, text, '\n')}
	return manaKeywords;
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

	constructor(color, name, description, rawData){
		let visibilityBehavior = new CardVisibilityBehavior(CardLocation.library, false);
		let entranceBehavior = new CardEntranceBehavior(false, false, shouldEnterTapped(description));
		let battlefieldBehavior = new CardBattlefieldBehavior(false, false, true, false, false, false, false, TapPurpose.mana)
		let exitBehavior = new CardExitBehavior(true, false, CardLocation.graveyard);
		let behavior = new CardBehavior(visibilityBehavior, entranceBehavior, battlefieldBehavior, exitBehavior);
		super(Type.land, color, 0, name, description, behavior, rawData)
		this.setManaTapAmount(identifyManaToTap(description));
	}

	setManaTapAmount(manaValueArray){
		manaValueArray.forEach((keyword)=>{
			if(this.manaTapAmount.hasOwnProperty(ColorKeywords[keyword])){
				this.manaTapAmount[ColorKeywords[keyword]]++;
			}
		});

		if(manaValueArray.includes(' or ')){
			this.manaAmountIsMutuallyExclusive = true;
		} else {
			this.manaAmountIsMutuallyExclusive = false;
		}
	}
}

class NonBasicLand extends BasicLand{
	constructor(color, name, description, rawData){
		super(color, name, description, rawData);
		this.identifyOtherTappingBehaviors(description);
		// this.log()

	}

	identifyOtherTappingBehaviors(description){
		// console.log('\n\n', description);

		// {T}: Add {C}.
		// {G/W}, {T}: Add {G}{G}, {G}{W}, or {W}{W}.' <--- FUCK.
		// ^^^^ and also the comma. fuck.
		let keywords = identifyKeywords(description);
		// console.log(description, keywords);
	}

}

const deck = [];
const lands = [];

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

console.log('dudeaaaaaakkkkkkk')

export { lands, deck };
