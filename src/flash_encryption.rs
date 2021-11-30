#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x20 - "]
    pub buffer_: [crate::Reg<buffer_::BUFFER__SPEC>; 8],
    #[doc = "0x20 - "]
    pub start: crate::Reg<start::START_SPEC>,
    #[doc = "0x24 - "]
    pub address: crate::Reg<address::ADDRESS_SPEC>,
    #[doc = "0x28 - "]
    pub done: crate::Reg<done::DONE_SPEC>,
}
#[doc = "BUFFER_ register accessor: an alias for `Reg<BUFFER__SPEC>`"]
pub type BUFFER_ = crate::Reg<buffer_::BUFFER__SPEC>;
#[doc = ""]
pub mod buffer_;
#[doc = "START register accessor: an alias for `Reg<START_SPEC>`"]
pub type START = crate::Reg<start::START_SPEC>;
#[doc = ""]
pub mod start;
#[doc = "ADDRESS register accessor: an alias for `Reg<ADDRESS_SPEC>`"]
pub type ADDRESS = crate::Reg<address::ADDRESS_SPEC>;
#[doc = ""]
pub mod address;
#[doc = "DONE register accessor: an alias for `Reg<DONE_SPEC>`"]
pub type DONE = crate::Reg<done::DONE_SPEC>;
#[doc = ""]
pub mod done;
