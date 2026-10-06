// The downstream consumer matrix (docs/downstream-consumers.md): the usage
// and the requirements of relative-meta-logic and formal-ai, each mapped to a
// meta-language capability, the ledger rows that cover it, the tests of this
// repository that exercise it and a status. `validateConsumerMatrix` fails
// when a mapped row or test does not exist.

export const CONSUMER_MATRIX = 'docs/downstream-consumers.md';

// The consumers the vision names, by the heading of their matrix section.
export const DOWNSTREAM_CONSUMERS = Object.freeze({
  'relative-meta-logic': 'link-foundation/relative-meta-logic',
  'formal-ai': 'link-assistant/formal-ai',
});

export const CONSUMER_STATUSES = Object.freeze(['covered by tests', 'not yet verified', 'not yet implemented']);

const COLUMNS = ['Consumer usage or requirement', 'meta-language capability', 'Ledger rows', 'Tests', 'Status'];

// The cells of a Markdown table row, with `|` inside backticks kept.
function cells(line) {
  const result = [];
  let current = '';
  let code = false;
  for (const character of line.trim().replace(/^\||\|$/gu, '')) {
    if (character === '`') code = !code;
    if (character === '|' && !code) {
      result.push(current.trim());
      current = '';
    } else {
      current += character;
    }
  }
  result.push(current.trim());
  return result;
}

const codeSpans = (text) => [...text.matchAll(/`([^`]+)`/gu)].map(([, span]) => span);

/** The sections of the matrix, one per consumer, with their mappings. */
export function parseConsumerMatrix(markdown) {
  const sections = markdown.split(/^## /mu).slice(1);
  const scope = sections.find((section) => section.startsWith('Audit scope')) ?? '';
  const consumers = {};
  for (const [heading, repository] of Object.entries(DOWNSTREAM_CONSUMERS)) {
    const section = sections.find((candidate) => candidate.split('\n')[0].trim() === heading);
    const inspected = scope.split('\n- ').find((item) => item.includes(`https://github.com/${repository}>`));
    const commit = inspected?.match(/commit `([0-9a-f]{40})`/u)?.[1] ?? null;
    const lines = (section ?? '').split('\n').filter((line) => line.startsWith('|'));
    const header = lines.length > 0 ? cells(lines[0]) : [];
    const mappings = lines.slice(2).filter((line) => {
      const row = cells(line);
      return row.join('|') !== COLUMNS.join('|') && !row.every((cell) => /^:?-+:?$/u.test(cell));
    }).map((line) => {
      const [usage, capability, rows, tests, status] = cells(line);
      return {
        usage,
        capability,
        rows: codeSpans(rows).filter((span) => span.startsWith('I195-')),
        tests: codeSpans(tests).filter((span) => span.includes('/')),
        status,
      };
    });
    consumers[heading] = { repository, commit, header, mappings, present: section !== undefined };
  }
  return { consumers };
}

/**
 * The problems of `matrix`: a missing consumer section, inspected commit or
 * mapping, an unknown status, and a ledger row or test that does not exist.
 */
export function validateConsumerMatrix(matrix, { rowIds, fileExists }) {
  const errors = [];
  for (const [heading, consumer] of Object.entries(matrix.consumers)) {
    if (!consumer.present) {
      errors.push(`${CONSUMER_MATRIX} has no ${heading} section`);
      continue;
    }
    if (consumer.commit === null) errors.push(`${CONSUMER_MATRIX} records no inspected ${consumer.repository} commit`);
    if (consumer.header.join('|') !== COLUMNS.join('|')) {
      errors.push(`the ${heading} table has the columns ${COLUMNS.join(', ')}`);
    }
    if (consumer.mappings.length === 0) errors.push(`the ${heading} table maps no usage`);
    for (const { usage, rows, tests, status } of consumer.mappings) {
      const label = `${heading}: ${usage.slice(0, 60)}`;
      if (!CONSUMER_STATUSES.includes(status)) errors.push(`${label} has the unknown status ${status}`);
      for (const row of rows) if (!rowIds.has(row)) errors.push(`${label} maps the missing ledger row ${row}`);
      for (const test of tests) if (!fileExists(test)) errors.push(`${label} maps the missing test ${test}`);
      if (rows.length === 0) errors.push(`${label} maps no ledger row`);
      if (status === 'covered by tests' && tests.length === 0) errors.push(`${label} is covered by tests but names none`);
    }
  }
  return errors;
}

/** The consumers whose usage each ledger row covers, by row id. */
export function downstreamByRow(matrix) {
  const byRow = new Map();
  for (const consumer of Object.values(matrix.consumers)) {
    for (const { rows } of consumer.mappings) {
      for (const row of rows) byRow.set(row, [...new Set([...(byRow.get(row) ?? []), consumer.repository])].sort());
    }
  }
  return byRow;
}
