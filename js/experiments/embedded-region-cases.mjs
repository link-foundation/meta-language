// Prints the regions detectEmbeddedRegions finds for the shared region-case
// sources, to review before freezing parity/fixtures/embedded-region-cases.json.
import { detectEmbeddedRegions } from '../src/regions.js';

const cases = [
  ['HTML', 'Both', '<script>let a = 1;</script>'],
  ['HTML', 'Both', '<SCRIPT type="module">import x from "y";</SCRIPT>'],
  ['HTML', 'Both', '<script type="text/javascript; charset=utf-8">a()</script>'],
  ['HTML', 'Both', '<script type="importmap">{"imports": {}}</script>'],
  ['HTML', 'Both', '<script type="application/ld+json">{"@type": "Thing"}</script>'],
  ['HTML', 'Both', '<script type="text/typescript">let a: number = 1;</script>'],
  ['HTML', 'Both', '<script type="text/x-template"><div>{{ a }}</div></script>'],
  ['HTML', 'Both', '<script src="app.js"></script>'],
  ['HTML', 'Both', '<style media="screen">p { color: red; }</style>'],
  ['HTML', 'Both', '<style type="text/less">@c: red;</style>'],
  ['HTML', 'Both', '<p STYLE=color:red>a</p><p style="">b</p><p style=\'margin: 0\'>c</p>'],
  ['HTML', 'Both', '<!-- <script>no()</script> --><div>ok</div>'],
  ['Markdown', 'Both', '```js\nlet a = 1;\n```\n'],
  ['Markdown', 'Both', '~~~python\ndef f(): pass\n~~~\n'],
  ['Markdown', 'Both', '```unknownlang\nx\n```\n'],
  ['Markdown', 'Both', '```\nfn main() {}\n```\n'],
  ['Markdown', 'NameDriven', '```\nfn main() {}\n```\n'],
  ['Markdown', 'ContentDriven', '```text\nSELECT 1;\n```\n'],
  ['Markdown', 'Both', '    fn indented() {}\n'],
  ['Markdown', 'Both', 'A <b>bold <i>nested</i></b> word.\n'],
  ['Markdown', 'Both', 'Break<br/>here and <img src="a.png"> and </u> stray.\n'],
  ['Markdown', 'Both', '<div>\n<p>block</p>\n</div>\n\nAfter.\n'],
  ['Markdown', 'Both', '- item with <kbd>Ctrl</kbd>\n'],
];
const bytes = (text) => new TextEncoder().encode(text);
for (const [host, policy, source] of cases) {
  const regions = detectEmbeddedRegions(source, host, policy).map((region) => {
    const { start, end } = region.span().byteRange;
    return [region.language(), start, end, new TextDecoder().decode(bytes(source).slice(start, end))];
  });
  console.log(JSON.stringify({ host, policy, source, regions }));
}
