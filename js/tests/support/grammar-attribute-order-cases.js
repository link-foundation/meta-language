// Authored expectations; normal producers encode these values as Links data.
export function grammarAttributeOrderCases() {
  return [
    { id: 'insertion-and-overwrite', keys: ['z', 'b', 'a', 'b'], order: ['z', 'b', 'a'],
      rendered: '(document {a=2,b=3,z=0} "x")' },
    { id: 'integer-boundaries', keys: ['b', '10', '2', '4294967294', '0', '01', '4294967295', 'a', '10'],
      order: ['0', '2', '10', '4294967294', 'b', '01', '4294967295', 'a'],
      rendered: '(document {"0"=4,"01"=5,"10"=8,"2"=2,"4294967294"=3,"4294967295"=6,a=7,b=0} "x")' },
    { id: 'unicode-and-prototype-names', keys: ['\uE000', '𐀀', 'A', '🦀', 'constructor', '__proto__'],
      order: ['\uE000', '𐀀', 'A', '🦀', 'constructor', '__proto__'],
      rendered: '(document {A=2,__proto__=5,constructor=4,"𐀀"=1,"🦀"=3,"\uE000"=0} "x")' },
  ];
}

export function grammarAttributeIntegerNameCases() {
  return [['0',0],['1',1],['10',10],['4294967294',4294967294],['4294967295',-1],
    ['9999999999',-1],['000',-1],['01',-1],['',-1],['-0',-1],['+1',-1],['1.0',-1],
    ['1e0',-1],[' 1',-1],['١',-1],['１',-1],['𐀀',-1],['10000000000',-1]]
    .map(([name,index])=>({name,index}));
}
