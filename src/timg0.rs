#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - "]
    pub t0config: crate::Reg<t0config::T0CONFIG_SPEC>,
    #[doc = "0x04 - "]
    pub t0lo: crate::Reg<t0lo::T0LO_SPEC>,
    #[doc = "0x08 - "]
    pub t0hi: crate::Reg<t0hi::T0HI_SPEC>,
    #[doc = "0x0c - "]
    pub t0update: crate::Reg<t0update::T0UPDATE_SPEC>,
    #[doc = "0x10 - "]
    pub t0alarmlo: crate::Reg<t0alarmlo::T0ALARMLO_SPEC>,
    #[doc = "0x14 - "]
    pub t0alarmhi: crate::Reg<t0alarmhi::T0ALARMHI_SPEC>,
    #[doc = "0x18 - "]
    pub t0loadlo: crate::Reg<t0loadlo::T0LOADLO_SPEC>,
    #[doc = "0x1c - "]
    pub t0loadhi: crate::Reg<t0loadhi::T0LOADHI_SPEC>,
    #[doc = "0x20 - "]
    pub t0load: crate::Reg<t0load::T0LOAD_SPEC>,
    #[doc = "0x24 - "]
    pub t1config: crate::Reg<t1config::T1CONFIG_SPEC>,
    #[doc = "0x28 - "]
    pub t1lo: crate::Reg<t1lo::T1LO_SPEC>,
    #[doc = "0x2c - "]
    pub t1hi: crate::Reg<t1hi::T1HI_SPEC>,
    #[doc = "0x30 - "]
    pub t1update: crate::Reg<t1update::T1UPDATE_SPEC>,
    #[doc = "0x34 - "]
    pub t1alarmlo: crate::Reg<t1alarmlo::T1ALARMLO_SPEC>,
    #[doc = "0x38 - "]
    pub t1alarmhi: crate::Reg<t1alarmhi::T1ALARMHI_SPEC>,
    #[doc = "0x3c - "]
    pub t1loadlo: crate::Reg<t1loadlo::T1LOADLO_SPEC>,
    #[doc = "0x40 - "]
    pub t1loadhi: crate::Reg<t1loadhi::T1LOADHI_SPEC>,
    #[doc = "0x44 - "]
    pub t1load: crate::Reg<t1load::T1LOAD_SPEC>,
    #[doc = "0x48 - "]
    pub wdtconfig0: crate::Reg<wdtconfig0::WDTCONFIG0_SPEC>,
    #[doc = "0x4c - "]
    pub wdtconfig1: crate::Reg<wdtconfig1::WDTCONFIG1_SPEC>,
    #[doc = "0x50 - "]
    pub wdtconfig2: crate::Reg<wdtconfig2::WDTCONFIG2_SPEC>,
    #[doc = "0x54 - "]
    pub wdtconfig3: crate::Reg<wdtconfig3::WDTCONFIG3_SPEC>,
    #[doc = "0x58 - "]
    pub wdtconfig4: crate::Reg<wdtconfig4::WDTCONFIG4_SPEC>,
    #[doc = "0x5c - "]
    pub wdtconfig5: crate::Reg<wdtconfig5::WDTCONFIG5_SPEC>,
    #[doc = "0x60 - "]
    pub wdtfeed: crate::Reg<wdtfeed::WDTFEED_SPEC>,
    #[doc = "0x64 - "]
    pub wdtwprotect: crate::Reg<wdtwprotect::WDTWPROTECT_SPEC>,
    #[doc = "0x68 - "]
    pub rtccalicfg: crate::Reg<rtccalicfg::RTCCALICFG_SPEC>,
    #[doc = "0x6c - "]
    pub rtccalicfg1: crate::Reg<rtccalicfg1::RTCCALICFG1_SPEC>,
    #[doc = "0x70 - "]
    pub lactconfig: crate::Reg<lactconfig::LACTCONFIG_SPEC>,
    #[doc = "0x74 - "]
    pub lactrtc: crate::Reg<lactrtc::LACTRTC_SPEC>,
    #[doc = "0x78 - "]
    pub lactlo: crate::Reg<lactlo::LACTLO_SPEC>,
    #[doc = "0x7c - "]
    pub lacthi: crate::Reg<lacthi::LACTHI_SPEC>,
    #[doc = "0x80 - "]
    pub lactupdate: crate::Reg<lactupdate::LACTUPDATE_SPEC>,
    #[doc = "0x84 - "]
    pub lactalarmlo: crate::Reg<lactalarmlo::LACTALARMLO_SPEC>,
    #[doc = "0x88 - "]
    pub lactalarmhi: crate::Reg<lactalarmhi::LACTALARMHI_SPEC>,
    #[doc = "0x8c - "]
    pub lactloadlo: crate::Reg<lactloadlo::LACTLOADLO_SPEC>,
    #[doc = "0x90 - "]
    pub lactloadhi: crate::Reg<lactloadhi::LACTLOADHI_SPEC>,
    #[doc = "0x94 - "]
    pub lactload: crate::Reg<lactload::LACTLOAD_SPEC>,
    #[doc = "0x98 - "]
    pub int_ena_timers: crate::Reg<int_ena_timers::INT_ENA_TIMERS_SPEC>,
    #[doc = "0x9c - "]
    pub int_raw_timers: crate::Reg<int_raw_timers::INT_RAW_TIMERS_SPEC>,
    #[doc = "0xa0 - "]
    pub int_st_timers: crate::Reg<int_st_timers::INT_ST_TIMERS_SPEC>,
    #[doc = "0xa4 - "]
    pub int_clr_timers: crate::Reg<int_clr_timers::INT_CLR_TIMERS_SPEC>,
    _reserved42: [u8; 0x50],
    #[doc = "0xf8 - "]
    pub ntimers_date: crate::Reg<ntimers_date::NTIMERS_DATE_SPEC>,
    #[doc = "0xfc - "]
    pub timgclk: crate::Reg<timgclk::TIMGCLK_SPEC>,
}
#[doc = "T0CONFIG register accessor: an alias for `Reg<T0CONFIG_SPEC>`"]
pub type T0CONFIG = crate::Reg<t0config::T0CONFIG_SPEC>;
#[doc = ""]
pub mod t0config;
#[doc = "T0LO register accessor: an alias for `Reg<T0LO_SPEC>`"]
pub type T0LO = crate::Reg<t0lo::T0LO_SPEC>;
#[doc = ""]
pub mod t0lo;
#[doc = "T0HI register accessor: an alias for `Reg<T0HI_SPEC>`"]
pub type T0HI = crate::Reg<t0hi::T0HI_SPEC>;
#[doc = ""]
pub mod t0hi;
#[doc = "T0UPDATE register accessor: an alias for `Reg<T0UPDATE_SPEC>`"]
pub type T0UPDATE = crate::Reg<t0update::T0UPDATE_SPEC>;
#[doc = ""]
pub mod t0update;
#[doc = "T0ALARMLO register accessor: an alias for `Reg<T0ALARMLO_SPEC>`"]
pub type T0ALARMLO = crate::Reg<t0alarmlo::T0ALARMLO_SPEC>;
#[doc = ""]
pub mod t0alarmlo;
#[doc = "T0ALARMHI register accessor: an alias for `Reg<T0ALARMHI_SPEC>`"]
pub type T0ALARMHI = crate::Reg<t0alarmhi::T0ALARMHI_SPEC>;
#[doc = ""]
pub mod t0alarmhi;
#[doc = "T0LOADLO register accessor: an alias for `Reg<T0LOADLO_SPEC>`"]
pub type T0LOADLO = crate::Reg<t0loadlo::T0LOADLO_SPEC>;
#[doc = ""]
pub mod t0loadlo;
#[doc = "T0LOADHI register accessor: an alias for `Reg<T0LOADHI_SPEC>`"]
pub type T0LOADHI = crate::Reg<t0loadhi::T0LOADHI_SPEC>;
#[doc = ""]
pub mod t0loadhi;
#[doc = "T0LOAD register accessor: an alias for `Reg<T0LOAD_SPEC>`"]
pub type T0LOAD = crate::Reg<t0load::T0LOAD_SPEC>;
#[doc = ""]
pub mod t0load;
#[doc = "T1CONFIG register accessor: an alias for `Reg<T1CONFIG_SPEC>`"]
pub type T1CONFIG = crate::Reg<t1config::T1CONFIG_SPEC>;
#[doc = ""]
pub mod t1config;
#[doc = "T1LO register accessor: an alias for `Reg<T1LO_SPEC>`"]
pub type T1LO = crate::Reg<t1lo::T1LO_SPEC>;
#[doc = ""]
pub mod t1lo;
#[doc = "T1HI register accessor: an alias for `Reg<T1HI_SPEC>`"]
pub type T1HI = crate::Reg<t1hi::T1HI_SPEC>;
#[doc = ""]
pub mod t1hi;
#[doc = "T1UPDATE register accessor: an alias for `Reg<T1UPDATE_SPEC>`"]
pub type T1UPDATE = crate::Reg<t1update::T1UPDATE_SPEC>;
#[doc = ""]
pub mod t1update;
#[doc = "T1ALARMLO register accessor: an alias for `Reg<T1ALARMLO_SPEC>`"]
pub type T1ALARMLO = crate::Reg<t1alarmlo::T1ALARMLO_SPEC>;
#[doc = ""]
pub mod t1alarmlo;
#[doc = "T1ALARMHI register accessor: an alias for `Reg<T1ALARMHI_SPEC>`"]
pub type T1ALARMHI = crate::Reg<t1alarmhi::T1ALARMHI_SPEC>;
#[doc = ""]
pub mod t1alarmhi;
#[doc = "T1LOADLO register accessor: an alias for `Reg<T1LOADLO_SPEC>`"]
pub type T1LOADLO = crate::Reg<t1loadlo::T1LOADLO_SPEC>;
#[doc = ""]
pub mod t1loadlo;
#[doc = "T1LOADHI register accessor: an alias for `Reg<T1LOADHI_SPEC>`"]
pub type T1LOADHI = crate::Reg<t1loadhi::T1LOADHI_SPEC>;
#[doc = ""]
pub mod t1loadhi;
#[doc = "T1LOAD register accessor: an alias for `Reg<T1LOAD_SPEC>`"]
pub type T1LOAD = crate::Reg<t1load::T1LOAD_SPEC>;
#[doc = ""]
pub mod t1load;
#[doc = "WDTCONFIG0 register accessor: an alias for `Reg<WDTCONFIG0_SPEC>`"]
pub type WDTCONFIG0 = crate::Reg<wdtconfig0::WDTCONFIG0_SPEC>;
#[doc = ""]
pub mod wdtconfig0;
#[doc = "WDTCONFIG1 register accessor: an alias for `Reg<WDTCONFIG1_SPEC>`"]
pub type WDTCONFIG1 = crate::Reg<wdtconfig1::WDTCONFIG1_SPEC>;
#[doc = ""]
pub mod wdtconfig1;
#[doc = "WDTCONFIG2 register accessor: an alias for `Reg<WDTCONFIG2_SPEC>`"]
pub type WDTCONFIG2 = crate::Reg<wdtconfig2::WDTCONFIG2_SPEC>;
#[doc = ""]
pub mod wdtconfig2;
#[doc = "WDTCONFIG3 register accessor: an alias for `Reg<WDTCONFIG3_SPEC>`"]
pub type WDTCONFIG3 = crate::Reg<wdtconfig3::WDTCONFIG3_SPEC>;
#[doc = ""]
pub mod wdtconfig3;
#[doc = "WDTCONFIG4 register accessor: an alias for `Reg<WDTCONFIG4_SPEC>`"]
pub type WDTCONFIG4 = crate::Reg<wdtconfig4::WDTCONFIG4_SPEC>;
#[doc = ""]
pub mod wdtconfig4;
#[doc = "WDTCONFIG5 register accessor: an alias for `Reg<WDTCONFIG5_SPEC>`"]
pub type WDTCONFIG5 = crate::Reg<wdtconfig5::WDTCONFIG5_SPEC>;
#[doc = ""]
pub mod wdtconfig5;
#[doc = "WDTFEED register accessor: an alias for `Reg<WDTFEED_SPEC>`"]
pub type WDTFEED = crate::Reg<wdtfeed::WDTFEED_SPEC>;
#[doc = ""]
pub mod wdtfeed;
#[doc = "WDTWPROTECT register accessor: an alias for `Reg<WDTWPROTECT_SPEC>`"]
pub type WDTWPROTECT = crate::Reg<wdtwprotect::WDTWPROTECT_SPEC>;
#[doc = ""]
pub mod wdtwprotect;
#[doc = "RTCCALICFG register accessor: an alias for `Reg<RTCCALICFG_SPEC>`"]
pub type RTCCALICFG = crate::Reg<rtccalicfg::RTCCALICFG_SPEC>;
#[doc = ""]
pub mod rtccalicfg;
#[doc = "RTCCALICFG1 register accessor: an alias for `Reg<RTCCALICFG1_SPEC>`"]
pub type RTCCALICFG1 = crate::Reg<rtccalicfg1::RTCCALICFG1_SPEC>;
#[doc = ""]
pub mod rtccalicfg1;
#[doc = "LACTCONFIG register accessor: an alias for `Reg<LACTCONFIG_SPEC>`"]
pub type LACTCONFIG = crate::Reg<lactconfig::LACTCONFIG_SPEC>;
#[doc = ""]
pub mod lactconfig;
#[doc = "LACTRTC register accessor: an alias for `Reg<LACTRTC_SPEC>`"]
pub type LACTRTC = crate::Reg<lactrtc::LACTRTC_SPEC>;
#[doc = ""]
pub mod lactrtc;
#[doc = "LACTLO register accessor: an alias for `Reg<LACTLO_SPEC>`"]
pub type LACTLO = crate::Reg<lactlo::LACTLO_SPEC>;
#[doc = ""]
pub mod lactlo;
#[doc = "LACTHI register accessor: an alias for `Reg<LACTHI_SPEC>`"]
pub type LACTHI = crate::Reg<lacthi::LACTHI_SPEC>;
#[doc = ""]
pub mod lacthi;
#[doc = "LACTUPDATE register accessor: an alias for `Reg<LACTUPDATE_SPEC>`"]
pub type LACTUPDATE = crate::Reg<lactupdate::LACTUPDATE_SPEC>;
#[doc = ""]
pub mod lactupdate;
#[doc = "LACTALARMLO register accessor: an alias for `Reg<LACTALARMLO_SPEC>`"]
pub type LACTALARMLO = crate::Reg<lactalarmlo::LACTALARMLO_SPEC>;
#[doc = ""]
pub mod lactalarmlo;
#[doc = "LACTALARMHI register accessor: an alias for `Reg<LACTALARMHI_SPEC>`"]
pub type LACTALARMHI = crate::Reg<lactalarmhi::LACTALARMHI_SPEC>;
#[doc = ""]
pub mod lactalarmhi;
#[doc = "LACTLOADLO register accessor: an alias for `Reg<LACTLOADLO_SPEC>`"]
pub type LACTLOADLO = crate::Reg<lactloadlo::LACTLOADLO_SPEC>;
#[doc = ""]
pub mod lactloadlo;
#[doc = "LACTLOADHI register accessor: an alias for `Reg<LACTLOADHI_SPEC>`"]
pub type LACTLOADHI = crate::Reg<lactloadhi::LACTLOADHI_SPEC>;
#[doc = ""]
pub mod lactloadhi;
#[doc = "LACTLOAD register accessor: an alias for `Reg<LACTLOAD_SPEC>`"]
pub type LACTLOAD = crate::Reg<lactload::LACTLOAD_SPEC>;
#[doc = ""]
pub mod lactload;
#[doc = "INT_ENA_TIMERS register accessor: an alias for `Reg<INT_ENA_TIMERS_SPEC>`"]
pub type INT_ENA_TIMERS = crate::Reg<int_ena_timers::INT_ENA_TIMERS_SPEC>;
#[doc = ""]
pub mod int_ena_timers;
#[doc = "INT_RAW_TIMERS register accessor: an alias for `Reg<INT_RAW_TIMERS_SPEC>`"]
pub type INT_RAW_TIMERS = crate::Reg<int_raw_timers::INT_RAW_TIMERS_SPEC>;
#[doc = ""]
pub mod int_raw_timers;
#[doc = "INT_ST_TIMERS register accessor: an alias for `Reg<INT_ST_TIMERS_SPEC>`"]
pub type INT_ST_TIMERS = crate::Reg<int_st_timers::INT_ST_TIMERS_SPEC>;
#[doc = ""]
pub mod int_st_timers;
#[doc = "INT_CLR_TIMERS register accessor: an alias for `Reg<INT_CLR_TIMERS_SPEC>`"]
pub type INT_CLR_TIMERS = crate::Reg<int_clr_timers::INT_CLR_TIMERS_SPEC>;
#[doc = ""]
pub mod int_clr_timers;
#[doc = "NTIMERS_DATE register accessor: an alias for `Reg<NTIMERS_DATE_SPEC>`"]
pub type NTIMERS_DATE = crate::Reg<ntimers_date::NTIMERS_DATE_SPEC>;
#[doc = ""]
pub mod ntimers_date;
#[doc = "TIMGCLK register accessor: an alias for `Reg<TIMGCLK_SPEC>`"]
pub type TIMGCLK = crate::Reg<timgclk::TIMGCLK_SPEC>;
#[doc = ""]
pub mod timgclk;
