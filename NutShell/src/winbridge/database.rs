//! WinBridge compatibility database metadata.

/// Compatibility rating tiers for Windows applications in The Cache.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CompatibilityRating {
    /// Runs out of the box with excellent behavior.
    Excellent,
    /// Runs well with minor issues.
    Good,
    /// Runs with known workarounds.
    WorkaroundRequired,
    /// Unsupported or not yet validated.
    Unsupported,
}

/// Source of a compatibility database entry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EntrySource {
    /// Maintained by Rodent, Inc. compatibility engineers.
    Official,
    /// Crowdsourced from the NutOS community.
    Community,
}

/// Skeleton metadata stored for a WinBridge-compatible app.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CompatibilityEntry {
    /// Stable application identifier used by The Cache.
    pub app_id: &'static str,
    /// Compatibility rating for the application.
    pub rating: CompatibilityRating,
    /// Whether app-specific fixes are available.
    pub has_workarounds: bool,
    /// Entry provenance.
    pub source: EntrySource,
}

impl CompatibilityEntry {
    /// Create a community entry with no app-specific workaround metadata yet.
    #[must_use]
    pub const fn community_unvalidated(app_id: &'static str) -> Self {
        Self {
            app_id,
            rating: CompatibilityRating::Unsupported,
            has_workarounds: false,
            source: EntrySource::Community,
        }
    }
}
