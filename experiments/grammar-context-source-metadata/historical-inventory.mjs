// Retained inverse-model fixtures belong to their explicit original source profile.
import assert from 'node:assert/strict';
import {checkOutcomeInventory} from '../grammar-outcome-network/native-transport/wire/generate.mjs';

/** Validate the entire actual current profile before selecting its complete
 * frozen predecessor. Current wire publication continues to use schema two. */
export function projectHistoricalOutcomeInventory(inventory) {
  checkOutcomeInventory(inventory);
  const historical=structuredClone(inventory);
  if(historical.sourceViews.schemaVersion===2){
    assert.equal(historical.sourceViews.profile,'complete current grammar context source views 1');
    assert.equal(historical.sourceViews.predecessorView.schemaVersion,1);
    historical.sourceViews=historical.sourceViews.predecessorView;
  }
  checkOutcomeInventory(historical);
  return historical;
}
