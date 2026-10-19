// Preloaded only by the prepared-snapshot evidence child. Denial is active
// before importing the public package, including named Node filesystem imports.
import fs from 'node:fs';
import fsp from 'node:fs/promises';
import http from 'node:http';
import https from 'node:https';
import net from 'node:net';
import tls from 'node:tls';
import childProcess from 'node:child_process';
import { syncBuiltinESMExports } from 'node:module';
import { resolve, sep } from 'node:path';
import { fileURLToPath } from 'node:url';
import { readBootstrapStructuredDataLino } from '../../../scripts/lib/structured-data-bootstrap.mjs';

const root = resolve(process.env.META_LANGUAGE_TOOLING_ROOT);
const manifest = readBootstrapStructuredDataLino(fs.readFileSync(resolve(root, 'parity/grammars/external-assets.lino'), 'utf8'));
const denied = new Set(Object.keys(manifest.assets).map(file => resolve(root, file)));
const cache = resolve(root, '.external-grammars');
const configuredCache = process.env.META_LANGUAGE_EXTERNAL_GRAMMAR_CACHE
  ? resolve(process.env.META_LANGUAGE_EXTERNAL_GRAMMAR_CACHE) : cache;
const check = path => {
  if (typeof path === 'number') return;
  const actual = resolve(path instanceof URL ? fileURLToPath(path) : Buffer.isBuffer(path) ? path.toString() : String(path));
  if (denied.has(actual) || actual === cache || actual.startsWith(`${cache}${sep}`)
    || actual === configuredCache || actual.startsWith(`${configuredCache}${sep}`)) {
    throw new Error(`external grammar asset access denied: ${actual}`);
  }
};
for (const name of ['readFileSync', 'readFile', 'openSync', 'open', 'createReadStream']) {
  const original = fs[name];
  fs[name] = function(path, ...args) { check(path); return original.call(this, path, ...args); };
}
for (const name of ['readFile', 'open']) {
  const original = fsp[name];
  fsp[name] = function(path, ...args) { check(path); return original.call(this, path, ...args); };
}
const denyNetwork = () => { throw new Error('external grammar evidence requires offline execution'); };
globalThis.fetch = denyNetwork;
http.request = http.get = https.request = https.get = denyNetwork;
net.connect = net.createConnection = tls.connect = denyNetwork;
for (const name of ['spawn', 'spawnSync', 'exec', 'execSync', 'execFile', 'execFileSync', 'fork']) {
  childProcess[name] = () => { throw new Error('prepared native snapshot evidence cannot delegate to an external process'); };
}
syncBuiltinESMExports();
