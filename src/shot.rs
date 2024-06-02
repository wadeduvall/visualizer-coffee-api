use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    #[serde_as(as = "Vec<DisplayFromStr>")]
    espresso_flow: Vec<f32>,
    #[serde_as(as = "Vec<DisplayFromStr>")]
    espresso_weight: Vec<f32>,
    #[serde_as(as = "Vec<DisplayFromStr>")]
    espresso_pressure: Vec<f32>,
    #[serde_as(as = "Vec<DisplayFromStr>")]
    espresso_flow_goal: Vec<f32>,
    #[serde_as(as = "Vec<DisplayFromStr>")]
    espresso_flow_weight: Vec<f32>,
    // FIXME: Can this be Vec<i32>?
    #[serde_as(as = "Vec<DisplayFromStr>")]
    espresso_state_change: Vec<f32>,
    #[serde_as(as = "Vec<DisplayFromStr>")]
    espresso_pressure_goal: Vec<f32>,
    #[serde_as(as = "Vec<DisplayFromStr>")]
    espresso_flow_weight_raw: Vec<f32>,
    #[serde_as(as = "Vec<DisplayFromStr>")]
    espresso_temperature_mix: Vec<f32>,
    #[serde_as(as = "Vec<DisplayFromStr>")]
    espresso_water_dispensed: Vec<f32>,
    #[serde_as(as = "Vec<DisplayFromStr>")]
    espresso_temperature_goal: Vec<f32>,
    #[serde_as(as = "Vec<DisplayFromStr>")]
    espresso_temperature_basket: Vec<f32>,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct Shot {
    id: String,
    profile_title: String,
    user_id: String,
    #[serde_as(as = "DisplayFromStr")]
    drink_tds: f32,
    #[serde_as(as = "DisplayFromStr")]
    drink_ey: f32,
    espresso_enjoyment: u8,
    #[serde_as(as = "DisplayFromStr")]
    bean_weight: f32,
    #[serde_as(as = "DisplayFromStr")]
    drink_weight: f32,
    grinder_model: Option<String>,
    #[serde_as(as = "DisplayFromStr")]
    grinder_setting: f32,
    bean_brand: Option<String>,
    bean_type: Option<String>,
    roast_date: Option<DateTime<Local>>,
    espresso_notes: Option<String>,
    roast_level: Option<String>,
    bean_notes: Option<String>,
    start_time: DateTime<Local>,
    metadata: Option<String>,
    #[serde_as(as = "Option<Vec<DisplayFromStr>>")]
    timeframe: Option<Vec<f32>>,
    data: Option<Data>,
    duration: f32,
    image_preview: String,
    profile_url: String,
}
