import {fdescribe, describe, expect, it, fit, xit, xdescribe} from "@jest/globals";
import {identifyKeywords, identifyManaToTap} from "@/CardHelpers";


describe("identifyKeywords", ()=>{
	it("works", ()=>{
		expect(1).toBe(1);
	});

	it("identifies tap keywords", ()=>{
		let str = "{T}: Prevent the next 1 damage that would be dealt to " +
			"any target this turn.\n{T}: Prevent the next 2 damage that would " +
			"be dealt to target artifact creature this turn.";
			expect(identifyKeywords(str)).toContain("{T}");
	});
	it("identifies color keywords", ()=>{
		let str = "Emerge {7}{G} (You may cast this spell by sacrificing a " +
			"creature and paying the emerge cost reduced by that creature's " +
			"mana value.)\\nWhen you cast this spell, " +
			"you may destroy target artifact or enchantment.";
		let result = identifyKeywords(str);
		expect(result).toContain("{7}");
		expect(result).toContain("{G}");
	});
	it("identifies statement terminators (aka periods)", ()=>{
		let str = "{T}: Prevent the next 1 damage that would be dealt to " +
			"any target this turn.\n{T}: Prevent the next 2 damage that would " +
			"be dealt to target artifact creature this turn.";
		expect(identifyKeywords(str)).toContain(".");
	});
	it("identifies logical qualifiers (and and or) but NOT words" +
		"that contain and or or like 'hand' and 'terminator'", ()=>{
		let str = "This and that, or but not terminator- aNd case insensitive oR OR or";
		let result = identifyKeywords(str);

		expect(result).toContain("AND");
		expect(result).toContain("OR");
		expect(result.length).toBe(4);
	});
	it("returns an empty array if no matches are found", ()=>{
		let str = "This, that, whole bunch of other stuff but no keywords";
		let result = identifyKeywords(str);
		expect(result).toEqual([]);
	});
	it("works fine on empty strings", ()=>{
		expect(identifyKeywords("")).toEqual([]);
	});
});

describe("identifyManaToTap", ()=>{
	it("searches for the {T} keyword and returns any mana keywords" +
		"encountered before terminator", ()=>{
		let str = "{T}: Add {C}.";
		let result = identifyManaToTap(str);
		expect(result).toContain("{C}");
		expect(result).not.toContain("{T}");
		expect(result).not.toContain("ADD");
	});
	 it("works for mana ", ()=>{
		let str = "{T}: Add {C}." +
							"{T}: Add {R}." +
							"{T}: Add {U}.";
		let result = identifyManaToTap(str);
		expect(result).toEqual(["{C}", "{R}", "{U}"]);
	})
	// "text": "{T}: Add {C}.\n{1}, {T}: Put a storage counter on Molten Slagheap.\n{1}, Remove X storage counters from Molten Slagheap: Add X mana in any combination of {B} and/or {R}.",

});
