Generated contextual literal scanners share equivalent body decisions and test
opening spellings before parser-context queries, reducing scanner work within
the existing parse budget. Lexical following conditions retain fallback
policies, and profiles can describe escaped line continuations while retaining
their exact source text. Both runtimes consume these generated Links operations.
Profiles can group competing lexical tokens into one ordered decision using
the shared parser-validity bits, preserving whitespace-sensitive operator and
command-argument choices.
Delimiter policies can declare an empty closing delimiter for a literal that
ends at the input boundary, with guarded opening delimiters. This preserves
Ruby's percent literals opened by whitespace without losing percent operators.
