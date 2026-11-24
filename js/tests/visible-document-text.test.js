import assert from 'node:assert/strict';
import test from 'node:test';
import { LinkNetwork } from '../src/index.js';
import { readProjectData } from '../scripts/project-data.mjs';
import { recordIssue195Observations } from './support/issue-195-observations.js';

const fixtureFile = 'parity/fixtures/visible-document-text.lino';
const fixtures = readProjectData(new URL(`../../${fixtureFile}`, import.meta.url));

const groups = [
  ['HTML', 'html', 'I195-I238-HTML-VISIBLE-TEXT', ['htmlVisibleTextAndOptions', 'htmlVisibleTextKeepsSourceRanges']],
  ['Markdown', 'markdown', 'I195-I238-MARKDOWN-VISIBLE-TEXT', ['markdownVisibleText', 'markdownVisibleTextKeepsSourceRanges']],
  ['DOCX', 'docx', 'I195-I238-WORDPROCESSINGML-VISIBLE-TEXT', ['wordprocessingmlVisibleText', 'wordprocessingmlVisibleTextKeepsSourceRanges']],
];

for (const [language, group, requirementId, assertions] of groups) {
  test(`${language} projects visible text with the original source ranges`, (context) => {
    for (const { source, text, options, selections } of fixtures[group]) {
      const original = LinkNetwork.parse(source, language);
      const network = LinkNetwork.fromLino(original.toLino());
      const projection = network.visibleText(options);
      assert.equal(projection.text, text);
      assert.equal(network.reconstructText(), source);
      assert.deepEqual(projection, original.visibleText(options));

      const sourceBytes = new TextEncoder().encode(source);
      let textEnd = 0;
      let sourceEnd = 0;
      for (const span of projection.map) {
        assert.equal(span.textStart, textEnd);
        assert.ok(span.textEnd > span.textStart && span.textEnd <= text.length, JSON.stringify(span));
        assert.ok(span.sourceStart >= sourceEnd && span.sourceEnd >= span.sourceStart && span.sourceEnd <= sourceBytes.length, JSON.stringify(span));
        textEnd = span.textEnd;
        sourceEnd = span.sourceEnd;
      }
      assert.equal(textEnd, text.length);
      for (const [visible, expectedSource] of selections) {
        const start = text.indexOf(visible);
        assert.ok(start >= 0, visible);
        const range = projection.sourceRange(start, start + visible.length);
        assert.equal(new TextDecoder().decode(sourceBytes.subarray(range.start, range.end)), expectedSource);
      }
      assert.equal(projection.sourceRange(0, 0), null);
      assert.equal(projection.sourceRange(text.length, text.length + 1), null);
    }
    recordIssue195Observations({
      requirementId, suffix: 'behavior', fixtureId: requirementId.toLowerCase(), fixtureFile, assertions, testName: context.name,
    });
  });
}
