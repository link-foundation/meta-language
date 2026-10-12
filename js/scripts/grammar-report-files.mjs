// Persist authoring reports as structured Links data; external tool protocols
// retain their own JSON wire formats.
import { writeProjectData } from './project-data.mjs';

export function writeGrammarReport(file, report) {
  return writeProjectData(file, report, { format: 'lino' });
}
