use std::{env, fs, path::Path};

use kaitai::{BytesReader, KStruct, OptRc};

use crate::{
    autel_drone_log::AutelDroneLog,
    log_processing::{
        build_messages, build_metadata, build_video_spans, collect_events, transform_events_to_rows,
    },
};

pub fn run() {
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
    let mut rows = transform_events_to_rows(&events, *g.header().start_time_ms());

    let messages = build_messages(&g);
    let message_str = serde_json::to_string(&messages).unwrap();

    rows[0].messages = Some(message_str);
    rows[0].metadata = Some(metadata_str);

    let spans = build_video_spans(&g);
    for row in &mut rows {
        row.is_video = spans
            .iter()
            .any(|s| s.start_ms <= row.timestamp_ms && row.timestamp_ms <= s.end_ms);
    }

    let mut wtr = csv::Writer::from_path(output_file_name).unwrap();
    for row in rows {
        wtr.serialize(row).unwrap();
    }
    wtr.flush().unwrap();
}
