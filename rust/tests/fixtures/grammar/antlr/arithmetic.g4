grammar Arithmetic;

expr
    : term (('+' | '-') term)*
    ;

term
    : factor (('*' | '/') factor)*
    ;

factor
    : INT
    | ID
    | '(' expr ')'
    ;

INT
    : [0-9]+
    ;

ID
    : [a-zA-Z_] [a-zA-Z_0-9]*
    ;

WS
    : [ \t\r\n]+ -> skip
    ;
