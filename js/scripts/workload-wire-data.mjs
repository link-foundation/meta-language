// Child tools keep their actual JSON interoperability protocol. Successful
// observations are retained as inspectable links and the temporary wire files
// leave only after the complete linked output is written.
import { randomUUID } from 'node:crypto';
import { createReadStream } from 'node:fs';
import { mkdir, open, readFile, rename, rm } from 'node:fs/promises';
import { dirname } from 'node:path';
import { createInterface } from 'node:readline';
import { structuredDataRecordToLino } from '../src/structured-data-records.js';
import { writeProjectData } from './project-data.mjs';

export async function consumeWorkloadWireData(wireFile, linkedFile) {
  const value = JSON.parse(await readFile(wireFile, 'utf8'));
  writeProjectData(linkedFile, value, { format: 'lino' });
  await rm(wireFile);
  return value;
}

export async function consumeWorkloadWireEvents(wireFile, linkedFile) {
  await mkdir(dirname(linkedFile), { recursive: true });
  const temporary = `${linkedFile}.${randomUUID()}.tmp`;
  const events = [];
  try {
    const output = await open(temporary, 'wx');
    try {
      const source = createReadStream(wireFile);
      const input = createInterface({ input: source, crlfDelay: Infinity });
      try {
        for await (const line of input) {
          if (!line.trim()) continue;
          const event = JSON.parse(line);
          await output.writeFile(structuredDataRecordToLino(event));
          events.push(event);
        }
      } finally {
        input.close();
        source.destroy();
      }
    } finally {
      await output.close();
    }
    await rename(temporary, linkedFile);
    await rm(wireFile);
    return events;
  } finally {
    await rm(temporary, { force: true });
  }
}
