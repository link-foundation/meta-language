#!/usr/bin/env python3
"""Rewrite `assert!(x.is_empty())` / `assert!(!x.is_empty())` sites flagged by
clippy::assert_is_empty (Rust 1.99) into the `assert_eq!` / `assert_ne!` forms
clippy suggests, using the receiver types resolved by hand.

Usage: python3 experiments/issue-195-rewrite-assert-is-empty.py rust
"""
import re
import sys
from pathlib import Path

STR, SLICE = '""', '[]'
def arr(t): return f'[] as [{t}; 0]'
AS_SLICE = 'as_slice'

# (relative path, line) -> empty value; AS_SLICE means `x.as_slice(), []`.
SITES = {
    'tests/integration/grammar_translate.rs': {28: arr('String')},
    'tests/integration/grammar_cli_pipeline.rs': {33: STR},
    'tests/integration/grammar_pipeline_rust.rs': {15: STR, 16: STR, 17: STR},
    'tests/integration/grammar_pipeline_gbnf.rs': {21: STR},
    'tests/integration/grammar_pipeline_js.rs': {14: STR},
    'tests/integration/cli.rs': {103: SLICE},
    'tests/unit/inference_eval.rs': {330: arr('String')},
    'tests/unit/issue_195_evidence.rs': {31: STR, 32: STR},
    'tests/unit/issue_195_generative.rs': {
        226: arr('String'), 227: arr('String'), 228: arr('String'),
        434: arr('&Value'), 540: arr('String')},
    'tests/unit/grammar_validate.rs': {199: arr('GrammarDiagnostic')},
    'tests/unit/query_algebra.rs': {117: SLICE},
    'tests/unit/natural_language_grammar.rs': {
        91: arr('&meta_language::Link'), 92: arr('&meta_language::Link')},
    'tests/unit/pdf_document.rs': {152: arr('BlockNode')},
    'tests/unit/access_mode.rs': {36: SLICE},
    'tests/unit/grammar_emit_rust.rs': {11: arr('String')},
    'tests/unit/inference_cfg.rs': {95: SLICE},
    'tests/unit/translation_emit_rocq.rs': {
        48: arr('meta_language::translation::emit_common::Assumption')},
    'tests/unit/grammar_emit.rs': {
        428: arr('String'), 447: arr('String'), 477: arr('String'), 667: arr('String')},
    'tests/unit/issue_195_conformance.rs': {
        202: arr('u8'), 227: arr('u8'), 250: arr('String'), 253: arr('String')},
    'tests/unit/issue_195_translation_pairs.rs': {523: STR, 527: STR},
    'tests/unit/grammar_emit_antlr_lark.rs': {780: arr('String'), 798: AS_SLICE},
    'tests/unit/docx_document.rs': {173: arr('BlockNode'), 182: arr('BlockNode')},
    'tests/unit/grammar_emit_tree_sitter.rs': {31: arr('String')},
    'tests/unit/grammar_emit_javascript.rs': {111: arr('String'), 153: arr('String')},
    'tests/unit/graphql_adapter.rs': {33: SLICE},
    'tests/unit/binary_format.rs': {26: arr('u8')},
    'tests/unit/issue_195_project_semantics.rs': {337: arr('Value')},
    'tests/unit/language_profile.rs': {89: SLICE, 119: SLICE},
}

def rewrite(text, line, empty):
    offsets = [0]
    for current in text.splitlines(keepends=True):
        offsets.append(offsets[-1] + len(current))
    start = text.index('assert!(', offsets[line - 1])
    assert start < offsets[line], f'no assert! on line {line}'
    depth, i = 0, start + len('assert!')
    while True:
        if text[i] == '(':
            depth += 1
        elif text[i] == ')':
            depth -= 1
            if depth == 0:
                break
        i += 1
    inner = text[start + len('assert!('):i].strip()
    negated = inner.startswith('!')
    receiver = inner[1:] if negated else inner
    match = re.fullmatch(r'(.*)\s*\.is_empty\(\)\s*,?', receiver, re.S)
    assert match, f'line {line}: unexpected condition {inner!r}'
    receiver = match.group(1).rstrip()
    if empty == AS_SLICE:
        receiver, empty = f'{receiver}.as_slice()', '[]'
    macro = 'assert_ne!' if negated else 'assert_eq!'
    return text[:start] + f'{macro}({receiver}, {empty})' + text[i + 1:]

def main(root):
    for relative, sites in SITES.items():
        path = Path(root) / relative
        text = path.read_text()
        # Rewrite bottom-up so earlier line numbers stay valid.
        for line in sorted(sites, reverse=True):
            text = rewrite(text, line, sites[line])
        path.write_text(text)
        print(f'{relative}: {len(sites)} sites')

if __name__ == '__main__':
    main(sys.argv[1] if len(sys.argv) > 1 else 'rust')
