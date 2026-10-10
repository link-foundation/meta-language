// Port the pinned HTML tag scanner to inspectable Links operations. The tag
// classifications and containment exclusions are read from its pinned header.
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { gunzipSync } from 'node:zlib';
import { fileURLToPath } from 'node:url';
import { resolve } from 'node:path';
import { parseTreeSitterPattern, renderTreeSitterPattern } from '../src/grammar-importers/tree-sitter-native.js';
import { parseGrammarLinks, renderGrammarLinks } from '../src/grammar-links.js';
import { delimitedScanner } from '../scripts/scanner-families.mjs';
const pattern = text => renderTreeSitterPattern(parseTreeSitterPattern(text));
const literal = text => `(literal ${[...Buffer.from(text)].map(byte => /[A-Za-z0-9_.-]/u.test(String.fromCharCode(byte)) ? String.fromCharCode(byte) : `%${byte.toString(16).toUpperCase().padStart(2,'0')}`).join('')})`;
const text = value => `(text ${value})`;
const equal = (a,b) => `(equal ${a} ${b})`;
const branch = (condition,yes,no=null) => `(if ${condition} (then ${yes})${no===null?'':` (else ${no})`})`;
const any = conditions => `(some ${conditions.join(' ')})`;
const member = (value,names) => any(names.map(name=>equal(value,text(name))));
const top='(top html_tags)';
const present='(greater (depth html_tags) (integer 0))';
const namePattern=pattern('[\\p{L}\\p{N}:-]+');
const whitespace=pattern('[ \\t\\n\\r\\v\\f]');
export function htmlScanner() {
 const header=gunzipSync(readFileSync(new URL('../../parity/grammars/sources/tree-sitter-html-0.23.2.tag.h.gz',import.meta.url))).toString('utf8');
 const sources=JSON.parse(readFileSync(new URL('../../parity/grammars/sources.json',import.meta.url),'utf8'));
 const expected=sources.sources.find(source=>source.id==='tree-sitter-html')?.scanner.ports.includes.find(source=>source.path==='src/tag.h')?.sha256;
 if(createHash('sha256').update(header).digest('hex')!==expected)throw new Error('HTML tag classifications differ from the pinned source header');
 const known=[...header.matchAll(/\{"([A-Z0-9_]+)",\s+[A-Z0-9_]+\s*\}/gu)].map(m=>m[1]).filter(name=>name!=='CUSTOM');
 const voidNames=header.match(/typedef enum \{([\s\S]*?)END_OF_VOID_TAGS/u)[1].match(/[A-Z][A-Z0-9_]*/gu);
 const paragraph=header.match(/TAG_TYPES_NOT_ALLOWED_IN_PARAGRAPHS\[\]\s*=\s*\{([^}]+)\}/u)[1].match(/[A-Z][A-Z0-9_]*/gu);
 const matchesTop=`(next (predicate ${namePattern} ${equal('(uppercase (matched))',top)}))`;
 const sameType=`(next (predicate ${namePattern} (some ${equal('(uppercase (matched))',top)} (all (not ${member('(uppercase (matched))',known)}) (not ${member(top,known)})))))`;
 const closes='(pop html_tags) (emit implicit_end_tag)';
 // Look deeper without changing the final stack except for the one implicit
 // closure. The temporary stack is emptied before every successful emit.
 const restore='(while (greater (depth html_pending) (integer 0)) (do (push html_tags (top html_pending)) (pop html_pending)))';
 const deeper=`${branch(matchesTop,'fail')} (set html_found (integer 0)) (while ${present} (do ${branch(sameType,'(set html_found (integer 1))')} (push html_pending ${top}) (pop html_tags))) ${restore} ${branch(equal('(variable html_found)','(integer 1)'),closes)} fail`;
 const notContained=any([
  `(all ${equal(top,text('LI'))} (next ${pattern('[Ll][Ii](?![\\p{L}\\p{N}:-])')}))`,
  `(all ${member(top,['DT','DD'])} (next ${pattern('[Dd][TtDd](?![\\p{L}\\p{N}:-])')}))`,
  `(all ${equal(top,text('P'))} (next ${pattern(`(?:${paragraph.map(name=>[...name].map(c=>`[${c}${c.toLowerCase()}]`).join('')).join('|')})(?![\\p{L}\\p{N}:-])`)}))`,
  `(all ${equal(top,text('COLGROUP'))} (not (next ${pattern('[Cc][Oo][Ll](?![\\p{L}\\p{N}:-])')})))`,
  `(all ${member(top,['RB','RT','RP'])} (next ${pattern('[Rr][BbTtPp](?![\\p{L}\\p{N}:-])')}))`,
  `(all ${equal(top,text('OPTGROUP'))} (next ${pattern('[Oo][Pp][Tt][Gg][Rr][Oo][Uu][Pp](?![\\p{L}\\p{N}:-])')}))`,
  `(all ${equal(top,text('TR'))} (next ${pattern('[Tt][Rr](?![\\p{L}\\p{N}:-])')}))`,
  `(all ${member(top,['TD','TH'])} (next ${pattern('[Tt][DdHhRr](?![\\p{L}\\p{N}:-])')}))`,
 ]);
 const implicit=`${branch(`(next ${literal('/')})`,`advance ${deeper}`)} ${branch(`(all ${present} ${member(top,voidNames)})`,closes)} ${branch(`(all ${present} (some ${notContained} (all ${member(top,['HTML','HEAD','BODY'])} (next (optional ${namePattern})) atEnd)))`,closes)} fail`;
 const readName=`(consume ${namePattern}) (set html_name (uppercase (matched)))`;
 // A forbidden child follows the pinned scanner's mandatory implicit close;
 // do not keep a speculative branch that nests it inside the old parent.
 const start=`${branch(`(all ${present} (some ${member(top,voidNames)} ${notContained}))`,'fail')} ${readName} (push html_tags (variable html_name)) ${branch(equal('(variable html_name)',text('SCRIPT')),'(emit script_start_tag_name)')} ${branch(equal('(variable html_name)',text('STYLE')),'(emit style_start_tag_name)')} (emit start_tag_name)`;
 const end=`${readName} ${branch(`(all ${present} ${equal('(variable html_name)',top)})`,'(pop html_tags) (emit end_tag_name)','(emit erroneous_end_tag_name)')}`;
 const rawLoop = delimiter => {
  const cases=[...delimiter].map((character,index)=>branch(equal('(variable html_index)',`(integer ${index})`),branch(`(next ${pattern(/[A-Z]/u.test(character)?`[${character}${character.toLowerCase()}]`:character.replace(/[\\^$.*+?()[\]{}|]/gu,'\\$&'))})`,`(set html_index (integer ${index+1})) ${index===delimiter.length-1?'(emit raw_text)':'advance'}`,'(set html_index (integer 0)) advance mark'))).join(' ');
  // Exactly one transition per input scalar, with the same partial-delimiter
  // behavior as the original scanner rather than a substring search.
  return `(set html_index (integer 0)) mark (while (not (some atEnd (next ${literal('\0')}))) (do (set html_previous (variable html_index)) ${cases.replaceAll('(variable html_index)', '(variable html_previous)')})) (emit raw_text)`;
 };
 const raw=`${branch(`(not ${present})`,'fail')} ${branch(equal(top,text('SCRIPT')),rawLoop('</SCRIPT'),rawLoop('</STYLE'))}`;
 const comment=delimitedScanner({name:'html_comment',token:'comment',opening:'<!--',closing:'-->',rejected:['\0']});
 const commentOps=comment.slice(comment.indexOf('(operations ')+12,-3);
 const operations=`${branch('(all (valid raw_text) (not (expected (ref start_tag_name))) (not (expected (ref end_tag_name))))',raw)} (while (next ${whitespace}) (do (skip ${whitespace}))) ${branch('(valid comment)',commentOps)} ${branch('(valid self_closing_tag_end)',`(consume ${literal('/>')}) ${branch(`(not ${present})`,'fail')} (pop html_tags) (emit self_closing_tag_end)`)} ${branch('(all (valid implicit_end_tag) (some atEnd (next (literal %3C))))',`mark ${branch(`(next ${literal('<')})`,'advance')} ${implicit}`)} ${branch('(some (valid start_tag_name) (valid script_start_tag_name) (valid style_start_tag_name))',start)} ${branch(`(all (valid erroneous_end_tag_name) (some (not ${present}) (not (expected (ref end_tag_name)))))`,'fail')} ${branch('(some (valid end_tag_name) (valid erroneous_end_tag_name))',end)} fail`;
 const listing = `(scanner html_tag_context (tokens start_tag_name script_start_tag_name style_start_tag_name end_tag_name erroneous_end_tag_name self_closing_tag_end implicit_end_tag raw_text comment) (operations ${operations}))\n`;
 return renderGrammarLinks(parseGrammarLinks(`(grammar (start unused))\n${listing}(rule unused normal (literal x))\n`)).split("\n").filter(line=>line.startsWith("(scanner ")).join("\n")+"\n";
}
if(process.argv[1]&&resolve(process.argv[1])===fileURLToPath(import.meta.url))process.stdout.write(htmlScanner());
