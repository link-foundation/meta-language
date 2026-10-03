// Oracle mapping gate of issue #195: every language of the four-language
// conformance fixtures (parity/fixtures/issue-195-conformance/manifest.json)
// is compared with trees that the pinned native tree-sitter CLI printed using
// the pinned grammar of that same language, from inputs whose digests,
// revisions and licenses are recorded. Swapping an oracle, mapping a language
// to another grammar, dropping its real projects or replacing the oracle tool
// is reported, so the comparison cannot silently be weakened.
import { createHash } from 'node:crypto';

import { languageSupport } from '../src/index.js';
import { grammarFile } from './grammar-files.mjs';

const COMMIT = /^[0-9a-f]{40}$/u;
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');

/**
 * Problems of the conformance manifest against the grammar lock; `read` reads
 * a path relative to the fixture directory and `readRepository` one relative
 * to the repository root, both as bytes.
 */
export function conformanceOracleProblems(manifest, { lock, read, readRepository }) {
  const problems = [];
  const digest = (path, reader = read) => {
    try {
      return sha256(reader(path));
    } catch {
      return null;
    }
  };
  const nonempty = (path, reader) => {
    try {
      return reader(path).length > 0;
    } catch {
      return false;
    }
  };
  if (manifest.oracle?.tool !== lock.treeSitterCli) {
    problems.push(`the oracle tool ${manifest.oracle?.tool} is not the pinned ${lock.treeSitterCli}`);
  }
  for (const [file, expected] of Object.entries(manifest.inputs ?? {})) {
    if (digest(file) !== expected) problems.push(`input ${file} does not match its digest`);
  }
  for (const [language, details] of Object.entries(manifest.languages ?? {})) {
    const grammar = details.grammar ?? {};
    const pinned = lock.grammars[grammar.id];
    if (languageSupport(grammar.id)?.name !== language) problems.push(`${language} is mapped to the grammar ${grammar.id}`);
    if (!pinned || grammar.parserSha256 !== pinned.parserSha256) problems.push(`${language} grammar parser is not the pinned one`);
    if (pinned && grammar.license !== grammarFile(pinned, pinned.license)) problems.push(`${language} grammar license is not the pinned one`);
    if (!COMMIT.test(grammar.revision ?? '')) problems.push(`${language} grammar revision is not a commit`);
    if (!nonempty(grammar.license, readRepository)) problems.push(`${language} grammar license is missing`);
    let oracle = null;
    try {
      oracle = JSON.parse(read(details.oracle));
    } catch {
      problems.push(`${language} oracle ${details.oracle} is unreadable`);
    }
    if (oracle && oracle.language !== language) problems.push(`${language} oracle ${details.oracle} is the ${oracle.language} oracle`);
    if (digest(details.oracle) !== details.oracleSha256) problems.push(`${language} oracle does not match its digest`);
    const corpus = details.corpus ?? {};
    if (!corpus.url?.includes(grammar.revision)) problems.push(`${language} corpus is not taken from the pinned grammar revision`);
    for (const [file, expected] of Object.entries(corpus.files ?? {})) {
      if (digest(`${corpus.directory}/${file}`) !== expected) problems.push(`${language} corpus ${file} does not match its digest`);
    }
    if (!(details.projects?.length > 0)) problems.push(`${language} has no real projects`);
    for (const project of details.projects ?? []) {
      if (!COMMIT.test(project.commit ?? '')) problems.push(`${language} project ${project.file} is not pinned to a commit`);
      if (!project.url?.includes(project.commit)) problems.push(`${language} project ${project.url} does not name its commit`);
      if (digest(project.file) !== project.sha256) problems.push(`${language} project ${project.file} does not match its digest`);
      if (!nonempty(project.licenseFile, read)) problems.push(`${language} project license ${project.licenseFile} is missing`);
    }
  }
  return problems;
}
