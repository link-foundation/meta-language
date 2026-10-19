Partition native CI test execution into deterministic physical shards while
retaining the original logical suites, platforms, filters, and deadlines.
Authenticate each shard's complete inventory and output, and verify that their
union executes every original test exactly once before acceptance proceeds.
