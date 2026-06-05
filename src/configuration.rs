/// Trivia attachment strategy.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TriviaAttachmentPolicy {
    /// Attach trivia to the containing syntax link.
    ContainmentLink,
    /// Attach trivia to the token link.
    TokenLink,
    /// Emit both attachment links when they can coexist.
    Both,
}

/// Region detection strategy for mixed-language parsing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RegionDetectionPolicy {
    /// Use explicit region names such as fenced-code language tags.
    NameDriven,
    /// Use content sniffing.
    ContentDriven,
    /// Use name-driven detection first and content-driven detection as a fallback.
    Both,
}

/// Configuration for parse-to-network operations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ParseConfiguration {
    trivia_attachment_policy: TriviaAttachmentPolicy,
    region_detection_policy: RegionDetectionPolicy,
}

impl ParseConfiguration {
    /// Creates parse configuration with the supplied trivia policy.
    #[must_use]
    pub const fn new(trivia_attachment_policy: TriviaAttachmentPolicy) -> Self {
        Self {
            trivia_attachment_policy,
            region_detection_policy: RegionDetectionPolicy::Both,
        }
    }

    /// Returns configuration with a mixed-language region detection policy.
    #[must_use]
    pub const fn with_region_detection_policy(
        mut self,
        region_detection_policy: RegionDetectionPolicy,
    ) -> Self {
        self.region_detection_policy = region_detection_policy;
        self
    }

    /// Trivia attachment policy.
    #[must_use]
    pub const fn trivia_attachment_policy(self) -> TriviaAttachmentPolicy {
        self.trivia_attachment_policy
    }

    /// Mixed-language region detection policy.
    #[must_use]
    pub const fn region_detection_policy(self) -> RegionDetectionPolicy {
        self.region_detection_policy
    }
}

impl Default for ParseConfiguration {
    fn default() -> Self {
        Self::new(TriviaAttachmentPolicy::Both)
    }
}
