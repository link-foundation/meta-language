// The files js/scripts/run-formal-ai-workloads.mjs writes into the clean
// JavaScript consumer that installs the meta-language npm artifact for the
// link-assistant/formal-ai workloads (requirement
// I195-DOWNSTREAM-FORMAL-AI-WORKLOADS). formal-ai's npm package declares no
// meta-language dependency, so the consumer runs formal-ai's own data files:
// - formal-ai-helpers.mjs: the `formalAiHelpers` consumer code of
//   js/scripts/issue-195-formal-ai-workload-probes.mjs over the installed package;
// - formal-ai-workloads.test.mjs: node:test workloads, one or more per formal-ai
//   data file, each named `<data file> › <what it runs>` so the runner files
//   every outcome under the data file it ran;
// - issue-195-meta-language-probe.mjs: the probe writing the outputs both
//   runtimes compare and the sharedConceptsReused and distinctionsPreserved checks.
import { formalAiHelpers } from './issue-195-formal-ai-workload-probes.mjs';

/** The environment variable naming the inputs file the workload tests read. */
export const JAVASCRIPT_INPUTS_VARIABLE = 'ISSUE_195_FORMAL_AI_INPUTS';

/** The separator between the data file and the workload in a JavaScript test name. */
export const TEST_NAME_SEPARATOR = ' › ';

/** The consumer module binding the helpers to the installed package. */
export function consumerHelpersModule() {
  return `// Issue 195: formal-ai's data through the installed meta-language package.\n` +
    `import * as api from 'meta-language';\n\nexport const helpers = (${formalAiHelpers.toString()})(api);\n`;
}

/** The node:test workloads over formal-ai's data files. */
export const JAVASCRIPT_WORKLOAD_TESTS = String.raw`// Issue 195 workloads: link-assistant/formal-ai's data files through the installed meta-language
// package. Every test name starts with the formal-ai data file it runs.
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';

import { helpers } from './formal-ai-helpers.mjs';

const inputs = JSON.parse(readFileSync(process.env.ISSUE_195_FORMAL_AI_INPUTS, 'utf8'));
const files = inputs.dataFiles;
const outputs = new Map();
const documentOutput = (document) => {
  if (!outputs.has(document.name)) outputs.set(document.name, helpers.documentOutput(document));
  return outputs.get(document.name);
};
const none = [];

for (const document of inputs.documents.filter(({ language }) => language === 'lino')) {
  test(document.source + ' › parses as LiNo, round-trips and fully matches', () => {
    const output = documentOutput(document);
    assert.equal(output.roundTrip, true);
    assert.equal(output.clean, true);
    assert.ok(output.namedNodes.length > 0, 'the parse has named syntax links');
  });
  test(document.source + ' › keeps its syntax links through toLino and fromLino', () => {
    const output = documentOutput(document);
    assert.deepEqual(output.reload, { namedNodes: output.namedNodes.length, sameNamedNodes: true, roundTrip: true });
  });
}

for (const program of inputs.documents.filter(({ source, language }) => source === files.helloWorldPrograms && language !== 'lino')) {
  test(files.helloWorldPrograms + ' › ' + program.name + ' parses as ' + program.language + ', round-trips and fully matches', () => {
    const output = documentOutput(program);
    assert.equal(output.roundTrip, true);
    assert.equal(output.clean, true);
    assert.ok(output.namedNodes.length > 0, 'the parse has named syntax links');
  });
}

test(files.programGrammars + ' › every declared grammar label has a parsed hello-world program', () => {
  assert.ok(inputs.labels.length > 0, 'the catalog declares labels');
  const parsed = new Set(inputs.documents.filter(({ source, language }) => source === files.helloWorldPrograms && language !== 'lino')
    .filter((program) => documentOutput(program).namedNodes.length > 0).map(({ language }) => language));
  assert.deepEqual(inputs.labels.filter((label) => !parsed.has(label)), none);
});

test(files.programGrammars + ' › a quoted literal is replaced through its links', () => {
  const cases = inputs.linkEdits.filter(({ language }) => language === 'lino');
  assert.ok(cases.length > 0);
  assert.deepEqual(helpers.linkEditFailures(cases, inputs.linkEditRulesText), none);
});

test(files.projectionRules + ' › loads as a translation rule set with every seed rule', () => {
  const output = helpers.ruleSetOutput(inputs.ruleSetText);
  assert.equal(output.error, undefined, output.error);
  assert.deepEqual(output.rules, inputs.rules.map(({ name }) => name));
  assert.deepEqual(output.reloadRules, output.rules);
});

test(files.projectionRules + ' › loads as a links network with every rule, refusal and no-form row', () => {
  const output = helpers.seedNetworkOutput(inputs.ruleSetText);
  assert.deepEqual(output, {
    translationRules: inputs.rules.length,
    refusalRows: inputs.classification.refused.length,
    noformRows: inputs.classification.noform.length,
  });
});

test(files.projectionRules + ' › every match s-expression parses as written', () => {
  const queries = helpers.queryOutputs(inputs.rules, []);
  const rejected = Object.entries(queries).filter(([, query]) => query.asWritten !== null);
  assert.deepEqual(rejected.slice(0, 5).map(([name, query]) => name + ': ' + query.asWritten), none,
    rejected.length + ' of ' + inputs.rules.length + ' match s-expressions are rejected');
});

test(files.projectionRules + ' › classifies the ruled, refused and no-form kinds', () => {
  assert.deepEqual(helpers.seedClassificationFailures(inputs), none);
});

test(files.projectionRules + " › projects formal-ai's issue 1138 samples", () => {
  assert.deepEqual(helpers.projectionFailures(inputs.projections, inputs.projectionLabels, inputs.ruleSetText), none);
});

test(files.linkEditRules + ' › declares the three link-edit shapes', () => {
  assert.deepEqual(helpers.ruleShapes(inputs.linkEditRulesText).map(({ rule }) => rule),
    ['insert_member', 'replace_literal', 'rename_identifier']);
});

test(files.linkEditRules + " › applies formal-ai's issue 1085 link edits", () => {
  const cases = inputs.linkEdits.filter(({ language }) => language === 'rust');
  assert.ok(cases.length > 0);
  assert.deepEqual(helpers.linkEditFailures(cases, inputs.linkEditRulesText), none);
});
`;

/** The JavaScript probe; argument 2 is the observation file it writes, argument 3 the inputs file. */
export const JAVASCRIPT_PROBE = String.raw`// Issue 195 consumer probe for formal-ai's data, run from the clean consumer against the installed
// meta-language artifact. It writes the outputs the runner compares with formal-ai's Rust functions.
import assert from 'node:assert/strict';
import { readFileSync, writeFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';

import { helpers } from './formal-ai-helpers.mjs';

const inputs = JSON.parse(readFileSync(process.argv[3], 'utf8'));
const packageUrl = import.meta.resolve('meta-language');
const packageRoot = new URL('./node_modules/meta-language/', import.meta.url).href;

function check(name, body) {
  try {
    const detail = body();
    return { name, holds: true, detail: detail ?? null };
  } catch (error) {
    return { name, holds: false, detail: String(error?.message ?? error) };
  }
}

function assertion(checks) {
  return { holds: checks.every(({ holds }) => holds), checks };
}

const helperUrl = new URL('./formal-ai-helpers.mjs', import.meta.url);
const sharedConceptsReused = assertion([
  check('the installed package resolves from the consumer', () => {
    assert.ok(packageUrl.startsWith(packageRoot), fileURLToPath(packageUrl));
    const manifest = JSON.parse(readFileSync(new URL('package.json', packageRoot), 'utf8'));
    assert.match(readFileSync(helperUrl, 'utf8'), /from 'meta-language'/u, 'the helpers import the package itself');
    return { packageUrl, version: manifest.version };
  }),
]);

const outputs = helpers.outputs(inputs);
const distinctionsPreserved = assertion(helpers.distinctionChecks(inputs));

writeFileSync(process.argv[2], JSON.stringify({ packageUrl, outputs, sharedConceptsReused, distinctionsPreserved }, null, 2) + '\n');
`;
