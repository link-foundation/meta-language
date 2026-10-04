// `options { caseInsensitive = true; }` makes the lexer match literals, ranges
// and sets in either case, and a rule's own options may turn it off, as the
// grammars-v4 PHP lexer does. `returns`, `throws` and `locals` declare
// target-language values and join the rule's doc, as in grammars-v4 Swift.
grammar Keywords;
options { superClass = Base; caseInsensitive = true; }
items : (ECHO | WORD | TAGGED)+ EOF ;
ECHO : 'echo' ;
WORD : 'a'..'c' [x-z_]+ ;
fragment NAME options { caseInsensitive = false; } : [a-z]+ ;
TAGGED returns [int count] locals [int indexBefore = -1] : '<' NAME '>' ;
WS : [ \t]+ -> skip ;
