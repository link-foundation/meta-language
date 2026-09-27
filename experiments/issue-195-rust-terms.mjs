import { analyzeProgram } from '../js/src/index.js';
const [language, source] = process.argv.slice(2);
const program = analyzeProgram(source, language);
for (const m of program.sourceMappings) console.log(m.term.padEnd(34), JSON.stringify(program.source.slice(m.start, m.end)).slice(0, 60));
