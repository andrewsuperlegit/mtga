import {ColorKeywords, LogicalQuantifiers, LOGLEVEL} from "@/Enums";

const manaSearchRegex = /\{\w+\}|(\sor\s)|(\sand\s)|(add)|\./ig;

function identifyKeywords(text: string): string[] {
	let keywords:string[] = [];
	let matches = text.match(manaSearchRegex);
	if(matches) keywords = matches.map((elem) => elem.trim().toUpperCase());

	if (LOGLEVEL === 'DEBUG') console.log('\nidentifyKeywords', keywords, text, '\n');

	return keywords;
}

function identifyManaToTap(text: string): string[] {
	let keywords = identifyKeywords(text);
	let indexOfTap = keywords.findIndex((text) => text === '{T}');
	let sliced = keywords.slice(indexOfTap);
	let endOfSentenceIndex = sliced.findIndex((text) => text === '.');
	let manaKeywords = sliced.slice(0, endOfSentenceIndex);

	if (manaKeywords.includes(LogicalQuantifiers.ADD)) {
		manaKeywords = manaKeywords.filter((keyword) => ColorKeywords.hasOwnProperty(keyword));
		let nextStr = sliced.slice(endOfSentenceIndex).join(" ");
		manaKeywords = manaKeywords.concat(identifyManaToTap(nextStr));
	}
	if (LOGLEVEL === 'DEBUG') console.log('\nidentifyManaToTap', manaKeywords, text, '\n');

	return manaKeywords;
}

export {identifyManaToTap};
export {identifyKeywords};