#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x54],
    #[doc = "0x54 - Baseband control register"]
    pub bbpd_ctrl: crate::Reg<bbpd_ctrl::BBPD_CTRL_SPEC>,
}
#[doc = "BBPD_CTRL register accessor: an alias for `Reg<BBPD_CTRL_SPEC>`"]
pub type BBPD_CTRL = crate::Reg<bbpd_ctrl::BBPD_CTRL_SPEC>;
#[doc = "Baseband control register"]
pub mod bbpd_ctrl;
