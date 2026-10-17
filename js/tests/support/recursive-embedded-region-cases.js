// Authored default-parser boundaries, not replacement language grammars.
function recursiveRegionPoint(source, offset) {
  const lines = source.slice(0, offset).split('\n');
  return { row: lines.length - 1, column: Buffer.byteLength(lines.at(-1)) };
}

export function recursiveEmbeddedRegionCases() {
  const javascript = 'const value = "café";';
  const css = '.x { color: red; }';
  const html = `<section><script>${javascript}</script><style>${css}</style></section>\r\n`;
  const rust = 'fn main() {}\n';
  const nestedJavascript = 'const answer = 42;\n';
  const markdown = `## Nested\n\n\`\`\`JavaScript\n${nestedJavascript}\`\`\`\n`;
  const returnedMarkdown = `\n\`\`\`Rust\n${rust}\`\`\`\n`;
  const returnedHtml = `<script type="text/markdown">${returnedMarkdown}</script>\n`;
  const plain = 'plain prose café\n';
  const cases = [
    { name: 'Markdown HTML JavaScript and CSS descendants after original Rust sibling', host: 'Markdown', policy: 'Both',
      source: `# Before π\r\n\r\n\`\`\`\`html\r\n${html}\`\`\`\`\r\n\r\n\`\`\`Rust\n${rust}\`\`\`\n`,
      regions: [{ language: 'HTML', source: html, parent: -1 }, { language: 'Rust', source: rust, parent: -1 },
        { language: 'JavaScript', source: javascript, parent: 0 }, { language: 'CSS', source: css, parent: 0 }] },
    { name: 'Markdown within Markdown retains the complete outer source', host: 'Markdown', policy: 'Both',
      source: `# Outer\n\n\`\`\`\`markdown\n${markdown}\`\`\`\`\n`,
      regions: [{ language: 'Markdown', source: markdown, parent: -1 },
        { language: 'JavaScript', source: nestedJavascript, parent: 0 }] },
    { name: 'Markdown HTML Markdown Rust returns to an earlier language', host: 'Markdown', policy: 'Both',
      source: `\`\`\`\`html\n${returnedHtml}\`\`\`\`\n`,
      regions: [{ language: 'HTML', source: returnedHtml, parent: -1 },
        { language: 'Markdown', source: returnedMarkdown, parent: 0 }, { language: 'Rust', source: rust, parent: 1 }] },
    { name: 'plain content retains one existing txt boundary without a duplicate self-region', host: 'Markdown', policy: 'ContentDriven',
      source: `# Notes\n\`\`\`\n${plain}\`\`\`\n`, regions: [{ language: 'txt', source: plain, parent: -1 }] },
  ];
  for (const fixture of cases) {
    fixture.regions = fixture.regions.map(region => {
      const start = fixture.source.indexOf(region.source);
      if (start < 0 || fixture.source.indexOf(region.source, start + 1) !== -1) throw new Error('an authored recursive region has one exact source occurrence');
      const end = start + region.source.length;
      return { ...region, startByte: Buffer.byteLength(fixture.source.slice(0, start)),
        endByte: Buffer.byteLength(fixture.source.slice(0, end)),
        startPoint: recursiveRegionPoint(fixture.source, start), endPoint: recursiveRegionPoint(fixture.source, end) };
    });
  }
  return { schemaVersion: 1, id: 'recursive-default-embedded-regions',
    boundary: 'Existing Markdown and HTML CST detectors and registered default parsers, identity byte mapping only; not generic any/all selection or global equivalence evidence.',
    cases };
}
