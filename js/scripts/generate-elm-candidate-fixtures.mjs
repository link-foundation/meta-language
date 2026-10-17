#!/usr/bin/env node
import { structuredDataToLino } from '../src/structured-data-links.js';
import { readProjectData, resolveProjectDataPath } from './project-data.mjs';
// Only the authored focused sources and two explicitly named pinned cases run
// here. Whole upstream corpus execution remains a CI promotion requirement.
import { readFileSync, writeFileSync } from 'node:fs';
import { elmCandidateSource, elmLayoutCandidate } from './elm-layout-candidate.mjs';
import { corpusFileCases } from './import-native-grammars.mjs';
import { oracleSnapshot } from './native-grammar-rows.mjs';

const root = new URL('../../', import.meta.url);
export const ELM_CANDIDATE_FIXTURE = 'parity/fixtures/native-grammars/elm-candidate.json';
export const ELM_CANDIDATE_GRAMMAR = 'parity/grammars/candidates/elm/native.lino';
export const ELM_CANDIDATE_SCANNER = 'parity/grammars/candidates/elm/scanner.lino';

/** Independent oracle observations for the candidate's focused authoring corpus. */
export function elmCandidateFixtures(candidate = elmLayoutCandidate()) {
  const authored = [
    ['simple declaration', 'a = 1\n'],
    ['inline section close', 'a = let b = 1 in b\n'],
    ['several declarations in one layout section', 'a =\n    let\n        b = 1\n        c = 2\n    in\n    b + c\n'],
    ['case branches and final section close', 'a = case x of\n    Just y -> y\n    Nothing -> 0\n'],
    ['nested inline and multiline sections', 'a =\n    let\n        b = let c = 1 in c\n    in\n    b\n'],
    ['line comment between declarations', 'a =\n    let\n        b = 1\n        -- retained comment\n        c = 2\n    in\n    b + c\n'],
    ['nested block comment content', 'a = {- outer {- inner -} outer -} 1\n'],
    ['multiline content retains comment markers', 'a = """-- text\n{- body -}\nλ \\" text \\"\n"""\n'],
    ['multiline escapes and Unicode', 'a = """a\\nλ"""\n'],
    ['minus before Unicode identifier', 'a = -é\n'],
    ['closing parenthesis closes a section', 'a = (case x of Just y -> y)\n'],
    ['comma closes a record field section', 'a = { first = case x of Just y -> y, second = 2 }\n'],
    ['shader content delimiter', 'shader = [glsl| void main() { gl_FragColor = vec4(1.0); } |]\n'],
  ].map(([name, source]) => ({ name, source }));
  const upstream = [
    ['test/corpus/let.txt', 'let with two variables separated by unindented comment'],
    ['test/corpus/case.txt', 'Nested case are all closed?'],
  ].map(([file, title]) => {
    const selected = corpusFileCases(elmCandidateSource(file)).find((entry) => entry.title === title);
    if (!selected) throw new Error(`the pinned Elm corpus lost ${file}: ${title}`);
    return { name: title, source: selected.source, upstream: { file, title } };
  });
  const matches = [...authored, ...upstream].map((entry) => {
    const observation = oracleSnapshot(entry.source, 'Elm');
    if (observation.recovers) throw new Error(`the independent Elm oracle rejects focused source ${entry.name}`);
    return { ...entry, rows: observation.rows };
  });
  const rejections = [
    ['missing section body', 'a = let b = 1 in\n'],
    ['unclosed multiline string', 'a = """unterminated\n'],
    ['unclosed block comment', 'a = {- unterminated\n'],
  ].map(([name, source]) => {
    if (!oracleSnapshot(source, 'Elm').recovers) throw new Error(`the independent Elm oracle accepts invalid source ${name}`);
    return { name, source };
  });
  const lock = readProjectData(new URL('js/src/vendor/grammars/grammar-lock.json', root)).grammars.elm;
  if (lock.version !== candidate.source.crate.version) throw new Error('Elm candidate and independent oracle versions differ');
  return {
    language: 'Elm', grammar: ELM_CANDIDATE_GRAMMAR,
    sourceRegister: 'parity/grammars/candidates/elm/source.json',
    revision: candidate.source.revision,
    oracle: { crate: lock.crate, version: lock.version, parserSha256: lock.parserSha256, wasmSha256: lock.wasmSha256 },
    ...candidate.projection, matches, divergences: [], rejections,
  };
}

export function elmCandidateOutputs() {
  const candidate = elmLayoutCandidate();
  return [
    [ELM_CANDIDATE_FIXTURE, structuredDataToLino(elmCandidateFixtures(candidate))],
    [ELM_CANDIDATE_GRAMMAR, candidate.text],
    [ELM_CANDIDATE_SCANNER, candidate.scanner],
  ];
}

if (process.argv[1] && new URL(process.argv[1], 'file:').href === import.meta.url) {
  for (const [file, text] of elmCandidateOutputs()) {
    const path = new URL(file, root);
    if (process.argv.includes('--check')) {
      if (readFileSync(resolveProjectDataPath(path), 'utf8') !== text) throw new Error(`${file}: Elm candidate generation drift`);
    } else writeFileSync(resolveProjectDataPath(path), text);
  }
}
