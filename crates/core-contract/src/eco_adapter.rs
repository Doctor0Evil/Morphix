use std::fmt;

/// Minimal ecological context passed into all impact scorers.
/// This stays abstract but is shaped for STAC-like EO plus
/// biodiversity attributes.[web:148]
#[derive(Clone, Debug)]
pub struct EcoContext {
    /// STAC collection or dataset identifier (e.g., Sentinel‑2, GBIF dataset ID).
    pub dataset_id: String,
    /// Optional geo region, e.g., GeoHash or WKT; kept simple here.
    pub region_hint: Option<String>,
    /// Optional taxon key or ecological feature identifier.
    pub taxon_or_feature: Option<String>,
    /// Freeform metadata JSON as text (adapter backends parse this).
    pub raw_metadata: Option<String>,
}

/// A scalar impact score plus a short, human-readable reason string.
/// This mirrors “explainable scorer” patterns where score and explanation
/// are always paired.[file:71]
#[derive(Clone, Debug)]
pub struct ImpactScore {
    pub value: f32,        // typically in [0,1] after normalization
    pub explanation: String,
}

impl ImpactScore {
    pub fn clamped(value: f32, explanation: impl Into<String>) -> Self {
        let v = value.clamp(0.0, 1.0);
        Self {
            value: v,
            explanation: explanation.into(),
        }
    }
}

impl fmt::Display for ImpactScore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.3} – {}", self.value, self.explanation)
    }
}

/// Base trait: any ecological impact scorer must at least be able to
/// compute a numeric score from a context.
pub trait Scorer {
    type Context;

    fn score(&self, ctx: &Self::Context) -> ImpactScore;
}

/// Explainable scorer: always returns a machine- and human-readable
/// justification along with the numeric score.
pub trait ExplainableScorer: Scorer {}

/// Auditable scorer: in a full system this would emit provenance
/// events to a ledger; here we just require an ID for logging.[file:69]
pub trait AuditableScorer: ExplainableScorer {
    fn scorer_id(&self) -> &'static str;
}

/// Sealed pattern to keep external crates from implementing the
/// marker traits directly; they implement concrete structs instead.
mod sealed {
    pub trait Sealed {}
}
use sealed::Sealed;

/// Blanket impls binding the hierarchy together.
impl<T> ExplainableScorer for T where T: Scorer<Context = EcoContext> + Sealed {}

impl<T> AuditableScorer for T
where
    T: Scorer<Context = EcoContext> + Sealed,
{
    fn scorer_id(&self) -> &'static str {
        std::any::type_name::<T>()
    }
}

/// Adapter trait: wrap any external ecological API client (GBIF, STAC, etc.)
/// into a unified, type-safe EcoContext → ImpactScore interface.[web:131][web:148]
pub trait EcoImpactAdapter: Send + Sync {
    fn name(&self) -> &'static str;

    /// Compute an impact score for the given context.
    fn compute_impact(&self, ctx: &EcoContext) -> ImpactScore;
}

/// Main trait-object type used by AI-chat and orchestration code.
/// This is the bounded dynamic dispatch surface:
///   Box<dyn EcoImpactAdapter>
/// so you can hot-swap implementations at runtime without recompiling.[web:141]
pub type EcoImpactAdapterBox = Box<dyn EcoImpactAdapter>;
