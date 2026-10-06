// The grammars the lowering section of parity/fixtures/grammar-importers.json
// records, in the native links form both runtimes read, with samples written
// by hand. `features` holds every construct a notation may lack, each in an
// encoding that accepts the same texts, plus rule kinds and documentation;
// `lookahead` is a PEG grammar whose ordered choice, lookahead and arbitrary
// character most notations only approximate.
export const LOWERING_GRAMMARS = [
  {
    id: 'features',
    links: [
      '(grammar (format peg) (start message))',
      '(rule message normal (seq (ref greeting) (optional (ref sep)) (ref name) (repeat 1 3 (ref digit)) (repeat0 (literal %21)) (optional (ref shout))) (doc the%20start%20rule))',
      '(rule greeting normal (choice unordered (literal hello) (literal hi)))',
      '(rule sep token (class plain (char %20) (char %2C)))',
      '(rule name normal (capture labeled who (repeat1 (ref letter))))',
      '(rule letter normal (range a z))',
      '(rule digit atomic (class plain (range 0 9)))',
      '(rule shout normal (seq (literal %3F) (literalInsensitive ok) (repeat 2 unbounded (literal %3F))))',
      '',
    ].join('\n'),
    accepts: ['hello,abc1', 'hi xyz123!!', 'helloq7?OK??', 'hiab12?oK???'],
    rejects: ['hello', 'hi abc1234', 'hey abc1', 'hiabc1?ok?', 'hi;abc1'],
  },
  {
    id: 'lookahead',
    links: [
      '(grammar (format peg) (start list))',
      '(rule list normal (seq (ref item) (repeat0 (seq (literal %2C) (ref item)))))',
      '(rule item normal (choice ordered (literal ab) (literal a) (ref other)))',
      '(rule other normal (seq (not (literal x)) any))',
      '',
    ].join('\n'),
    accepts: ['ab', 'a,ab', 'q'],
    rejects: ['x', 'ab,'],
  },
];
