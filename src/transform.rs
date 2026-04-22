use crate::{
    autel_drone_log::AutelDroneLog_FullRecordT,
    models::{ControlInput, CsvRow, Event},
};

pub fn rc_input_to_control(left_x: u16, left_y: u16, right_x: u16, right_y: u16) -> ControlInput {
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

pub fn lerp_f32(a: f32, b: f32, t: f32) -> f32 {
    a + ((b - a) * t)
}

pub fn lerp_f32_opt(a: Option<f32>, b: Option<f32>, t: f32) -> Option<f32> {
    match (a, b) {
        (Some(x), Some(y)) => Some(x + ((y - x) * t)),
        (Some(x), None) => Some(x),
        (None, Some(y)) => Some(y),
        (None, None) => None,
    }
}

pub fn interpolate_photo_row(
    prev: &CsvRow,
    next: &CsvRow,
    photo: &Event,
    start_timestamp_ms: u64,
) -> Option<CsvRow> {
    let Event::Photo { timestamp_ms } = photo else {
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
    row.battery_remained_capacity_mah =
        lerp_f32(prev.battery_remained_capacity_mah, next.battery_remained_capacity_mah, t);

    Some(row)
}

pub fn full_to_row(header_start_time_ms: u64, fr: &AutelDroneLog_FullRecordT) -> CsvRow {
    let mission_time_ms = *fr.mission_time_ms();
    let timestamp_ms = header_start_time_ms + mission_time_ms as u64;
    let att = fr.drone_attitude();
    let cfg = fr.configuration();
    let ctrl = fr.controller_input();
    let power = cfg.power_info();

    let ci = rc_input_to_control(
        *ctrl.left_stick_x(),
        *ctrl.left_stick_y(),
        *ctrl.right_stick_x(),
        *ctrl.right_stick_y(),
    );
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
        alt_m: *att.drone_altitude(),
        distance_to_home_m: *cfg.home_info().distance_to_home_m(),
        height_m: *att.drone_altitude(),
        speed_ms: f32::sqrt(f32::powi(*att.drone_x_speed(), 2) + f32::powi(*att.drone_y_speed(), 2)),
        velocity_x_ms: *att.drone_x_speed(),
        velocity_y_ms: *att.drone_y_speed(),
        velocity_z_ms: *att.drone_z_speed(),
        battery_percent: *power.remaining_charge_pc(),
        battery_voltage_v: *power.voltage_mv() / 1000.0,
        battery_temp_c: if *power.battery_temperature_deg_c() <= 120.0 {
            Some(*power.battery_temperature_deg_c())
        } else {
            None
        },
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
