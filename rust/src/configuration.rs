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

/// Natural-language identification backend.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LanguageIdentificationDetector {
    /// Use `lingua` for language identification.
    Lingua,
    /// Use `whatlang` for language identification.
    Whatlang,
}

/// Configured amount of formal meaning to expose during reconstruction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormalizationLevel {
    /// Natural surface text in the requested target language.
    Natural,
    /// Predicate form using target-language labels where they are available.
    Lexical,
    /// Predicate form using shared concept identifiers.
    Concept,
    /// Link-like proposition form including the truth marker.
    Logical,
}

/// Direction for text/network conversion.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NaturalizationDirection {
    /// Prefer target-language natural text when reconstructing.
    Naturalize,
    /// Prefer the configured formal representation when reconstructing.
    Formalize,
}

/// Whether the engine produces a mutable network or an enforced read-only one.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AccessMode {
    /// The network is editable: mutators such as `insert_link` and
    /// `apply_substitution` are available on `&mut LinkNetwork`.
    #[default]
    Mutable,
    /// The network is frozen: only `&self` operations (query, project,
    /// reconstruct, verify, serialize) are reachable, and mutation attempts at
    /// the engine boundary fail with a clear diagnostic.
    ReadOnly,
}

impl AccessMode {
    /// Human-readable access-mode name.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Mutable => "mutable",
            Self::ReadOnly => "read-only",
        }
    }

    /// Whether this access mode permits mutation.
    #[must_use]
    pub const fn is_mutable(self) -> bool {
        matches!(self, Self::Mutable)
    }

    /// Whether this access mode forbids mutation.
    #[must_use]
    pub const fn is_read_only(self) -> bool {
        matches!(self, Self::ReadOnly)
    }
}

/// Configuration for parse-to-network operations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ParseConfiguration {
    trivia_attachment_policy: TriviaAttachmentPolicy,
    region_detection_policy: RegionDetectionPolicy,
    language_identification_detector: LanguageIdentificationDetector,
    formalization_level: FormalizationLevel,
    naturalization_direction: NaturalizationDirection,
    access_mode: AccessMode,
    profile: Option<&'static str>,
}

impl ParseConfiguration {
    /// Creates parse configuration with the supplied trivia policy.
    #[must_use]
    pub const fn new(trivia_attachment_policy: TriviaAttachmentPolicy) -> Self {
        Self {
            trivia_attachment_policy,
            region_detection_policy: RegionDetectionPolicy::Both,
            language_identification_detector: LanguageIdentificationDetector::Lingua,
            formalization_level: FormalizationLevel::Natural,
            naturalization_direction: NaturalizationDirection::Naturalize,
            access_mode: AccessMode::Mutable,
            profile: None,
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

    /// Returns configuration with a natural-language identification backend.
    #[must_use]
    pub const fn with_language_identification_detector(
        mut self,
        detector: LanguageIdentificationDetector,
    ) -> Self {
        self.language_identification_detector = detector;
        self
    }

    /// Returns configuration with a formalization detail level.
    #[must_use]
    pub const fn with_formalization_level(
        mut self,
        formalization_level: FormalizationLevel,
    ) -> Self {
        self.formalization_level = formalization_level;
        self
    }

    /// Returns configuration with a naturalization/formalization direction.
    #[must_use]
    pub const fn with_naturalization_direction(
        mut self,
        naturalization_direction: NaturalizationDirection,
    ) -> Self {
        self.naturalization_direction = naturalization_direction;
        self
    }

    /// Returns configuration with a read-only or mutable engine access mode.
    #[must_use]
    pub const fn with_access_mode(mut self, access_mode: AccessMode) -> Self {
        self.access_mode = access_mode;
        self
    }

    /// Returns configuration with a language capability profile name.
    ///
    /// Built-in profiles such as `"JavaScript"` are materialized as queryable
    /// links during parsing. User-declared profiles can be supplied directly to
    /// transform-time APIs.
    #[must_use]
    pub const fn with_profile(mut self, profile: &'static str) -> Self {
        self.profile = Some(profile);
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

    /// Natural-language identification backend.
    #[must_use]
    pub const fn language_identification_detector(self) -> LanguageIdentificationDetector {
        self.language_identification_detector
    }

    /// Formalization detail level.
    #[must_use]
    pub const fn formalization_level(self) -> FormalizationLevel {
        self.formalization_level
    }

    /// Naturalization/formalization direction.
    #[must_use]
    pub const fn naturalization_direction(self) -> NaturalizationDirection {
        self.naturalization_direction
    }

    /// Read-only or mutable engine access mode.
    #[must_use]
    pub const fn access_mode(self) -> AccessMode {
        self.access_mode
    }

    /// Configured language capability profile name.
    #[must_use]
    pub const fn profile(self) -> Option<&'static str> {
        self.profile
    }
}

impl Default for ParseConfiguration {
    fn default() -> Self {
        Self::new(TriviaAttachmentPolicy::Both)
    }
}
