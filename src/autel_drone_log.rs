// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(irrefutable_let_patterns)]
#![allow(unused_comparisons)]

extern crate kaitai;
use kaitai::*;
use std::convert::{TryFrom, TryInto};
use std::cell::{Ref, Cell, RefCell};
use std::rc::{Rc, Weak};

/**
 * This has been mostly reverse engineered from actual logs, but also by poking around the app's source code.
 * Most fields are self-explanatory and mostly make sense.
 * Only log version 3 is supported, but it seems from the code that versions 1 and 2 are much the same but with fewer
 * fields. It wouldn't be much work to add them if there was a need.
 * There are two main types of record: full and base. Base records are odd because they are about 4 times more frequent
 * than full, but often have identical latitude and longitudes as the previous full record but with a different
 * timestamp. Becuase of this it causes the location of the drone to create a stair-step effect where the altitude
 * varies but the horizontal position stays fixed.
 * As there is less information in a base record anyway, it's probably best to just ignore them.
 */

#[derive(Default, Debug, Clone)]
pub struct AutelDroneLog {
    pub _root: SharedType<AutelDroneLog>,
    pub _parent: SharedType<AutelDroneLog>,
    pub _self: SharedType<Self>,
    magic: RefCell<Vec<u8>>,
    log_version: RefCell<u32>,
    header: RefCell<OptRc<AutelDroneLog_HeaderT>>,
    records: RefCell<Vec<OptRc<AutelDroneLog_RecordT>>>,
    _io: RefCell<BytesReader>,
}
impl KStruct for AutelDroneLog {
    type Root = AutelDroneLog;
    type Parent = AutelDroneLog;

    fn read<S: KStream>(
        self_rc: &OptRc<Self>,
        _io: &S,
        _root: SharedType<Self::Root>,
        _parent: SharedType<Self::Parent>,
    ) -> KResult<()> {
        *self_rc._io.borrow_mut() = _io.clone();
        self_rc._root.set(_root.get());
        self_rc._parent.set(_parent.get());
        self_rc._self.set(Ok(self_rc.clone()));
        let _rrc = self_rc._root.get_value().borrow().upgrade();
        let _prc = self_rc._parent.get_value().borrow().upgrade();
        let _r = _rrc.as_ref().unwrap();
        *self_rc.magic.borrow_mut() = _io.read_bytes(8 as usize)?.into();
        if !(*self_rc.magic() == vec![0x41u8, 0x55u8, 0x54u8, 0x45u8, 0x4cu8, 0x5fu8, 0x46u8, 0x52u8]) {
            return Err(KError::ValidationFailed(ValidationFailedError { kind: ValidationKind::NotEqual, src_path: "/seq/0".to_string() }));
        }
        *self_rc.log_version.borrow_mut() = _io.read_u4le()?.into();
        if !(((*self_rc.log_version() as u32) == (3 as u32))) {
            return Err(KError::ValidationFailed(ValidationFailedError { kind: ValidationKind::NotEqual, src_path: "/seq/1".to_string() }));
        }
        let t = Self::read_into::<_, AutelDroneLog_HeaderT>(&*_io, Some(self_rc._root.clone()), Some(self_rc._self.clone()))?.into();
        *self_rc.header.borrow_mut() = t;
        *self_rc.records.borrow_mut() = Vec::new();
        {
            let mut _i = 0;
            while !_io.is_eof() {
                let t = Self::read_into::<_, AutelDroneLog_RecordT>(&*_io, Some(self_rc._root.clone()), Some(self_rc._self.clone()))?.into();
                self_rc.records.borrow_mut().push(t);
                _i += 1;
            }
        }
        Ok(())
    }
}
impl AutelDroneLog {
}
impl AutelDroneLog {
    pub fn magic(&self) -> Ref<'_, Vec<u8>> {
        self.magic.borrow()
    }
}

/**
 * We only support version 3 logs.
 */
impl AutelDroneLog {
    pub fn log_version(&self) -> Ref<'_, u32> {
        self.log_version.borrow()
    }
}
impl AutelDroneLog {
    pub fn header(&self) -> Ref<'_, OptRc<AutelDroneLog_HeaderT>> {
        self.header.borrow()
    }
}
impl AutelDroneLog {
    pub fn records(&self) -> Ref<'_, Vec<OptRc<AutelDroneLog_RecordT>>> {
        self.records.borrow()
    }
}
impl AutelDroneLog {
    pub fn _io(&self) -> Ref<'_, BytesReader> {
        self._io.borrow()
    }
}
#[derive(Debug, PartialEq, Clone)]
pub enum AutelDroneLog_EBatteryState {
    NominalDischargeTemperature,
    UnderDischargeTemperature,
    OverDischargeTemperature,
    Unknown(i64),
}

impl TryFrom<i64> for AutelDroneLog_EBatteryState {
    type Error = KError;
    fn try_from(flag: i64) -> KResult<AutelDroneLog_EBatteryState> {
        match flag {
            0 => Ok(AutelDroneLog_EBatteryState::NominalDischargeTemperature),
            4 => Ok(AutelDroneLog_EBatteryState::UnderDischargeTemperature),
            8 => Ok(AutelDroneLog_EBatteryState::OverDischargeTemperature),
            _ => Ok(AutelDroneLog_EBatteryState::Unknown(flag)),
        }
    }
}

impl From<&AutelDroneLog_EBatteryState> for i64 {
    fn from(v: &AutelDroneLog_EBatteryState) -> Self {
        match *v {
            AutelDroneLog_EBatteryState::NominalDischargeTemperature => 0,
            AutelDroneLog_EBatteryState::UnderDischargeTemperature => 4,
            AutelDroneLog_EBatteryState::OverDischargeTemperature => 8,
            AutelDroneLog_EBatteryState::Unknown(v) => v
        }
    }
}

impl Default for AutelDroneLog_EBatteryState {
    fn default() -> Self { AutelDroneLog_EBatteryState::Unknown(0) }
}

#[derive(Debug, PartialEq, Clone)]
pub enum AutelDroneLog_EFlightMode {
    Disarm,
    MotorSpinning,
    Landing,
    Takeoff,
    AttiFlight,
    GpsFlight,
    Ioc,
    NormalGoHome,
    LowBatteryGoHome,
    ExceedRangeGoHome,
    RcLostGoHome,
    GoHomeHover,
    WaypointMode,
    WaypointModeHold,
    MissionGoHome,
    FollowFollow,
    OrbitOrbit,
    FollowHold,
    OrbitHold,
    Shoot360,
    Epic,
    Rise,
    FadeAway,
    IntoSky,
    Boomerang,
    Screw,
    Parabola,
    Asteroid,
    CircleRound,
    DollyZoom,
    Tripod,
    Photographer,
    Rectangle,
    RectangleHold,
    Polygon,
    PolygonHold,
    MotionDelay,
    MotionDelayPause,
    ObliqueMission,
    ObliqueMissionPause,
    PanoramicMission,
    TrackCommonMode,
    TrackParallelMode,
    TrackLockedMode,
    PointFlyInside,
    PointFlyOutside,
    Unknown(i64),
}

impl TryFrom<i64> for AutelDroneLog_EFlightMode {
    type Error = KError;
    fn try_from(flag: i64) -> KResult<AutelDroneLog_EFlightMode> {
        match flag {
            0 => Ok(AutelDroneLog_EFlightMode::Disarm),
            1 => Ok(AutelDroneLog_EFlightMode::MotorSpinning),
            2 => Ok(AutelDroneLog_EFlightMode::Landing),
            3 => Ok(AutelDroneLog_EFlightMode::Takeoff),
            4 => Ok(AutelDroneLog_EFlightMode::AttiFlight),
            5 => Ok(AutelDroneLog_EFlightMode::GpsFlight),
            6 => Ok(AutelDroneLog_EFlightMode::Ioc),
            7 => Ok(AutelDroneLog_EFlightMode::NormalGoHome),
            8 => Ok(AutelDroneLog_EFlightMode::LowBatteryGoHome),
            9 => Ok(AutelDroneLog_EFlightMode::ExceedRangeGoHome),
            10 => Ok(AutelDroneLog_EFlightMode::RcLostGoHome),
            11 => Ok(AutelDroneLog_EFlightMode::GoHomeHover),
            12 => Ok(AutelDroneLog_EFlightMode::WaypointMode),
            13 => Ok(AutelDroneLog_EFlightMode::WaypointModeHold),
            14 => Ok(AutelDroneLog_EFlightMode::MissionGoHome),
            15 => Ok(AutelDroneLog_EFlightMode::FollowFollow),
            16 => Ok(AutelDroneLog_EFlightMode::OrbitOrbit),
            17 => Ok(AutelDroneLog_EFlightMode::FollowHold),
            18 => Ok(AutelDroneLog_EFlightMode::OrbitHold),
            19 => Ok(AutelDroneLog_EFlightMode::Shoot360),
            20 => Ok(AutelDroneLog_EFlightMode::Epic),
            21 => Ok(AutelDroneLog_EFlightMode::Rise),
            22 => Ok(AutelDroneLog_EFlightMode::FadeAway),
            23 => Ok(AutelDroneLog_EFlightMode::IntoSky),
            24 => Ok(AutelDroneLog_EFlightMode::Boomerang),
            25 => Ok(AutelDroneLog_EFlightMode::Screw),
            26 => Ok(AutelDroneLog_EFlightMode::Parabola),
            27 => Ok(AutelDroneLog_EFlightMode::Asteroid),
            28 => Ok(AutelDroneLog_EFlightMode::CircleRound),
            29 => Ok(AutelDroneLog_EFlightMode::DollyZoom),
            30 => Ok(AutelDroneLog_EFlightMode::Tripod),
            31 => Ok(AutelDroneLog_EFlightMode::Photographer),
            32 => Ok(AutelDroneLog_EFlightMode::Rectangle),
            33 => Ok(AutelDroneLog_EFlightMode::RectangleHold),
            34 => Ok(AutelDroneLog_EFlightMode::Polygon),
            35 => Ok(AutelDroneLog_EFlightMode::PolygonHold),
            36 => Ok(AutelDroneLog_EFlightMode::MotionDelay),
            37 => Ok(AutelDroneLog_EFlightMode::MotionDelayPause),
            38 => Ok(AutelDroneLog_EFlightMode::ObliqueMission),
            39 => Ok(AutelDroneLog_EFlightMode::ObliqueMissionPause),
            40 => Ok(AutelDroneLog_EFlightMode::PanoramicMission),
            200 => Ok(AutelDroneLog_EFlightMode::TrackCommonMode),
            201 => Ok(AutelDroneLog_EFlightMode::TrackParallelMode),
            202 => Ok(AutelDroneLog_EFlightMode::TrackLockedMode),
            203 => Ok(AutelDroneLog_EFlightMode::PointFlyInside),
            204 => Ok(AutelDroneLog_EFlightMode::PointFlyOutside),
            _ => Ok(AutelDroneLog_EFlightMode::Unknown(flag)),
        }
    }
}

impl From<&AutelDroneLog_EFlightMode> for i64 {
    fn from(v: &AutelDroneLog_EFlightMode) -> Self {
        match *v {
            AutelDroneLog_EFlightMode::Disarm => 0,
            AutelDroneLog_EFlightMode::MotorSpinning => 1,
            AutelDroneLog_EFlightMode::Landing => 2,
            AutelDroneLog_EFlightMode::Takeoff => 3,
            AutelDroneLog_EFlightMode::AttiFlight => 4,
            AutelDroneLog_EFlightMode::GpsFlight => 5,
            AutelDroneLog_EFlightMode::Ioc => 6,
            AutelDroneLog_EFlightMode::NormalGoHome => 7,
            AutelDroneLog_EFlightMode::LowBatteryGoHome => 8,
            AutelDroneLog_EFlightMode::ExceedRangeGoHome => 9,
            AutelDroneLog_EFlightMode::RcLostGoHome => 10,
            AutelDroneLog_EFlightMode::GoHomeHover => 11,
            AutelDroneLog_EFlightMode::WaypointMode => 12,
            AutelDroneLog_EFlightMode::WaypointModeHold => 13,
            AutelDroneLog_EFlightMode::MissionGoHome => 14,
            AutelDroneLog_EFlightMode::FollowFollow => 15,
            AutelDroneLog_EFlightMode::OrbitOrbit => 16,
            AutelDroneLog_EFlightMode::FollowHold => 17,
            AutelDroneLog_EFlightMode::OrbitHold => 18,
            AutelDroneLog_EFlightMode::Shoot360 => 19,
            AutelDroneLog_EFlightMode::Epic => 20,
            AutelDroneLog_EFlightMode::Rise => 21,
            AutelDroneLog_EFlightMode::FadeAway => 22,
            AutelDroneLog_EFlightMode::IntoSky => 23,
            AutelDroneLog_EFlightMode::Boomerang => 24,
            AutelDroneLog_EFlightMode::Screw => 25,
            AutelDroneLog_EFlightMode::Parabola => 26,
            AutelDroneLog_EFlightMode::Asteroid => 27,
            AutelDroneLog_EFlightMode::CircleRound => 28,
            AutelDroneLog_EFlightMode::DollyZoom => 29,
            AutelDroneLog_EFlightMode::Tripod => 30,
            AutelDroneLog_EFlightMode::Photographer => 31,
            AutelDroneLog_EFlightMode::Rectangle => 32,
            AutelDroneLog_EFlightMode::RectangleHold => 33,
            AutelDroneLog_EFlightMode::Polygon => 34,
            AutelDroneLog_EFlightMode::PolygonHold => 35,
            AutelDroneLog_EFlightMode::MotionDelay => 36,
            AutelDroneLog_EFlightMode::MotionDelayPause => 37,
            AutelDroneLog_EFlightMode::ObliqueMission => 38,
            AutelDroneLog_EFlightMode::ObliqueMissionPause => 39,
            AutelDroneLog_EFlightMode::PanoramicMission => 40,
            AutelDroneLog_EFlightMode::TrackCommonMode => 200,
            AutelDroneLog_EFlightMode::TrackParallelMode => 201,
            AutelDroneLog_EFlightMode::TrackLockedMode => 202,
            AutelDroneLog_EFlightMode::PointFlyInside => 203,
            AutelDroneLog_EFlightMode::PointFlyOutside => 204,
            AutelDroneLog_EFlightMode::Unknown(v) => v
        }
    }
}

impl Default for AutelDroneLog_EFlightMode {
    fn default() -> Self { AutelDroneLog_EFlightMode::Unknown(0) }
}


#[derive(Default, Debug, Clone)]
pub struct AutelDroneLog_BaseRecordT {
    pub _root: SharedType<AutelDroneLog>,
    pub _parent: SharedType<AutelDroneLog_RecordT>,
    pub _self: SharedType<Self>,
    mission_time_ms: RefCell<u32>,
    drone_attitude: RefCell<OptRc<AutelDroneLog_DroneAttitudeT>>,
    controller_input: RefCell<OptRc<AutelDroneLog_ControllerT>>,
    rc_mode: RefCell<u8>,
    offline_duration: RefCell<u32>,
    rc_buttons: RefCell<u8>,
    controller_heading: RefCell<f64>,
    oa_sensor: RefCell<OptRc<AutelDroneLog_OaSensorT>>,
    param1: RefCell<u32>,
    param2: RefCell<u32>,
    _io: RefCell<BytesReader>,
}
impl KStruct for AutelDroneLog_BaseRecordT {
    type Root = AutelDroneLog;
    type Parent = AutelDroneLog_RecordT;

    fn read<S: KStream>(
        self_rc: &OptRc<Self>,
        _io: &S,
        _root: SharedType<Self::Root>,
        _parent: SharedType<Self::Parent>,
    ) -> KResult<()> {
        *self_rc._io.borrow_mut() = _io.clone();
        self_rc._root.set(_root.get());
        self_rc._parent.set(_parent.get());
        self_rc._self.set(Ok(self_rc.clone()));
        let _rrc = self_rc._root.get_value().borrow().upgrade();
        let _prc = self_rc._parent.get_value().borrow().upgrade();
        let _r = _rrc.as_ref().unwrap();
        *self_rc.mission_time_ms.borrow_mut() = _io.read_u4le()?.into();
        let t = Self::read_into::<_, AutelDroneLog_DroneAttitudeT>(&*_io, Some(self_rc._root.clone()), None)?.into();
        *self_rc.drone_attitude.borrow_mut() = t;
        let t = Self::read_into::<_, AutelDroneLog_ControllerT>(&*_io, Some(self_rc._root.clone()), None)?.into();
        *self_rc.controller_input.borrow_mut() = t;
        *self_rc.rc_mode.borrow_mut() = _io.read_u1()?.into();
        *self_rc.offline_duration.borrow_mut() = _io.read_u4le()?.into();
        *self_rc.rc_buttons.borrow_mut() = _io.read_u1()?.into();
        *self_rc.controller_heading.borrow_mut() = _io.read_f8le()?.into();
        let t = Self::read_into::<_, AutelDroneLog_OaSensorT>(&*_io, Some(self_rc._root.clone()), None)?.into();
        *self_rc.oa_sensor.borrow_mut() = t;
        *self_rc.param1.borrow_mut() = _io.read_u4le()?.into();
        *self_rc.param2.borrow_mut() = _io.read_u4le()?.into();
        Ok(())
    }
}
impl AutelDroneLog_BaseRecordT {
}
impl AutelDroneLog_BaseRecordT {
    pub fn mission_time_ms(&self) -> Ref<'_, u32> {
        self.mission_time_ms.borrow()
    }
}
impl AutelDroneLog_BaseRecordT {
    pub fn drone_attitude(&self) -> Ref<'_, OptRc<AutelDroneLog_DroneAttitudeT>> {
        self.drone_attitude.borrow()
    }
}
impl AutelDroneLog_BaseRecordT {
    pub fn controller_input(&self) -> Ref<'_, OptRc<AutelDroneLog_ControllerT>> {
        self.controller_input.borrow()
    }
}

/**
 * Always seems to be 0 in my logs.
 */
impl AutelDroneLog_BaseRecordT {
    pub fn rc_mode(&self) -> Ref<'_, u8> {
        self.rc_mode.borrow()
    }
}

/**
 * Unsure, but presumably how long the drone has been out of communication with the controller.
 */
impl AutelDroneLog_BaseRecordT {
    pub fn offline_duration(&self) -> Ref<'_, u32> {
        self.offline_duration.borrow()
    }
}

/**
 * It's not clear which buttons map to which bits. More testing is needed.
 */
impl AutelDroneLog_BaseRecordT {
    pub fn rc_buttons(&self) -> Ref<'_, u8> {
        self.rc_buttons.borrow()
    }
}

/**
 * Always seems to be 0 with my controller. This is probably used on a controller with a compass.
 */
impl AutelDroneLog_BaseRecordT {
    pub fn controller_heading(&self) -> Ref<'_, f64> {
        self.controller_heading.borrow()
    }
}
impl AutelDroneLog_BaseRecordT {
    pub fn oa_sensor(&self) -> Ref<'_, OptRc<AutelDroneLog_OaSensorT>> {
        self.oa_sensor.borrow()
    }
}

/**
 * Padding/future use
 */
impl AutelDroneLog_BaseRecordT {
    pub fn param1(&self) -> Ref<'_, u32> {
        self.param1.borrow()
    }
}

/**
 * Padding/future use
 */
impl AutelDroneLog_BaseRecordT {
    pub fn param2(&self) -> Ref<'_, u32> {
        self.param2.borrow()
    }
}
impl AutelDroneLog_BaseRecordT {
    pub fn _io(&self) -> Ref<'_, BytesReader> {
        self._io.borrow()
    }
}

#[derive(Default, Debug, Clone)]
pub struct AutelDroneLog_ConfigurationT {
    pub _root: SharedType<AutelDroneLog>,
    pub _parent: SharedType<AutelDroneLog_FullRecordT>,
    pub _self: SharedType<Self>,
    flight_mode: RefCell<AutelDroneLog_EFlightMode>,
    camera_mode: RefCell<u8>,
    gnss_rssi: RefCell<u8>,
    controller_rssi: RefCell<u8>,
    stick_mode: RefCell<u8>,
    home_info: RefCell<OptRc<AutelDroneLog_HomeInfoT>>,
    distance_flown: RefCell<f32>,
    drone_warning_flags: RefCell<u32>,
    drone_ext_warning_flags: RefCell<u32>,
    gimbal_warning_flags: RefCell<u32>,
    time_remaining: RefCell<u32>,
    back_time: RefCell<u32>,
    satellite_count: RefCell<u8>,
    power_info: RefCell<OptRc<AutelDroneLog_PowerInfoT>>,
    settings: RefCell<OptRc<AutelDroneLog_SettingsT>>,
    _io: RefCell<BytesReader>,
}
impl KStruct for AutelDroneLog_ConfigurationT {
    type Root = AutelDroneLog;
    type Parent = AutelDroneLog_FullRecordT;

    fn read<S: KStream>(
        self_rc: &OptRc<Self>,
        _io: &S,
        _root: SharedType<Self::Root>,
        _parent: SharedType<Self::Parent>,
    ) -> KResult<()> {
        *self_rc._io.borrow_mut() = _io.clone();
        self_rc._root.set(_root.get());
        self_rc._parent.set(_parent.get());
        self_rc._self.set(Ok(self_rc.clone()));
        let _rrc = self_rc._root.get_value().borrow().upgrade();
        let _prc = self_rc._parent.get_value().borrow().upgrade();
        let _r = _rrc.as_ref().unwrap();
        *self_rc.flight_mode.borrow_mut() = (_io.read_u1()? as i64).try_into()?;
        *self_rc.camera_mode.borrow_mut() = _io.read_u1()?.into();
        *self_rc.gnss_rssi.borrow_mut() = _io.read_u1()?.into();
        *self_rc.controller_rssi.borrow_mut() = _io.read_u1()?.into();
        *self_rc.stick_mode.borrow_mut() = _io.read_u1()?.into();
        let t = Self::read_into::<_, AutelDroneLog_HomeInfoT>(&*_io, Some(self_rc._root.clone()), Some(self_rc._self.clone()))?.into();
        *self_rc.home_info.borrow_mut() = t;
        *self_rc.distance_flown.borrow_mut() = _io.read_f4le()?.into();
        *self_rc.drone_warning_flags.borrow_mut() = _io.read_u4le()?.into();
        *self_rc.drone_ext_warning_flags.borrow_mut() = _io.read_u4le()?.into();
        *self_rc.gimbal_warning_flags.borrow_mut() = _io.read_u4le()?.into();
        *self_rc.time_remaining.borrow_mut() = _io.read_u4le()?.into();
        *self_rc.back_time.borrow_mut() = _io.read_u4le()?.into();
        *self_rc.satellite_count.borrow_mut() = _io.read_u1()?.into();
        let t = Self::read_into::<_, AutelDroneLog_PowerInfoT>(&*_io, Some(self_rc._root.clone()), Some(self_rc._self.clone()))?.into();
        *self_rc.power_info.borrow_mut() = t;
        let t = Self::read_into::<_, AutelDroneLog_SettingsT>(&*_io, Some(self_rc._root.clone()), Some(self_rc._self.clone()))?.into();
        *self_rc.settings.borrow_mut() = t;
        Ok(())
    }
}
impl AutelDroneLog_ConfigurationT {
}

/**
 * Enum acquired from com.autel.common.flycontroller.FlyMode
 */
impl AutelDroneLog_ConfigurationT {
    pub fn flight_mode(&self) -> Ref<'_, AutelDroneLog_EFlightMode> {
        self.flight_mode.borrow()
    }
}

/**
 * Haven't got to the bottom of this yet.
 */
impl AutelDroneLog_ConfigurationT {
    pub fn camera_mode(&self) -> Ref<'_, u8> {
        self.camera_mode.borrow()
    }
}

/**
 * GNSS signal strength in percent.
 */
impl AutelDroneLog_ConfigurationT {
    pub fn gnss_rssi(&self) -> Ref<'_, u8> {
        self.gnss_rssi.borrow()
    }
}

/**
 * Controller signal strength in percent.
 */
impl AutelDroneLog_ConfigurationT {
    pub fn controller_rssi(&self) -> Ref<'_, u8> {
        self.controller_rssi.borrow()
    }
}

/**
 * I thought it corresponded to standard RC modes - https://drones.stackexchange.com/questions/186/what-are-modes-of-a-transmitter-controller
 * But in com.autel.common.remotecontroller.RemoteControllerCommandStickMode we have an enum: 0 - USA, 1 - CHINA, 2 - JAPAN, -1 - UNKNOWN.
 * But in GPS_FLIGHT flight_mode it is 2 and in ATTI_FLIGHT flight_mode it is 1.
 */
impl AutelDroneLog_ConfigurationT {
    pub fn stick_mode(&self) -> Ref<'_, u8> {
        self.stick_mode.borrow()
    }
}
impl AutelDroneLog_ConfigurationT {
    pub fn home_info(&self) -> Ref<'_, OptRc<AutelDroneLog_HomeInfoT>> {
        self.home_info.borrow()
    }
}
impl AutelDroneLog_ConfigurationT {
    pub fn distance_flown(&self) -> Ref<'_, f32> {
        self.distance_flown.borrow()
    }
}
impl AutelDroneLog_ConfigurationT {
    pub fn drone_warning_flags(&self) -> Ref<'_, u32> {
        self.drone_warning_flags.borrow()
    }
}
impl AutelDroneLog_ConfigurationT {
    pub fn drone_ext_warning_flags(&self) -> Ref<'_, u32> {
        self.drone_ext_warning_flags.borrow()
    }
}
impl AutelDroneLog_ConfigurationT {
    pub fn gimbal_warning_flags(&self) -> Ref<'_, u32> {
        self.gimbal_warning_flags.borrow()
    }
}

/**
 * Estimate of how much time is left before the battery is drained.
 */
impl AutelDroneLog_ConfigurationT {
    pub fn time_remaining(&self) -> Ref<'_, u32> {
        self.time_remaining.borrow()
    }
}

/**
 * Unknown meaning. Always seems to be 0, and few calls in the code set it.
 * Perhaps meant to be set to how long it'll take to get back to the TOLP.
 */
impl AutelDroneLog_ConfigurationT {
    pub fn back_time(&self) -> Ref<'_, u32> {
        self.back_time.borrow()
    }
}
impl AutelDroneLog_ConfigurationT {
    pub fn satellite_count(&self) -> Ref<'_, u8> {
        self.satellite_count.borrow()
    }
}
impl AutelDroneLog_ConfigurationT {
    pub fn power_info(&self) -> Ref<'_, OptRc<AutelDroneLog_PowerInfoT>> {
        self.power_info.borrow()
    }
}
impl AutelDroneLog_ConfigurationT {
    pub fn settings(&self) -> Ref<'_, OptRc<AutelDroneLog_SettingsT>> {
        self.settings.borrow()
    }
}
impl AutelDroneLog_ConfigurationT {
    pub fn _io(&self) -> Ref<'_, BytesReader> {
        self._io.borrow()
    }
}

#[derive(Default, Debug, Clone)]
pub struct AutelDroneLog_ControllerT {
    pub _root: SharedType<AutelDroneLog>,
    pub _parent: SharedType<KStructUnit>,
    pub _self: SharedType<Self>,
    left_stick_x: RefCell<u16>,
    left_stick_y: RefCell<u16>,
    right_stick_x: RefCell<u16>,
    right_stick_y: RefCell<u16>,
    _io: RefCell<BytesReader>,
}
impl KStruct for AutelDroneLog_ControllerT {
    type Root = AutelDroneLog;
    type Parent = KStructUnit;

    fn read<S: KStream>(
        self_rc: &OptRc<Self>,
        _io: &S,
        _root: SharedType<Self::Root>,
        _parent: SharedType<Self::Parent>,
    ) -> KResult<()> {
        *self_rc._io.borrow_mut() = _io.clone();
        self_rc._root.set(_root.get());
        self_rc._parent.set(_parent.get());
        self_rc._self.set(Ok(self_rc.clone()));
        let _rrc = self_rc._root.get_value().borrow().upgrade();
        let _prc = self_rc._parent.get_value().borrow().upgrade();
        let _r = _rrc.as_ref().unwrap();
        *self_rc.left_stick_x.borrow_mut() = _io.read_u2le()?.into();
        *self_rc.left_stick_y.borrow_mut() = _io.read_u2le()?.into();
        *self_rc.right_stick_x.borrow_mut() = _io.read_u2le()?.into();
        *self_rc.right_stick_y.borrow_mut() = _io.read_u2le()?.into();
        Ok(())
    }
}
impl AutelDroneLog_ControllerT {
}
impl AutelDroneLog_ControllerT {
    pub fn left_stick_x(&self) -> Ref<'_, u16> {
        self.left_stick_x.borrow()
    }
}
impl AutelDroneLog_ControllerT {
    pub fn left_stick_y(&self) -> Ref<'_, u16> {
        self.left_stick_y.borrow()
    }
}
impl AutelDroneLog_ControllerT {
    pub fn right_stick_x(&self) -> Ref<'_, u16> {
        self.right_stick_x.borrow()
    }
}
impl AutelDroneLog_ControllerT {
    pub fn right_stick_y(&self) -> Ref<'_, u16> {
        self.right_stick_y.borrow()
    }
}
impl AutelDroneLog_ControllerT {
    pub fn _io(&self) -> Ref<'_, BytesReader> {
        self._io.borrow()
    }
}

/**
 * Gimbal yaw seems to be in the same frame as drone yaw, but gimbal pitch and roll are relative to the airframe.
 * I think, anyway. More tests need to be done.
 */

#[derive(Default, Debug, Clone)]
pub struct AutelDroneLog_DroneAttitudeT {
    pub _root: SharedType<AutelDroneLog>,
    pub _parent: SharedType<KStructUnit>,
    pub _self: SharedType<Self>,
    drone_latitude: RefCell<f32>,
    drone_longitude: RefCell<f32>,
    drone_altitude: RefCell<f32>,
    drone_x_speed: RefCell<f32>,
    drone_y_speed: RefCell<f32>,
    drone_z_speed: RefCell<f32>,
    gimbal_pitch: RefCell<f32>,
    gimbal_roll: RefCell<f32>,
    gimbal_yaw: RefCell<f32>,
    drone_pitch: RefCell<f32>,
    drone_roll: RefCell<f32>,
    drone_yaw: RefCell<f32>,
    _io: RefCell<BytesReader>,
}
impl KStruct for AutelDroneLog_DroneAttitudeT {
    type Root = AutelDroneLog;
    type Parent = KStructUnit;

    fn read<S: KStream>(
        self_rc: &OptRc<Self>,
        _io: &S,
        _root: SharedType<Self::Root>,
        _parent: SharedType<Self::Parent>,
    ) -> KResult<()> {
        *self_rc._io.borrow_mut() = _io.clone();
        self_rc._root.set(_root.get());
        self_rc._parent.set(_parent.get());
        self_rc._self.set(Ok(self_rc.clone()));
        let _rrc = self_rc._root.get_value().borrow().upgrade();
        let _prc = self_rc._parent.get_value().borrow().upgrade();
        let _r = _rrc.as_ref().unwrap();
        *self_rc.drone_latitude.borrow_mut() = _io.read_f4le()?.into();
        *self_rc.drone_longitude.borrow_mut() = _io.read_f4le()?.into();
        *self_rc.drone_altitude.borrow_mut() = _io.read_f4le()?.into();
        *self_rc.drone_x_speed.borrow_mut() = _io.read_f4le()?.into();
        *self_rc.drone_y_speed.borrow_mut() = _io.read_f4le()?.into();
        *self_rc.drone_z_speed.borrow_mut() = _io.read_f4le()?.into();
        *self_rc.gimbal_pitch.borrow_mut() = _io.read_f4le()?.into();
        *self_rc.gimbal_roll.borrow_mut() = _io.read_f4le()?.into();
        *self_rc.gimbal_yaw.borrow_mut() = _io.read_f4le()?.into();
        *self_rc.drone_pitch.borrow_mut() = _io.read_f4le()?.into();
        *self_rc.drone_roll.borrow_mut() = _io.read_f4le()?.into();
        *self_rc.drone_yaw.borrow_mut() = _io.read_f4le()?.into();
        Ok(())
    }
}
impl AutelDroneLog_DroneAttitudeT {
}
impl AutelDroneLog_DroneAttitudeT {
    pub fn drone_latitude(&self) -> Ref<'_, f32> {
        self.drone_latitude.borrow()
    }
}
impl AutelDroneLog_DroneAttitudeT {
    pub fn drone_longitude(&self) -> Ref<'_, f32> {
        self.drone_longitude.borrow()
    }
}

/**
 * In metres above TOLP.
 */
impl AutelDroneLog_DroneAttitudeT {
    pub fn drone_altitude(&self) -> Ref<'_, f32> {
        self.drone_altitude.borrow()
    }
}

/**
 * In m/s.
 */
impl AutelDroneLog_DroneAttitudeT {
    pub fn drone_x_speed(&self) -> Ref<'_, f32> {
        self.drone_x_speed.borrow()
    }
}

/**
 * In m/s.
 */
impl AutelDroneLog_DroneAttitudeT {
    pub fn drone_y_speed(&self) -> Ref<'_, f32> {
        self.drone_y_speed.borrow()
    }
}

/**
 * In m/s.
 */
impl AutelDroneLog_DroneAttitudeT {
    pub fn drone_z_speed(&self) -> Ref<'_, f32> {
        self.drone_z_speed.borrow()
    }
}

/**
 * In degrees.
 */
impl AutelDroneLog_DroneAttitudeT {
    pub fn gimbal_pitch(&self) -> Ref<'_, f32> {
        self.gimbal_pitch.borrow()
    }
}

/**
 * In degrees.
 */
impl AutelDroneLog_DroneAttitudeT {
    pub fn gimbal_roll(&self) -> Ref<'_, f32> {
        self.gimbal_roll.borrow()
    }
}

/**
 * In degrees.
 */
impl AutelDroneLog_DroneAttitudeT {
    pub fn gimbal_yaw(&self) -> Ref<'_, f32> {
        self.gimbal_yaw.borrow()
    }
}

/**
 * In radians.
 */
impl AutelDroneLog_DroneAttitudeT {
    pub fn drone_pitch(&self) -> Ref<'_, f32> {
        self.drone_pitch.borrow()
    }
}

/**
 * In radians.
 */
impl AutelDroneLog_DroneAttitudeT {
    pub fn drone_roll(&self) -> Ref<'_, f32> {
        self.drone_roll.borrow()
    }
}

/**
 * In radians.
 */
impl AutelDroneLog_DroneAttitudeT {
    pub fn drone_yaw(&self) -> Ref<'_, f32> {
        self.drone_yaw.borrow()
    }
}
impl AutelDroneLog_DroneAttitudeT {
    pub fn _io(&self) -> Ref<'_, BytesReader> {
        self._io.borrow()
    }
}

#[derive(Default, Debug, Clone)]
pub struct AutelDroneLog_FullRecordT {
    pub _root: SharedType<AutelDroneLog>,
    pub _parent: SharedType<AutelDroneLog_RecordT>,
    pub _self: SharedType<Self>,
    mission_time_ms: RefCell<u32>,
    drone_attitude: RefCell<OptRc<AutelDroneLog_DroneAttitudeT>>,
    controller_input: RefCell<OptRc<AutelDroneLog_ControllerT>>,
    rc_mode: RefCell<u8>,
    offline_duration: RefCell<u32>,
    rc_buttons: RefCell<u8>,
    controller_heading: RefCell<f64>,
    oa_sensor: RefCell<OptRc<AutelDroneLog_OaSensorT>>,
    configuration: RefCell<OptRc<AutelDroneLog_ConfigurationT>>,
    param1: RefCell<u32>,
    param2: RefCell<u32>,
    param3: RefCell<u32>,
    param4: RefCell<u32>,
    param5: RefCell<u32>,
    _io: RefCell<BytesReader>,
}
impl KStruct for AutelDroneLog_FullRecordT {
    type Root = AutelDroneLog;
    type Parent = AutelDroneLog_RecordT;

    fn read<S: KStream>(
        self_rc: &OptRc<Self>,
        _io: &S,
        _root: SharedType<Self::Root>,
        _parent: SharedType<Self::Parent>,
    ) -> KResult<()> {
        *self_rc._io.borrow_mut() = _io.clone();
        self_rc._root.set(_root.get());
        self_rc._parent.set(_parent.get());
        self_rc._self.set(Ok(self_rc.clone()));
        let _rrc = self_rc._root.get_value().borrow().upgrade();
        let _prc = self_rc._parent.get_value().borrow().upgrade();
        let _r = _rrc.as_ref().unwrap();
        *self_rc.mission_time_ms.borrow_mut() = _io.read_u4le()?.into();
        let t = Self::read_into::<_, AutelDroneLog_DroneAttitudeT>(&*_io, Some(self_rc._root.clone()), None)?.into();
        *self_rc.drone_attitude.borrow_mut() = t;
        let t = Self::read_into::<_, AutelDroneLog_ControllerT>(&*_io, Some(self_rc._root.clone()), None)?.into();
        *self_rc.controller_input.borrow_mut() = t;
        *self_rc.rc_mode.borrow_mut() = _io.read_u1()?.into();
        *self_rc.offline_duration.borrow_mut() = _io.read_u4le()?.into();
        *self_rc.rc_buttons.borrow_mut() = _io.read_u1()?.into();
        *self_rc.controller_heading.borrow_mut() = _io.read_f8le()?.into();
        let t = Self::read_into::<_, AutelDroneLog_OaSensorT>(&*_io, Some(self_rc._root.clone()), None)?.into();
        *self_rc.oa_sensor.borrow_mut() = t;
        let t = Self::read_into::<_, AutelDroneLog_ConfigurationT>(&*_io, Some(self_rc._root.clone()), Some(self_rc._self.clone()))?.into();
        *self_rc.configuration.borrow_mut() = t;
        *self_rc.param1.borrow_mut() = _io.read_u4le()?.into();
        *self_rc.param2.borrow_mut() = _io.read_u4le()?.into();
        *self_rc.param3.borrow_mut() = _io.read_u4le()?.into();
        *self_rc.param4.borrow_mut() = _io.read_u4le()?.into();
        *self_rc.param5.borrow_mut() = _io.read_u4le()?.into();
        Ok(())
    }
}
impl AutelDroneLog_FullRecordT {
}
impl AutelDroneLog_FullRecordT {
    pub fn mission_time_ms(&self) -> Ref<'_, u32> {
        self.mission_time_ms.borrow()
    }
}
impl AutelDroneLog_FullRecordT {
    pub fn drone_attitude(&self) -> Ref<'_, OptRc<AutelDroneLog_DroneAttitudeT>> {
        self.drone_attitude.borrow()
    }
}
impl AutelDroneLog_FullRecordT {
    pub fn controller_input(&self) -> Ref<'_, OptRc<AutelDroneLog_ControllerT>> {
        self.controller_input.borrow()
    }
}

/**
 * Always seems to be 0.
 */
impl AutelDroneLog_FullRecordT {
    pub fn rc_mode(&self) -> Ref<'_, u8> {
        self.rc_mode.borrow()
    }
}

/**
 * Unsure, but presumably how long the drone has been out of communication with the controller.
 */
impl AutelDroneLog_FullRecordT {
    pub fn offline_duration(&self) -> Ref<'_, u32> {
        self.offline_duration.borrow()
    }
}

/**
 * It's not clear which buttons map to which bits. More testing is needed.
 */
impl AutelDroneLog_FullRecordT {
    pub fn rc_buttons(&self) -> Ref<'_, u8> {
        self.rc_buttons.borrow()
    }
}

/**
 * Always seems to be 0 with my controller. This is probably used on a controller with a compass.
 */
impl AutelDroneLog_FullRecordT {
    pub fn controller_heading(&self) -> Ref<'_, f64> {
        self.controller_heading.borrow()
    }
}
impl AutelDroneLog_FullRecordT {
    pub fn oa_sensor(&self) -> Ref<'_, OptRc<AutelDroneLog_OaSensorT>> {
        self.oa_sensor.borrow()
    }
}
impl AutelDroneLog_FullRecordT {
    pub fn configuration(&self) -> Ref<'_, OptRc<AutelDroneLog_ConfigurationT>> {
        self.configuration.borrow()
    }
}

/**
 * Padding/future use
 */
impl AutelDroneLog_FullRecordT {
    pub fn param1(&self) -> Ref<'_, u32> {
        self.param1.borrow()
    }
}

/**
 * Padding/future use
 */
impl AutelDroneLog_FullRecordT {
    pub fn param2(&self) -> Ref<'_, u32> {
        self.param2.borrow()
    }
}

/**
 * Padding/future use
 */
impl AutelDroneLog_FullRecordT {
    pub fn param3(&self) -> Ref<'_, u32> {
        self.param3.borrow()
    }
}

/**
 * Padding/future use
 */
impl AutelDroneLog_FullRecordT {
    pub fn param4(&self) -> Ref<'_, u32> {
        self.param4.borrow()
    }
}

/**
 * Padding/future use
 */
impl AutelDroneLog_FullRecordT {
    pub fn param5(&self) -> Ref<'_, u32> {
        self.param5.borrow()
    }
}
impl AutelDroneLog_FullRecordT {
    pub fn _io(&self) -> Ref<'_, BytesReader> {
        self._io.borrow()
    }
}

#[derive(Default, Debug, Clone)]
pub struct AutelDroneLog_HeaderT {
    pub _root: SharedType<AutelDroneLog>,
    pub _parent: SharedType<AutelDroneLog>,
    pub _self: SharedType<Self>,
    header_size: RefCell<u16>,
    drone_serial: RefCell<String>,
    battery_serial: RefCell<String>,
    location: RefCell<String>,
    drone_type: RefCell<u8>,
    distance: RefCell<f32>,
    flight_duration_s: RefCell<u32>,
    max_altitude: RefCell<f32>,
    video_duration: RefCell<u32>,
    start_time_ms: RefCell<u64>,
    timezone: RefCell<u32>,
    start_latitude: RefCell<f32>,
    start_longitude: RefCell<f32>,
    image_count: RefCell<u16>,
    video_count: RefCell<u16>,
    firmware_length: RefCell<u16>,
    firmware_info: RefCell<String>,
    _io: RefCell<BytesReader>,
}
impl KStruct for AutelDroneLog_HeaderT {
    type Root = AutelDroneLog;
    type Parent = AutelDroneLog;

    fn read<S: KStream>(
        self_rc: &OptRc<Self>,
        _io: &S,
        _root: SharedType<Self::Root>,
        _parent: SharedType<Self::Parent>,
    ) -> KResult<()> {
        *self_rc._io.borrow_mut() = _io.clone();
        self_rc._root.set(_root.get());
        self_rc._parent.set(_parent.get());
        self_rc._self.set(Ok(self_rc.clone()));
        let _rrc = self_rc._root.get_value().borrow().upgrade();
        let _prc = self_rc._parent.get_value().borrow().upgrade();
        let _r = _rrc.as_ref().unwrap();
        *self_rc.header_size.borrow_mut() = _io.read_u2le()?.into();
        *self_rc.drone_serial.borrow_mut() = bytes_to_str(&_io.read_bytes(18 as usize)?.into(), "UTF-8")?;
        *self_rc.battery_serial.borrow_mut() = bytes_to_str(&_io.read_bytes(32 as usize)?.into(), "UTF-8")?;
        *self_rc.location.borrow_mut() = bytes_to_str(&_io.read_bytes(64 as usize)?.into(), "UTF-8")?;
        *self_rc.drone_type.borrow_mut() = _io.read_u1()?.into();
        *self_rc.distance.borrow_mut() = _io.read_f4le()?.into();
        *self_rc.flight_duration_s.borrow_mut() = _io.read_u4le()?.into();
        *self_rc.max_altitude.borrow_mut() = _io.read_f4le()?.into();
        *self_rc.video_duration.borrow_mut() = _io.read_u4le()?.into();
        *self_rc.start_time_ms.borrow_mut() = _io.read_u8le()?.into();
        *self_rc.timezone.borrow_mut() = _io.read_u4le()?.into();
        *self_rc.start_latitude.borrow_mut() = _io.read_f4le()?.into();
        *self_rc.start_longitude.borrow_mut() = _io.read_f4le()?.into();
        *self_rc.image_count.borrow_mut() = _io.read_u2le()?.into();
        *self_rc.video_count.borrow_mut() = _io.read_u2le()?.into();
        *self_rc.firmware_length.borrow_mut() = _io.read_u2le()?.into();
        *self_rc.firmware_info.borrow_mut() = bytes_to_str(&_io.read_bytes(*self_rc.firmware_length() as usize)?.into(), "UTF-8")?;
        Ok(())
    }
}
impl AutelDroneLog_HeaderT {
}
impl AutelDroneLog_HeaderT {
    pub fn header_size(&self) -> Ref<'_, u16> {
        self.header_size.borrow()
    }
}
impl AutelDroneLog_HeaderT {
    pub fn drone_serial(&self) -> Ref<'_, String> {
        self.drone_serial.borrow()
    }
}
impl AutelDroneLog_HeaderT {
    pub fn battery_serial(&self) -> Ref<'_, String> {
        self.battery_serial.borrow()
    }
}
impl AutelDroneLog_HeaderT {
    pub fn location(&self) -> Ref<'_, String> {
        self.location.borrow()
    }
}
impl AutelDroneLog_HeaderT {
    pub fn drone_type(&self) -> Ref<'_, u8> {
        self.drone_type.borrow()
    }
}
impl AutelDroneLog_HeaderT {
    pub fn distance(&self) -> Ref<'_, f32> {
        self.distance.borrow()
    }
}
impl AutelDroneLog_HeaderT {
    pub fn flight_duration_s(&self) -> Ref<'_, u32> {
        self.flight_duration_s.borrow()
    }
}
impl AutelDroneLog_HeaderT {
    pub fn max_altitude(&self) -> Ref<'_, f32> {
        self.max_altitude.borrow()
    }
}
impl AutelDroneLog_HeaderT {
    pub fn video_duration(&self) -> Ref<'_, u32> {
        self.video_duration.borrow()
    }
}
impl AutelDroneLog_HeaderT {
    pub fn start_time_ms(&self) -> Ref<'_, u64> {
        self.start_time_ms.borrow()
    }
}
impl AutelDroneLog_HeaderT {
    pub fn timezone(&self) -> Ref<'_, u32> {
        self.timezone.borrow()
    }
}
impl AutelDroneLog_HeaderT {
    pub fn start_latitude(&self) -> Ref<'_, f32> {
        self.start_latitude.borrow()
    }
}
impl AutelDroneLog_HeaderT {
    pub fn start_longitude(&self) -> Ref<'_, f32> {
        self.start_longitude.borrow()
    }
}
impl AutelDroneLog_HeaderT {
    pub fn image_count(&self) -> Ref<'_, u16> {
        self.image_count.borrow()
    }
}
impl AutelDroneLog_HeaderT {
    pub fn video_count(&self) -> Ref<'_, u16> {
        self.video_count.borrow()
    }
}
impl AutelDroneLog_HeaderT {
    pub fn firmware_length(&self) -> Ref<'_, u16> {
        self.firmware_length.borrow()
    }
}
impl AutelDroneLog_HeaderT {
    pub fn firmware_info(&self) -> Ref<'_, String> {
        self.firmware_info.borrow()
    }
}
impl AutelDroneLog_HeaderT {
    pub fn _io(&self) -> Ref<'_, BytesReader> {
        self._io.borrow()
    }
}

#[derive(Default, Debug, Clone)]
pub struct AutelDroneLog_HomeInfoT {
    pub _root: SharedType<AutelDroneLog>,
    pub _parent: SharedType<AutelDroneLog_ConfigurationT>,
    pub _self: SharedType<Self>,
    home_latitude: RefCell<f32>,
    home_longitude: RefCell<f32>,
    distance_to_home_m: RefCell<f32>,
    _io: RefCell<BytesReader>,
}
impl KStruct for AutelDroneLog_HomeInfoT {
    type Root = AutelDroneLog;
    type Parent = AutelDroneLog_ConfigurationT;

    fn read<S: KStream>(
        self_rc: &OptRc<Self>,
        _io: &S,
        _root: SharedType<Self::Root>,
        _parent: SharedType<Self::Parent>,
    ) -> KResult<()> {
        *self_rc._io.borrow_mut() = _io.clone();
        self_rc._root.set(_root.get());
        self_rc._parent.set(_parent.get());
        self_rc._self.set(Ok(self_rc.clone()));
        let _rrc = self_rc._root.get_value().borrow().upgrade();
        let _prc = self_rc._parent.get_value().borrow().upgrade();
        let _r = _rrc.as_ref().unwrap();
        *self_rc.home_latitude.borrow_mut() = _io.read_f4le()?.into();
        *self_rc.home_longitude.borrow_mut() = _io.read_f4le()?.into();
        *self_rc.distance_to_home_m.borrow_mut() = _io.read_f4le()?.into();
        Ok(())
    }
}
impl AutelDroneLog_HomeInfoT {
}
impl AutelDroneLog_HomeInfoT {
    pub fn home_latitude(&self) -> Ref<'_, f32> {
        self.home_latitude.borrow()
    }
}
impl AutelDroneLog_HomeInfoT {
    pub fn home_longitude(&self) -> Ref<'_, f32> {
        self.home_longitude.borrow()
    }
}
impl AutelDroneLog_HomeInfoT {
    pub fn distance_to_home_m(&self) -> Ref<'_, f32> {
        self.distance_to_home_m.borrow()
    }
}
impl AutelDroneLog_HomeInfoT {
    pub fn _io(&self) -> Ref<'_, BytesReader> {
        self._io.borrow()
    }
}

#[derive(Default, Debug, Clone)]
pub struct AutelDroneLog_OaSensorT {
    pub _root: SharedType<AutelDroneLog>,
    pub _parent: SharedType<KStructUnit>,
    pub _self: SharedType<Self>,
    timestamp: RefCell<f64>,
    front_sensor: RefCell<f32>,
    rear_sensor: RefCell<f32>,
    left_sensor: RefCell<f32>,
    right_sensor: RefCell<f32>,
    top_sensor: RefCell<f32>,
    bottom_sensor: RefCell<f32>,
    _io: RefCell<BytesReader>,
}
impl KStruct for AutelDroneLog_OaSensorT {
    type Root = AutelDroneLog;
    type Parent = KStructUnit;

    fn read<S: KStream>(
        self_rc: &OptRc<Self>,
        _io: &S,
        _root: SharedType<Self::Root>,
        _parent: SharedType<Self::Parent>,
    ) -> KResult<()> {
        *self_rc._io.borrow_mut() = _io.clone();
        self_rc._root.set(_root.get());
        self_rc._parent.set(_parent.get());
        self_rc._self.set(Ok(self_rc.clone()));
        let _rrc = self_rc._root.get_value().borrow().upgrade();
        let _prc = self_rc._parent.get_value().borrow().upgrade();
        let _r = _rrc.as_ref().unwrap();
        *self_rc.timestamp.borrow_mut() = _io.read_f8le()?.into();
        *self_rc.front_sensor.borrow_mut() = _io.read_f4le()?.into();
        *self_rc.rear_sensor.borrow_mut() = _io.read_f4le()?.into();
        *self_rc.left_sensor.borrow_mut() = _io.read_f4le()?.into();
        *self_rc.right_sensor.borrow_mut() = _io.read_f4le()?.into();
        *self_rc.top_sensor.borrow_mut() = _io.read_f4le()?.into();
        *self_rc.bottom_sensor.borrow_mut() = _io.read_f4le()?.into();
        Ok(())
    }
}
impl AutelDroneLog_OaSensorT {
}
impl AutelDroneLog_OaSensorT {
    pub fn timestamp(&self) -> Ref<'_, f64> {
        self.timestamp.borrow()
    }
}
impl AutelDroneLog_OaSensorT {
    pub fn front_sensor(&self) -> Ref<'_, f32> {
        self.front_sensor.borrow()
    }
}
impl AutelDroneLog_OaSensorT {
    pub fn rear_sensor(&self) -> Ref<'_, f32> {
        self.rear_sensor.borrow()
    }
}
impl AutelDroneLog_OaSensorT {
    pub fn left_sensor(&self) -> Ref<'_, f32> {
        self.left_sensor.borrow()
    }
}
impl AutelDroneLog_OaSensorT {
    pub fn right_sensor(&self) -> Ref<'_, f32> {
        self.right_sensor.borrow()
    }
}
impl AutelDroneLog_OaSensorT {
    pub fn top_sensor(&self) -> Ref<'_, f32> {
        self.top_sensor.borrow()
    }
}
impl AutelDroneLog_OaSensorT {
    pub fn bottom_sensor(&self) -> Ref<'_, f32> {
        self.bottom_sensor.borrow()
    }
}
impl AutelDroneLog_OaSensorT {
    pub fn _io(&self) -> Ref<'_, BytesReader> {
        self._io.borrow()
    }
}

/**
 * Inserted into the stream when the photo is taken.
 * I think once the image is saved rather than when the button is pressed, but I could be wrong.
 */

#[derive(Default, Debug, Clone)]
pub struct AutelDroneLog_PhotoRecordT {
    pub _root: SharedType<AutelDroneLog>,
    pub _parent: SharedType<AutelDroneLog_RecordT>,
    pub _self: SharedType<Self>,
    file_name: RefCell<String>,
    timestamp: RefCell<u64>,
    latitude: RefCell<f32>,
    longitude: RefCell<f32>,
    _io: RefCell<BytesReader>,
}
impl KStruct for AutelDroneLog_PhotoRecordT {
    type Root = AutelDroneLog;
    type Parent = AutelDroneLog_RecordT;

    fn read<S: KStream>(
        self_rc: &OptRc<Self>,
        _io: &S,
        _root: SharedType<Self::Root>,
        _parent: SharedType<Self::Parent>,
    ) -> KResult<()> {
        *self_rc._io.borrow_mut() = _io.clone();
        self_rc._root.set(_root.get());
        self_rc._parent.set(_parent.get());
        self_rc._self.set(Ok(self_rc.clone()));
        let _rrc = self_rc._root.get_value().borrow().upgrade();
        let _prc = self_rc._parent.get_value().borrow().upgrade();
        let _r = _rrc.as_ref().unwrap();
        *self_rc.file_name.borrow_mut() = bytes_to_str(&_io.read_bytes(64 as usize)?.into(), "UTF-8")?;
        *self_rc.timestamp.borrow_mut() = _io.read_u8le()?.into();
        *self_rc.latitude.borrow_mut() = _io.read_f4le()?.into();
        *self_rc.longitude.borrow_mut() = _io.read_f4le()?.into();
        Ok(())
    }
}
impl AutelDroneLog_PhotoRecordT {
}
impl AutelDroneLog_PhotoRecordT {
    pub fn file_name(&self) -> Ref<'_, String> {
        self.file_name.borrow()
    }
}

/**
 * Milliseconds since Unix epoch rather than "mission" time.
 */
impl AutelDroneLog_PhotoRecordT {
    pub fn timestamp(&self) -> Ref<'_, u64> {
        self.timestamp.borrow()
    }
}
impl AutelDroneLog_PhotoRecordT {
    pub fn latitude(&self) -> Ref<'_, f32> {
        self.latitude.borrow()
    }
}
impl AutelDroneLog_PhotoRecordT {
    pub fn longitude(&self) -> Ref<'_, f32> {
        self.longitude.borrow()
    }
}
impl AutelDroneLog_PhotoRecordT {
    pub fn _io(&self) -> Ref<'_, BytesReader> {
        self._io.borrow()
    }
}

#[derive(Default, Debug, Clone)]
pub struct AutelDroneLog_PowerInfoT {
    pub _root: SharedType<AutelDroneLog>,
    pub _parent: SharedType<AutelDroneLog_ConfigurationT>,
    pub _self: SharedType<Self>,
    designed_volume: RefCell<u32>,
    maximum_capacity_mah: RefCell<u32>,
    current_capacity_mah: RefCell<f32>,
    voltage_mv: RefCell<f32>,
    current_draw: RefCell<f32>,
    remaining_charge_pc: RefCell<u8>,
    battery_temperature_deg_c: RefCell<f32>,
    battery_state: RefCell<AutelDroneLog_EBatteryState>,
    battery_discharge_count: RefCell<u32>,
    cell_count: RefCell<u8>,
    cell_voltage: RefCell<Vec<f32>>,
    _io: RefCell<BytesReader>,
}
impl KStruct for AutelDroneLog_PowerInfoT {
    type Root = AutelDroneLog;
    type Parent = AutelDroneLog_ConfigurationT;

    fn read<S: KStream>(
        self_rc: &OptRc<Self>,
        _io: &S,
        _root: SharedType<Self::Root>,
        _parent: SharedType<Self::Parent>,
    ) -> KResult<()> {
        *self_rc._io.borrow_mut() = _io.clone();
        self_rc._root.set(_root.get());
        self_rc._parent.set(_parent.get());
        self_rc._self.set(Ok(self_rc.clone()));
        let _rrc = self_rc._root.get_value().borrow().upgrade();
        let _prc = self_rc._parent.get_value().borrow().upgrade();
        let _r = _rrc.as_ref().unwrap();
        *self_rc.designed_volume.borrow_mut() = _io.read_u4le()?.into();
        *self_rc.maximum_capacity_mah.borrow_mut() = _io.read_u4le()?.into();
        *self_rc.current_capacity_mah.borrow_mut() = _io.read_f4le()?.into();
        *self_rc.voltage_mv.borrow_mut() = _io.read_f4le()?.into();
        *self_rc.current_draw.borrow_mut() = _io.read_f4le()?.into();
        *self_rc.remaining_charge_pc.borrow_mut() = _io.read_u1()?.into();
        *self_rc.battery_temperature_deg_c.borrow_mut() = _io.read_f4le()?.into();
        *self_rc.battery_state.borrow_mut() = (_io.read_u1()? as i64).try_into()?;
        *self_rc.battery_discharge_count.borrow_mut() = _io.read_u4le()?.into();
        *self_rc.cell_count.borrow_mut() = _io.read_u1()?.into();
        *self_rc.cell_voltage.borrow_mut() = Vec::new();
        let l_cell_voltage = 8;
        for _i in 0..l_cell_voltage {
            self_rc.cell_voltage.borrow_mut().push(_io.read_f4le()?.into());
        }
        Ok(())
    }
}
impl AutelDroneLog_PowerInfoT {
}

/**
 * Unknown meaning. Always seems to match battery_discharge_count, but perhaps should be the nominal capacity?
 * My Evo II v3 battery XE3_7100_1155 has a nominal capacity of 7100mAh.
 */
impl AutelDroneLog_PowerInfoT {
    pub fn designed_volume(&self) -> Ref<'_, u32> {
        self.designed_volume.borrow()
    }
}
impl AutelDroneLog_PowerInfoT {
    pub fn maximum_capacity_mah(&self) -> Ref<'_, u32> {
        self.maximum_capacity_mah.borrow()
    }
}
impl AutelDroneLog_PowerInfoT {
    pub fn current_capacity_mah(&self) -> Ref<'_, f32> {
        self.current_capacity_mah.borrow()
    }
}
impl AutelDroneLog_PowerInfoT {
    pub fn voltage_mv(&self) -> Ref<'_, f32> {
        self.voltage_mv.borrow()
    }
}
impl AutelDroneLog_PowerInfoT {
    pub fn current_draw(&self) -> Ref<'_, f32> {
        self.current_draw.borrow()
    }
}
impl AutelDroneLog_PowerInfoT {
    pub fn remaining_charge_pc(&self) -> Ref<'_, u8> {
        self.remaining_charge_pc.borrow()
    }
}
impl AutelDroneLog_PowerInfoT {
    pub fn battery_temperature_deg_c(&self) -> Ref<'_, f32> {
        self.battery_temperature_deg_c.borrow()
    }
}

/**
 * Flags whether the battery is too hot or too cold to discharge.
 */
impl AutelDroneLog_PowerInfoT {
    pub fn battery_state(&self) -> Ref<'_, AutelDroneLog_EBatteryState> {
        self.battery_state.borrow()
    }
}

/**
 * Really seems to be the number of charges rather than discharges.
 * That is, using it twice without charging in between doesn't increase the counter.
 */
impl AutelDroneLog_PowerInfoT {
    pub fn battery_discharge_count(&self) -> Ref<'_, u32> {
        self.battery_discharge_count.borrow()
    }
}
impl AutelDroneLog_PowerInfoT {
    pub fn cell_count(&self) -> Ref<'_, u8> {
        self.cell_count.borrow()
    }
}
impl AutelDroneLog_PowerInfoT {
    pub fn cell_voltage(&self) -> Ref<'_, Vec<f32>> {
        self.cell_voltage.borrow()
    }
}
impl AutelDroneLog_PowerInfoT {
    pub fn _io(&self) -> Ref<'_, BytesReader> {
        self._io.borrow()
    }
}

#[derive(Default, Debug, Clone)]
pub struct AutelDroneLog_RecordT {
    pub _root: SharedType<AutelDroneLog>,
    pub _parent: SharedType<AutelDroneLog>,
    pub _self: SharedType<Self>,
    record_type: RefCell<u8>,
    body: RefCell<Option<AutelDroneLog_RecordT_Body>>,
    _io: RefCell<BytesReader>,
}
#[derive(Debug, Clone)]
pub enum AutelDroneLog_RecordT_Body {
    AutelDroneLog_FullRecordT(OptRc<AutelDroneLog_FullRecordT>),
    AutelDroneLog_BaseRecordT(OptRc<AutelDroneLog_BaseRecordT>),
    AutelDroneLog_PhotoRecordT(OptRc<AutelDroneLog_PhotoRecordT>),
    AutelDroneLog_VideoRecordT(OptRc<AutelDroneLog_VideoRecordT>),
}
impl From<&AutelDroneLog_RecordT_Body> for OptRc<AutelDroneLog_FullRecordT> {
    fn from(v: &AutelDroneLog_RecordT_Body) -> Self {
        if let AutelDroneLog_RecordT_Body::AutelDroneLog_FullRecordT(x) = v {
            return x.clone();
        }
        panic!("expected AutelDroneLog_RecordT_Body::AutelDroneLog_FullRecordT, got {:?}", v)
    }
}
impl From<OptRc<AutelDroneLog_FullRecordT>> for AutelDroneLog_RecordT_Body {
    fn from(v: OptRc<AutelDroneLog_FullRecordT>) -> Self {
        Self::AutelDroneLog_FullRecordT(v)
    }
}
impl From<&AutelDroneLog_RecordT_Body> for OptRc<AutelDroneLog_BaseRecordT> {
    fn from(v: &AutelDroneLog_RecordT_Body) -> Self {
        if let AutelDroneLog_RecordT_Body::AutelDroneLog_BaseRecordT(x) = v {
            return x.clone();
        }
        panic!("expected AutelDroneLog_RecordT_Body::AutelDroneLog_BaseRecordT, got {:?}", v)
    }
}
impl From<OptRc<AutelDroneLog_BaseRecordT>> for AutelDroneLog_RecordT_Body {
    fn from(v: OptRc<AutelDroneLog_BaseRecordT>) -> Self {
        Self::AutelDroneLog_BaseRecordT(v)
    }
}
impl From<&AutelDroneLog_RecordT_Body> for OptRc<AutelDroneLog_PhotoRecordT> {
    fn from(v: &AutelDroneLog_RecordT_Body) -> Self {
        if let AutelDroneLog_RecordT_Body::AutelDroneLog_PhotoRecordT(x) = v {
            return x.clone();
        }
        panic!("expected AutelDroneLog_RecordT_Body::AutelDroneLog_PhotoRecordT, got {:?}", v)
    }
}
impl From<OptRc<AutelDroneLog_PhotoRecordT>> for AutelDroneLog_RecordT_Body {
    fn from(v: OptRc<AutelDroneLog_PhotoRecordT>) -> Self {
        Self::AutelDroneLog_PhotoRecordT(v)
    }
}
impl From<&AutelDroneLog_RecordT_Body> for OptRc<AutelDroneLog_VideoRecordT> {
    fn from(v: &AutelDroneLog_RecordT_Body) -> Self {
        if let AutelDroneLog_RecordT_Body::AutelDroneLog_VideoRecordT(x) = v {
            return x.clone();
        }
        panic!("expected AutelDroneLog_RecordT_Body::AutelDroneLog_VideoRecordT, got {:?}", v)
    }
}
impl From<OptRc<AutelDroneLog_VideoRecordT>> for AutelDroneLog_RecordT_Body {
    fn from(v: OptRc<AutelDroneLog_VideoRecordT>) -> Self {
        Self::AutelDroneLog_VideoRecordT(v)
    }
}
impl KStruct for AutelDroneLog_RecordT {
    type Root = AutelDroneLog;
    type Parent = AutelDroneLog;

    fn read<S: KStream>(
        self_rc: &OptRc<Self>,
        _io: &S,
        _root: SharedType<Self::Root>,
        _parent: SharedType<Self::Parent>,
    ) -> KResult<()> {
        *self_rc._io.borrow_mut() = _io.clone();
        self_rc._root.set(_root.get());
        self_rc._parent.set(_parent.get());
        self_rc._self.set(Ok(self_rc.clone()));
        let _rrc = self_rc._root.get_value().borrow().upgrade();
        let _prc = self_rc._parent.get_value().borrow().upgrade();
        let _r = _rrc.as_ref().unwrap();
        *self_rc.record_type.borrow_mut() = _io.read_u1()?.into();
        match *self_rc.record_type() {
            0 => {
                let t = Self::read_into::<_, AutelDroneLog_FullRecordT>(&*_io, Some(self_rc._root.clone()), Some(self_rc._self.clone()))?.into();
                *self_rc.body.borrow_mut() = Some(t);
            }
            1 => {
                let t = Self::read_into::<_, AutelDroneLog_BaseRecordT>(&*_io, Some(self_rc._root.clone()), Some(self_rc._self.clone()))?.into();
                *self_rc.body.borrow_mut() = Some(t);
            }
            14 => {
                let t = Self::read_into::<_, AutelDroneLog_PhotoRecordT>(&*_io, Some(self_rc._root.clone()), Some(self_rc._self.clone()))?.into();
                *self_rc.body.borrow_mut() = Some(t);
            }
            15 => {
                let t = Self::read_into::<_, AutelDroneLog_VideoRecordT>(&*_io, Some(self_rc._root.clone()), Some(self_rc._self.clone()))?.into();
                *self_rc.body.borrow_mut() = Some(t);
            }
            _ => {}
        }
        Ok(())
    }
}
impl AutelDroneLog_RecordT {
}

/**
 * In the code there are 6 record types, but only 4 seem to appear in the wild.
 * The other two mirror the full_record_t and base_record_t but are prefixed with InSide in the code.
 * This might be for some sort of internal logging perhaps. They appear to be identical to their OutSide
 * companions except the InSide versions lack drone latitude and longitude fields, and some others.
 * If they need implementing, look here com.autel.modelblib.lib.domain.model.flightlog.engine.FlightRecordInSideFullModel
 * in the Explorer apk. InSideFull has a record type of 2, and InSideBase has a record type of 3.
 * I have a couple of corrupt log files that seem to have some InSide records, so my conjecture is that
 * InSide records get logged at flight time, then turned into OutSide records once the drone has landed.
 */
impl AutelDroneLog_RecordT {
    pub fn record_type(&self) -> Ref<'_, u8> {
        self.record_type.borrow()
    }
}
impl AutelDroneLog_RecordT {
    pub fn body(&self) -> Ref<'_, Option<AutelDroneLog_RecordT_Body>> {
        self.body.borrow()
    }
}
impl AutelDroneLog_RecordT {
    pub fn _io(&self) -> Ref<'_, BytesReader> {
        self._io.borrow()
    }
}

#[derive(Default, Debug, Clone)]
pub struct AutelDroneLog_SettingsT {
    pub _root: SharedType<AutelDroneLog>,
    pub _parent: SharedType<AutelDroneLog_ConfigurationT>,
    pub _self: SharedType<Self>,
    oa_warning_enabled: RefCell<u32>,
    oa_ext_warning_enabled: RefCell<u32>,
    oa_error_code: RefCell<u32>,
    max_altitude_m: RefCell<f32>,
    return_to_home_altitude_m: RefCell<f32>,
    beginner_mode_enabled: RefCell<u8>,
    low_battery_threshold_enabled: RefCell<u8>,
    critical_battery_threshold_enabled: RefCell<u8>,
    max_flight_distance_m: RefCell<f32>,
    max_horizontal_speed: RefCell<f32>,
    oa_enabled: RefCell<u8>,
    radar_enabled: RefCell<u8>,
    max_error: RefCell<u32>,
    _io: RefCell<BytesReader>,
}
impl KStruct for AutelDroneLog_SettingsT {
    type Root = AutelDroneLog;
    type Parent = AutelDroneLog_ConfigurationT;

    fn read<S: KStream>(
        self_rc: &OptRc<Self>,
        _io: &S,
        _root: SharedType<Self::Root>,
        _parent: SharedType<Self::Parent>,
    ) -> KResult<()> {
        *self_rc._io.borrow_mut() = _io.clone();
        self_rc._root.set(_root.get());
        self_rc._parent.set(_parent.get());
        self_rc._self.set(Ok(self_rc.clone()));
        let _rrc = self_rc._root.get_value().borrow().upgrade();
        let _prc = self_rc._parent.get_value().borrow().upgrade();
        let _r = _rrc.as_ref().unwrap();
        *self_rc.oa_warning_enabled.borrow_mut() = _io.read_u4le()?.into();
        *self_rc.oa_ext_warning_enabled.borrow_mut() = _io.read_u4le()?.into();
        *self_rc.oa_error_code.borrow_mut() = _io.read_u4le()?.into();
        *self_rc.max_altitude_m.borrow_mut() = _io.read_f4le()?.into();
        *self_rc.return_to_home_altitude_m.borrow_mut() = _io.read_f4le()?.into();
        *self_rc.beginner_mode_enabled.borrow_mut() = _io.read_u1()?.into();
        *self_rc.low_battery_threshold_enabled.borrow_mut() = _io.read_u1()?.into();
        *self_rc.critical_battery_threshold_enabled.borrow_mut() = _io.read_u1()?.into();
        *self_rc.max_flight_distance_m.borrow_mut() = _io.read_f4le()?.into();
        *self_rc.max_horizontal_speed.borrow_mut() = _io.read_f4le()?.into();
        *self_rc.oa_enabled.borrow_mut() = _io.read_u1()?.into();
        *self_rc.radar_enabled.borrow_mut() = _io.read_u1()?.into();
        *self_rc.max_error.borrow_mut() = _io.read_u4le()?.into();
        Ok(())
    }
}
impl AutelDroneLog_SettingsT {
}
impl AutelDroneLog_SettingsT {
    pub fn oa_warning_enabled(&self) -> Ref<'_, u32> {
        self.oa_warning_enabled.borrow()
    }
}
impl AutelDroneLog_SettingsT {
    pub fn oa_ext_warning_enabled(&self) -> Ref<'_, u32> {
        self.oa_ext_warning_enabled.borrow()
    }
}
impl AutelDroneLog_SettingsT {
    pub fn oa_error_code(&self) -> Ref<'_, u32> {
        self.oa_error_code.borrow()
    }
}
impl AutelDroneLog_SettingsT {
    pub fn max_altitude_m(&self) -> Ref<'_, f32> {
        self.max_altitude_m.borrow()
    }
}
impl AutelDroneLog_SettingsT {
    pub fn return_to_home_altitude_m(&self) -> Ref<'_, f32> {
        self.return_to_home_altitude_m.borrow()
    }
}
impl AutelDroneLog_SettingsT {
    pub fn beginner_mode_enabled(&self) -> Ref<'_, u8> {
        self.beginner_mode_enabled.borrow()
    }
}
impl AutelDroneLog_SettingsT {
    pub fn low_battery_threshold_enabled(&self) -> Ref<'_, u8> {
        self.low_battery_threshold_enabled.borrow()
    }
}
impl AutelDroneLog_SettingsT {
    pub fn critical_battery_threshold_enabled(&self) -> Ref<'_, u8> {
        self.critical_battery_threshold_enabled.borrow()
    }
}
impl AutelDroneLog_SettingsT {
    pub fn max_flight_distance_m(&self) -> Ref<'_, f32> {
        self.max_flight_distance_m.borrow()
    }
}
impl AutelDroneLog_SettingsT {
    pub fn max_horizontal_speed(&self) -> Ref<'_, f32> {
        self.max_horizontal_speed.borrow()
    }
}
impl AutelDroneLog_SettingsT {
    pub fn oa_enabled(&self) -> Ref<'_, u8> {
        self.oa_enabled.borrow()
    }
}
impl AutelDroneLog_SettingsT {
    pub fn radar_enabled(&self) -> Ref<'_, u8> {
        self.radar_enabled.borrow()
    }
}
impl AutelDroneLog_SettingsT {
    pub fn max_error(&self) -> Ref<'_, u32> {
        self.max_error.borrow()
    }
}
impl AutelDroneLog_SettingsT {
    pub fn _io(&self) -> Ref<'_, BytesReader> {
        self._io.borrow()
    }
}

/**
 * Inserted into the stream _when the video has finished_ BUT with a timestamp of when it was started!
 */

#[derive(Default, Debug, Clone)]
pub struct AutelDroneLog_VideoRecordT {
    pub _root: SharedType<AutelDroneLog>,
    pub _parent: SharedType<AutelDroneLog_RecordT>,
    pub _self: SharedType<Self>,
    file_name: RefCell<String>,
    timestamp: RefCell<u64>,
    latitude: RefCell<f32>,
    longitude: RefCell<f32>,
    duration_s: RefCell<u32>,
    _io: RefCell<BytesReader>,
}
impl KStruct for AutelDroneLog_VideoRecordT {
    type Root = AutelDroneLog;
    type Parent = AutelDroneLog_RecordT;

    fn read<S: KStream>(
        self_rc: &OptRc<Self>,
        _io: &S,
        _root: SharedType<Self::Root>,
        _parent: SharedType<Self::Parent>,
    ) -> KResult<()> {
        *self_rc._io.borrow_mut() = _io.clone();
        self_rc._root.set(_root.get());
        self_rc._parent.set(_parent.get());
        self_rc._self.set(Ok(self_rc.clone()));
        let _rrc = self_rc._root.get_value().borrow().upgrade();
        let _prc = self_rc._parent.get_value().borrow().upgrade();
        let _r = _rrc.as_ref().unwrap();
        *self_rc.file_name.borrow_mut() = bytes_to_str(&_io.read_bytes(64 as usize)?.into(), "UTF-8")?;
        *self_rc.timestamp.borrow_mut() = _io.read_u8le()?.into();
        *self_rc.latitude.borrow_mut() = _io.read_f4le()?.into();
        *self_rc.longitude.borrow_mut() = _io.read_f4le()?.into();
        *self_rc.duration_s.borrow_mut() = _io.read_u4le()?.into();
        Ok(())
    }
}
impl AutelDroneLog_VideoRecordT {
}
impl AutelDroneLog_VideoRecordT {
    pub fn file_name(&self) -> Ref<'_, String> {
        self.file_name.borrow()
    }
}

/**
 * Milliseconds since Unix epoch rather than "mission" time of when the video started recording.
 */
impl AutelDroneLog_VideoRecordT {
    pub fn timestamp(&self) -> Ref<'_, u64> {
        self.timestamp.borrow()
    }
}
impl AutelDroneLog_VideoRecordT {
    pub fn latitude(&self) -> Ref<'_, f32> {
        self.latitude.borrow()
    }
}
impl AutelDroneLog_VideoRecordT {
    pub fn longitude(&self) -> Ref<'_, f32> {
        self.longitude.borrow()
    }
}
impl AutelDroneLog_VideoRecordT {
    pub fn duration_s(&self) -> Ref<'_, u32> {
        self.duration_s.borrow()
    }
}
impl AutelDroneLog_VideoRecordT {
    pub fn _io(&self) -> Ref<'_, BytesReader> {
        self._io.borrow()
    }
}
