mod autel_drone_log;

use std::{env, fs, path::Path};
use kaitai::{BytesReader, KStruct, OptRc};
use serde::Serialize;
use chrono::{Utc, TimeZone, FixedOffset};
use convert_case::{Case, Casing};

use crate::autel_drone_log::{AutelDroneLog, AutelDroneLog_FullRecordT, AutelDroneLog_RecordT_Body};

struct ControlInput {
    aileron: f32,
    elevator: f32,
    throttle: f32,
    rudder: f32,
}

#[derive(Debug, Clone, Serialize)]
struct CsvRow {
    #[serde(skip_serializing)]
    timestamp_ms: u64,
    time_s: f32,
    lat: f32,
    lng: f32,
    alt_m: f32,
    distance_to_home_m: f32,
    height_m: f32,
    speed_ms: f32,
    velocity_x_ms: f32,
    velocity_y_ms: f32,
    velocity_z_ms: f32,
    battery_percent: u8,
    battery_voltage_v: f32,
    battery_temp_c: Option<f32>,
    cell_voltages: String,
    satellites: u8,
    rc_signal: u8,
    pitch_deg: f32,
    roll_deg: f32,
    yaw_deg: f32,
    gimbal_pitch_deg: f32,
    gimbal_roll_deg: f32,
    gimbal_yaw_deg: f32,
    rc_aileron: f32,
    rc_elevator: f32,
    rc_throttle: f32,
    rc_rudder: f32,
    is_photo: bool,
    is_video: bool,
    flight_mode: Option<String>,
    battery_full_capacity_mah: u32,
    battery_remained_capacity_mah: f32,
    messages: Option<String>,
    metadata: Option<String>
}

#[derive(Debug, Clone)]
enum Event {
    Full(CsvRow),
    Photo {
        timestamp_ms: u64
    },
}

struct VideoSpan {
    start_ms: u64,
    end_ms: u64,
}

#[derive(Serialize)]
struct Message {
    timestamp_ms: u32,
    #[serde(rename = "type")]
    msg_type: String,
    message: String,
}

#[derive(Serialize)]
struct Metadata {
    display_name: String,
    drone_model: String,
    drone_serial: Option<String>,
    battery_serial: Option<String>,
    start_time: String,
    cycle_count: u32,
    duration_secs: u32,
    home_lat: f32,
    home_lon: f32,
}

fn rc_input_to_control(left_x: u16, left_y: u16, right_x: u16, right_y: u16) -> ControlInput {
    // Assumes mode 2. I can't find a way to get the mode from the log.
    let rudder = ((left_x as i16 - 1024) as f32 / 1024.0) * 100.0;
    let elevator = ((left_y as i16 - 1024) as f32 / 1024.0) * 100.0;
    let aileron = -((right_x as i16 - 1024) as f32 / 1024.0) * 100.0;
    let throttle = -((right_y as i16 - 1024) as f32 / 1024.0) * 100.0;

    ControlInput {
        aileron,
        elevator,
        throttle,
        rudder,
    }
}

fn lerp_f32(a: f32, b: f32, t: f32) -> f32 {
    a + ((b - a) * t)
}

fn lerp_f32_opt(a: Option<f32>, b: Option<f32>, t: f32) -> Option<f32> {
    match (a, b) {
        (Some(x), Some(y)) => Some(x + ((y - x) * t)),
        (Some(x), None) => Some(x),
        (None, Some(y)) => Some(y),
        (None, None) => None,
    }
}

fn interpolate_photo_row(prev: &CsvRow, next: &CsvRow, photo: &Event, start_timestamp_ms: u64) -> Option<CsvRow> {
    let Event::Photo {
        timestamp_ms,
    } = photo else {
        return None;
    };

    if next.time_s <= prev.time_s {
        return None;
    }

    let event_time_s = (*timestamp_ms - start_timestamp_ms) as f32 / 1000.0;

    let t = (event_time_s - prev.time_s) / (next.time_s - prev.time_s);

    let mut row = prev.clone();

    row.timestamp_ms = *timestamp_ms;
    row.time_s = event_time_s;
    row.lat = lerp_f32(prev.lat, next.lat, t);
    row.lng = lerp_f32(prev.lng, next.lng, t);
    row.alt_m = lerp_f32(prev.alt_m, next.alt_m, t);
    row.is_photo = true;
    row.distance_to_home_m = lerp_f32(prev.distance_to_home_m, next.distance_to_home_m, t);
    row.height_m = lerp_f32(prev.height_m, next.height_m, t);
    row.speed_ms = lerp_f32(prev.speed_ms, next.speed_ms, t);
    row.velocity_x_ms = lerp_f32(prev.velocity_x_ms, next.velocity_x_ms, t);
    row.velocity_y_ms = lerp_f32(prev.velocity_y_ms, next.velocity_y_ms, t);
    row.velocity_z_ms = lerp_f32(prev.velocity_z_ms, next.velocity_z_ms, t);
    row.battery_voltage_v = lerp_f32(prev.battery_voltage_v, next.battery_voltage_v, t);
    row.battery_temp_c = lerp_f32_opt(prev.battery_temp_c, next.battery_temp_c, t);
    row.pitch_deg = lerp_f32(prev.pitch_deg, next.pitch_deg, t);
    row.roll_deg = lerp_f32(prev.roll_deg, next.roll_deg, t);
    row.yaw_deg = lerp_f32(prev.yaw_deg, next.yaw_deg, t);
    row.gimbal_pitch_deg = lerp_f32(prev.gimbal_pitch_deg, next.gimbal_pitch_deg, t);
    row.gimbal_roll_deg = lerp_f32(prev.gimbal_roll_deg, next.gimbal_roll_deg, t);
    row.gimbal_yaw_deg = lerp_f32(prev.gimbal_yaw_deg, next.gimbal_yaw_deg, t);
    row.rc_aileron = lerp_f32(prev.rc_aileron, next.rc_aileron, t);
    row.rc_elevator = lerp_f32(prev.rc_elevator, next.rc_elevator, t);
    row.rc_throttle = lerp_f32(prev.rc_throttle, next.rc_throttle, t);
    row.rc_rudder = lerp_f32(prev.rc_rudder, next.rc_rudder, t);
    row.battery_remained_capacity_mah = lerp_f32(prev.battery_remained_capacity_mah, next.battery_remained_capacity_mah, t);

    Some(row)
}

fn full_to_row(
    header_start_time_ms: u64,
    fr: &AutelDroneLog_FullRecordT,
) -> CsvRow {
    let mission_time_ms = *fr.mission_time_ms();
    let timestamp_ms = header_start_time_ms + mission_time_ms as u64;
    let att = fr.drone_attitude();
    let cfg = fr.configuration();
    let ctrl = fr.controller_input();
    let power = cfg.power_info();

    let ci = rc_input_to_control(*ctrl.left_stick_x(), *ctrl.left_stick_y(), *ctrl.right_stick_x(), *ctrl.right_stick_y());
    let voltages = power.cell_voltage();
    let cell_count = *power.cell_count() as usize;

    let voltage_string = format!(
        "[{}]",
        voltages[..cell_count]
            .iter()
            .map(|v| format!("{:.3}", v / 1000.0))
            .collect::<Vec<_>>()
            .join(", ")
    );

    CsvRow {
        timestamp_ms,
        time_s: mission_time_ms as f32 / 1000.0,
        lat: *att.drone_latitude(),
        lng: *att.drone_longitude(),
        // This should be ellipsoidal height, but we're not privy to that information in the flight logs.
        alt_m: *att.drone_altitude(),
        distance_to_home_m: *cfg.home_info().distance_to_home_m(),
        height_m: *att.drone_altitude(),
        // ODL calculates speed as just the horizontal component.
        speed_ms: f32::sqrt(f32::powi(*att.drone_x_speed(), 2) + f32::powi(*att.drone_y_speed(), 2)),
        velocity_x_ms: *att.drone_x_speed(),
        velocity_y_ms: *att.drone_y_speed(),
        velocity_z_ms: *att.drone_z_speed(),
        battery_percent: *power.remaining_charge_pc(),
        battery_voltage_v: *power.voltage_mv() / 1000.0,
        battery_temp_c: if *power.battery_temperature_deg_c() <= 120.0 { Some(*power.battery_temperature_deg_c()) } else { None },
        cell_voltages: voltage_string,
        satellites: *cfg.satellite_count(),
        rc_signal: *cfg.controller_rssi(),
        pitch_deg: f32::to_degrees(*att.drone_pitch()),
        roll_deg: f32::to_degrees(*att.drone_roll()),
        yaw_deg: f32::to_degrees(*att.drone_yaw()),
        gimbal_pitch_deg: *att.gimbal_pitch(),
        gimbal_roll_deg: *att.gimbal_roll(),
        gimbal_yaw_deg: *att.gimbal_yaw(),
        rc_aileron: ci.aileron,
        rc_elevator: ci.elevator,
        rc_throttle: ci.throttle,
        rc_rudder: ci.rudder,
        is_photo: false,
        is_video: false,
        flight_mode: Some(format!("{:?}", *cfg.flight_mode())),
        battery_full_capacity_mah: *power.maximum_capacity_mah(),
        battery_remained_capacity_mah: *power.current_capacity_mah(),
        messages: None,
        metadata: None,
    }
}

fn collect_events(log: &AutelDroneLog) -> Vec<Event> {
    let header = log.header();
    let start_time_ms = *header.start_time_ms();

    log.records()
        .iter()
        .filter_map(|r| {
            let body = r.body();
            match body.as_ref() {
                Some(AutelDroneLog_RecordT_Body::AutelDroneLog_FullRecordT(fr)) => {
                    Some(Event::Full(full_to_row(start_time_ms, fr)))
                }

                Some(AutelDroneLog_RecordT_Body::AutelDroneLog_PhotoRecordT(ph)) => {
                    Some(Event::Photo {
                        timestamp_ms: *ph.timestamp()
                    })
                }

                _ => None,
            }
        })
        .collect()
}

fn build_video_spans(log: &AutelDroneLog) -> Vec<VideoSpan> {
    log.records()
        .iter()
        .filter_map(|r| {
            let body = r.body();
            match body.as_ref() {
                Some(AutelDroneLog_RecordT_Body::AutelDroneLog_VideoRecordT(vr)) => {
                    Some(VideoSpan {
                        start_ms: *vr.timestamp(),
                        end_ms: *vr.timestamp() + (*vr.duration_s() * 1000) as u64,
                    })
                }

                _ => None,
            }
        })
        .collect()
}

fn build_messages(log: &AutelDroneLog) -> Vec<Message> {
    let mut last_state: Option<String> = None;

    log.records()
        .iter()
        .filter_map(|r| {
            let body = r.body();
            match body.as_ref() {
                Some(AutelDroneLog_RecordT_Body::AutelDroneLog_FullRecordT(r)) => {
                    let state = Some(format!("{:?}", *r.configuration().flight_mode()));

                    if last_state != state && state != None {
                        last_state = state.clone();

                        return Some(Message {
                            timestamp_ms: *r.mission_time_ms(),
                            msg_type: "tip".to_string(),
                            message: state.unwrap().to_case(Case::Title),
                        })
                    }

                    return None;
                }

                _ => None,
            }
        })
        .collect()
}

fn expand_events_to_rows(events: &[Event], start_timestamp_ms: u64) -> Vec<CsvRow> {
    let mut fulls: Vec<&CsvRow> = Vec::new();
    for ev in events {
        if let Event::Full(row) = ev {
            fulls.push(row);
        }
    }

    let mut out = Vec::new();
    let mut last_full: Option<&CsvRow> = None;

    for (idx, ev) in events.iter().enumerate() {
        match ev {
            Event::Full(row) => {
                out.push(row.clone());
                last_full = Some(row);
            }

            Event::Photo { timestamp_ms, .. } => {
                let Some(prev) = last_full else {
                    continue;
                };

                let next = events[idx + 1..].iter().find_map(|e| {
                    if let Event::Full(row) = e {
                        Some(row)
                    } else {
                        None
                    }
                });

                match next {
                    Some(next_full)
                    if prev.timestamp_ms <= *timestamp_ms
                        && *timestamp_ms <= next_full.timestamp_ms =>
                        {
                            if let Some(row) = interpolate_photo_row(prev, next_full, ev, start_timestamp_ms) {
                                out.push(row);
                            }
                        }

                    _ => {
                        // fallback: if no next full exists, clone prev and stamp photo values
                        if let Event::Photo {
                            timestamp_ms
                        } = ev
                        {
                            let mut row = prev.clone();
                            row.timestamp_ms = *timestamp_ms;
                            row.is_photo = true;
                            out.push(row);
                        }
                    }
                }
            }
        }
    }

    out.sort_by_key(|r| r.timestamp_ms);
    out
}

fn build_metadata(log: &AutelDroneLog) -> Metadata {
    let header = log.header();
    let records = log.records();
    let first = records.first().unwrap();
    let body = first.body();
    let full = match body.as_ref() {
        Some(AutelDroneLog_RecordT_Body::AutelDroneLog_FullRecordT(fr)) => fr,
        _ => unreachable!("first record is expected to be full"),
    };

    let utc = Utc.timestamp_millis_opt(*header.start_time_ms() as i64).unwrap();
    let offset = FixedOffset::east_opt(*header.timezone() as i32 * 3600).unwrap();
    let dt = utc.with_timezone(&offset);

    let log_location = header.location().trim_end_matches('\0').to_string();

    let location = if log_location.is_empty() {
        "Unknown location".to_string()
    } else {
        log_location
    };

    Metadata {
        display_name: location,
        drone_model: "Autel EVO II v3".to_string(),
        drone_serial: Some(header.drone_serial().trim_end_matches('\0').to_string()),
        battery_serial: Some(header.battery_serial().trim_end_matches('\0').to_string()),
        start_time: dt.to_rfc3339(),
        cycle_count: *full.configuration().power_info().battery_discharge_count(),
        duration_secs: *header.flight_duration_s(),
        home_lat: *header.start_latitude(),
        home_lon: *header.start_longitude(),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <input_file> <output_file>", args[0]);
        std::process::exit(1);
    }

    let input_file_name = &args[1];
    let output_file_name = &args[2];

    if !Path::new(input_file_name).exists() {
        eprintln!("Input file does not exist: {}", input_file_name);
        eprintln!("Usage: {} <input_file> <output_file>", args[0]);
        std::process::exit(1);
    }

    let bytes = fs::read(input_file_name).unwrap();
    let io = BytesReader::from(bytes);
    let g: OptRc<AutelDroneLog> = AutelDroneLog::read_into(&io, None, None).unwrap();

    let metadata = build_metadata(&g);

    if metadata.drone_serial.is_none() || metadata.drone_serial.as_ref().unwrap().is_empty() {
        eprintln!("No drone serial number found in log");
        std::process::exit(1);
    }

    let metadata_str = serde_json::to_string(&metadata).unwrap();

    let events = collect_events(&g);

    let mut rows = expand_events_to_rows(&events, *g.header().start_time_ms());

    let messages = build_messages(&g);
    let message_str = serde_json::to_string(&messages).unwrap();

    rows[0].messages = Some(message_str);
    rows[0].metadata = Some(metadata_str);

    let spans = build_video_spans(&g);
    for row in &mut rows {
        row.is_video = spans.iter().any(|s| s.start_ms <= row.timestamp_ms && row.timestamp_ms <= s.end_ms);
    }

    let mut wtr = csv::Writer::from_path(output_file_name).unwrap();
    for row in rows {
        wtr.serialize(row).unwrap();
    }
    wtr.flush().unwrap();
}
