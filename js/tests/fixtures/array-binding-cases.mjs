// Independent original sources and outputs; no canonical source rewriting.
export const accepted = [
  {
    id: 'rest-source-once-elision',
    source: "function source(){console.log('source');return [2,3,5];}function selected(){const [head,,...tail]=source();return head+tail.length+tail[0];}console.log(selected());",
    expected: 'source\n8\n',
  },
  {
    id: 'zero-start-rest-and-empty-tail',
    source: 'function count(values){const [...copy]=values;const [head,...tail]=copy;return tail.length;}console.log(count([1]),count([1,2,3]));',
    expected: '0 2\n',
  },
  {
    id: 'left-to-right-defaults-read-earlier-binding',
    source: '/** @param {number[]} values @returns {number} */function selected(values){const [first=fallback(2),second=fallback(first+1),...tail]=values;return first+second+tail.length;}function fallback(value){console.log(value);return value;}console.log(selected([]));',
    expected: '2\n3\n5\n',
  },
  {
    id: 'present-zero-skips-fallback',
    source: 'function fallback(){console.log("unexpected");return 7;}function selected(values){const [head=fallback(),...tail]=values;return head+tail.length;}console.log(selected([0]));',
    expected: '0\n',
  },
  {
    id: 'present-false-and-empty-text-skip-default',
    source: 'function bool(values){const [head=true]=values;return head;}function text(values){const [head="fallback"]=values;return head;}console.log(bool([false]),text([""])==="");',
    expected: 'false true\n',
  },
  {
    id: 'rest-of-source-alias-preserves-values',
    source: 'function selected(values){const alias=values;const [head,...tail]=alias;return head+tail[0]+tail.length;}console.log(selected([2,3,5]));',
    expected: '7\n',
  },
  {
    id: 'for-of-per-iteration-rest',
    source: 'function total(rows){let sum=0;for(const [head,...tail] of rows)sum+=head+tail.length;return sum;}console.log(total([[2,3],[4]]));',
    expected: '7\n',
  },
  {
    id: 'present-nan-and-negative-zero-skip-default',
    source: 'function fallback(){console.log("unexpected");return 7;}function selected(values){const [head=fallback()]=values;return head;}console.log(Number.isNaN(selected([NaN])),1/selected([-0]));',
    expected: 'true -Infinity\n',
  },
];

export const refused = [
  ['rest-trailing-comma', 'function f(xs){const [...tail,]=xs;return tail;}'],
  ['rest-not-last', 'function f(xs){const [...tail,head]=xs;return tail;}'],
  ['rest-default', 'function f(xs){const [...tail=[]]=xs;return tail;}'],
  ['duplicate-name', 'function f(xs){const [head,...head]=xs;return head;}'],
  ['reserved-name', 'function f(xs){const [...default]=xs;return 1;}'],
  ['reserved-prefix', 'function f(xs){const [...ml_values]=xs;return 1;}'],
  ['self-default-tdz', 'function f(xs){const [head=head]=xs;return head;}'],
  ['later-default-tdz', 'function f(xs){const [first=second,second=2]=xs;return first;}'],
  ['rhs-tdz-shadow', 'function f(){const values=[1];{const [values]=values;return values;}}'],
  ['nullable-default-unproved', 'function f(){const [value=7]=[null];return value;}console.log(f());'],
  ['string-iterator-not-array', 'function f(){const [...tail]="😀x";return tail.length;}console.log(f());'],
  ['nested-rest-unproved', 'function f(rows){const [[head,...tail]]=rows;return tail.length;}'],
  ['structured-default-unproved', 'function f(){const [value=[7]]=[];return value.length;}console.log(f());'],
];
