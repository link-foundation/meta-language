/// Half-open byte range in source text.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ByteRange {
    start: usize,
    end: usize,
}

impl ByteRange {
    /// Creates a byte range.
    ///
    /// # Panics
    ///
    /// Panics when `start` is greater than `end`.
    #[must_use]
    pub const fn new(start: usize, end: usize) -> Self {
        assert!(start <= end, "byte range start must not exceed end");
        Self { start, end }
    }

    /// First byte in the range.
    #[must_use]
    pub const fn start(self) -> usize {
        self.start
    }

    /// Byte immediately after the range.
    #[must_use]
    pub const fn end(self) -> usize {
        self.end
    }

    /// Returns `true` when this range intersects `other`.
    #[must_use]
    pub const fn intersects(self, other: Self) -> bool {
        if self.start == self.end || other.start == other.end {
            self.start <= other.end && other.start <= self.end
        } else {
            self.start < other.end && other.start < self.end
        }
    }
}

/// Row and column point in source text.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Point {
    row: usize,
    column: usize,
}

impl Point {
    /// Creates a row/column point.
    #[must_use]
    pub const fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }

    /// Zero-based row.
    #[must_use]
    pub const fn row(self) -> usize {
        self.row
    }

    /// Zero-based column.
    #[must_use]
    pub const fn column(self) -> usize {
        self.column
    }
}

/// Source span attached to a link.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SourceSpan {
    byte_range: ByteRange,
    start_point: Point,
    end_point: Point,
}

impl SourceSpan {
    /// Creates a source span from byte and point ranges.
    #[must_use]
    pub const fn new(byte_range: ByteRange, start_point: Point, end_point: Point) -> Self {
        Self {
            byte_range,
            start_point,
            end_point,
        }
    }

    /// Byte range covered by the span.
    #[must_use]
    pub const fn byte_range(self) -> ByteRange {
        self.byte_range
    }

    /// Start row/column point.
    #[must_use]
    pub const fn start_point(self) -> Point {
        self.start_point
    }

    /// End row/column point.
    #[must_use]
    pub const fn end_point(self) -> Point {
        self.end_point
    }
}
