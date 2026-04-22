#file: noinspection YAMLIncompatibleTypes
meta:
  id: autel_drone_log
  title: Autel drone log v3
  file-extension: log
  encoding: UTF-8
  endian: le

doc: |
  This has been mostly reverse engineered from actual logs, but also by poking around the app's source code.
  Most fields are self-explanatory and mostly make sense.
  Only log version 3 is supported, but it seems from the code that versions 1 and 2 are much the same but with fewer
  fields. It wouldn't be much work to add them if there was a need.
  There are two main types of record: full and base. Base records are odd because they are about 4 times more frequent
  than full, but often have identical latitude and longitudes as the previous full record but with a different
  timestamp. Becuase of this it causes the location of the drone to create a stair-step effect where the altitude
  varies but the horizontal position stays fixed.
  As there is less information in a base record anyway, it's probably best to just ignore them.
seq:
  - id: magic
    contents: AUTEL_FR
  - id: log_version
    type: u4
    valid: 0x03
    doc: We only support version 3 logs.
  - id: header
    type: header_t
  - id: records
    type: record_t
    repeat: eos

types:
  record_t:
    seq:
      - id: record_type
        type: u1
        doc: |
          In the code there are 6 record types, but only 4 seem to appear in the wild.
          The other two mirror the full_record_t and base_record_t but are prefixed with InSide in the code.
          This might be for some sort of internal logging perhaps. They appear to be identical to their OutSide
          companions except the InSide versions lack drone latitude and longitude fields, and some others.
          If they need implementing, look here com.autel.modelblib.lib.domain.model.flightlog.engine.FlightRecordInSideFullModel
          in the Explorer apk. InSideFull has a record type of 2, and InSideBase has a record type of 3.
          I have a couple of corrupt log files that seem to have some InSide records, so my conjecture is that
          InSide records get logged at flight time, then turned into OutSide records once the drone has landed.
      - id: body
        type:
          switch-on: record_type
          cases:
            0: full_record_t
            1: base_record_t
            14: photo_record_t
            15: video_record_t

  full_record_t:
    seq:
      - id: mission_time_ms
        type: u4
      - id: drone_attitude
        type: drone_attitude_t
      - id: controller_input
        type: controller_t
      - id: rc_mode
        type: u1
        doc: Always seems to be 0.
      - id: offline_duration
        type: u4
        doc: Unsure, but presumably how long the drone has been out of communication with the controller.
      - id: rc_buttons
        type: u1
        doc: It's not clear which buttons map to which bits. More testing is needed.
      - id: controller_heading
        type: f8
        doc: Always seems to be 0 with my controller. This is probably used on a controller with a compass.
      - id: oa_sensor
        type: oa_sensor_t
      - id: configuration
        type: configuration_t
      - id: param1
        type: u4
        doc: Padding/future use
      - id: param2
        type: u4
        doc: Padding/future use
      - id: param3
        type: u4
        doc: Padding/future use
      - id: param4
        type: u4
        doc: Padding/future use
      - id: param5
        type: u4
        doc: Padding/future use

  base_record_t:
    seq:
      - id: mission_time_ms
        type: u4
      - id: drone_attitude
        type: drone_attitude_t
      - id: controller_input
        type: controller_t
      - id: rc_mode
        type: u1
        doc: Always seems to be 0 in my logs.
      - id: offline_duration
        type: u4
        doc: Unsure, but presumably how long the drone has been out of communication with the controller.
      - id: rc_buttons
        type: u1
        doc: It's not clear which buttons map to which bits. More testing is needed.
      - id: controller_heading
        type: f8
        doc: Always seems to be 0 with my controller. This is probably used on a controller with a compass.
      - id: oa_sensor
        type: oa_sensor_t
      - id: param1
        type: u4
        doc: Padding/future use
      - id: param2
        type: u4
        doc: Padding/future use

  photo_record_t:
    doc: |
      Inserted into the stream when the photo is taken.
      I think once the image is saved rather than when the button is pressed, but I could be wrong.
    seq:
      - id: file_name
        type: str
        size: 64
      - id: timestamp
        type: u8
        doc: Milliseconds since Unix epoch rather than "mission" time.
      - id: latitude
        type: f4
      - id: longitude
        type: f4

  video_record_t:
    doc: |
      Inserted into the stream _when the video has finished_ BUT with a timestamp of when it was started!
    seq:
      - id: file_name
        type: str
        size: 64
      - id: timestamp
        type: u8
        doc: Milliseconds since Unix epoch rather than "mission" time of when the video started recording.
      - id: latitude
        type: f4
      - id: longitude
        type: f4
      - id: duration_s
        type: u4

  drone_attitude_t:
    doc: |
      Gimbal yaw seems to be in the same frame as drone yaw, but gimbal pitch and roll are relative to the airframe.
      I think, anyway. More tests need to be done.
    seq:
      - id: drone_latitude
        type: f4
      - id: drone_longitude
        type: f4
      - id: drone_altitude
        type: f4
        doc: In metres above TOLP.
      - id: drone_x_speed
        type: f4
        doc: In m/s.
      - id: drone_y_speed
        type: f4
        doc: In m/s.
      - id: drone_z_speed
        type: f4
        doc: In m/s.
      - id: gimbal_pitch
        type: f4
        doc: In degrees.
      - id: gimbal_roll
        type: f4
        doc: In degrees.
      - id: gimbal_yaw
        type: f4
        doc: In degrees.
      - id: drone_pitch
        type: f4
        doc: In radians.
      - id: drone_roll
        type: f4
        doc: In radians.
      - id: drone_yaw
        type: f4
        doc: In radians.

  configuration_t:
    seq:
      - id: flight_mode
        type: u1
        enum: e_flight_mode
        doc: Enum acquired from com.autel.common.flycontroller.FlyMode
      - id: camera_mode
        type: u1
        doc: Haven't got to the bottom of this yet.
      - id: gnss_rssi
        type: u1
        doc: GNSS signal strength in percent.
      - id: controller_rssi
        type: u1
        doc: Controller signal strength in percent.
      - id: stick_mode
        type: u1
        doc: |
          I thought it corresponded to standard RC modes - https://drones.stackexchange.com/questions/186/what-are-modes-of-a-transmitter-controller
          But in com.autel.common.remotecontroller.RemoteControllerCommandStickMode we have an enum: 0 - USA, 1 - CHINA, 2 - JAPAN, -1 - UNKNOWN.
          But in GPS_FLIGHT flight_mode it is 2 and in ATTI_FLIGHT flight_mode it is 1.
      - id: home_info
        type: home_info_t
      - id: distance_flown
        type: f4
      - id: drone_warning_flags  # Need to figure these flags out.
        type: u4
      - id: drone_ext_warning_flags  # Need to figure these flags out.
        type: u4
      - id: gimbal_warning_flags  # Need to figure these flags out.
        type: u4
      - id: time_remaining
        type: u4
        doc: Estimate of how much time is left before the battery is drained.
      - id: back_time
        type: u4
        doc: |
          Unknown meaning. Always seems to be 0, and few calls in the code set it.
          Perhaps meant to be set to how long it'll take to get back to the TOLP.
      - id: satellite_count
        type: u1
      - id: power_info
        type: power_info_t
      - id: settings
        type: settings_t

  power_info_t:
    seq:
      - id: designed_volume
        type: u4
        doc: |
          Unknown meaning. Always seems to match battery_discharge_count, but perhaps should be the nominal capacity?
          My Evo II v3 battery XE3_7100_1155 has a nominal capacity of 7100mAh.
      - id: maximum_capacity_mah
        type: u4
      - id: current_capacity_mah
        type: f4
      - id: voltage_mv
        type: f4
      - id: current_draw
        type: f4
      - id: remaining_charge_pc
        type: u1
      - id: battery_temperature_deg_c
        type: f4
      - id: battery_state
        type: u1
        enum: e_battery_state
        doc: Flags whether the battery is too hot or too cold to discharge.
      - id: battery_discharge_count
        type: u4
        doc: |
          Really seems to be the number of charges rather than discharges.
          That is, using it twice without charging in between doesn't increase the counter.
      - id: cell_count
        type: u1
      - id: cell_voltage
        type: f4
        repeat: expr
        repeat-expr: 8

  settings_t:
    seq:
      - id: oa_warning_enabled
        type: u4
      - id: oa_ext_warning_enabled  # ?
        type: u4
      - id: oa_error_code  # ?
        type: u4
      - id: max_altitude_m
        type: f4
      - id: return_to_home_altitude_m
        type: f4
      - id: beginner_mode_enabled
        type: u1
      - id: low_battery_threshold_enabled
        type: u1
      - id: critical_battery_threshold_enabled
        type: u1
      - id: max_flight_distance_m
        type: f4
      - id: max_horizontal_speed
        type: f4
      - id: oa_enabled
        type: u1
      - id: radar_enabled
        type: u1
      - id: max_error
        type: u4

  home_info_t:
    seq:
      - id: home_latitude
        type: f4
      - id: home_longitude
        type: f4
      - id: distance_to_home_m
        type: f4

  oa_sensor_t:
    seq:
      - id: timestamp
        type: f8
      - id: front_sensor
        type: f4
      - id: rear_sensor
        type: f4
      - id: left_sensor
        type: f4
      - id: right_sensor
        type: f4
      - id: top_sensor
        type: f4
      - id: bottom_sensor
        type: f4

  controller_t:
    seq:
      - id: left_stick_x
        type: u2
      - id: left_stick_y
        type: u2
      - id: right_stick_x
        type: u2
      - id: right_stick_y
        type: u2

  header_t:
    seq:
      - id: header_size
        type: u2
      - id: drone_serial
        type: str
        size: 18
      - id: battery_serial
        type: str
        size: 32
      - id: location
        type: str
        size: 64
      - id: drone_type
        type: u1
      - id: distance
        type: f4
      - id: flight_duration_s
        type: u4
      - id: max_altitude
        type: f4
      - id: video_duration
        type: u4
      - id: start_time_ms
        type: u8
      - id: timezone
        type: u4
      - id: start_latitude
        type: f4
      - id: start_longitude
        type: f4
      - id: image_count
        type: u2
      - id: video_count
        type: u2
      - id: firmware_length
        type: u2
      - id: firmware_info
        type: str
        size: firmware_length

enums:
  e_flight_mode:
    0: disarm
    1: motor_spinning
    2: landing
    3: takeoff
    4: atti_flight
    5: gps_flight
    6: ioc
    7: normal_go_home
    8: low_battery_go_home
    9: exceed_range_go_home
    10: rc_lost_go_home
    11: go_home_hover
    12: waypoint_mode
    13: waypoint_mode_hold
    14: mission_go_home
    15: follow_follow
    16: orbit_orbit
    17: follow_hold
    18: orbit_hold
    19: shoot_360
    20: epic
    21: rise
    22: fade_away
    23: into_sky
    24: boomerang
    25: screw
    26: parabola
    27: asteroid
    28: circle_round
    29: dolly_zoom
    30: tripod
    31: photographer
    32: rectangle
    33: rectangle_hold
    34: polygon
    35: polygon_hold
    36: motion_delay
    37: motion_delay_pause
    38: oblique_mission
    39: oblique_mission_pause
    40: panoramic_mission
    200: track_common_mode
    201: track_parallel_mode
    202: track_locked_mode
    203: point_fly_inside
    204: point_fly_outside

  e_battery_state:
    0: nominal_discharge_temperature
    4: under_discharge_temperature
    8: over_discharge_temperature
