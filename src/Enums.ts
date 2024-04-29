export enum Type{
	artifact= "Artifact"
,	battle= "Battle"
,	commander= "Commander"
,	creature= "Creature"
,	enchantment= "Enchantment"
,	equipment= "Equipment"
,	instant= "Instant"
,	land= "Land"
, planeswalker= "Planeswalker"
, socery= "Sorcery"
, basic = "Basic"
, legendary = "Legendary"
, snow = "Snow"
};

export enum CardLocation {
	exile= 'exile'
,	graveyard= 'graveyard'
,	hand= 'hand'
,	library= 'library'
};

export enum Color {
	B = "B"
,	U = "U"
,	C = "C"
,	G = "G"
,	R = "R"
,	W = "W"
}

export enum ColorMap{
	black = "B"
,	blue = "U"
,	colorless = "C"
,	green = "G"
,	red = "R"
,	white = "W"
}

function createBracketWrappedColors(){
	const bracketed = {};
	for (const [key, value] of Object.entries(Color)) {
		bracketed[`{${key}}`] = value;
	}
	return bracketed;
}

export const ColorKeywords = createBracketWrappedColors();


export type Multicolored = Color[];

export enum LandTypes {
	swamp= 'swamp'
,	plain= 'plain'
,	forest= 'forest'
, mountain= 'mountain'
, island= 'island'
};

export enum TapPurpose{
	mana= 'mana'
,	action= 'action'
}
