// Literal declarations only; the author reads bytes rather than importing it.
const failure='literal failure';
export const nativeLiteralAssertions=[
 {operation:'readNativeLiteralNumber',arguments:[NaN],expected:NaN},
 {operation:'readNativeLiteralNumber',arguments:[Infinity],expected:Infinity},
 {operation:'readNativeLiteralNumber',arguments:[-Infinity],expected:-Infinity},
 {operation:'readNativeLiteralNumber',arguments:[-0],expected:-0},
 {operation:'readNativeLiteralChoice',arguments:[false],expected:failure},
];
nativeLiteralAssertions.push({operation:'readNativeLiteralChoice',arguments:[true],expected:'yes'});
nativeLiteralAssertions.push(...[{operation:'readNativeLiteralChoice',arguments:[false],expected:failure}]);
export const mixedLiteralAssertions=[
 {operation:'readNativeLiteralChoice',arguments:[false],expected:failure},
 {operation:'readNativeLiteralBoolean',arguments:[true],expected:true},
];
