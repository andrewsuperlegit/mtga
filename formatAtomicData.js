import dataset from './data/ModernAtomic.json' with { type: "json" };
import * as fs from 'node:fs/promises';

for(let cardName in dataset.data){
	let card = dataset.data[cardName];

	if(card.length > 1){
		for(let cardindex in card){
			let subcard = card[cardindex];
			delete subcard.foreignData;
			delete subcard.legalities;
			delete subcard.purchaseUrls;
			delete subcard.rulings;
			delete subcard.printings;
		}
	} else {
		let subcard = card[0];
		delete subcard.foreignData;
		delete subcard.legalities;
		delete subcard.purchaseUrls;
		delete subcard.rulings;
		delete subcard.printings;

	}
}


(async function main(){
	try{
		await fs.writeFile('./noForeignModernAtomic.json', JSON.stringify(dataset.data));
		console.log('good job dude');
	} catch(err){
		console.error('hey bro calm down')
		console.warn(err);
	}


})();
