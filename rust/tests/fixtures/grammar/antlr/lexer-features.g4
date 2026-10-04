// The lexer features the grammars-v4 grammars use: a comment before the colon,
// EOF, `\u` escapes and `\p{...}` properties in sets, and the `skip` and
// `channel(...)` commands, which make a token trivia.
grammar Doc;
doc
  // read up to the end
  : ITEM+ EOF ;
ITEM : [\u0041-\u{5A}\p{Nd}\p{Greek}] ;
WS : [ ]+ -> skip ;
COMMENT : '#' ~[\n]* -> channel(HIDDEN) ;
