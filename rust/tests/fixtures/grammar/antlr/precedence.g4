// Left-recursive alternatives climb by precedence: earlier alternatives bind
// tighter, `<assoc=right>` makes one right-associative, as the grammars-v4
// JavaScript and Java grammars write them. Sets naming UTF-16 surrogates keep
// only the scalar values, and a channel may be a number, as in grammars-v4 Lua.
grammar Precedence;
expr
    : <assoc=right> expr '^' expr
    | expr '*' expr
    | expr '+' expr
    | ID
    ;
ID : ~[\u0000-@\uD800-\uDBFF]+ ;
LONE : [\uDC00] ;
NL : [\n] -> channel(2) ;
