/// Tree-sitter-compatible parse status flags modeled as link metadata.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LinkFlags {
    bits: u8,
}

impl LinkFlags {
    const IS_ERROR: u8 = 0b0001;
    const HAS_ERROR: u8 = 0b0010;
    const IS_MISSING: u8 = 0b0100;
    const IS_EXTRA: u8 = 0b1000;

    /// Clean link flags.
    #[must_use]
    pub const fn clean() -> Self {
        Self { bits: 0 }
    }

    /// Flags for an error link.
    #[must_use]
    pub const fn error() -> Self {
        Self {
            bits: Self::IS_ERROR,
        }
    }

    /// Flags for a link that contains an error below it.
    #[must_use]
    pub const fn containing_error() -> Self {
        Self {
            bits: Self::HAS_ERROR,
        }
    }

    /// Flags for a missing link.
    #[must_use]
    pub const fn missing() -> Self {
        Self {
            bits: Self::IS_MISSING,
        }
    }

    /// Flags for an extra/trivia link.
    #[must_use]
    pub const fn extra() -> Self {
        Self {
            bits: Self::IS_EXTRA,
        }
    }

    /// Returns flags with the error bit enabled.
    #[must_use]
    pub const fn with_error(mut self) -> Self {
        self.bits |= Self::IS_ERROR;
        self
    }

    /// Returns flags with the containing-error bit enabled.
    #[must_use]
    pub const fn with_containing_error(mut self) -> Self {
        self.bits |= Self::HAS_ERROR;
        self
    }

    /// Returns flags with the missing bit enabled.
    #[must_use]
    pub const fn with_missing(mut self) -> Self {
        self.bits |= Self::IS_MISSING;
        self
    }

    /// Returns flags with the extra/trivia bit enabled.
    #[must_use]
    pub const fn with_extra(mut self) -> Self {
        self.bits |= Self::IS_EXTRA;
        self
    }

    /// Whether this link is an error link.
    #[must_use]
    pub const fn is_error(self) -> bool {
        self.bits & Self::IS_ERROR != 0
    }

    /// Whether this link contains an error below it.
    #[must_use]
    pub const fn has_error(self) -> bool {
        self.bits & Self::HAS_ERROR != 0
    }

    /// Whether this link is missing from the source text.
    #[must_use]
    pub const fn is_missing(self) -> bool {
        self.bits & Self::IS_MISSING != 0
    }

    /// Whether this link is extra source trivia.
    #[must_use]
    pub const fn is_extra(self) -> bool {
        self.bits & Self::IS_EXTRA != 0
    }
}
