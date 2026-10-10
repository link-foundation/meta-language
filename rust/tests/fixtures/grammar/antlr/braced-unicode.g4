// Braced literal/range forms used by the pinned Python 3.14 grammar:
// antlr/grammars-v4 7df52be94698550d219d299d04105c6bafadd9c3
// python/python3_14/PythonLexer.g4 (MIT, Robert Einhorn).
grammar BracedUnicode;
entry: DIGIT EMOJI LIMIT NUL;
DIGIT: '\u{0030}' .. '\u{0039}';
EMOJI: '\u{1F600}';
LIMIT: '\u{10FFFF}';
NUL: '\u{0}';
