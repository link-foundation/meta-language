// Serialized into the clean installed consumer. A Rust target is parsed again
// after removing translation provenance, so restoration cannot stand in for
// executable target code. Native Rust execution remains the CI matrix's job.
export async function formalAiSemanticProjections(api, cases) {
  const observations = [];
  const runtimeItems = (translation) => translation.items.filter(({ status }) => !['comment', 'provenance'].includes(status));
  const executable = (translation) => runtimeItems(translation).length > 0 &&
    runtimeItems(translation).every(({ status }) => ['translated', 'kept'].includes(status));
  for (const entry of cases) {
    const failures = [];
    try {
      const translation = api.selfTranslate(entry.source, entry.from, entry.target);
      if (!executable(translation)) failures.push('the source contains carried or missing runtime items');
      let javascript = translation;
      if (entry.target === 'Rust') {
        // These fixtures contain no runtime string literals with line comments.
        // Only full comment lines from the translation envelope are removed.
        const lintPrelude = '#![allow(unused, unreachable_patterns, non_snake_case, non_camel_case_types, invalid_nan_comparisons)]';
        const targetSource = translation.code.split('\n').filter((line) =>
          !line.trimStart().startsWith('//') && line.trim() !== lintPrelude).join('\n');
        javascript = api.selfTranslate(targetSource, 'Rust', 'JavaScript');
        if (!executable(javascript)) failures.push('the emitted Rust cannot be independently translated from its own definitions');
      }
      const module = await import(`data:text/javascript;base64,${Buffer.from(javascript.code).toString('base64')}`);
      const calls = entry.calls.map((call) => ({ arguments: call.arguments, expected: call.result,
        observed: module[entry.function](...call.arguments) }));
      for (const call of calls) if (!Object.is(call.observed, call.expected)) failures.push('a semantic call differs from the independent fixture');
      observations.push({ id: entry.id, from: entry.from, target: entry.target,
        code: translation.code, items: translation.items, calls, failures });
    } catch (error) {
      observations.push({ id: entry.id, from: entry.from, target: entry.target, failures: [String(error?.message ?? error)] });
    }
  }
  return observations;
}
