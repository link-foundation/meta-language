// One-off: wrote the first parity/issue-195-sources.json from the live
// discussion. Later revisions go through check-issue-195-sources.mjs --update.
import { writeFileSync } from 'node:fs';
import { fetchLiveDiscussion, revisionOf, sourceTitle } from '../js/scripts/issue-195-sources.mjs';
import { ISSUE_195_SOURCES } from '../js/scripts/issue-195-requirements.mjs';

const ROLES = {
  issue: ['Original request: full JavaScript, Rust, Lean and Rocq representation, transformation and translation, default CSTs, parity and delivery', ['I195-CST-', 'I195-EMBED-', 'I195-IMPORT-', 'I195-SEM-', 'I195-XFORM-', 'I195-TRANSLATE-', 'I195-RENAME-', 'I195-SHARED-', 'I195-PARITY-', 'I195-NATIVE-', 'I195-DELIVERY-']],
  clarification: ['Default CSTs for the complete language inventory, full four-language representation and transformation, evidence and release delivery', ['I195-CST-', 'I195-EMBED-', 'I195-SEM-', 'I195-XFORM-', 'I195-TRANSLATE-', 'I195-CONFORMANCE-', 'I195-DELIVERY-']],
  acceptanceGate: ['Executable acceptance gate: atomic ledger, full-requirements CI, behavioral matrix, 12 translations, gate fault injection, packaging and truthful status', ['I195-']],
  completeScope: ['Complete the entire scope in this PR; never weaken the gate or substitute escalation for delivery', ['I195-']],
  deliveryCorrection: ['Real semantic translation, acceptance evidence from observed execution, semantic analysis rather than labels', ['I195-TRANSLATE-', 'I195-SEM-', 'I195-XFORM-', 'I195-RENAME-', 'I195-GATE-', 'I195-SEMANTICS-']],
  fullDelivery: ['Fully deliver every requirement in this PR', ['I195-']],
  cacheCleanup: ['Completion audit and the automatic, safe cleanup of every build-cache class', ['I195-CACHE-']],
  remainingAudit: ['Audit of the remaining work at c5eb2303 (its failure counts are historical): conformance, generative testing, full language semantics, default CSTs, cache cleanup and publication', ['I195-CONFORMANCE-', 'I195-GENERATIVE-', 'I195-SEM-', 'I195-SEMANTICS-', 'I195-TRANSLATE-', 'I195-CST-', 'I195-CACHE-', 'I195-DELIVERY-']],
  repositoryDirective: ['Repository-wide vision: committed specification and register, dependency and Links Notation fitness, native merged grammars, import/export round trips, automatic merge, readable names, consumers, independent acceptance, cleanup and publication', ['I195-VISION-', 'I195-DEPENDENCY-', 'I195-LINO-', 'I195-GRAMMAR-', 'I195-INTERCHANGE-', 'I195-MERGE-', 'I195-NAMING-', 'I195-SEMANTICS-', 'I195-DOWNSTREAM-', 'I195-ACCEPTANCE-', 'I195-CACHE-', 'I195-DELIVERY-', 'I195-DOCUMENTATION-']],
};
const STATUS_REPORTS = ['5797717687', '5800036404', '5800231003', '5800423288', '5800652847', '5800842752'];

const live = await fetchLiveDiscussion();
const byId = new Map(live.comments.map((comment) => [String(comment.id), comment]));
const requirementSources = Object.entries(ISSUE_195_SOURCES).map(([key, url]) => {
  const id = key === 'issue' ? 195 : Number(url.split('issuecomment-')[1]);
  const current = key === 'issue' ? live.issue : byId.get(String(id));
  const [role, ledgerCoverage] = ROLES[key];
  return {
    key, id, kind: key === 'issue' ? 'issue' : 'comment', url,
    title: key === 'issue' ? current.title : sourceTitle(current.body),
    role, ledgerCoverage, ...revisionOf(current),
  };
});
const statusReports = STATUS_REPORTS.map((id) => {
  const current = byId.get(id);
  return {
    id: Number(id), kind: 'comment', url: `https://github.com/link-foundation/meta-language/pull/196#issuecomment-${id}`,
    title: sourceTitle(current.body), role: 'Solver status report; carries no requirements', ...revisionOf(current),
  };
});
const register = {
  schemaVersion: 1,
  repository: 'link-foundation/meta-language',
  issue: 195,
  pullRequest: 196,
  hashAlgorithm: 'sha256 of the exact UTF-8 body returned by the GitHub REST API',
  reconciledAt: new Date().toISOString().slice(0, 10),
  classificationRule: 'Every comment is a requirement source, a status report, or automation output recognized by AUTOMATION_MARKERS in js/scripts/issue-195-sources.mjs.',
  requirementSources,
  statusReports,
};
writeFileSync(new URL('../parity/issue-195-sources.json', import.meta.url), `${JSON.stringify(register, null, 2)}\n`);
console.log(requirementSources.map((s) => `${s.key} ${s.sha256.slice(0, 12)} ${s.updatedAt}`).join('\n'));
