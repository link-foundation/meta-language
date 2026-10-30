// Deferred tool-boundary inputs. No Git operation occurs in these controls.
import test from 'node:test';
import assert from 'node:assert/strict';
import {acceptSourceLibraryFilePath} from '../scripts/stage-source-library-files.mjs';
test('complete source recipes admit extracted helper modules within their own root',()=>{
 for(const path of ['library.rs','library/part_1.rs','library/ml_big_integer_conversion.rs','library/ml_big_integer_conversion/part_5.rs'])assert.equal(acceptSourceLibraryFilePath('library',path),true);
});
test('complete source recipes cannot escape into other modules or arbitrary paths',()=>{
 for(const path of ['other.rs','other/part_1.rs','library_extra/part_1.rs','library/../other.rs','library//part_1.rs','library/./part_1.rs','/library/part_1.rs','library/part_1.rs/extra','library/a\\b.rs','library/part_1.json','library/.git/config.rs','library/'+Array(9).fill('nested').join('/')+'/part_1.rs'])assert.equal(acceptSourceLibraryFilePath('library',path),false);
 for(const stem of ['',null,'../library','library/name'])assert.equal(acceptSourceLibraryFilePath(stem,'library.rs'),false);
});
