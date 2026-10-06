// Quote, escape, and concatenation edge cases compared across both runtimes.
export const cases = [
  ['bnf', '<a> ::= "x\\\\y"\n'],
  ['bnf', '<a> ::= "\\""\n'],
  ['bnf', "<a> ::= '\"'\n"],
  ['bnf', '<a> ::= "\'"\n'],
  ['bnf', '<a> ::= \'x"\' "\'y"\n'],
  ['bnf', '<a> ::= "x;y" ; comment\n'],
  ['ebnf', 'a = "x\\\\y" ;\n'],
  ['ebnf', 'a = \'"\' ;\n'],
  ['ebnf', 'a = "\\"" ;\n'],
  ['ebnf', 'a = "\\t\\/\\b" ;\n'],
  ['ebnf', 'a = "\\q" ;\n'],
  ['ebnf', 'a = "x" "y" ;\n'],
  ['ebnf', 'a = "say ""hi""" ;\n'],
];
