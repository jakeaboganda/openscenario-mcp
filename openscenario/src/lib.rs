//! OpenSCENARIO library for scenario generation and validation

pub mod catalog;
pub mod entities;
pub mod error;
pub mod opendrive_validator;
pub mod parser;
pub mod position;
pub mod scenario;
pub mod storyboard;
pub mod validation;
pub mod version;
pub mod xml;

pub use catalog::{Catalog, CatalogEntry, CatalogType};
pub use entities::{Entity, MiscObjectParams, PedestrianParams, VehicleCategory, VehicleParams};
pub use error::{Result, ScenarioError};
pub use position::Position;
pub use scenario::{ParameterDeclaration, ParameterType, Scenario};
pub use storyboard::{
    Act, Action, ByEntityCondition, ByValueCondition, Condition, ConditionEdge, ConditionGroup,
    ConditionKind, EntityCondition, Event, LaneChangeAction, ParameterCondition, Rule, SpeedAction,
    SpeedCondition, Storyboard, TransitionShape, Trigger, TriggeringEntities,
    TriggeringEntitiesRule,
};
pub use validation::{ValidationReport, XsdValidator};
pub use version::OpenScenarioVersion;
