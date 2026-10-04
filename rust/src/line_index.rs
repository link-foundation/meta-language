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
    /// Byte offset of the first byte of each line, ascending. `starts[0]` is
    /// always `0`, so the table is never empty and every search finds a line.
    starts: Vec<usize>,
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
        Self { starts }
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

    /// Row holding `byte` together with the byte offset that row starts at.
    fn row_and_line_start(&self, byte: usize) -> (usize, usize) {
        // Index of the last line start at or before `byte`; `starts[0] == 0`
        // keeps the subtraction in range.
        let row = self.starts.partition_point(|start| *start <= byte) - 1;
        (row, self.starts[row])
    }
}
