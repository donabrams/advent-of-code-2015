wrapIt = ([x, y, z]) ->
	max = Math.max x, y, z
	2 * (x*y + y*z + x*z) + x*y*z/max

parseDimensions = (presentDimensions) ->
	presentDimensions.split( 'x' )
		.map (a)-> parseInt a, 10

parseDimensionsList = (presentDimensionsList) ->
	(parseDimensions dimension for dimension in presentDimensionsList.split '\n')

getSquareFootage = (presentDimensionsList) ->
	(wrapIt dimension for dimension in parseDimensionsList presentDimensionsList)
		.reduce (totalFootage, addtlFootage) -> totalFootage + addtlFootage

module.exports = 
	wrapIt: wrapIt
	parseDimensions: parseDimensions
	parseDimensionsList: parseDimensionsList
	getSquareFootage: getSquareFootage
