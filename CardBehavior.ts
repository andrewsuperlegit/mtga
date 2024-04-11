import {CardLocation, TapPurpose} from './Enums.js';

/*
	CardBehaviors are how the card behaves as a card.
	Like... if when you're playing, what you PHYSICALLY can denote with the card via its position
	orientation, location, etc.
*/
export class CardBehavior{
	currentLocation: CardLocation;
	visibilityBehavior: CardVisibilityBehavior;
	entranceBehavior: CardEntranceBehavior;
	battlefieldBehavior: CardBattlefieldBehavior;
	exitBehavior: CardExitBehavior;
	canHaveSummoningSickness: boolean;
	constructor(visibilityBehavior, entranceBehavior, battlefieldBehavior, exitBehavior){
		this.currentLocation = visibilityBehavior.currentLocation;
		this.visibilityBehavior = visibilityBehavior;
		this.canHaveSummoningSickness = entranceBehavior.canHaveSummoningSickness;
		this.entranceBehavior = entranceBehavior;
		this.battlefieldBehavior = battlefieldBehavior;
		this.exitBehavior = exitBehavior;
	}
}

export class CardVisibilityBehavior{
	currentLocation: CardLocation;
	isRevealed: boolean;
	constructor(currentLocation, isRevealed){
		this.currentLocation = currentLocation;
		this.isRevealed = isRevealed;
	}
}

export class CardEntranceBehavior{
	canHaveSummoningSickness: boolean;
	entersBattlefieldOnInstantStack: boolean;
	entersBattlefieldTapped: boolean;
	constructor(canHaveSummoningSickness, entersBattlefieldOnInstantStack, entersBattlefieldTapped){
		this.entersBattlefieldOnInstantStack = entersBattlefieldOnInstantStack;
		this.entersBattlefieldTapped = entersBattlefieldTapped;
		this.canHaveSummoningSickness = canHaveSummoningSickness;
	}
}

export class CardBattlefieldBehavior{
	canAttack: boolean; // is it POSSIBLE for the card to attack?
	canBlock: boolean; // is it POSSIBLE for the card to block?
	canTap: boolean; // is it POSSIBLE for the card to tap at all?
	canTurnFaceUp: boolean; //is it POSSIBLE for the card to turn facedown/faceup?
	isFaceDown: boolean;
	isSummonSick: boolean;
	isTapped: boolean;
	tapPurpose: TapPurpose;
	constructor(canAttack, canBlock, canTap, canTurnFaceUp, isFaceDown, isSummonSick, isTapped, tapPurpose){
		this.canAttack = canAttack;
		this.canBlock = canBlock;
		this.canTap = canTap;
		this.canTurnFaceUp = canTurnFaceUp;
		this.isFaceDown = isFaceDown;
		this.isTapped = isTapped;
		this.tapPurpose = tapPurpose;
	}
}

export class CardExitBehavior{
	hitsGraveyardOnDeath: boolean;
	hitsExileOnDeath: boolean;
	locationOnDeath: CardLocation;
	constructor(hitsGraveyardOnDeath, hitsExileOnDeath, locationOnDeath){
		this.hitsGraveyardOnDeath = hitsGraveyardOnDeath;
		this.hitsExileOnDeath = hitsExileOnDeath;
		this.locationOnDeath = locationOnDeath;
	}
}


export function shouldEnterTapped(cardInfoText){
	return cardInfoText.includes("enters the battlefield tapped");
}
