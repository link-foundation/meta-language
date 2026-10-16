// A full cache must keep admitting work at the current source position.
// Retire an earlier completed prefix; active recursion and all memory charges
// remain intact. Small parses retain their original admission decisions.
const RULE_MEMO_PAGE_WIDTH=65536;

function readRuleMemoPosition(key) {
  if(typeof key!=='string')return null;
  const first=key.indexOf('|'),second=key.indexOf('|',first+1);
  if(first<1||second<first+2)return null;
  const identity=key.slice(0,first),rule=Number(identity);
  if(!Number.isSafeInteger(rule)||rule<0||String(rule)!==identity)return null;
  const text=key.slice(first+1,second),position=Number(text);
  return Number.isSafeInteger(position)&&position>=0&&String(position)===text?position:null;
}

export function retireCompletedRuleMemoPrefix(memo,position) {
  const current=Math.floor(position/RULE_MEMO_PAGE_WIDTH);
  if(current<1)return 0;
  let retired=0;
  for(const[key,entry]of memo){
    if(entry.evaluating||entry.involved||!Array.isArray(entry.results))continue;
    const at=readRuleMemoPosition(key);
    if(at===null)continue;
    // Creation order need not be source order. A current or later completed
    // entry ends this retirement pass; retain any older entries after it.
    if(Math.floor(at/RULE_MEMO_PAGE_WIDTH)>=current)break;
    memo.delete(key);retired++;
  }
  return retired;
}
