use serde::Serialize;

#[derive(Debug, Clone)]
pub struct ControlInput {
    pub aileron: f32,
    pub elevator: f32,
    pub throttle: f32,
    pub rudder: f32,
}

#[derive(Debug, Clone, Serialize)]
pub struct CsvRow {
    #[serde(skip_serializing)]
    pub timestamp_ms: u64,
    pub time_s: f32,
    pub lat: f32,
    pub lng: f32,
    pub alt_m: f32,
    pub distance_to_home_m: f32,
    pub height_m: f32,
    pub speed_ms: f32,
    pub velocity_x_ms: f32,
    pub velocity_y_ms: f32,
    pub velocity_z_ms: f32,
    pub battery_percent: u8,
    pub battery_voltage_v: f32,
    pub battery_temp_c: Option<f32>,
    pub cell_voltages: String,
    pub satellites: u8,
    pub rc_signal: u8,
    pub pitch_deg: f32,
    pub roll_deg: f32,
    pub yaw_deg: f32,
    pub gimbal_pitch_deg: f32,
    pub gimbal_roll_deg: f32,
    pub gimbal_yaw_deg: f32,
    pub rc_aileron: f32,
    pub rc_elevator: f32,
    pub rc_throttle: f32,
    pub rc_rudder: f32,
    pub is_photo: bool,
    pub is_video: bool,
    pub flight_mode: Option<String>,
    pub battery_full_capacity_mah: u32,
    pub battery_remained_capacity_mah: f32,
    pub messages: Option<String>,
    pub metadata: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Event {
    Full(CsvRow),
    Photo { timestamp_ms: u64 },
}

#[derive(Debug, Clone)]
pub struct VideoSpan {
    pub start_ms: u64,
    pub end_ms: u64,
}

#[derive(Serialize)]
pub struct Message {
    pub timestamp_ms: u32,
    #[serde(rename = "type")]
    pub msg_type: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct Metadata {
    pub display_name: String,
    pub drone_model: String,
    pub drone_serial: Option<String>,
    pub battery_serial: Option<String>,
    pub start_time: String,
    pub cycle_count: u32,
    pub duration_secs: u32,
    pub home_lat: f32,
    pub home_lon: f32,
}
