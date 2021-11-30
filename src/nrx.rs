#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0xd4],
    #[doc = "0xd4 - WiFi RX control register"]
    pub nrxpd_ctrl: crate::Reg<nrxpd_ctrl::NRXPD_CTRL_SPEC>,
}
#[doc = "NRXPD_CTRL register accessor: an alias for `Reg<NRXPD_CTRL_SPEC>`"]
pub type NRXPD_CTRL = crate::Reg<nrxpd_ctrl::NRXPD_CTRL_SPEC>;
#[doc = "WiFi RX control register"]
pub mod nrxpd_ctrl;
