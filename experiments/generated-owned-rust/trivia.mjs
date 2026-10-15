// Interpret only the actual Rust trivia intervals between complete source tokens.
import {acceptRustSourceWhitespace,acceptRustSourceByteOrderMark} from '../../js/src/translation/rust-source-whitespace-rules.js';
import {acceptGeneratedOwnedRustOrdinaryComment} from './rules.mjs';

export function readGeneratedOwnedRustTrivia(units,start,policy){
 const fail=reason=>{throw new TypeError('Generated whole owned Rust trivia: '+reason);};
 const starts=(at,text)=>[...text].every((letter,index)=>units[at+index]===letter.charCodeAt(0));
 let at=0;
 while(at<units.length){
  if(acceptRustSourceWhitespace(units[at])||acceptRustSourceByteOrderMark(units[at],start+at===0)){at++;continue;}
  const line=starts(at,policy.trivia.lineComment),block=starts(at,policy.trivia.blockCommentOpen);
  if(!line&&!block)fail('active or unsupported source text is not whitespace or an ordinary comment');
  if(!acceptGeneratedOwnedRustOrdinaryComment(line,units[at+2]??-1,units[at+3]??-1))fail('a doc comment requires its own actual attribute interpretation');
  at+=2;
  if(line){while(at<units.length&&units[at]!==10)at++;continue;}
  let depth=1;
  while(at<units.length&&depth){
   if(starts(at,policy.trivia.blockCommentOpen)){depth++;at+=2;}
   else if(starts(at,policy.trivia.blockCommentClose)){depth--;at+=2;}
   else at++;
  }
  if(depth)fail('the ordinary nested block comment is unterminated');
 }
}
