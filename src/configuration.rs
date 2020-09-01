use serde::Deserialize;

#[derive(Deserialize)]
pub struct Configuration {
    // pub max_halite: u32,
    // pub ship_cost: u32,
    // pub dropoff_cost: u32,
    #[serde(rename = "MAX_TURNS")]
    pub max_turns: u32,
    // pub extract_ratio: u32,
    // pub move_cost_ratio: u32,
    // pub inspiration_enabled: bool,
    // pub inspiration_radius: u32,
    // pub inspiration_ship_count: u32,
    // pub inspired_extract_ratio: u32,
    // pub inspired_bonus_multiplier: f64,
    // pub inspired_move_cost_ratio: u32,
}
