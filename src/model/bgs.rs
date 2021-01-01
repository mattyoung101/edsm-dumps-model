use serde::{Deserialize, Serialize};
use strum::EnumIter;

use super::util::DisplayViaSerde;
use crate::display_via_serde;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ActiveState {
    pub state: State,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter)]
#[serde(deny_unknown_fields)]
pub enum Allegiance {
    Alliance,
    Empire,
    Federation,
    Independent,
    #[serde(rename = "Pilots Federation")]
    PilotsFederation,
}

display_via_serde!(Allegiance);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ControllingFaction {
    pub id: Option<u64>,
    // Attributes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allegiance: Option<Allegiance>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub government: Option<Government>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_player: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter)]
#[serde(deny_unknown_fields)]
pub enum Economy {
    None,
    Agriculture,
    Colony,
    Damaged,
    Extraction,
    #[serde(rename = "Fleet Carrier")]
    FleetCarrier,
    #[serde(rename = "High Tech")]
    HighTech,
    Industrial,
    Military,
    Prison,
    Refinery,
    Repair,
    Rescue,
    Service,
    Terraforming,
    Tourism,
}

display_via_serde!(Economy);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter)]
#[serde(deny_unknown_fields)]
pub enum Government {
    None,
    Anarchy,
    Communism,
    Confederacy,
    Cooperative,
    Corporate,
    Democracy,
    Dictatorship,
    Feudal,
    Patronage,
    Prison,
    #[serde(rename = "Prison colony")]
    PrisonColony,
    Theocracy,
    #[serde(rename = "Workshop (Engineer)")]
    WorkshopEngineer,
    #[serde(rename = "Fleet Carrier")]
    FleetCarrier,
}

display_via_serde!(Government);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, EnumIter)]
#[serde(deny_unknown_fields)]
pub enum Happiness {
    Despondent,
    Unhappy,
    Discontented,
    None,
    Happy,
    Elated,
}

display_via_serde!(Happiness);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct PendingState {
    pub state: State,
    pub trend: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct RecoveringState {
    pub state: State,
    pub trend: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, EnumIter)]
#[serde(deny_unknown_fields)]
pub enum Security {
    Anarchy,
    Low,
    Medium,
    High,
}

display_via_serde!(Security);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter)]
#[serde(deny_unknown_fields)]
pub enum State {
    Blight,
    Boom,
    Bust,
    #[serde(rename = "Civil liberty")]
    CivilLiberty,
    #[serde(rename = "Civil unrest")]
    CivilUnrest,
    #[serde(rename = "Civil war")]
    CivilWar,
    Drought,
    Election,
    Expansion,
    Famine,
    #[serde(rename = "Infrastructure Failure")]
    InfrastructureFailure,
    Investment,
    Lockdown,
    #[serde(rename = "Natural Disaster")]
    NaturalDisaster,
    None,
    Outbreak,
    #[serde(rename = "Pirate attack")]
    PirateAttack,
    #[serde(rename = "Public Holiday")]
    PublicHoliday,
    Retreat,
    #[serde(rename = "Terrorist Attack")]
    TerroristAttack,
    War,
}

display_via_serde!(State);
