// The commented, irregularly formatted sources of the reverse conversion
// fixture, one per lossless format. Run directly, it imports each source and
// prints its native listing and same-format emission.
import { grammarImporter, grammarEmitter, renderNativeGrammar } from '../js/src/index.js';

export const SOURCES = {
  abnf: `; a greeting: a word, one space and a number
message   =  word SP number   ; the start rule

word = 1*ALPHA
; digits only
number=1*DIGIT
`,
  antlr: `// a greeting grammar
grammar Message;

/* the start rule */
message : word ' ' number ;

word   : LETTER+ ;
number : DIGIT+ ;   // one or more digits

LETTER : [A-Za-z] ;
DIGIT  : [0-9] ;
`,
  bnf: `<message> ::= <word> " " <number>

<word>   ::=   <letter> <word>|<letter>
<number> ::= <digit> <number> | <digit>
<letter> ::= "M" | "e" | "t" | "a" | "A"
<digit>  ::= "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"
`,
  ebnf: `(* a greeting: a word, one space and a number *)
message = word , " " , number ;

(* letters and digits *)
word   = letter , { letter } ;
number = digit, {digit};
letter = "M" | "e" | "t" | "a" | "A" ;
digit  = "0" | "1" | "2" | "3" | "4"
       | "5" | "6" | "7" | "8" | "9" ;
`,
  gbnf: `# a greeting: a word, one space and a number
root   ::= word " " number

word   ::= [A-Za-z]+   # letters
number ::= [0-9]+
`,
  lark: `// a greeting: a word, one space and a number
start: word " " number

word: LETTER+
number : DIGIT+   // digits

LETTER: /[A-Za-z]/
DIGIT: /[0-9]/
`,
  pest: `// a greeting: a word, one space and a number
message = { word ~ " " ~ number }

word   = @{ letter+ }   // letters
number = @{ digit+ }
letter = _{ 'A'..'Z' | 'a'..'z' }
digit  = _{ '0'..'9' }
`,
  'tree-sitter-json': `{
  "name": "message",
  "rules": {
    "message": {"type": "SEQ", "members": [
      {"type": "SYMBOL", "name": "word"},
      {"type": "STRING", "value": " "},
      {"type": "SYMBOL", "name": "number"}
    ]},
    "word": {"type": "REPEAT1", "content": {"type": "PATTERN", "value": "[A-Za-z]"}},
    "number": {"type": "TOKEN", "content": {"type": "REPEAT1", "content": {"type": "PATTERN", "value": "[0-9]"}}}
  },
  "extras": []
}
`,
};

if (import.meta.url === `file://${process.argv[1]}`) {
  for (const [format, source] of Object.entries(SOURCES)) {
    console.log(`=== ${format}`);
    try {
      const grammar = grammarImporter(format)(source);
      console.log(renderNativeGrammar(grammar));
      const emitted = grammarEmitter(format)(grammar);
      console.log('--- emitted, lossy:', emitted.report.lossy);
      console.log(emitted.source);
    } catch (error) {
      console.log('ERROR', error.message);
    }
  }
}
