const UP = '(';
const DOWN = ')';

export default function moveItSanta(directions) {
	return directions.split('').reduce((floor, character)=> {
		if (character === UP) return floor + 1;
		if (character === DOWN) return floor - 1;
		throw new ERROR('HOHOHO, MERRY WTF');
	}, 0);
};