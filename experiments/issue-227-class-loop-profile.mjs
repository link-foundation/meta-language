import {performance} from 'node:perf_hooks';
import {captureJavaScriptOrdinaryClassModule} from '../js/src/translation/javascript-ordinary-class-source.js';
import {prepareJavaScriptOrdinaryClassModule} from '../js/src/translation/javascript-ordinary-class-module.js';
import {runJavaScriptSourceTransition} from '../js/src/node-source-modules.js';

const iterations = Number(process.argv[2] ?? 12);
if (!Number.isSafeInteger(iterations) || iterations < 1 || iterations > 100) throw new RangeError('use 1 to 100 iterations');
const source = `class Counter { run() { let value = 0; for (let index = 0; index < ${iterations}; index += 1) value = value + 1; return value; } } export function read() { return new Counter().run(); }`;
const start = performance.now();
const capture = captureJavaScriptOrdinaryClassModule(source, {identifier: 'loop-profile.js', entries: ['read']});
const instance = prepareJavaScriptOrdinaryClassModule(capture, {realmIdentity: 53n});
const ready = performance.now();
const result = runJavaScriptSourceTransition(instance.embedding, instance.begin('read', []), {steps: 8192});
console.log({iterations, prepareMilliseconds: ready - start, runMilliseconds: performance.now() - ready,
  kind: result.transition.$, value: result.transition.value, objects: instance.embedding.heap().objects.length});
