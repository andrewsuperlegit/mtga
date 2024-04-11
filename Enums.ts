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

export enum Color{
	B = "black"
,	U = "blue"
,	C = "colorless"
,	G = "green"
,	R = "red"
,	W = "white"
}
export enum ColorMap{
	black = "B"
,	blue = "U"
,	colorless = "C"
,	green = "G"
,	red = "R"
,	white = "W"
}


export type Multicolored = Color[];

export enum LandTypes {
	swamp= 'swamp'
,	plain= 'plain'
,	forest= 'forest'
, mountain= 'mountain'
, island= 'island'
};
