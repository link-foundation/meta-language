import { readFileSync } from 'node:fs';

export const jsonDocumentPolicy = Object.fromEntries(
  readFileSync(new URL('./data/json-document/schema.lino', import.meta.url), 'utf8').trim().split('\n').map(row => {
    const [kind, ...keywords] = row.slice(1, -1).split(' ');
    return [kind, keywords];
  }),
);
