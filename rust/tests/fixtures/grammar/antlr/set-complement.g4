// `~` complements a set: a character class, a one-character literal, a range
// or an alternation of those, and matches one character outside it, as the
// grammars-v4 CSV string does. `~` over anything else stays a lookahead.
grammar Complement;
strings : STRING+ EOF ;
STRING : '"' ('""' | ~'"')* '"' ;
SET : ~('a' | 'b'..'c' | [d\p{Nd}]) ~'xy' ;
