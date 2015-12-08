import {expect} from 'chai';
import santa from './santa';

describe('advent day 1', ()=> {
	it ('(()) and ()() both result in floor 0.', ()=> {
		expect(santa('(())')).to.equal(0);
		expect(santa('()()')).to.equal(0);
	});
	it ('((( and (()(()( both result in floor 3.', ()=> {
		expect(santa('(((')).to.equal(3);
		expect(santa('(()(()(')).to.equal(3);
	});
	it ('))((((( also results in floor 3.', ()=> {
		expect(santa('))(((((')).to.equal(3);
	});
	it ('()) and ))( both result in floor -1 (the first basement level).', ()=> {
		expect(santa('())')).to.equal(-1);
		expect(santa('))(')).to.equal(-1);
	});
	it ('))) and )())()) both result in floor -3.', ()=> {
		expect(santa(')))')).to.equal(-3);
		expect(santa(')())())')).to.equal(-3);
	});
});