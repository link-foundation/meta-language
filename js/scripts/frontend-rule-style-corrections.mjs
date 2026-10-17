// Complete owning-function styles for the CI117015296223 frontend diagnostics.
// Source decisions stay in frontend-rules.js; these actions select Rust loans
// and equivalent branch, return and missing-value spellings only.
import assert from 'node:assert/strict';
import {readPortableSourceParameterStyles} from './portable-source-library-style.mjs';
import {simplifyPortableProductLibraryStyles,simplifyRustOptionIdentityMatches} from './reviewed-rust-branch-styles.mjs';
import {readSourceTextRustBlock,simplifySourceTextRustParentheses} from './source-text-byte-library-style.mjs';

export const frontendRuleNativeStyleId='reviewed-frontend-native-styles-117015296223';
export const frontendTypeQueryNativeStyleId='reviewed-frontend-type-query-and-optional-tuple-styles';

/** Derive exact, bounded contexts from the actual checked source library. */
export function readFrontendRuleStyleCorrections(source) {
  const names=['read_unit_alternative_expression_form','read_lexical_winner_index'];
  const continuations=[...source.matchAll(/^pub fn (ml_read_lexical_winner_index_loop[0-9]+)\(/gmu)].map(match=>match[1]);
  assert.equal(continuations.length,1,'the checked lexical election has its actual single loop owner');
  names.push(continuations[0]);
  const blocks=names.map(name=>readSourceTextRustBlock(source,'fn',name));
  const loans=readPortableSourceParameterStyles(blocks.join('\n\n')).source;
  return names.flatMap((name,index)=>{
    const borrowed=readSourceTextRustBlock(loans,'fn',name);
    const to=simplifyPortableProductLibraryStyles(simplifyRustOptionIdentityMatches(borrowed));
    assert.doesNotMatch(to,/#\[(?:allow|expect)\(/u,'a reviewed frontend style needs no lint suppression');
    return to===blocks[index]?[]:[{op:'replace',field:'source',from:blocks[index],to}];
  });
}

/** Derive current type-query and optional-tuple styles without rewriting older records. */
export function readFrontendTypeQueryStyleCorrections(source) {
  const names=['read_type_query_result','accept_java_script_optional_tuple_context'];
  const blocks=names.map(name=>readSourceTextRustBlock(source,'fn',name));
  const loans=readPortableSourceParameterStyles(blocks.join('\n\n')).source;
  return names.flatMap((name,index)=>{
    const borrowed=readSourceTextRustBlock(loans,'fn',name);
    const to=simplifySourceTextRustParentheses(simplifyPortableProductLibraryStyles(borrowed));
    assert.doesNotMatch(to,/#\[(?:allow|expect)\(/u,'a current frontend style needs no lint suppression');
    return to===blocks[index]?[]:[{op:'replace',field:'source',from:blocks[index],to}];
  });
}
