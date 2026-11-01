//! Independent constructor/default/own-field literals retain every CI cell.
#[test]
fn actual_closed_class_collection_defaults_retain_all_source_and_native_cells(){
 super::translation_collection_copies::assert_collection_fixture(
  "../js/scripts/produce-closed-class-collection-defaults.mjs",
  "closed class collection defaults controls 1",
 );
}
