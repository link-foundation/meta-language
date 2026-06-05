/// Many-valued semantic truth value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TruthValue {
    True,
    False,
    Unknown,
    Both,
}

impl TruthValue {
    /// Logical conjunction.
    #[must_use]
    pub const fn and(self, other: Self) -> Self {
        match (self, other) {
            (Self::False, _) | (_, Self::False) => Self::False,
            (Self::True, value) | (value, Self::True) => value,
            (Self::Both, _) | (_, Self::Both) => Self::Both,
            (Self::Unknown, Self::Unknown) => Self::Unknown,
        }
    }

    /// Logical disjunction.
    #[must_use]
    pub const fn or(self, other: Self) -> Self {
        match (self, other) {
            (Self::True, _) | (_, Self::True) => Self::True,
            (Self::False, value) | (value, Self::False) => value,
            (Self::Both, _) | (_, Self::Both) => Self::Both,
            (Self::Unknown, Self::Unknown) => Self::Unknown,
        }
    }

    /// Logical negation.
    #[must_use]
    pub const fn negate(self) -> Self {
        match self {
            Self::True => Self::False,
            Self::False => Self::True,
            Self::Unknown => Self::Unknown,
            Self::Both => Self::Both,
        }
    }
}
