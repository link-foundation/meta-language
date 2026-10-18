// PRIVATE host adapter template; the producer appends exact CST-derived visitors.
// This is typed storage transport, not semantic JavaScript self-translation.
use std::collections::{HashMap, HashSet};

use super::{
    Encoder, StructuredDataLinks, StructuredDataLinksLimits, exact_json_number, fail, write_number,
};
use crate::{LinkId, LinkNetwork, ParseOutcome};

fn next_depth(depth: usize, increment: usize) -> Result<usize, String> {
    depth
        .checked_add(increment)
        .ok_or_else(|| fail("the outcome exceeds its structural resource limit"))
}

struct CountArray {
    depth: usize,
    expected: usize,
    seen: usize,
}

struct CountSink {
    limits: StructuredDataLinksLimits,
    occurrences: usize,
    bytes: usize,
}

impl CountSink {
    fn occurrence(&mut self, depth: usize, bytes: usize) -> Result<(), String> {
        let occurrences = self.occurrences.checked_add(1)
            .ok_or_else(|| fail("the outcome exceeds its structural resource limit"))?;
        let total_bytes = self.bytes.checked_add(bytes)
            .ok_or_else(|| fail("the outcome exceeds its scalar byte resource limit"))?;
        if occurrences > self.limits.node_limit || depth > self.limits.depth_limit {
            return Err(fail("the outcome exceeds its structural resource limit"));
        }
        if total_bytes > self.limits.byte_limit {
            return Err(fail("the outcome exceeds its scalar byte resource limit"));
        }
        self.occurrences = occurrences;
        self.bytes = total_bytes;
        Ok(())
    }
}

impl OutcomeWireSink for CountSink {
    type Node = ();
    type Object = usize;
    type Array = CountArray;

    fn begin_object(&mut self, depth: usize) -> Result<usize, String> {
        self.occurrence(depth, 0)?;
        Ok(depth)
    }

    fn object_field<F>(&mut self, object: &mut usize, key: &str, visit: F) -> Result<(), String>
    where F: FnOnce(&mut Self, usize) -> Result<(), String> {
        let child_depth = next_depth(*object, 2)?;
        self.occurrence(next_depth(*object, 1)?, 0)?;
        self.string(key, child_depth)?;
        visit(self, child_depth)
    }

    fn end_object(&mut self, _object: usize) -> Result<(), String> { Ok(()) }

    fn begin_array(&mut self, count: usize, depth: usize) -> Result<CountArray, String> {
        self.occurrence(depth, 0)?;
        Ok(CountArray { depth, expected: count, seen: 0 })
    }

    fn array_item<F>(&mut self, array: &mut CountArray, visit: F) -> Result<(), String>
    where F: FnOnce(&mut Self, usize) -> Result<(), String> {
        if array.seen >= array.expected {
            return Err(fail("the outcome array changed during traversal"));
        }
        visit(self, next_depth(array.depth, 1)?)?;
        array.seen += 1;
        Ok(())
    }

    fn end_array(&mut self, array: CountArray) -> Result<(), String> {
        if array.seen != array.expected {
            return Err(fail("the outcome array changed during traversal"));
        }
        Ok(())
    }

    fn null(&mut self, depth: usize) -> Result<(), String> { self.occurrence(depth, 0) }

    fn boolean(&mut self, value: bool, depth: usize) -> Result<(), String> {
        self.occurrence(depth, if value { 4 } else { 5 })
    }

    fn string(&mut self, value: &str, depth: usize) -> Result<(), String> {
        self.occurrence(depth, value.len())
    }

    fn usize(&mut self, value: usize, depth: usize) -> Result<(), String> {
        let integer = u64::try_from(value).map_err(|_| fail("a JSON integer is not binary64"))?;
        self.json_number(&serde_json::Number::from(integer), depth)
    }

    fn i64(&mut self, value: i64, depth: usize) -> Result<(), String> {
        self.json_number(&serde_json::Number::from(value), depth)
    }

    fn json_number(&mut self, value: &serde_json::Number, depth: usize) -> Result<(), String> {
        // Existing formatter's binary64 scratch is bounded constant work, not
        // an input-sized output tree. No replacement numerical algorithm.
        let spelling = write_number(exact_json_number(value)?)?;
        self.occurrence(depth, spelling.len())
    }
}

struct DirectObject {
    id: LinkId,
    depth: usize,
    references: Vec<LinkId>,
}

struct DirectArray {
    object: DirectObject,
    expected: usize,
}

struct DirectSink {
    encoder: Encoder,
}

impl DirectSink {
    fn new(limits: StructuredDataLinksLimits) -> Self {
        Self { encoder: Encoder {
            network: LinkNetwork::new(), limits, bytes: 0,
            scalars: HashMap::new(), container_aliases: HashMap::new(),
            containers: HashMap::new(), consumed_aliases: HashSet::new(), heights: HashMap::new(),
        } }
    }

    fn reserve(references: &mut Vec<LinkId>, count: usize) -> Result<(), String> {
        references.try_reserve(count)
            .map_err(|_| fail("unable to reserve the bounded outcome references"))
    }

    fn finish(&mut self, object: DirectObject) -> LinkId {
        self.encoder.network.set_references(object.id, &object.references);
        object.id
    }
}

impl OutcomeWireSink for DirectSink {
    type Node = LinkId;
    type Object = DirectObject;
    type Array = DirectArray;

    fn begin_object(&mut self, depth: usize) -> Result<DirectObject, String> {
        let id = self.encoder.node("object value", None, &[], depth)?;
        Ok(DirectObject { id, depth, references: Vec::new() })
    }

    fn object_field<F>(&mut self, object: &mut DirectObject, key: &str, visit: F) -> Result<(), String>
    where F: FnOnce(&mut Self, usize) -> Result<LinkId, String> {
        Self::reserve(&mut object.references, 1)?;
        let child_depth = next_depth(object.depth, 2)?;
        let key = self.encoder.scalar("string value", Some(key), child_depth)?;
        let value = visit(self, child_depth)?;
        let pair = self.encoder.node("key value pair", None, &[key, value], next_depth(object.depth, 1)?)?;
        object.references.push(pair);
        Ok(())
    }

    fn end_object(&mut self, object: DirectObject) -> Result<LinkId, String> {
        Ok(self.finish(object))
    }

    fn begin_array(&mut self, count: usize, depth: usize) -> Result<DirectArray, String> {
        let id = self.encoder.node("array value", None, &[], depth)?;
        let mut references = Vec::new();
        Self::reserve(&mut references, count)?;
        Ok(DirectArray { object: DirectObject { id, depth, references }, expected: count })
    }

    fn array_item<F>(&mut self, array: &mut DirectArray, visit: F) -> Result<(), String>
    where F: FnOnce(&mut Self, usize) -> Result<LinkId, String> {
        if array.object.references.len() >= array.expected {
            return Err(fail("the outcome array changed during traversal"));
        }
        Self::reserve(&mut array.object.references, 1)?;
        let value = visit(self, next_depth(array.object.depth, 1)?)?;
        array.object.references.push(value);
        Ok(())
    }

    fn end_array(&mut self, array: DirectArray) -> Result<LinkId, String> {
        if array.object.references.len() != array.expected {
            return Err(fail("the outcome array changed during traversal"));
        }
        Ok(self.finish(array.object))
    }

    fn null(&mut self, depth: usize) -> Result<LinkId, String> {
        self.encoder.scalar("null value", None, depth)
    }

    fn boolean(&mut self, value: bool, depth: usize) -> Result<LinkId, String> {
        self.encoder.scalar("boolean value", Some(if value { "true" } else { "false" }), depth)
    }

    fn string(&mut self, value: &str, depth: usize) -> Result<LinkId, String> {
        self.encoder.scalar("string value", Some(value), depth)
    }

    fn usize(&mut self, value: usize, depth: usize) -> Result<LinkId, String> {
        let integer = u64::try_from(value).map_err(|_| fail("a JSON integer is not binary64"))?;
        self.json_number(&serde_json::Number::from(integer), depth)
    }

    fn i64(&mut self, value: i64, depth: usize) -> Result<LinkId, String> {
        self.json_number(&serde_json::Number::from(value), depth)
    }

    fn json_number(&mut self, value: &serde_json::Number, depth: usize) -> Result<LinkId, String> {
        self.encoder.scalar("number value", Some(&write_number(exact_json_number(value)?)?), depth)
    }
}

/// Encode a complete borrowed parse outcome into a fresh inspectable graph.
/// This finite transport profile refuses depths above 256. Its node and byte
/// limits bound expanded occurrences, independently of historical codec APIs.
pub fn encode_grammar_parse_outcome_links(
    outcome: &ParseOutcome,
    limits: StructuredDataLinksLimits,
) -> Result<StructuredDataLinks, String> {
    if limits.node_limit == 0 || limits.depth_limit == 0 || limits.byte_limit == 0 {
        return Err(fail("resource limits must be positive"));
    }
    if limits.depth_limit > 256 {
        return Err(fail("the borrowed outcome profile requires a depth limit at most 256"));
    }
    let mut count = CountSink { limits, occurrences: 0, bytes: 0 };
    wire_parse_outcome(outcome, &mut count, 1)?;
    // Count the schema root LAST as in the ordered plan. The whole counting
    // pass still finishes before any input-sized output allocation.
    count.occurrence(0, 1)?;
    let mut sink = DirectSink::new(limits);
    let body = wire_parse_outcome(outcome, &mut sink, 1)?;
    let root = sink.encoder.node("structured data", Some("1"), &[body], 0)?;
    Ok(StructuredDataLinks { network: sink.encoder.network, root })
}
