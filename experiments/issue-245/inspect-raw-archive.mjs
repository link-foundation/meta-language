// Capture the archive produced by the real premerge source or a selected
// historical checker. This probe never rewrites product files.
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { readFileSync, writeFileSync } from 'node:fs';
import { registerHooks } from 'node:module';

const [revision, output] = process.argv.slice(2);
if (!revision || !output) throw new Error('usage: node inspect-raw-archive.mjs <current|git-revision> <archive-output>');
const root = new URL('../../', import.meta.url);
const file = 'js/src/translation/check.js';
if (revision !== 'current') {
  const source = execFileSync('git', ['show', `${revision}:${file}`], { cwd: root, encoding: 'utf8' });
  const checker = new URL(file, root).href;
  registerHooks({
    load(url, context, next) {
      if (url === checker) return { shortCircuit: true, format: 'module', source };
      return next(url, context);
    },
  });
}
const hash = value => createHash('sha256').update(value).digest('hex');
const { parseJavaScript } = await import(new URL('js/src/translation/javascript.js', root));
const { checkProgram } = await import(new URL(file, root));
const { encodeTypedProgramLinks } = await import(new URL('js/src/typed-program-links.js', root));
const { encodeLinkNetworkBinary } = await import(new URL('js/src/link-network-binary.js', root));
const source = readFileSync(new URL('experiments/original-link-complete-table-projection/raw-rules.mjs', root), 'utf8');
const program = checkProgram(parseJavaScript(source, { referenceSemantics: false }));
const packet = encodeTypedProgramLinks(program);
const text = packet.network.toLino();
const recordLimit = packet.network.links().length;
if (recordLimit > 1_000_000 || Buffer.byteLength(text) > 64 * 1024 * 1024) throw new RangeError('archive exceeds the checked projection budget');
const binary = encodeLinkNetworkBinary(packet.network, { recordLimit, byteLimit: 64 * 1024 * 1024 });
writeFileSync(output, text);
console.log(JSON.stringify({
  revision, source: hash(source), textBytes: Buffer.byteLength(text), textSha256: hash(text),
  binaryBytes: binary.byteLength, binarySha256: hash(binary),
}));
