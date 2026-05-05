//! Windows service to NutOS service translation declarations.

/// Windows Service Control Manager operations recognized by WinBridge.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ServiceOperation {
    /// Register a service.
    Install,
    /// Start a service.
    Start,
    /// Stop a service.
    Stop,
    /// Query service status.
    QueryStatus,
}

/// NutOS service operation selected for a Windows service request.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NutServiceEquivalent {
    /// Create or update a service unit.
    RegisterUnit,
    /// Activate a service unit.
    ActivateUnit,
    /// Deactivate a service unit.
    DeactivateUnit,
    /// Read service state.
    InspectUnit,
}

/// Planned Windows service translation route.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ServiceTranslation {
    /// Source Service Control Manager operation.
    pub operation: ServiceOperation,
    /// Target NutOS service operation.
    pub target: NutServiceEquivalent,
}

impl ServiceTranslation {
    /// Return the NutOS service equivalent for a Windows service operation.
    #[must_use]
    pub const fn for_operation(operation: ServiceOperation) -> Self {
        let target = match operation {
            ServiceOperation::Install => NutServiceEquivalent::RegisterUnit,
            ServiceOperation::Start => NutServiceEquivalent::ActivateUnit,
            ServiceOperation::Stop => NutServiceEquivalent::DeactivateUnit,
            ServiceOperation::QueryStatus => NutServiceEquivalent::InspectUnit,
        };

        Self { operation, target }
    }
}
