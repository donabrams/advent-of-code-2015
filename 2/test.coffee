expect = require('chai').expect
{parseDimensions, parseDimensionsList, getSquareFootage, wrapIt} = require './getSquareFootage'

describe 'Day 2: wrapping', () ->
	describe 'wrapIt()', () ->
		it 'requires 58 ft^2 of wrapping paper for a 2x3x4 present', () ->
			expect( wrapIt [2, 3, 4] ).to.equal 58
		it 'requires 43 ft^2 of wrapping paper for a 1x1x10 present', () ->
			expect( wrapIt [1, 1, 10] ).to.equal 43
		it 'requires 43 ft^2 of wrapping paper for a 10x1x1 present', () ->
			expect( wrapIt [10, 1, 1] ).to.equal 43
		it 'requires 848 ft^2 of wrapping paper for a 14x12x8 present', () ->
			expect( wrapIt [14, 12, 8] ).to.equal 848

	describe 'parseDimensions()', () ->
		it 'knows that "20x3x11" is [20, 3, 11]', () ->
			expect( parseDimensions "20x3x11" ).to.deep.equal [20, 3, 11]

	describe 'parseDimensionsList()', () ->
		dimensionList = """2x3x4
1x1x10
14x12x8"""
		it 'knows that "2x3x4\n1x1x10\n14x12x8" is [[2, 3, 4], [1, 1, 10], [14, 12, 8]]', () ->
			expect( parseDimensionsList dimensionList).to.deep.equal [[2, 3, 4], [1, 1, 10], [14, 12, 8]]

	describe 'getSquareFootage()', () ->
		dimensionList = """2x3x4
1x1x10
14x12x8"""
		it 'knows that "2x3x4\n1x1x10\n14x12x8" is 949 ft^2', () ->
			expect( getSquareFootage dimensionList).to.equal 949