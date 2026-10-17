// Authenticated provider revisions for exact-source experiment overlays.
// Historical overlay documents and production sources are never rewritten here.
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { readProjectData, checkProjectData, writeProjectData } from '../../js/scripts/project-data.mjs';

const hash = bytes => createHash('sha256').update(bytes).digest('hex');
export const reviewedOverlayProviderPolicy = {
  schemaVersion: 1,
  model: 'reviewed exact overlay provider revisions 1',
  historicalOverlays: [
    { experiment: 'rust-text-certificate-adapter', sourceSha256: 'e8d1a6d619309e4b578d1bc36711d4cb5569d9d840d6865ef5aa899afcee613e' },
    { experiment: 'rust-raw-binding-certificate-adapter', sourceSha256: '7da6c08ef9154ac5f5fa333d1791e9a652c7f70631efbe8f9c764c9c21ebc061' },
  ],
  revisions: [
    { sourceCommit: '85639e6de8b24911d129f93facdeeaf106ab79b7',
      review: 'The unchanged historical provider hashes in both original overlay documents.',
      providers: [
        { module: 'typed-program-links.js', sourceSha256: 'af6d379d7b407edc8b2e9f357643a1dea67e5b73887e7b56c4e36ee654d33bb4' },
        { module: 'typed-program-links-validation.js', sourceSha256: '557f0fdda16cb21f2e671d579d52df5b033972463a11c71bb57fe86343e10815' },
        { module: 'typed-program-checked-projection.js', sourceSha256: '28bdfa8b275d5d17cb6865aab0e0d24571f623976a3004015a9ffd3563c44cb8' },
        { module: 'translation/emit-javascript.js', sourceSha256: '1c3477f0b1813ba6f938caad1391643edb712be6623b376b092415d5e0364416' },
        { module: 'translation/emit-rust.js', sourceSha256: '6af4ea8888712f1449a2842e327ef4c24eb457d5e821f1fc2274d2c6f35614c0' },
      ] },
    { sourceCommit: '93c65ca1ada6c5001a56acc4430500e55062027a',
      review: 'Reviewed complete provider changes: callable binding owners, ordered union/record projection, retained optional results, shared target renderers and explicit public array consumption. Every original overlay hook remains uniquely located; all raw-owner, descriptor and saved-LinkId checks remain required.',
      providers: [
        { module: 'typed-program-links.js', sourceSha256: 'a0f46c4fdca3ed9bb729da7a701266bf8d7609ba142bc3009cd6eafe39983458' },
        { module: 'typed-program-links-validation.js', sourceSha256: '0d6e3b6a2b9883ac4f50b4383fe07823cda091961e72922d21a8821ba86226e3' },
        { module: 'typed-program-checked-projection.js', sourceSha256: 'd32f1193251a24a7b189afd920bea253b2ad645ce4009600b6b34888571be826' },
        { module: 'translation/emit-javascript.js', sourceSha256: 'b4b1c287cd9c80848edd13b52fe521ec0b649f853e287dfd4e9b701dbacd8aaa' },
        { module: 'translation/emit-rust.js', sourceSha256: 'f1c3adcbee1adf091b9983009f822b279f43463e6660b22dfa399b98c9863d3d' },
      ] },
    { sourceCommit: '10efc6489c30b7bbf6ea5383780da44a457e9aa2',
      review: 'Reviewed the complete B47 provider diff: only the two target emitters changed. Both import the shared Array.from mapping renderer; JavaScript routes checked map expressions to it, while Rust delegates its existing ordered receiver/callback evaluation and parameter-count formatting. Codec, validation and recheck providers are byte-identical to the preceding revision. Every original overlay hook remains uniquely located and every original transformation and owner check remains required.',
      providers: [
        { module: 'typed-program-links.js', sourceSha256: 'a0f46c4fdca3ed9bb729da7a701266bf8d7609ba142bc3009cd6eafe39983458' },
        { module: 'typed-program-links-validation.js', sourceSha256: '0d6e3b6a2b9883ac4f50b4383fe07823cda091961e72922d21a8821ba86226e3' },
        { module: 'typed-program-checked-projection.js', sourceSha256: 'd32f1193251a24a7b189afd920bea253b2ad645ce4009600b6b34888571be826' },
        { module: 'translation/emit-javascript.js', sourceSha256: 'ef5e10a72d358f7c98fe3ac0dbfc9699a5b2499a86911c851a1300693e84b5f6' },
        { module: 'translation/emit-rust.js', sourceSha256: '79acfcfe14b78e81c604f31f8331f2b2845faffc4d9f342463dc18135c9fe3a6' },
      ] },
    { sourceCommit: 'fcad9384b8760e15ab7b065cb91d2423c5e1b561',
      review: 'Reviewed the complete provider diff against 10efc6489c30b7bbf6ea5383780da44a457e9aa2: only translation/emit-rust.js changed, importing typeError and using it for the existing union target refusal. The exact refusal reason remains unchanged while its error kind now matches the native type diagnostic; unions remain refused. The other four providers, all exact overlay hooks and every certificate, raw-owner, descriptor and saved-LinkId requirement remain unchanged.',
      providers: [
        { module: 'typed-program-links.js', sourceSha256: 'a0f46c4fdca3ed9bb729da7a701266bf8d7609ba142bc3009cd6eafe39983458' },
        { module: 'typed-program-links-validation.js', sourceSha256: '0d6e3b6a2b9883ac4f50b4383fe07823cda091961e72922d21a8821ba86226e3' },
        { module: 'typed-program-checked-projection.js', sourceSha256: 'd32f1193251a24a7b189afd920bea253b2ad645ce4009600b6b34888571be826' },
        { module: 'translation/emit-javascript.js', sourceSha256: 'ef5e10a72d358f7c98fe3ac0dbfc9699a5b2499a86911c851a1300693e84b5f6' },
        { module: 'translation/emit-rust.js', sourceSha256: 'c43c340c38769be58bc25b7c5ff806cab041a08600eef34bcf2154c1a9fdd7fe' },
      ] },
  ],
};

/** Read only one complete reviewed provider revision; a byte edit requires review. */
export function readReviewedOverlayProviderSources({ experiment, modules, sourceRoot = new URL('../../js/src/', import.meta.url), readSource = module => readFileSync(new URL(module, sourceRoot), 'utf8') }) {
  const historical = reviewedOverlayProviderPolicy.historicalOverlays.find(row => row.experiment === experiment);
  if (!historical) throw new TypeError('the historical overlay experiment is not registered');
  const historicalFile = new URL('../' + experiment + '/data/overlays.lino', import.meta.url);
  if (hash(readFileSync(historicalFile)) !== historical.sourceSha256) throw new TypeError('the original historical overlay document changed: ' + experiment);
  const baseline = readProjectData(historicalFile), expected = reviewedOverlayProviderPolicy.revisions[0].providers;
  if (baseline.length !== expected.length || baseline.some((row, index) => row.module !== expected[index].module || row.sourceSha256 !== expected[index].sourceSha256)) throw new TypeError('the historical overlay provider identities differ');
  if (modules.length !== expected.length || modules.some((module, index) => module !== expected[index].module)) throw new TypeError('the exact overlay module inventory changed');
  const sources = new Map(modules.map(module => [module, readSource(module)]));
  const selected = reviewedOverlayProviderPolicy.revisions.filter(revision => revision.providers.every(row => hash(sources.get(row.module)) === row.sourceSha256));
  if (selected.length !== 1) throw new TypeError('the exact overlay providers have no unique reviewed revision; review actual source changes before refreshing');
  return { sources, revision: selected[0] };
}

export function writeReviewedOverlayProviderPolicy(check) {
  const file = new URL('./data/provider-revisions.lino', import.meta.url);
  if (check) checkProjectData(file, reviewedOverlayProviderPolicy);
  else writeProjectData(file, reviewedOverlayProviderPolicy);
}
