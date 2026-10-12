// Internal worker; it uses the same measurement implementation and never
// executes source programs or invokes a native compiler.
import { parentPort, workerData } from 'node:worker_threads';
import { DecoratorSet } from '../src/decorators.js';
import { readStructuredDataLino, structuredDataToLino } from '../src/structured-data-links.js';
import { measureSelfTranslationModule } from './generate-self-translation-report.mjs';

if (!parentPort) throw new Error('report measurement worker requires its parent');
const decorators = DecoratorSet.fromLino(workerData);
parentPort.on('message', text => {
  const { module, context } = readStructuredDataLino(text);
  let result;
  try {
    if (typeof module !== 'string' || !module.startsWith('js/src/') || !module.endsWith('.js')
      || module.includes('\\') || module.includes('\0')
      || module.split('/').some(part => part === '..' || part === '.' || !part)) throw new Error('report module must be a source file inside js/src');
    result = { module, row: measureSelfTranslationModule(module, decorators, context) };
  } catch (error) { result = { module, error: String(error?.message ?? error) }; }
  parentPort.postMessage(structuredDataToLino(result));
});
