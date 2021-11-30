#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - "]
    pub start: crate::Reg<start::START_SPEC>,
    #[doc = "0x04 - "]
    pub idle: crate::Reg<idle::IDLE_SPEC>,
    #[doc = "0x08 - "]
    pub mode: crate::Reg<mode::MODE_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x10..0x30 - "]
    pub key_: [crate::Reg<key_::KEY__SPEC>; 8],
    #[doc = "0x30..0x40 - "]
    pub text_: [crate::Reg<text_::TEXT__SPEC>; 4],
    #[doc = "0x40 - "]
    pub endian: crate::Reg<endian::ENDIAN_SPEC>,
}
#[doc = "START register accessor: an alias for `Reg<START_SPEC>`"]
pub type START = crate::Reg<start::START_SPEC>;
#[doc = ""]
pub mod start;
#[doc = "IDLE register accessor: an alias for `Reg<IDLE_SPEC>`"]
pub type IDLE = crate::Reg<idle::IDLE_SPEC>;
#[doc = ""]
pub mod idle;
#[doc = "MODE register accessor: an alias for `Reg<MODE_SPEC>`"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = ""]
pub mod mode;
#[doc = "KEY_ register accessor: an alias for `Reg<KEY__SPEC>`"]
pub type KEY_ = crate::Reg<key_::KEY__SPEC>;
#[doc = ""]
pub mod key_;
#[doc = "TEXT_ register accessor: an alias for `Reg<TEXT__SPEC>`"]
pub type TEXT_ = crate::Reg<text_::TEXT__SPEC>;
#[doc = ""]
pub mod text_;
#[doc = "ENDIAN register accessor: an alias for `Reg<ENDIAN_SPEC>`"]
pub type ENDIAN = crate::Reg<endian::ENDIAN_SPEC>;
#[doc = ""]
pub mod endian;
