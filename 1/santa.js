const UP = '(';
const DOWN = ')';

export default function moveItSanta(directions) {
	return directions.split('').reduce((floor, character)=> {
		if (character === UP) return floor + 1;
		if (character === DOWN) return floor - 1;
		throw new ERROR('HOHOHO, MERRY WTF');
	}, 0);
};

export function moveItSantaPt2(directions) {
	return directions.split('').reduce((floor, character, i)=> {
		if (floor.done) return floor;
		if (character === UP) return floor + 1;
		if (character === DOWN) {
			if (floor === 0) {
				return {
					done: true,
					basement: i
				};
			}
			return floor - 1;
		}
		throw new ERROR('HOHOHO, MERRY WTF');
	}, 0).basement+1;
};