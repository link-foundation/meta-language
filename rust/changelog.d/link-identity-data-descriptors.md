JavaScript link identity instances now require an own Number data property.
Accessor-backed and inherited representations are rejected without executing
their getters, preventing lookup, equality, insertion and archive writing from
observing different identities. Ordinary writable data identities and original
exact decimal or safe BigInt constructor inputs keep their behavior.
