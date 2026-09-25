import { readFile } from 'node:fs/promises';
import { gunzipSync } from 'node:zlib';
import { Language, Parser } from 'web-tree-sitter';
await Parser.init();
const lang = await Language.load(gunzipSync(await readFile('src/vendor/grammars/sql.wasm.gz')));
const p = new Parser(); p.setLanguage(lang);
for (const s of process.argv.slice(2)) { const t = p.parse(s); console.log(t.rootNode.hasError ? 'ERR ' : 'ok  ', JSON.stringify(s), t.rootNode.hasError ? t.rootNode.toString().slice(0,300) : ''); }
