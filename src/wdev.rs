#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Hardware random number generator register"]
    pub rnd: crate::Reg<rnd::RND_SPEC>,
}
#[doc = "RND register accessor: an alias for `Reg<RND_SPEC>`"]
pub type RND = crate::Reg<rnd::RND_SPEC>;
#[doc = "Hardware random number generator register"]
pub mod rnd;
