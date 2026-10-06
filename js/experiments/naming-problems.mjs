// Lists the readable-naming problems of the repository, one per line.
import { fileURLToPath } from 'node:url';
import { loadWordNet } from '../scripts/english-vocabulary.mjs';
import { checkRepositoryNames } from '../scripts/issue-195-naming.mjs';

const root = fileURLToPath(new URL('../../', import.meta.url));
const { problems } = checkRepositoryNames(root, loadWordNet());
for (const problem of problems) console.log(`${problem.kind}\t${problem.message ?? JSON.stringify(problem)}`);
