// One measurement worker overlaps source resolution on the runner's other CPU.
// Message data uses inspectable LiNo; local report runs remain sequential.
import { Worker } from 'node:worker_threads';
import { readStructuredDataLino, structuredDataToLino } from '../src/structured-data-links.js';

export function createReportMeasurementWorker(decoratorsText) {
  const worker = new Worker(new URL('./self-translation-report-worker.mjs', import.meta.url), {
    workerData: decoratorsText,
  });
  let pending = null;
  let closed = false;
  let failure = null;
  const reject = error => {
    failure = error;
    pending?.reject(error);
    pending = null;
  };
  worker.on('error', reject);
  worker.on('exit', code => { if (!closed) reject(new Error(`report measurement worker exited ${code}`)); });
  worker.on('message', text => {
    const request = pending;
    pending = null;
    if (!request) { reject(new Error('report measurement worker returned an unexpected result')); return; }
    try {
      const result = readStructuredDataLino(text);
      if (result.module !== request.module) throw new Error('report measurement worker returned a different module');
      if (result.error) request.reject(new Error(result.error));
      else request.resolve(result.row);
    } catch (error) { request.reject(error); }
  });
  return {
    measure(module, context) {
      if (closed || failure) return Promise.reject(failure ?? new Error('report measurement worker is closed'));
      if (pending) return Promise.reject(new Error('one report measurement may run at a time'));
      const text = structuredDataToLino({ module, context });
      return new Promise((resolve, reject) => {
        pending = { module, resolve, reject };
        worker.postMessage(text);
      });
    },
    async close() {
      closed = true;
      pending?.reject(new Error('report measurement worker closed before completion'));
      pending = null;
      await worker.terminate();
    },
  };
}
