import { readFileSync } from 'node:fs';

let rules;

export function documentProjectionRules(rule) {
  rules ??= new Map(readFileSync(new URL('./data/document-projection-rules.lino', import.meta.url), 'utf8')
    .trim().split('\n').map(line => {
      const [name, ...members] = line.slice(1, -1).split(' ');
      return [name, members];
    }));
  return rules.get(rule);
}
