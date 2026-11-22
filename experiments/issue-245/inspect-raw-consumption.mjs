// Compare the complete checked emission with its stored receipt at the call site.
import { Session } from 'node:inspector';
import { writeFileSync, readFileSync } from 'node:fs';
import { isDeepStrictEqual } from 'node:util';
import { readBootstrapStructuredDataLino } from '../../scripts/lib/structured-data-bootstrap.mjs';

const [output] = process.argv.slice(2);
if (!output) throw new Error('usage: node inspect-raw-consumption.mjs <difference-output>');
const session = new Session();
session.connect();
const post = (method, args = {}) => new Promise((resolve, reject) => {
  session.post(method, args, (error, result) => error ? reject(error) : resolve(result));
});
const marker = '  verifyOriginalLinkRawConsumingConservation(snapshotArtifact);';
const source = new URL('../original-link-complete-table-projection/raw-consuming-emission.mjs', import.meta.url);
const lines = readFileSync(source, 'utf8').split('\n');
const lineNumber = lines.indexOf(marker);
if (lineNumber < 0) throw new Error('the receipt verification call moved');
await post('Debugger.enable');
await post('Debugger.setBreakpointByUrl', { url: source.href, lineNumber });
session.on('Debugger.paused', ({ params }) => {
  const [{ callFrameId }] = params.callFrames;
  session.post('Debugger.evaluateOnCallFrame', { callFrameId, expression: 'JSON.stringify(snapshotArtifact)', returnByValue: true }, (error, reply) => {
    try {
      if (error) throw error;
      const current = JSON.parse(reply.result.value);
      const recorded = readBootstrapStructuredDataLino(readFileSync(new URL('data/raw-consuming-current-native-definitions.lino', source), 'utf8'));
      const differences = [];
      function compare(left, right, path) {
        if (isDeepStrictEqual(left, right)) return;
        if (left && right && typeof left === 'object' && typeof right === 'object') {
          const keys = new Set([...Object.keys(left), ...Object.keys(right)]);
          for (const key of keys) compare(left[key], right[key], [...path, key]);
          return;
        }
        const summarize = value => typeof value === 'string' && value.length > 300 ? { length: value.length, start: value.slice(0, 150) } : value;
        differences.push({ path, actual: summarize(left), recorded: summarize(right) });
      }
      compare(current, recorded, []);
      writeFileSync(output, JSON.stringify(differences, null, 2));
    } finally {
      session.post('Debugger.resume');
    }
  });
});
const { generateOriginalLinkRawConsumingEmission } = await import(source);
try {
  await generateOriginalLinkRawConsumingEmission(true);
} finally {
  session.disconnect();
}
