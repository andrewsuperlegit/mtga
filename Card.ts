import {Type, CardLocation, Color, ColorMap, Multicolored, LandTypes} from './Enums.js';
import dataset from './noForeignModernAtomic.json' with { type: "json" };

// console.log(Object.keys(dataset))
/*
"Swamp": [
	{
		"colorIdentity": [
			"B"
		],
		"colors": [],
		"convertedManaCost": 0,
		"edhrecSaltiness": 0.29,
		"firstPrinting": "LEA",
		"identifiers": {
			"scryfallOracleId": "56719f6a-1a6c-4c0a-8d21-18f7d7350b68"
		},
		"layout": "normal",
		"manaValue": 0,
		"name": "Swamp",
		"subtypes": [
			"Swamp"
		],
		"supertypes": [
			"Basic"
		],
		"text": "({T}: Add {B}.)",
		"type": "Basic Land — Swamp",
		"types": [
			"Land"
		]
	}
],
*/

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

/*
	CardBehavior is how the card behaves as a card.
	Like... if when you're playing, what you PHYSICALLY can denote with the card via its position
	orientation, location, etc.
*/
class CardBehavior{

	tapsForMana: boolean;
	entersBattlefieldTapped: boolean;
	hitsGraveyardOnDeath: boolean;
	hitsExileOnDeath: boolean;
	locationOnDeath: CardLocation;

	constructor(entersBattlefieldTapped, tapsForMana=false, hitsGraveyardOnDeath=true, hitsExileOnDeath=false, locationOnDeath=CardLocation.graveyard){
		this.tapsForMana = tapsForMana;
		this.entersBattlefieldTapped = entersBattlefieldTapped;
		this.hitsGraveyardOnDeath = hitsGraveyardOnDeath;
		this.hitsExileOnDeath = hitsExileOnDeath;
		this.locationOnDeath = locationOnDeath;
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


function shouldEnterTapped(cardInfoText){
	return cardInfoText.includes("enters the battlefield tapped");
}

class BasicLand extends StandardCard{
	constructor(color, name, description, rawData){
		let behavior = new CardBehavior(shouldEnterTapped(description), true);
		super(Type.land, color, 0, name, description, behavior, rawData)
	}
}

class NonbasicLand extends BasicLand{
	constructor(color, name, description, rawData){

		super(color, name, description, rawData);

	}
}

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


lands.forEach(l=>{
	// console.log(l.types, l.supertypes, l.subtypes, l.text)
});

/*
let ds = "Darkslick Shores";
let dsdesc = `${ds} enters the battlefield tapped unless you control two or fewer lands.`;
let dsbeh = new CardBehavior(true, true);
let blueBlackLand = new StandardCard(
	Type.land,
	[Color.black, Color.blue],
	0,
	ds,
	dsdesc,
	dsbeh
);

blueBlackLand.log();
*/
