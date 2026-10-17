// Persist authoring reports as structured Links data; external tool protocols
// retain their own JSON wire formats.
import { readFactoredGrammarReport, writeFactoredGrammarReport } from './grammar-report-fragments.mjs';

export function writeGrammarReport(file, report, options) {
  return writeFactoredGrammarReport(file, report, options);
}

export const readGrammarReport = readFactoredGrammarReport;
