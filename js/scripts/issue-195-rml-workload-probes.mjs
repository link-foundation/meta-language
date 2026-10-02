// The consumer probes js/scripts/run-rml-pr184-workloads.mjs places inside a
// checkout of relative-meta-logic pull request 184, so they import meta-language
// exactly as RML does: the JavaScript probe runs from RML's js/ directory and
// resolves the installed npm artifact, and the Rust probe is an integration test
// of RML's rust/ crate, compiled against the patched, unpacked crate in the same
// cargo invocation as the RML workload tests.
//
// Each probe observes three assertions of I195-DOWNSTREAM-RML-WORKLOADS:
// - sharedConceptsReused: RML's bridge hands out the package's own types (the
//   JavaScript re-exports are the identical classes; the Rust network binds to
//   meta_language::LinkNetwork and equals the crate's own parse), and RML's
//   truth and substitution planning compute what the package computes.
// - distinctionsPreserved: RML source round-trips byte for byte, including an
//   unmatched parenthesis in a quote and a comment, the network keeps the RML
//   language identity apart from JavaScript's, and RML's JavaScript identifier
//   rewrite leaves the same spelling inside a string and a comment alone.
// - foundationAuthorityStaysInRml: RML evaluates both its source and the
//   meta-language reconstruction with its own evaluator, whose results the
//   bridge reports unchanged; in JavaScript a synchronous resolve hook traces
//   every module edge, and the import closure of RML's evaluator
//   (src/rml-links.mjs) must contain no meta-language module while no package
//   module may load an RML module. The trace is falsifiable: one
//   `import 'meta-language'` added anywhere under the evaluator puts package
//   modules in its closure. The Rust side adds the runner's dependency-graph
//   and source checks, since a compiled crate has no loading trace.

/** The JavaScript probe; argument 2 is the observation file it writes. */
export const JAVASCRIPT_PROBE = String.raw`// Issue 195 consumer probe, run from inside relative-meta-logic's js/ directory against the
// installed meta-language artifact. A synchronous resolve hook records every module edge the
// RML sources and the package load, so the foundation check reads what was actually loaded.
import assert from 'node:assert/strict';
import { readFileSync, writeFileSync } from 'node:fs';
import { registerHooks } from 'node:module';
import { fileURLToPath } from 'node:url';

const edges = [];
registerHooks({
  resolve(specifier, context, nextResolve) {
    const result = nextResolve(specifier, context);
    edges.push({ parent: context.parentURL ?? null, specifier, child: result.url });
    return result;
  },
});

const rmlSource = new URL('./src/', import.meta.url).href;
const packageUrl = import.meta.resolve('meta-language');
const packageRoot = new URL('./node_modules/meta-language/', import.meta.url).href;
const bridge = await import('./src/rml-meta-language.mjs');
const evaluator = await import('./src/rml-links.mjs');
await import('./src/rml-theory-network.mjs');
await import('./src/rml-formal-corpus.mjs');
const api = await import('meta-language');

const SAMPLE = '(a: a is a)\n((a = a) has probability 1)\n(? (a = a))\n';
const QUOTED = "# it's (\n(a \"(\" b)\n";
const SHARED_CONCEPTS = [
  'LinkNetwork', 'LinkQuery', 'LinkType', 'ParseConfiguration', 'Probability', 'ProbabilisticTruthValue',
  'ReplacementRule', 'SubstitutionRule', 'TranslationRule', 'TranslationRuleSet', 'TruthValue',
];
const REWRITE_SOURCE = 'const oldName = call(oldName); const label = "oldName"; // oldName\n';
const REWRITE_EXPECTED = 'const newName = call(newName); const label = "oldName"; // oldName\n';

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

const metadata = (network) => [...network.links()].map((link) => link.metadata());

const sharedConceptsReused = assertion([
  check('installed package resolves from the RML consumer', () => {
    const resolved = fileURLToPath(packageUrl);
    assert.ok(packageUrl.startsWith(packageRoot), resolved);
    const manifest = JSON.parse(readFileSync(new URL('package.json', packageRoot), 'utf8'));
    const bridgeEdge = edges.find(({ parent, specifier }) => parent === new URL('rml-meta-language.mjs', rmlSource).href && specifier === 'meta-language');
    assert.equal(bridgeEdge?.child, packageUrl, 'the RML bridge resolves meta-language to the installed artifact');
    return { packageUrl, version: manifest.version };
  }),
  check('the RML bridge re-exports the package concepts themselves', () => {
    for (const name of SHARED_CONCEPTS) assert.equal(bridge[name], api[name], name);
    return SHARED_CONCEPTS;
  }),
  check('RML source is held in the package link network', () => {
    const network = bridge.parseRmlToMetaLanguage(SAMPLE);
    assert.ok(network instanceof api.LinkNetwork);
    const direct = api.LinkNetwork.parse(SAMPLE, 'RML', api.ParseConfiguration.default());
    assert.equal(network.toLino(), direct.toLino());
    return { links: network.len() };
  }),
  check('RML truth and substitution planning use the package semantics', () => {
    const half = api.ProbabilisticTruthValue.fromRatio(1, 2);
    assert.deepEqual(bridge.metaLanguageTruthSmoke(), {
      conjunction: api.TruthValue.True.and(api.TruthValue.Unknown).toString(),
      probabilityBasisPoints: api.Probability.fromRatio(1, 4).basisPoints(),
      probabilisticAndBasisPoints: half.and(half).trueProbability().basisPoints(),
    });
    const substitution = bridge.metaLanguageSubstitutionSmoke();
    assert.ok(substitution.updated >= 1 && substitution.changed);
    return { truth: bridge.metaLanguageTruthSmoke(), substitution };
  }),
]);

const distinctionsPreserved = assertion([
  check('RML source round-trips byte for byte', () => {
    for (const source of [SAMPLE, QUOTED]) {
      assert.equal(bridge.reconstructRmlFromMetaLanguage(bridge.parseRmlToMetaLanguage(source)), source);
    }
    assert.deepEqual(bridge.parseRmlLinksViaMetaLanguage(QUOTED), ["(a '(' b)"]);
    return [SAMPLE, QUOTED];
  }),
  check('the network keeps the RML language identity', () => {
    const rml = metadata(bridge.parseRmlToMetaLanguage(SAMPLE));
    const javascript = metadata(api.LinkNetwork.parse('const x = 1;\n', 'JavaScript', api.ParseConfiguration.default()));
    const language = (records) => records.find(({ linkType }) => linkType === 'Language')?.term;
    assert.equal(language(rml), 'RML');
    assert.equal(language(javascript), 'JavaScript');
    const spanned = rml.filter(({ span }) => span);
    assert.ok(spanned.length > 0 && spanned.every(({ language: name }) => name === 'RML'));
    return { languageLink: 'RML', spannedLinks: spanned.length };
  }),
  check('a JavaScript identifier rewrite leaves strings and comments alone', () => {
    const rewritten = bridge.rewriteJavaScriptIdentifierViaMetaLanguage(REWRITE_SOURCE, 'oldName', 'newName');
    assert.equal(rewritten.matchCount, 2);
    assert.equal(rewritten.source, REWRITE_EXPECTED);
    return { matchCount: rewritten.matchCount, source: rewritten.source };
  }),
]);

const evaluatorUrl = new URL('rml-links.mjs', rmlSource).href;
const insidePackage = (url) => url.startsWith(packageRoot);
const foundationAuthorityStaysInRml = assertion([
  check('the RML evaluator loads no meta-language module', () => {
    const closure = new Set([evaluatorUrl]);
    for (let grown = true; grown;) {
      grown = false;
      for (const { parent, child } of edges) {
        if (closure.has(parent) && !closure.has(child)) {
          closure.add(child);
          grown = true;
        }
      }
    }
    const rmlModules = [...closure].filter((url) => url.startsWith(rmlSource));
    assert.ok(rmlModules.length > 1, 'the trace recorded the evaluator imports');
    assert.deepEqual([...closure].filter(insidePackage), []);
    return { evaluatorModules: closure.size, rmlModules: rmlModules.map((url) => url.slice(rmlSource.length)) };
  }),
  check('no meta-language module loads an RML module', () => {
    const reverse = edges.filter(({ parent, child }) => parent && insidePackage(parent) && child.startsWith(rmlSource));
    assert.deepEqual(reverse, []);
    const manifest = JSON.parse(readFileSync(new URL('package.json', packageRoot), 'utf8'));
    const declared = Object.keys({ ...manifest.dependencies, ...manifest.peerDependencies, ...manifest.optionalDependencies });
    assert.ok(!declared.includes('relative-meta-logic'), 'the package declares no relative-meta-logic dependency');
    return { packageModules: new Set(edges.filter(({ child }) => insidePackage(child)).map(({ child }) => child)).size, declared };
  }),
  check('RML evaluates both the source and the meta-language reconstruction itself', () => {
    const report = bridge.rmlMetaLanguageParityReport(SAMPLE);
    const direct = evaluator.evaluate(SAMPLE);
    const meta = evaluator.evaluate(report.reconstructed);
    assert.deepEqual(report.directResults, direct.results);
    assert.deepEqual(report.directDiagnostics, direct.diagnostics);
    assert.deepEqual(report.metaResults, meta.results);
    assert.deepEqual(report.metaDiagnostics, meta.diagnostics);
    assert.deepEqual(direct.results, [1]);
    return { results: direct.results };
  }),
]);

writeFileSync(process.argv[2], JSON.stringify({
  packageUrl,
  moduleEdges: edges.length,
  sharedConceptsReused,
  distinctionsPreserved,
  foundationAuthorityStaysInRml,
}, null, 2) + '\n');
`;

/** A node:test reporter writing one JSON line per finished test or suite. */
export const TEST_EVENT_REPORTER = String.raw`// Writes one JSON line per finished test, so the runner reads outcomes instead of parsing the spec output.
export default async function* testEvents(source) {
  for await (const event of source) {
    if (event.type !== 'test:pass' && event.type !== 'test:fail') continue;
    const { name, file, nesting, details, skip, todo } = event.data;
    const outcome = event.type === 'test:fail' ? 'failed' : skip || todo ? 'skipped' : 'passed';
    const entry = { name, file: file ?? null, nesting, kind: details?.type ?? 'test', outcome };
    if (outcome === 'failed') {
      const error = details?.error;
      entry.error = String(error?.cause?.stack ?? error?.cause ?? error?.message ?? error).slice(0, 4000);
      // A test file whose process dies (for example killed for memory) fails with its signal or exit code.
      if (error?.signal || error?.exitCode != null) entry.process = { signal: error.signal ?? null, exitCode: error.exitCode ?? null };
    }
    yield JSON.stringify(entry) + '\n';
  }
}
`;

/** The test target name of the Rust probe inside RML's rust/tests. */
export const RUST_PROBE_TARGET = 'issue_195_meta_language_probe';

/** The Rust test of each probe assertion. */
export const RUST_PROBE_TESTS = Object.freeze({
  sharedConceptsReused: 'shared_concepts_reused',
  distinctionsPreserved: 'distinctions_preserved',
  foundationAuthorityStaysInRml: 'foundation_authority_stays_in_rml',
});

/**
 * The Rust probe. Each test writes its evidence to
 * $ISSUE_195_RML_PROBE_DIRECTORY/<assertion>.json after its assertions hold,
 * so a failing test leaves no evidence.
 */
export const RUST_PROBE = String.raw`//! Issue 195 consumer probe, copied into relative-meta-logic's rust/tests by
//! meta-language's js/scripts/run-rml-pr184-workloads.mjs and compiled against
//! the unpacked meta-language crate the runner patches in.

use std::path::PathBuf;

use meta_language::{
    LinkNetwork, LinkType, ParseConfiguration, ProbabilisticTruthValue, Probability, TruthValue,
};
use rml::evaluate;
use rml::meta_language_support::{
    meta_language_substitution_smoke, meta_language_truth_smoke, parse_rml_links_via_meta_language,
    parse_rml_to_meta_language, reconstruct_rml_from_meta_language,
    rewrite_javascript_identifier_via_meta_language, rml_meta_language_parity_report,
    RML_META_LANGUAGE,
};
use serde_json::{json, Value};

const SAMPLE: &str = "(a: a is a)\n((a = a) has probability 1)\n(? (a = a))\n";
const QUOTED: &str = "# it's (\n(a \"(\" b)\n";
const REWRITE_SOURCE: &str =
    "const oldName = call(oldName); const label = \"oldName\"; // oldName\n";
const REWRITE_EXPECTED: &str =
    "const newName = call(newName); const label = \"oldName\"; // oldName\n";

fn record(assertion: &str, evidence: &Value) {
    if let Some(directory) = std::env::var_os("ISSUE_195_RML_PROBE_DIRECTORY") {
        let path = PathBuf::from(directory).join(format!("{assertion}.json"));
        let text = serde_json::to_string_pretty(evidence).expect("the evidence serializes");
        std::fs::write(path, text).expect("the evidence is written");
    }
}

fn has_link(network: &LinkNetwork, link_type: LinkType, term: &str) -> bool {
    network.links().any(|link| {
        link.metadata().link_type() == Some(link_type) && link.metadata().term() == Some(term)
    })
}

fn has_document(network: &LinkNetwork, language: &str) -> bool {
    network.links().any(|link| {
        link.metadata().link_type() == Some(LinkType::Document)
            && link.metadata().language() == Some(language)
    })
}

#[test]
fn shared_concepts_reused() {
    // The binding compiles only when RML's network is the patched crate's own type.
    let network: LinkNetwork = parse_rml_to_meta_language(SAMPLE);
    let direct = LinkNetwork::parse(SAMPLE, RML_META_LANGUAGE, ParseConfiguration::default());
    assert_eq!(network, direct);

    let half = ProbabilisticTruthValue::from_ratio(1, 2).expect("valid probability ratio");
    let quarter = Probability::from_ratio(1, 4).expect("valid probability ratio");
    let truth = meta_language_truth_smoke();
    let conjunction = format!("{:?}", TruthValue::True.and(TruthValue::Unknown));
    assert_eq!(truth.conjunction, conjunction);
    assert_eq!(truth.probability_basis_points, quarter.basis_points());
    assert_eq!(
        truth.probabilistic_and_basis_points,
        half.and(half).true_probability().basis_points()
    );
    let substitution = meta_language_substitution_smoke();
    assert!(substitution.updated >= 1 && substitution.changed);

    record(
        "sharedConceptsReused",
        &json!({
            "checks": [
                {
                    "name": "RML source is held in the crate's own link network",
                    "holds": true,
                    "detail": { "links": network.len(), "equalsDirectParse": true },
                },
                {
                    "name": "RML truth and substitution planning use the crate semantics",
                    "holds": true,
                    "detail": {
                        "conjunction": truth.conjunction,
                        "probabilityBasisPoints": truth.probability_basis_points,
                        "probabilisticAndBasisPoints": truth.probabilistic_and_basis_points,
                        "substitutionUpdated": substitution.updated,
                    },
                },
            ],
        }),
    );
}

#[test]
fn distinctions_preserved() {
    for source in [SAMPLE, QUOTED] {
        assert_eq!(
            reconstruct_rml_from_meta_language(&parse_rml_to_meta_language(source)),
            source
        );
    }
    assert_eq!(
        parse_rml_links_via_meta_language(QUOTED).expect("the quoted sample is valid LiNo"),
        vec!["(a '(' b)".to_string()]
    );

    let network = parse_rml_to_meta_language(SAMPLE);
    let javascript =
        LinkNetwork::parse("const x = 1;\n", "JavaScript", ParseConfiguration::default());
    assert!(has_link(&network, LinkType::Language, RML_META_LANGUAGE));
    assert!(has_document(&network, RML_META_LANGUAGE));
    assert!(!has_link(&network, LinkType::Language, "JavaScript"));
    assert!(has_link(&javascript, LinkType::Language, "JavaScript"));
    assert!(!has_document(&javascript, RML_META_LANGUAGE));

    let rewritten =
        rewrite_javascript_identifier_via_meta_language(REWRITE_SOURCE, "oldName", "newName")
            .expect("the identifier rewrite succeeds");
    assert_eq!(rewritten.match_count, 2);
    assert_eq!(rewritten.source, REWRITE_EXPECTED);

    record(
        "distinctionsPreserved",
        &json!({
            "checks": [
                {
                    "name": "RML source round-trips byte for byte",
                    "holds": true,
                    "detail": [SAMPLE, QUOTED],
                },
                {
                    "name": "the network keeps the RML language identity",
                    "holds": true,
                    "detail": { "languageLink": RML_META_LANGUAGE, "documentLanguage": RML_META_LANGUAGE },
                },
                {
                    "name": "a JavaScript identifier rewrite leaves strings and comments alone",
                    "holds": true,
                    "detail": { "matchCount": rewritten.match_count, "source": rewritten.source },
                },
            ],
        }),
    );
}

#[test]
fn foundation_authority_stays_in_rml() {
    let report = rml_meta_language_parity_report(SAMPLE).expect("the sample is valid LiNo");
    let direct = evaluate(SAMPLE, None, None);
    let meta = evaluate(&report.reconstructed, None, None);
    assert_eq!(report.direct_results, direct.results);
    assert_eq!(report.direct_diagnostics, direct.diagnostics);
    assert_eq!(report.meta_results, meta.results);
    assert_eq!(report.meta_diagnostics, meta.diagnostics);
    assert!(!direct.results.is_empty());

    record(
        "foundationAuthorityStaysInRml",
        &json!({
            "checks": [
                {
                    "name": "RML evaluates both the source and the meta-language reconstruction itself",
                    "holds": true,
                    "detail": { "results": format!("{:?}", direct.results) },
                },
            ],
        }),
    );
}
`;
