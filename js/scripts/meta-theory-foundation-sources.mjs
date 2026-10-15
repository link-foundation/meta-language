#!/usr/bin/env node
// A source audit of adopted structural definitions, not an upstream proof run
// or evidence of project-wide theoretical application.
import { createHash } from 'node:crypto';
import { readFile } from 'node:fs/promises';
import { dirname, join, resolve } from 'node:path';
import { isDeepStrictEqual } from 'node:util';
import { fileURLToPath } from 'node:url';
import { LinkNetwork } from '../src/network.js';
import { LinkId, LinkType } from '../src/primitives.js';
import { FOUNDATION_MODELS, checkFoundationModels } from '../src/foundation-models.js';
import { readStructuredDataLino } from '../src/structured-data-links.js';
import { checkAssociativeStackSources } from './associative-stack-sources.mjs';
import { readProjectData, writeProjectData } from './project-data.mjs';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const auditFile = 'parity/associative-stack-audit.lino';
const repository = 'link-foundation/meta-theory';
const archive = 'archive/0.0.2/article.md';
const introduction = 'drafts/0.0.3/article/04-mathematical-introduction.md';
const meta = 'drafts/0.0.3/article/07-meta-theory.md';
const jsLink = 'js/src/primitives.js';
const jsNetwork = 'js/src/network.js';
const nativeNetwork = 'rust/src/link_network.rs';
const sharedFiles = ['parity/foundation-models.lino', 'js/src/data/foundation-models.lino', 'rust/src/data/foundation-models.lino'];
const digest = bytes => createHash('sha256').update(bytes).digest('hex');
const definitions = [
  {
    id: 'link reference',
    meaning: 'A link has its own reference; other links use that reference. The set-theoretic natural-number reference is an explicit representation premise, not a second kind of conceptual entity.',
    sources: [[archive, 'which has its own reference that other links can use to refer to it.'], [introduction, 'R ⊆ ℕ_0']],
    implementations: [[jsLink, 'export class LinkId {'], [nativeNetwork, 'pub struct LinkId(pub(crate) u64);']],
    model: 'finite operational reference representation',
    premises: ['finite operational references'],
    observations: ['referenceRoundTrip'],
  },
  {
    id: 'ordered recursive link tuple',
    meaning: 'A link contains an ordered tuple of references to links, retaining repeated references and arbitrary supported finite arity, including cycles.',
    sources: [[archive, 'is defined as an [n-tuple]']],
    implementations: [[jsLink, 'this._references = references.map((reference) => LinkId.from(reference));'], [nativeNetwork, 'pub(crate) references: Arc<[LinkId]>,']],
    model: 'ordered variable arity link representation',
    premises: ['finite operational references'],
    observations: ['arities', 'orderedRepeatedReferences', 'recursiveReferences', 'sourceFreeReferences'],
  },
  {
    id: 'reference to tuple network',
    meaning: 'The project stores a finite reference-to-ordered-tuple mapping. Mixed link arities are an operational generalization of the fixed-dimension networks in the draft, not an implementation of its network of all networks.',
    sources: [[introduction, 'N^n: R → T_n']],
    implementations: [[jsNetwork, 'this._links = new Map();'], [nativeNetwork, 'pub(crate) links: BTreeMap<LinkId, Arc<Link>>,']],
    model: 'finite mixed arity network',
    premises: ['finite operational references', 'bounded structural evidence'],
    observations: ['distinctReferenceMappings', 'sourceFreeReferences'],
  },
  {
    id: 'self referring point',
    meaning: 'Named interned points currently use the unary tuple containing their own reference. This adopts self-reference, while retaining the difference from the archived doublet vertex representation.',
    sources: [[archive, 'a link that begins and ends in itself'], [meta, 'все ссылки которой указывают на неё саму']],
    implementations: [[jsNetwork, '        [id],'], [nativeNetwork, 'references: Arc::from(vec![id]),']],
    model: 'unary self referring interned point',
    premises: ['point arity is explicit', 'bounded structural evidence'],
    observations: ['internedPointReferences', 'sameTermReusesReference', 'explicitDoubletReferences'],
  },
];
const premises = [
  {
    id: 'finite operational references',
    statement: 'The draft reference set is a subset of natural numbers. JavaScript currently accepts positive integral Number identifiers; Rust stores u64 identifiers. These finite operational choices do not implement unbounded natural-number addressing or establish equality of the packages entire identifier domains.',
    sources: [[introduction, 'R ⊆ ℕ_0']],
    implementations: [[jsLink, `function readLinkIdValue(value) {
  const identifier = value instanceof LinkId;
  const descriptor = identifier ? Object.getOwnPropertyDescriptor(value, 'value') : null;
  const original = identifier
    ? descriptor && Object.hasOwn(descriptor, 'value') ? descriptor.value : undefined
    : value;
  const text = typeof original === 'string' ? original.trim() : null;
  const numeric = typeof original === 'number' || typeof original === 'bigint'
    || text !== null && /^[0-9]+$/u.test(text)
    ? Number(original) : Number.NaN;
  if (identifier && typeof original !== 'number' || !Number.isSafeInteger(numeric) || numeric < 1) {
    throw new TypeError('link id must be an exact positive safe integer');
  }
  return numeric;
}`], [nativeNetwork, 'pub struct LinkId(pub(crate) u64);']],
    sharedModels: ['integer-model.binary64-number', 'integer-model.fixed-width-integer', 'integer-model.natural-number'],
  },
  {
    id: 'point arity is explicit',
    statement: 'A unary interned point and a doublet self-loop are distinct encodings. Explicit doublet links can be stored, but no faithful automatic projection between all point encodings is established by this audit.',
    sources: [[archive, 'a link that begins and ends in itself'], [meta, 'произвольной размерности']],
    implementations: [[jsNetwork, '        [id],'], [nativeNetwork, 'references: Arc::from(vec![id]),']],
    sharedModels: ['proof-system.bounded-property-check'],
  },
  {
    id: 'bounded structural evidence',
    statement: 'The captured draft contains definition and theorem excerpts. This audit executes bounded JavaScript structural observations and inspects native source; it executes neither native code nor upstream Rocq or Lean proofs. Self-reference supplies no universal proof logic, Type:Type axiom, arbitrary equivalence decision or consistency proof.',
    sources: [[meta, 'Definition MetaReference := Reference.'], [meta, 'Theorem meta_network_is_duplet_network :']],
    implementations: [['js/src/foundation-models.js', "new URL('./data/foundation-models.lino', import.meta.url)"], ['rust/src/foundation_models.rs', 'include_str!("data/foundation-models.lino")']],
    sharedModels: ['proof-system.bounded-property-check', 'proof-system.no-proof-checker', 'proof-system.lean-kernel', 'proof-system.rocq-kernel', 'universe-model.no-universes', 'universe-model.cumulative-type-hierarchy'],
  },
];

function capture(path, anchor, sources, origin = {}) {
  const bytes = sources.get(path);
  if (bytes === undefined) throw new Error(`${path}: an actual source snapshot is missing`);
  const text = Buffer.from(bytes).toString('utf8');
  const offset = text.indexOf(anchor);
  if (offset < 0) throw new Error(`${path}: the actual definition or implementation anchor is missing: ${anchor}`);
  const startLine = text.slice(0, offset).split('\n').length;
  const endLine = startLine + anchor.split('\n').length - 1;
  const excerpt = text.split('\n').slice(startLine - 1, endLine).join('\n');
  return { path, ...origin, sha256: digest(bytes), startLine, endLine, anchor, excerpt, excerptSha256: digest(excerpt) };
}

/** Prepare reproducible source and actual implementation traces as links data. */
export function buildMetaTheoryFoundationSources(audit, snapshots, projectSources) {
  const guide = audit.guides?.find(guide => guide.repository === repository);
  if (!guide) throw new Error('the pinned meta-theory guide is missing');
  const upstream = (path, anchor) => {
    const file = guide.files?.find(file => file.path === path);
    if (!file) throw new Error(`${path}: the registered immutable document is missing`);
    return capture(file.sourceFile, anchor, snapshots, { repository, revision: guide.revision, documentPath: path, url: file.url });
  };
  const implementation = ([path, anchor]) => capture(path, anchor, projectSources);
  return {
    schemaVersion: 1,
    repository,
    revision: guide.revision,
    sourceVersions: [
      { path: archive, version: '0.0.2', stage: 'archived release' },
      { path: introduction, version: '0.0.3', stage: 'draft' },
      { path: meta, version: '0.0.3', stage: 'draft' },
    ].map(document => ({ ...document, source: upstream(document.path, document.path === archive ? '# The Links Theory 0.0.2' : document.path === introduction ? '## Математическое введение в теорию связей' : '### Теория связей как мета-теория') })),
    versionBoundary: 'Archive and draft are different document versions and stages captured at the same immutable repository revision; the draft is not an archived release.',
    adoptedDefinitions: definitions.map(({ sources, implementations, ...definition }) => ({ ...definition, sources: sources.map(([path, anchor]) => upstream(path, anchor)), implementations: implementations.map(implementation) })),
    premises: premises.map(({ sources, implementations, ...premise }) => ({ ...premise, sources: sources.map(([path, anchor]) => upstream(path, anchor)), implementations: implementations.map(implementation) })),
    sharedModelSources: sharedFiles.map(path => ({ path, sha256: digest(projectSources.get(path) ?? '') })),
    retainedGaps: [
      { id: 'faithful doublet point projection', source: upstream(archive, 'a link that begins and ends in itself'), boundary: 'The current interned point uses one reference. Storing an explicit doublet is not a proof of a faithful projection for every existing point, grammar or concept.' },
      { id: 'upstream proof certification', source: upstream(meta, 'Theorem meta_network_is_duplet_network :'), boundary: 'The captured article excerpt contains a theorem statement and moving main source links. No upstream proof source snapshot, dependencies, kernel run or certificate is supplied by this audit.' },
      { id: 'project wide formal application', source: upstream(meta, 'Definition MetaNetwork := MetaReference -> MetaDuplet.'), boundary: 'Tracing these adopted structural definitions does not supply deep formal definitions for every concept or establish project-wide foundation application, universal self-typing, consumer proof authority or physical consistency.' },
    ],
    evidenceBoundary: { kind: 'pinned definition and premise source audit', javascriptExecution: 'bounded structural observations', nativeExecution: false, upstreamProofExecution: false, projectWideApplicationEvidence: false },
  };
}

/** Execute the actual JavaScript representations named by the source traces. */
export function observeMetaTheoryFoundationModels() {
  const network = new LinkNetwork();
  network.insertLinkWithOptionalId(10, []);
  network.insertLinkWithOptionalId(11, [10]);
  network.insertLinkWithOptionalId(12, [11, 10]);
  network.insertLinkWithOptionalId(13, [10, 11, 10]);
  network.insertLinkWithOptionalId(20, [21]);
  network.insertLinkWithOptionalId(21, [20]);
  network.insertLinkWithOptionalId(30, [30, 30]);
  const point = network.insertTypedPoint(LinkType.Concept, 'theory audit point');
  const reloaded = LinkNetwork.fromLino(network.toLino());
  const references = (value, id) => value.link(id).references().map(reference => reference.asU64());
  return {
    referenceRoundTrip: LinkId.from(13).asU64(),
    arities: [10, 11, 12, 13].map(id => references(network, id).length),
    orderedRepeatedReferences: references(network, 13),
    recursiveReferences: [20, 21].map(id => references(network, id)),
    sourceFreeReferences: [10, 11, 12, 13, 20, 21, 30, point].every(id => isDeepStrictEqual(references(network, id), references(reloaded, id))),
    distinctReferenceMappings: !isDeepStrictEqual(references(network, 12), references(network, 13)),
    internedPointReferences: references(network, point).map(id => id === point.asU64()),
    sameTermReusesReference: network.insertTypedPoint(LinkType.Concept, 'theory audit point').equals(point),
    explicitDoubletReferences: references(network, 30),
  };
}

/** Check immutable provenance, definition traces, premises and actual models. */
export function checkMetaTheoryFoundationSources(audit, snapshots, projectSources) {
  const problems = checkAssociativeStackSources(audit, snapshots);
  try {
    const expected = buildMetaTheoryFoundationSources(audit, snapshots, projectSources);
    if (!isDeepStrictEqual(audit.theorySourceAudit, expected)) problems.push('the meta-theory source stages, definition quotes, premises or actual implementation provenance differ from the inspected sources');
    const shared = sharedFiles.map(path => {
      const bytes = projectSources.get(path);
      if (bytes === undefined) throw new Error(`${path}: a shared model snapshot is missing`);
      return readStructuredDataLino(Buffer.from(bytes).toString('utf8'));
    });
    if (!shared.every(value => isDeepStrictEqual(value, shared[0])) || !isDeepStrictEqual(shared[0], FOUNDATION_MODELS)) problems.push('the actual JavaScript and Rust foundation model data differ from the shared register');
    if (checkFoundationModels(shared[0]).length) problems.push('the shared model register does not preserve its explicit model boundaries');
    const ids = new Set(shared[0].models.map(model => model.id));
    for (const premise of expected.premises) for (const id of premise.sharedModels) if (!ids.has(id)) problems.push(`${premise.id}: adopted premise names no actual shared model: ${id}`);
    const properties = id => shared[0].models.find(model => model.id === id)?.properties;
    if (properties('proof-system.bounded-property-check')?.['checks-proofs'] !== false
      || properties('proof-system.bounded-property-check')?.coverage !== 'finite inputs'
      || properties('proof-system.no-proof-checker')?.['checks-proofs'] !== false
      || properties('integer-model.natural-number')?.bounded !== false
      || properties('integer-model.binary64-number')?.bounded !== true
      || properties('integer-model.fixed-width-integer')?.bounded !== true
      || properties('universe-model.no-universes')?.['types-are-terms'] !== false
      || properties('universe-model.cumulative-type-hierarchy')?.cumulative !== true) {
      problems.push('the actual shared models contradict the recorded finite reference, bounded evidence or universe premises');
    }
    const observations = observeMetaTheoryFoundationModels();
    const required = { referenceRoundTrip: 13, arities: [0, 1, 2, 3], orderedRepeatedReferences: [10, 11, 10], recursiveReferences: [[21], [20]], sourceFreeReferences: true, distinctReferenceMappings: true, internedPointReferences: [true], sameTermReusesReference: true, explicitDoubletReferences: [30, 30] };
    if (!isDeepStrictEqual(observations, required)) problems.push('the actual structural models do not satisfy their adopted definition observations');
  } catch (failure) { problems.push(failure.message); }
  return problems;
}

async function main(args) {
  const audit = readProjectData(join(root, auditFile));
  const snapshots = new Map();
  for (const guide of audit.guides) for (const path of [guide.commitVerification.metadataFile, ...guide.files.map(file => file.sourceFile)]) snapshots.set(path, await readFile(join(root, path)));
  const paths = new Set([...definitions, ...premises].flatMap(entry => entry.implementations.map(([path]) => path)).concat(sharedFiles));
  const projectSources = new Map(await Promise.all([...paths].map(async path => [path, await readFile(join(root, path))])));
  if (args.includes('--update')) audit.theorySourceAudit = buildMetaTheoryFoundationSources(audit, snapshots, projectSources);
  const problems = checkMetaTheoryFoundationSources(audit, snapshots, projectSources);
  if (problems.length) throw new Error(`Meta-theory source audit failed:\n- ${problems.join('\n- ')}`);
  if (args.includes('--update')) writeProjectData(join(root, auditFile), audit);
  console.log('Meta-theory source audit: 1 immutable revision, 1 archived release, 2 draft documents, 4 adopted structural definitions and 3 explicit premises; upstream proof certification and project-wide application remain separate');
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) await main(process.argv.slice(2));
