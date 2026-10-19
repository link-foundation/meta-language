Fix native lexical election adapter compilation by serializing borrowed token
metadata strings and passing owned token leaves to the existing tree builder.
Lexer decisions, metadata fields and concrete tree contents remain unchanged.
