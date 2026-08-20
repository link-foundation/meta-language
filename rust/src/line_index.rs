use crate::Point;

/// Pre-computed line boundaries of a source text.
///
/// Parsers resolve a byte offset to a `(row, column)` point for every node and
/// every token they emit. Scanning the source from byte zero on each of those
/// lookups makes a parse `O(nodes * bytes)`, which is quadratic in file size and
/// left a 39 KB module parsing for over ten seconds (issue #193). Building this
/// table costs `O(bytes)` once per parse and turns every later lookup into a
/// binary search, so the parse stays `O(nodes * log lines)`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LineIndex {
    /// Total length of the indexed text in bytes.
    length: usize,
    /// Byte offset of the first byte of each line, ascending. `starts[0]` is
    /// always `0`, so the table is never empty and every search finds a line.
    starts: Vec<usize>,
    /// Byte offsets of UTF-8 continuation bytes, ascending. Empty for ASCII
    /// text, where a byte column already equals a character column.
    continuations: Vec<usize>,
}

impl LineIndex {
    /// Indexes `text` in a single pass.
    pub fn new(text: &str) -> Self {
        let mut starts = vec![0];
        starts.extend(
            text.bytes()
                .enumerate()
                .filter(|(_, value)| *value == b'\n')
                .map(|(index, _)| index + 1),
        );
        let continuations = if text.is_ascii() {
            Vec::new()
        } else {
            text.bytes()
                .enumerate()
                .filter(|(_, value)| is_continuation(*value))
                .map(|(index, _)| index)
                .collect()
        };

        Self {
            length: text.len(),
            starts,
            continuations,
        }
    }

    /// Point whose column counts the bytes between the line start and `byte`.
    ///
    /// Offsets past the end of the text keep counting from the last line start,
    /// which matches how tree-sitter reports positions inside synthetic
    /// suffixes appended before parsing.
    pub fn byte_point(&self, byte: usize) -> Point {
        let (row, line_start) = self.row_and_line_start(byte);
        Point::new(row, byte - line_start)
    }

    /// Point whose column counts the characters between the line start and
    /// `byte`. Offsets past the end of the text resolve to the final point.
    pub fn char_point(&self, byte: usize) -> Point {
        let byte = byte.min(self.length);
        let (row, line_start) = self.row_and_line_start(byte);
        let column = (byte - line_start) - self.continuations_within(line_start, byte);
        Point::new(row, column)
    }

    /// Row holding `byte` together with the byte offset that row starts at.
    fn row_and_line_start(&self, byte: usize) -> (usize, usize) {
        // Index of the last line start at or before `byte`; `starts[0] == 0`
        // keeps the subtraction in range.
        let row = self.starts.partition_point(|start| *start <= byte) - 1;
        (row, self.starts[row])
    }

    /// Number of UTF-8 continuation bytes in `start..end`.
    fn continuations_within(&self, start: usize, end: usize) -> usize {
        if self.continuations.is_empty() {
            return 0;
        }
        self.continuations.partition_point(|offset| *offset < end)
            - self.continuations.partition_point(|offset| *offset < start)
    }
}

/// Returns `true` for bytes that continue a multi-byte UTF-8 character, which
/// are exactly the bytes that carry no character of their own.
const fn is_continuation(byte: u8) -> bool {
    byte & 0b1100_0000 == 0b1000_0000
}
