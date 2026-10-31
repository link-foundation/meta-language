// Static source-authoring signals, not a lexer or a semantic coverage oracle.
/** Return [count, first UTF16 offset, first LF-delimited line] for each literal signal.
 * Strings/comments are intentionally not removed: candidates require review.
 * @param {string} source @param {string[]} needles @returns {number[]} */
export function readSourceConstructSignals(source,needles){
 const result=[];
 for(const needle of needles){
  let count=0,first=-1,line=-1;
  if(needle.length>0){let at=source.indexOf(needle);
   if(at>=0){first=at;line=1;for(let before=0;before<at;before+=1)if(source.charAt(before)==='\n')line+=1;}
   while(at>=0){count+=1;at=source.indexOf(needle,at+needle.length);}
  }
  result.push(count,first,line);
 }
 return result;
}
