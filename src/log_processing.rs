use chrono::{FixedOffset, TimeZone, Utc};
use convert_case::{Case, Casing};

use crate::{
    autel_drone_log::{AutelDroneLog, AutelDroneLog_RecordT_Body},
    models::{Event, Message, Metadata, VideoSpan},
    transform::{full_to_row, interpolate_photo_row},
};

/// Collects full rows and photo timestamps from the log.
pub fn collect_events(log: &AutelDroneLog) -> Vec<Event> {
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
                        timestamp_ms: *ph.timestamp(),
                    })
                }
                _ => None,
            }
        })
        .collect()
}

/// Builds inclusive start/end ranges for video playback spans.
pub fn build_video_spans(log: &AutelDroneLog) -> Vec<VideoSpan> {
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

/// Emits a message when the flight mode changes between full records.
pub fn build_messages(log: &AutelDroneLog) -> Vec<Message> {
    let mut last_state: Option<String> = None;

    log.records()
        .iter()
        .filter_map(|r| {
            let body = r.body();
            match body.as_ref() {
                Some(AutelDroneLog_RecordT_Body::AutelDroneLog_FullRecordT(r)) => {
                    let state = Some(format!("{:?}", *r.configuration().flight_mode()));

                    if last_state != state && state.is_some() {
                        last_state = state.clone();

                        Some(Message {
                            timestamp_ms: *r.mission_time_ms(),
                            msg_type: "tip".to_string(),
                            message: state.unwrap().to_case(Case::Title),
                        })
                    } else {
                        None
                    }
                }
                _ => None,
            }
        })
        .collect()
}

/// Transforms events into CSV rows and interpolates photo rows when possible.
pub fn transform_events_to_rows(
    events: &[Event],
    start_timestamp_ms: u64,
) -> Vec<crate::models::CsvRow> {
    let mut out = Vec::new();
    let mut last_full: Option<&crate::models::CsvRow> = None;

    for (idx, ev) in events.iter().enumerate() {
        match ev {
            Event::Full(row) => {
                out.push(row.clone());
                last_full = Some(row);
            }
            Event::Photo { timestamp_ms } => {
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
                        if let Some(row) =
                            interpolate_photo_row(prev, next_full, ev, start_timestamp_ms)
                        {
                            out.push(row);
                        }
                    }
                    _ => {
                        let mut row = prev.clone();
                        row.timestamp_ms = *timestamp_ms;
                        row.is_photo = true;
                        out.push(row);
                    }
                }
            }
        }
    }

    out.sort_by_key(|r| r.timestamp_ms);
    out
}

/// Builds ODL metadata from the log header and first full record.
pub fn build_metadata(log: &AutelDroneLog) -> Metadata {
    let header = log.header();
    let records = log.records();
    let first = records.first().unwrap();
    let body = first.body();
    let full = match body.as_ref() {
        Some(AutelDroneLog_RecordT_Body::AutelDroneLog_FullRecordT(fr)) => fr,
        _ => unreachable!("first record is expected to be full"),
    };

    let utc = Utc
        .timestamp_millis_opt(*header.start_time_ms() as i64)
        .unwrap();
    let offset = FixedOffset::east_opt(*header.timezone() as i32 * 3600).unwrap();
    let dt = utc.with_timezone(&offset);

    let log_location = header.location().trim_end_matches('\0').to_string();
    let location = if log_location.is_empty() {
        format!("{:.4}, {:.4}", header.start_latitude(), header.start_longitude())
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
