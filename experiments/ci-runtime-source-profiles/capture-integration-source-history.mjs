import assert from 'node:assert/strict';
import { fileURLToPath } from 'node:url';
import { captureGitArtifactCertificate } from '../../js/scripts/capture-git-artifact-certificate.mjs';
import { checkProjectData, writeProjectData } from '../../js/scripts/project-data.mjs';

const inputs = [
  ['rust/Cargo.toml', '8bc0ef9725388c7927acf1854870bf46f5e365be', 'acec16f4f563c7bd92a97e50b5efd59f0a3f3740b49a7c5d308edac62282a5dd'],
  ['js/src/translation/types.js', 'ec9f4efb0e9ef936114e09823981352f6db13700', 'e2b019e978a083ad378b9add056bab82bca5287ea885c3aaf1e325c110b81da0'],
];

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const args = process.argv.slice(2);
  assert.ok(args.length === 0 || (args.length === 1 && args[0] === '--check'));
  const sources = inputs.map(([path, revision, current]) => {
    const { blob, certificate } = captureGitArtifactCertificate(revision, path);
    const source = blob.toString('utf8');
    assert.deepEqual(Buffer.from(source), blob);
    return { path, revision, current, source, certificate };
  });
  const history = { schemaVersion: 1, sources };
  const file = new URL('../../parity/self-translation/integration-source-history.lino', import.meta.url);
  if (args.length) checkProjectData(file, history);
  else writeProjectData(file, history);
  console.log(`Authenticated ${sources.length} complete source predecessors.`);
}
