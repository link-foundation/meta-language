import {readOrderedRecordSourceProfiles} from './ordered-record-schema-profile.mjs';
const result=await readOrderedRecordSourceProfiles();
process.stdout.write('Ordered record schema source profiles: '+result.legacy.fields.length+' historical and '+result.current.fields.length+' active native facets.\n');
