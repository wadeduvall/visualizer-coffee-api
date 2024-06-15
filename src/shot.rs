use chrono::{DateTime, Local, Utc};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct ShotData {
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
    pub id: String,
    pub profile_title: String,
    pub user_id: String,
    #[serde_as(as = "DisplayFromStr")]
    pub drink_tds: f32,
    #[serde_as(as = "DisplayFromStr")]
    pub drink_ey: f32,
    pub espresso_enjoyment: u8,
    #[serde_as(as = "DisplayFromStr")]
    pub bean_weight: f32,
    #[serde_as(as = "DisplayFromStr")]
    pub drink_weight: f32,
    pub grinder_model: Option<String>,
    #[serde_as(as = "DisplayFromStr")]
    pub grinder_setting: f32,
    pub bean_brand: Option<String>,
    pub bean_type: Option<String>,
    pub roast_date: Option<DateTime<Local>>,
    pub espresso_notes: Option<String>,
    pub roast_level: Option<String>,
    pub bean_notes: Option<String>,
    pub start_time: DateTime<Local>,
    pub metadata: Option<String>,
    #[serde_as(as = "Option<Vec<DisplayFromStr>>")]
    pub timeframe: Option<Vec<f32>>,
    pub data: Option<ShotData>,
    pub duration: f32,
    pub image_preview: String,
    pub profile_url: String,
}

#[serde_as]
#[derive(Deserialize, Debug)]
pub struct Item {
    #[serde(with = "serde_with::chrono_0_4::datetime_utc_ts_seconds_from_any")]
    pub clock: DateTime<Utc>,
    pub id: String,
}

#[derive(Deserialize, Debug)]
pub struct ShotList {
    pub data: Vec<Item>,
}
