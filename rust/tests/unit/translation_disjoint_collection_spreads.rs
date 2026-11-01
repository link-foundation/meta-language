//! Every independent eager-spread input retains the complete future CI matrix.
#[test]
fn actual_eager_collection_spreads_retain_all_source_and_native_cells(){
 super::translation_collection_copies::assert_collection_fixture(
  "../js/scripts/produce-disjoint-collection-spreads.mjs",
  "closed eager collection spread controls 1",
 );
}
