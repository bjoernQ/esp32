#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - "]
    pub timer_load: crate::Reg<timer_load::TIMER_LOAD_SPEC>,
    #[doc = "0x04 - "]
    pub timer_count: crate::Reg<timer_count::TIMER_COUNT_SPEC>,
    #[doc = "0x08 - "]
    pub timer_ctrl: crate::Reg<timer_ctrl::TIMER_CTRL_SPEC>,
    #[doc = "0x0c - "]
    pub timer_int: crate::Reg<timer_int::TIMER_INT_SPEC>,
    #[doc = "0x10 - "]
    pub timer_alarm: crate::Reg<timer_alarm::TIMER_ALARM_SPEC>,
}
#[doc = "TIMER_LOAD register accessor: an alias for `Reg<TIMER_LOAD_SPEC>`"]
pub type TIMER_LOAD = crate::Reg<timer_load::TIMER_LOAD_SPEC>;
#[doc = ""]
pub mod timer_load;
#[doc = "TIMER_COUNT register accessor: an alias for `Reg<TIMER_COUNT_SPEC>`"]
pub type TIMER_COUNT = crate::Reg<timer_count::TIMER_COUNT_SPEC>;
#[doc = ""]
pub mod timer_count;
#[doc = "TIMER_CTRL register accessor: an alias for `Reg<TIMER_CTRL_SPEC>`"]
pub type TIMER_CTRL = crate::Reg<timer_ctrl::TIMER_CTRL_SPEC>;
#[doc = ""]
pub mod timer_ctrl;
#[doc = "TIMER_INT register accessor: an alias for `Reg<TIMER_INT_SPEC>`"]
pub type TIMER_INT = crate::Reg<timer_int::TIMER_INT_SPEC>;
#[doc = ""]
pub mod timer_int;
#[doc = "TIMER_ALARM register accessor: an alias for `Reg<TIMER_ALARM_SPEC>`"]
pub type TIMER_ALARM = crate::Reg<timer_alarm::TIMER_ALARM_SPEC>;
#[doc = ""]
pub mod timer_alarm;
