import dataset from './data/AtomicCards.json' with { type: "json" };
import * as fs from 'node:fs/promises';


for(let cardName in dataset.data){
	dataset.data[cardName] = dataset.data[cardName].filter(_card => _card.isFunny !== true )

	let card = dataset.data[cardName];
	if (card.length === 0) {
		delete dataset.data[cardName];
		continue;
	}

	if(card.length > 1){
		for(let cardindex in card){
			let subcard = card[cardindex];
			delete subcard.foreignData;
			delete subcard.legalities;
			delete subcard.purchaseUrls;
			delete subcard.rulings;
			delete subcard.printings;
			delete subcard.edhrecRank;
			delete subcard.edhrecSaltiness;
			delete subcard.firstPrinting;
			delete subcard.identifiers;
		}
	} else {
		let subcard = card[0];
		delete subcard.foreignData;
		delete subcard.legalities;
		delete subcard.purchaseUrls;
		delete subcard.rulings;
		delete subcard.printings;
		delete subcard.edhrecRank;
		delete subcard.edhrecSaltiness;
		delete subcard.firstPrinting;
		delete subcard.identifiers;
	}
}


(async function main(){
	try{
		let data = { library: dataset.data };
		await fs.writeFile('./Atomic.json', JSON.stringify(data, null, '  '));
		// return JSON.stringify(JSON.parse(text), null, this.step);

		console.log('good job dude');
	} catch(err){
		console.error('hey bro calm down')
		console.warn(err);
	}


})();
