//! Debuff definition (stat or effect penalty)


#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, Default)]
pub struct Debuff {
    pub id: String,
    pub name: String,
    pub affected_stats: Option<Vec<String>>,
    pub value: Option<f32>,
    pub duration_ms: Option<u64>,
    pub description: Option<String>,
    pub stacking: Option<bool>,
    pub max_stacks: Option<u32>,
    pub source: Option<String>,
    pub dispel_type: Option<String>, // e.g. magic, curse, poison
    pub effect_type: Option<String>, // e.g. disadvantage, vulnerability, condition
}
