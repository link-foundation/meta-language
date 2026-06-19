grammar Covering;

options { tokenVocab = Base; }
tokens { INDENT, DEDENT }
import Common;

entry
    : name=ID values+=item*? literalRange item
    ;

item
    : .
    | ~[;]
    | ~'x'
    ;

literalRange
    : 'a'..'z'
    ;

TOKEN
    : [a-z0-9_]
    ;

fragment DIGIT
    : [0-9]
    ;

COMMENT
    : '//' ~[\r\n]* -> channel(HIDDEN)
    ;

ACTIONED
    : {isStart()}? 'a' {emit();} -> type(ID)
    ;
