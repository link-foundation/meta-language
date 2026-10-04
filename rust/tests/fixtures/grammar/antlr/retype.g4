// `-> type(NAME)` gives a rule's tokens the type NAME, so the rule matches
// where NAME does: the grammars-v4 R lexer ends a comment as a line break. A
// type only `tokens {...}` declares gets the rules that retype to it.
grammar Lines;
tokens { WORD }
lines : (WORD | NL)* EOF ;
COMMENT : '#' ~[\r\n]* '\r'? '\n' -> type(NL) ;
NL : '\r'? '\n' ;
NAME : [a-z]+ -> type(WORD) ;
NUMBER : [0-9]+ -> type(WORD) ;
HIDDEN_NOTE : '%' ~[\r\n]* -> type(NL), channel(HIDDEN) ;
WS : [ \t]+ -> skip ;
