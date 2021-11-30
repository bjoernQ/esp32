#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - "]
    pub hsch0_conf0: crate::Reg<hsch0_conf0::HSCH0_CONF0_SPEC>,
    #[doc = "0x04 - "]
    pub hsch0_hpoint: crate::Reg<hsch0_hpoint::HSCH0_HPOINT_SPEC>,
    #[doc = "0x08 - "]
    pub hsch0_duty: crate::Reg<hsch0_duty::HSCH0_DUTY_SPEC>,
    #[doc = "0x0c - "]
    pub hsch0_conf1: crate::Reg<hsch0_conf1::HSCH0_CONF1_SPEC>,
    #[doc = "0x10 - "]
    pub hsch0_duty_r: crate::Reg<hsch0_duty_r::HSCH0_DUTY_R_SPEC>,
    #[doc = "0x14 - "]
    pub hsch1_conf0: crate::Reg<hsch1_conf0::HSCH1_CONF0_SPEC>,
    #[doc = "0x18 - "]
    pub hsch1_hpoint: crate::Reg<hsch1_hpoint::HSCH1_HPOINT_SPEC>,
    #[doc = "0x1c - "]
    pub hsch1_duty: crate::Reg<hsch1_duty::HSCH1_DUTY_SPEC>,
    #[doc = "0x20 - "]
    pub hsch1_conf1: crate::Reg<hsch1_conf1::HSCH1_CONF1_SPEC>,
    #[doc = "0x24 - "]
    pub hsch1_duty_r: crate::Reg<hsch1_duty_r::HSCH1_DUTY_R_SPEC>,
    #[doc = "0x28 - "]
    pub hsch2_conf0: crate::Reg<hsch2_conf0::HSCH2_CONF0_SPEC>,
    #[doc = "0x2c - "]
    pub hsch2_hpoint: crate::Reg<hsch2_hpoint::HSCH2_HPOINT_SPEC>,
    #[doc = "0x30 - "]
    pub hsch2_duty: crate::Reg<hsch2_duty::HSCH2_DUTY_SPEC>,
    #[doc = "0x34 - "]
    pub hsch2_conf1: crate::Reg<hsch2_conf1::HSCH2_CONF1_SPEC>,
    #[doc = "0x38 - "]
    pub hsch2_duty_r: crate::Reg<hsch2_duty_r::HSCH2_DUTY_R_SPEC>,
    #[doc = "0x3c - "]
    pub hsch3_conf0: crate::Reg<hsch3_conf0::HSCH3_CONF0_SPEC>,
    #[doc = "0x40 - "]
    pub hsch3_hpoint: crate::Reg<hsch3_hpoint::HSCH3_HPOINT_SPEC>,
    #[doc = "0x44 - "]
    pub hsch3_duty: crate::Reg<hsch3_duty::HSCH3_DUTY_SPEC>,
    #[doc = "0x48 - "]
    pub hsch3_conf1: crate::Reg<hsch3_conf1::HSCH3_CONF1_SPEC>,
    #[doc = "0x4c - "]
    pub hsch3_duty_r: crate::Reg<hsch3_duty_r::HSCH3_DUTY_R_SPEC>,
    #[doc = "0x50 - "]
    pub hsch4_conf0: crate::Reg<hsch4_conf0::HSCH4_CONF0_SPEC>,
    #[doc = "0x54 - "]
    pub hsch4_hpoint: crate::Reg<hsch4_hpoint::HSCH4_HPOINT_SPEC>,
    #[doc = "0x58 - "]
    pub hsch4_duty: crate::Reg<hsch4_duty::HSCH4_DUTY_SPEC>,
    #[doc = "0x5c - "]
    pub hsch4_conf1: crate::Reg<hsch4_conf1::HSCH4_CONF1_SPEC>,
    #[doc = "0x60 - "]
    pub hsch4_duty_r: crate::Reg<hsch4_duty_r::HSCH4_DUTY_R_SPEC>,
    #[doc = "0x64 - "]
    pub hsch5_conf0: crate::Reg<hsch5_conf0::HSCH5_CONF0_SPEC>,
    #[doc = "0x68 - "]
    pub hsch5_hpoint: crate::Reg<hsch5_hpoint::HSCH5_HPOINT_SPEC>,
    #[doc = "0x6c - "]
    pub hsch5_duty: crate::Reg<hsch5_duty::HSCH5_DUTY_SPEC>,
    #[doc = "0x70 - "]
    pub hsch5_conf1: crate::Reg<hsch5_conf1::HSCH5_CONF1_SPEC>,
    #[doc = "0x74 - "]
    pub hsch5_duty_r: crate::Reg<hsch5_duty_r::HSCH5_DUTY_R_SPEC>,
    #[doc = "0x78 - "]
    pub hsch6_conf0: crate::Reg<hsch6_conf0::HSCH6_CONF0_SPEC>,
    #[doc = "0x7c - "]
    pub hsch6_hpoint: crate::Reg<hsch6_hpoint::HSCH6_HPOINT_SPEC>,
    #[doc = "0x80 - "]
    pub hsch6_duty: crate::Reg<hsch6_duty::HSCH6_DUTY_SPEC>,
    #[doc = "0x84 - "]
    pub hsch6_conf1: crate::Reg<hsch6_conf1::HSCH6_CONF1_SPEC>,
    #[doc = "0x88 - "]
    pub hsch6_duty_r: crate::Reg<hsch6_duty_r::HSCH6_DUTY_R_SPEC>,
    #[doc = "0x8c - "]
    pub hsch7_conf0: crate::Reg<hsch7_conf0::HSCH7_CONF0_SPEC>,
    #[doc = "0x90 - "]
    pub hsch7_hpoint: crate::Reg<hsch7_hpoint::HSCH7_HPOINT_SPEC>,
    #[doc = "0x94 - "]
    pub hsch7_duty: crate::Reg<hsch7_duty::HSCH7_DUTY_SPEC>,
    #[doc = "0x98 - "]
    pub hsch7_conf1: crate::Reg<hsch7_conf1::HSCH7_CONF1_SPEC>,
    #[doc = "0x9c - "]
    pub hsch7_duty_r: crate::Reg<hsch7_duty_r::HSCH7_DUTY_R_SPEC>,
    #[doc = "0xa0 - "]
    pub lsch0_conf0: crate::Reg<lsch0_conf0::LSCH0_CONF0_SPEC>,
    #[doc = "0xa4 - "]
    pub lsch0_hpoint: crate::Reg<lsch0_hpoint::LSCH0_HPOINT_SPEC>,
    #[doc = "0xa8 - "]
    pub lsch0_duty: crate::Reg<lsch0_duty::LSCH0_DUTY_SPEC>,
    #[doc = "0xac - "]
    pub lsch0_conf1: crate::Reg<lsch0_conf1::LSCH0_CONF1_SPEC>,
    #[doc = "0xb0 - "]
    pub lsch0_duty_r: crate::Reg<lsch0_duty_r::LSCH0_DUTY_R_SPEC>,
    #[doc = "0xb4 - "]
    pub lsch1_conf0: crate::Reg<lsch1_conf0::LSCH1_CONF0_SPEC>,
    #[doc = "0xb8 - "]
    pub lsch1_hpoint: crate::Reg<lsch1_hpoint::LSCH1_HPOINT_SPEC>,
    #[doc = "0xbc - "]
    pub lsch1_duty: crate::Reg<lsch1_duty::LSCH1_DUTY_SPEC>,
    #[doc = "0xc0 - "]
    pub lsch1_conf1: crate::Reg<lsch1_conf1::LSCH1_CONF1_SPEC>,
    #[doc = "0xc4 - "]
    pub lsch1_duty_r: crate::Reg<lsch1_duty_r::LSCH1_DUTY_R_SPEC>,
    #[doc = "0xc8 - "]
    pub lsch2_conf0: crate::Reg<lsch2_conf0::LSCH2_CONF0_SPEC>,
    #[doc = "0xcc - "]
    pub lsch2_hpoint: crate::Reg<lsch2_hpoint::LSCH2_HPOINT_SPEC>,
    #[doc = "0xd0 - "]
    pub lsch2_duty: crate::Reg<lsch2_duty::LSCH2_DUTY_SPEC>,
    #[doc = "0xd4 - "]
    pub lsch2_conf1: crate::Reg<lsch2_conf1::LSCH2_CONF1_SPEC>,
    #[doc = "0xd8 - "]
    pub lsch2_duty_r: crate::Reg<lsch2_duty_r::LSCH2_DUTY_R_SPEC>,
    #[doc = "0xdc - "]
    pub lsch3_conf0: crate::Reg<lsch3_conf0::LSCH3_CONF0_SPEC>,
    #[doc = "0xe0 - "]
    pub lsch3_hpoint: crate::Reg<lsch3_hpoint::LSCH3_HPOINT_SPEC>,
    #[doc = "0xe4 - "]
    pub lsch3_duty: crate::Reg<lsch3_duty::LSCH3_DUTY_SPEC>,
    #[doc = "0xe8 - "]
    pub lsch3_conf1: crate::Reg<lsch3_conf1::LSCH3_CONF1_SPEC>,
    #[doc = "0xec - "]
    pub lsch3_duty_r: crate::Reg<lsch3_duty_r::LSCH3_DUTY_R_SPEC>,
    #[doc = "0xf0 - "]
    pub lsch4_conf0: crate::Reg<lsch4_conf0::LSCH4_CONF0_SPEC>,
    #[doc = "0xf4 - "]
    pub lsch4_hpoint: crate::Reg<lsch4_hpoint::LSCH4_HPOINT_SPEC>,
    #[doc = "0xf8 - "]
    pub lsch4_duty: crate::Reg<lsch4_duty::LSCH4_DUTY_SPEC>,
    #[doc = "0xfc - "]
    pub lsch4_conf1: crate::Reg<lsch4_conf1::LSCH4_CONF1_SPEC>,
    #[doc = "0x100 - "]
    pub lsch4_duty_r: crate::Reg<lsch4_duty_r::LSCH4_DUTY_R_SPEC>,
    #[doc = "0x104 - "]
    pub lsch5_conf0: crate::Reg<lsch5_conf0::LSCH5_CONF0_SPEC>,
    #[doc = "0x108 - "]
    pub lsch5_hpoint: crate::Reg<lsch5_hpoint::LSCH5_HPOINT_SPEC>,
    #[doc = "0x10c - "]
    pub lsch5_duty: crate::Reg<lsch5_duty::LSCH5_DUTY_SPEC>,
    #[doc = "0x110 - "]
    pub lsch5_conf1: crate::Reg<lsch5_conf1::LSCH5_CONF1_SPEC>,
    #[doc = "0x114 - "]
    pub lsch5_duty_r: crate::Reg<lsch5_duty_r::LSCH5_DUTY_R_SPEC>,
    #[doc = "0x118 - "]
    pub lsch6_conf0: crate::Reg<lsch6_conf0::LSCH6_CONF0_SPEC>,
    #[doc = "0x11c - "]
    pub lsch6_hpoint: crate::Reg<lsch6_hpoint::LSCH6_HPOINT_SPEC>,
    #[doc = "0x120 - "]
    pub lsch6_duty: crate::Reg<lsch6_duty::LSCH6_DUTY_SPEC>,
    #[doc = "0x124 - "]
    pub lsch6_conf1: crate::Reg<lsch6_conf1::LSCH6_CONF1_SPEC>,
    #[doc = "0x128 - "]
    pub lsch6_duty_r: crate::Reg<lsch6_duty_r::LSCH6_DUTY_R_SPEC>,
    #[doc = "0x12c - "]
    pub lsch7_conf0: crate::Reg<lsch7_conf0::LSCH7_CONF0_SPEC>,
    #[doc = "0x130 - "]
    pub lsch7_hpoint: crate::Reg<lsch7_hpoint::LSCH7_HPOINT_SPEC>,
    #[doc = "0x134 - "]
    pub lsch7_duty: crate::Reg<lsch7_duty::LSCH7_DUTY_SPEC>,
    #[doc = "0x138 - "]
    pub lsch7_conf1: crate::Reg<lsch7_conf1::LSCH7_CONF1_SPEC>,
    #[doc = "0x13c - "]
    pub lsch7_duty_r: crate::Reg<lsch7_duty_r::LSCH7_DUTY_R_SPEC>,
    #[doc = "0x140 - "]
    pub hstimer0_conf: crate::Reg<hstimer0_conf::HSTIMER0_CONF_SPEC>,
    #[doc = "0x144 - "]
    pub hstimer0_value: crate::Reg<hstimer0_value::HSTIMER0_VALUE_SPEC>,
    #[doc = "0x148 - "]
    pub hstimer1_conf: crate::Reg<hstimer1_conf::HSTIMER1_CONF_SPEC>,
    #[doc = "0x14c - "]
    pub hstimer1_value: crate::Reg<hstimer1_value::HSTIMER1_VALUE_SPEC>,
    #[doc = "0x150 - "]
    pub hstimer2_conf: crate::Reg<hstimer2_conf::HSTIMER2_CONF_SPEC>,
    #[doc = "0x154 - "]
    pub hstimer2_value: crate::Reg<hstimer2_value::HSTIMER2_VALUE_SPEC>,
    #[doc = "0x158 - "]
    pub hstimer3_conf: crate::Reg<hstimer3_conf::HSTIMER3_CONF_SPEC>,
    #[doc = "0x15c - "]
    pub hstimer3_value: crate::Reg<hstimer3_value::HSTIMER3_VALUE_SPEC>,
    #[doc = "0x160 - "]
    pub lstimer0_conf: crate::Reg<lstimer0_conf::LSTIMER0_CONF_SPEC>,
    #[doc = "0x164 - "]
    pub lstimer0_value: crate::Reg<lstimer0_value::LSTIMER0_VALUE_SPEC>,
    #[doc = "0x168 - "]
    pub lstimer1_conf: crate::Reg<lstimer1_conf::LSTIMER1_CONF_SPEC>,
    #[doc = "0x16c - "]
    pub lstimer1_value: crate::Reg<lstimer1_value::LSTIMER1_VALUE_SPEC>,
    #[doc = "0x170 - "]
    pub lstimer2_conf: crate::Reg<lstimer2_conf::LSTIMER2_CONF_SPEC>,
    #[doc = "0x174 - "]
    pub lstimer2_value: crate::Reg<lstimer2_value::LSTIMER2_VALUE_SPEC>,
    #[doc = "0x178 - "]
    pub lstimer3_conf: crate::Reg<lstimer3_conf::LSTIMER3_CONF_SPEC>,
    #[doc = "0x17c - "]
    pub lstimer3_value: crate::Reg<lstimer3_value::LSTIMER3_VALUE_SPEC>,
    #[doc = "0x180 - "]
    pub int_raw: crate::Reg<int_raw::INT_RAW_SPEC>,
    #[doc = "0x184 - "]
    pub int_st: crate::Reg<int_st::INT_ST_SPEC>,
    #[doc = "0x188 - "]
    pub int_ena: crate::Reg<int_ena::INT_ENA_SPEC>,
    #[doc = "0x18c - "]
    pub int_clr: crate::Reg<int_clr::INT_CLR_SPEC>,
    #[doc = "0x190 - "]
    pub conf: crate::Reg<conf::CONF_SPEC>,
    _reserved101: [u8; 0x68],
    #[doc = "0x1fc - "]
    pub date: crate::Reg<date::DATE_SPEC>,
}
#[doc = "HSCH0_CONF0 register accessor: an alias for `Reg<HSCH0_CONF0_SPEC>`"]
pub type HSCH0_CONF0 = crate::Reg<hsch0_conf0::HSCH0_CONF0_SPEC>;
#[doc = ""]
pub mod hsch0_conf0;
#[doc = "HSCH0_HPOINT register accessor: an alias for `Reg<HSCH0_HPOINT_SPEC>`"]
pub type HSCH0_HPOINT = crate::Reg<hsch0_hpoint::HSCH0_HPOINT_SPEC>;
#[doc = ""]
pub mod hsch0_hpoint;
#[doc = "HSCH0_DUTY register accessor: an alias for `Reg<HSCH0_DUTY_SPEC>`"]
pub type HSCH0_DUTY = crate::Reg<hsch0_duty::HSCH0_DUTY_SPEC>;
#[doc = ""]
pub mod hsch0_duty;
#[doc = "HSCH0_CONF1 register accessor: an alias for `Reg<HSCH0_CONF1_SPEC>`"]
pub type HSCH0_CONF1 = crate::Reg<hsch0_conf1::HSCH0_CONF1_SPEC>;
#[doc = ""]
pub mod hsch0_conf1;
#[doc = "HSCH0_DUTY_R register accessor: an alias for `Reg<HSCH0_DUTY_R_SPEC>`"]
pub type HSCH0_DUTY_R = crate::Reg<hsch0_duty_r::HSCH0_DUTY_R_SPEC>;
#[doc = ""]
pub mod hsch0_duty_r;
#[doc = "HSCH1_CONF0 register accessor: an alias for `Reg<HSCH1_CONF0_SPEC>`"]
pub type HSCH1_CONF0 = crate::Reg<hsch1_conf0::HSCH1_CONF0_SPEC>;
#[doc = ""]
pub mod hsch1_conf0;
#[doc = "HSCH1_HPOINT register accessor: an alias for `Reg<HSCH1_HPOINT_SPEC>`"]
pub type HSCH1_HPOINT = crate::Reg<hsch1_hpoint::HSCH1_HPOINT_SPEC>;
#[doc = ""]
pub mod hsch1_hpoint;
#[doc = "HSCH1_DUTY register accessor: an alias for `Reg<HSCH1_DUTY_SPEC>`"]
pub type HSCH1_DUTY = crate::Reg<hsch1_duty::HSCH1_DUTY_SPEC>;
#[doc = ""]
pub mod hsch1_duty;
#[doc = "HSCH1_CONF1 register accessor: an alias for `Reg<HSCH1_CONF1_SPEC>`"]
pub type HSCH1_CONF1 = crate::Reg<hsch1_conf1::HSCH1_CONF1_SPEC>;
#[doc = ""]
pub mod hsch1_conf1;
#[doc = "HSCH1_DUTY_R register accessor: an alias for `Reg<HSCH1_DUTY_R_SPEC>`"]
pub type HSCH1_DUTY_R = crate::Reg<hsch1_duty_r::HSCH1_DUTY_R_SPEC>;
#[doc = ""]
pub mod hsch1_duty_r;
#[doc = "HSCH2_CONF0 register accessor: an alias for `Reg<HSCH2_CONF0_SPEC>`"]
pub type HSCH2_CONF0 = crate::Reg<hsch2_conf0::HSCH2_CONF0_SPEC>;
#[doc = ""]
pub mod hsch2_conf0;
#[doc = "HSCH2_HPOINT register accessor: an alias for `Reg<HSCH2_HPOINT_SPEC>`"]
pub type HSCH2_HPOINT = crate::Reg<hsch2_hpoint::HSCH2_HPOINT_SPEC>;
#[doc = ""]
pub mod hsch2_hpoint;
#[doc = "HSCH2_DUTY register accessor: an alias for `Reg<HSCH2_DUTY_SPEC>`"]
pub type HSCH2_DUTY = crate::Reg<hsch2_duty::HSCH2_DUTY_SPEC>;
#[doc = ""]
pub mod hsch2_duty;
#[doc = "HSCH2_CONF1 register accessor: an alias for `Reg<HSCH2_CONF1_SPEC>`"]
pub type HSCH2_CONF1 = crate::Reg<hsch2_conf1::HSCH2_CONF1_SPEC>;
#[doc = ""]
pub mod hsch2_conf1;
#[doc = "HSCH2_DUTY_R register accessor: an alias for `Reg<HSCH2_DUTY_R_SPEC>`"]
pub type HSCH2_DUTY_R = crate::Reg<hsch2_duty_r::HSCH2_DUTY_R_SPEC>;
#[doc = ""]
pub mod hsch2_duty_r;
#[doc = "HSCH3_CONF0 register accessor: an alias for `Reg<HSCH3_CONF0_SPEC>`"]
pub type HSCH3_CONF0 = crate::Reg<hsch3_conf0::HSCH3_CONF0_SPEC>;
#[doc = ""]
pub mod hsch3_conf0;
#[doc = "HSCH3_HPOINT register accessor: an alias for `Reg<HSCH3_HPOINT_SPEC>`"]
pub type HSCH3_HPOINT = crate::Reg<hsch3_hpoint::HSCH3_HPOINT_SPEC>;
#[doc = ""]
pub mod hsch3_hpoint;
#[doc = "HSCH3_DUTY register accessor: an alias for `Reg<HSCH3_DUTY_SPEC>`"]
pub type HSCH3_DUTY = crate::Reg<hsch3_duty::HSCH3_DUTY_SPEC>;
#[doc = ""]
pub mod hsch3_duty;
#[doc = "HSCH3_CONF1 register accessor: an alias for `Reg<HSCH3_CONF1_SPEC>`"]
pub type HSCH3_CONF1 = crate::Reg<hsch3_conf1::HSCH3_CONF1_SPEC>;
#[doc = ""]
pub mod hsch3_conf1;
#[doc = "HSCH3_DUTY_R register accessor: an alias for `Reg<HSCH3_DUTY_R_SPEC>`"]
pub type HSCH3_DUTY_R = crate::Reg<hsch3_duty_r::HSCH3_DUTY_R_SPEC>;
#[doc = ""]
pub mod hsch3_duty_r;
#[doc = "HSCH4_CONF0 register accessor: an alias for `Reg<HSCH4_CONF0_SPEC>`"]
pub type HSCH4_CONF0 = crate::Reg<hsch4_conf0::HSCH4_CONF0_SPEC>;
#[doc = ""]
pub mod hsch4_conf0;
#[doc = "HSCH4_HPOINT register accessor: an alias for `Reg<HSCH4_HPOINT_SPEC>`"]
pub type HSCH4_HPOINT = crate::Reg<hsch4_hpoint::HSCH4_HPOINT_SPEC>;
#[doc = ""]
pub mod hsch4_hpoint;
#[doc = "HSCH4_DUTY register accessor: an alias for `Reg<HSCH4_DUTY_SPEC>`"]
pub type HSCH4_DUTY = crate::Reg<hsch4_duty::HSCH4_DUTY_SPEC>;
#[doc = ""]
pub mod hsch4_duty;
#[doc = "HSCH4_CONF1 register accessor: an alias for `Reg<HSCH4_CONF1_SPEC>`"]
pub type HSCH4_CONF1 = crate::Reg<hsch4_conf1::HSCH4_CONF1_SPEC>;
#[doc = ""]
pub mod hsch4_conf1;
#[doc = "HSCH4_DUTY_R register accessor: an alias for `Reg<HSCH4_DUTY_R_SPEC>`"]
pub type HSCH4_DUTY_R = crate::Reg<hsch4_duty_r::HSCH4_DUTY_R_SPEC>;
#[doc = ""]
pub mod hsch4_duty_r;
#[doc = "HSCH5_CONF0 register accessor: an alias for `Reg<HSCH5_CONF0_SPEC>`"]
pub type HSCH5_CONF0 = crate::Reg<hsch5_conf0::HSCH5_CONF0_SPEC>;
#[doc = ""]
pub mod hsch5_conf0;
#[doc = "HSCH5_HPOINT register accessor: an alias for `Reg<HSCH5_HPOINT_SPEC>`"]
pub type HSCH5_HPOINT = crate::Reg<hsch5_hpoint::HSCH5_HPOINT_SPEC>;
#[doc = ""]
pub mod hsch5_hpoint;
#[doc = "HSCH5_DUTY register accessor: an alias for `Reg<HSCH5_DUTY_SPEC>`"]
pub type HSCH5_DUTY = crate::Reg<hsch5_duty::HSCH5_DUTY_SPEC>;
#[doc = ""]
pub mod hsch5_duty;
#[doc = "HSCH5_CONF1 register accessor: an alias for `Reg<HSCH5_CONF1_SPEC>`"]
pub type HSCH5_CONF1 = crate::Reg<hsch5_conf1::HSCH5_CONF1_SPEC>;
#[doc = ""]
pub mod hsch5_conf1;
#[doc = "HSCH5_DUTY_R register accessor: an alias for `Reg<HSCH5_DUTY_R_SPEC>`"]
pub type HSCH5_DUTY_R = crate::Reg<hsch5_duty_r::HSCH5_DUTY_R_SPEC>;
#[doc = ""]
pub mod hsch5_duty_r;
#[doc = "HSCH6_CONF0 register accessor: an alias for `Reg<HSCH6_CONF0_SPEC>`"]
pub type HSCH6_CONF0 = crate::Reg<hsch6_conf0::HSCH6_CONF0_SPEC>;
#[doc = ""]
pub mod hsch6_conf0;
#[doc = "HSCH6_HPOINT register accessor: an alias for `Reg<HSCH6_HPOINT_SPEC>`"]
pub type HSCH6_HPOINT = crate::Reg<hsch6_hpoint::HSCH6_HPOINT_SPEC>;
#[doc = ""]
pub mod hsch6_hpoint;
#[doc = "HSCH6_DUTY register accessor: an alias for `Reg<HSCH6_DUTY_SPEC>`"]
pub type HSCH6_DUTY = crate::Reg<hsch6_duty::HSCH6_DUTY_SPEC>;
#[doc = ""]
pub mod hsch6_duty;
#[doc = "HSCH6_CONF1 register accessor: an alias for `Reg<HSCH6_CONF1_SPEC>`"]
pub type HSCH6_CONF1 = crate::Reg<hsch6_conf1::HSCH6_CONF1_SPEC>;
#[doc = ""]
pub mod hsch6_conf1;
#[doc = "HSCH6_DUTY_R register accessor: an alias for `Reg<HSCH6_DUTY_R_SPEC>`"]
pub type HSCH6_DUTY_R = crate::Reg<hsch6_duty_r::HSCH6_DUTY_R_SPEC>;
#[doc = ""]
pub mod hsch6_duty_r;
#[doc = "HSCH7_CONF0 register accessor: an alias for `Reg<HSCH7_CONF0_SPEC>`"]
pub type HSCH7_CONF0 = crate::Reg<hsch7_conf0::HSCH7_CONF0_SPEC>;
#[doc = ""]
pub mod hsch7_conf0;
#[doc = "HSCH7_HPOINT register accessor: an alias for `Reg<HSCH7_HPOINT_SPEC>`"]
pub type HSCH7_HPOINT = crate::Reg<hsch7_hpoint::HSCH7_HPOINT_SPEC>;
#[doc = ""]
pub mod hsch7_hpoint;
#[doc = "HSCH7_DUTY register accessor: an alias for `Reg<HSCH7_DUTY_SPEC>`"]
pub type HSCH7_DUTY = crate::Reg<hsch7_duty::HSCH7_DUTY_SPEC>;
#[doc = ""]
pub mod hsch7_duty;
#[doc = "HSCH7_CONF1 register accessor: an alias for `Reg<HSCH7_CONF1_SPEC>`"]
pub type HSCH7_CONF1 = crate::Reg<hsch7_conf1::HSCH7_CONF1_SPEC>;
#[doc = ""]
pub mod hsch7_conf1;
#[doc = "HSCH7_DUTY_R register accessor: an alias for `Reg<HSCH7_DUTY_R_SPEC>`"]
pub type HSCH7_DUTY_R = crate::Reg<hsch7_duty_r::HSCH7_DUTY_R_SPEC>;
#[doc = ""]
pub mod hsch7_duty_r;
#[doc = "LSCH0_CONF0 register accessor: an alias for `Reg<LSCH0_CONF0_SPEC>`"]
pub type LSCH0_CONF0 = crate::Reg<lsch0_conf0::LSCH0_CONF0_SPEC>;
#[doc = ""]
pub mod lsch0_conf0;
#[doc = "LSCH0_HPOINT register accessor: an alias for `Reg<LSCH0_HPOINT_SPEC>`"]
pub type LSCH0_HPOINT = crate::Reg<lsch0_hpoint::LSCH0_HPOINT_SPEC>;
#[doc = ""]
pub mod lsch0_hpoint;
#[doc = "LSCH0_DUTY register accessor: an alias for `Reg<LSCH0_DUTY_SPEC>`"]
pub type LSCH0_DUTY = crate::Reg<lsch0_duty::LSCH0_DUTY_SPEC>;
#[doc = ""]
pub mod lsch0_duty;
#[doc = "LSCH0_CONF1 register accessor: an alias for `Reg<LSCH0_CONF1_SPEC>`"]
pub type LSCH0_CONF1 = crate::Reg<lsch0_conf1::LSCH0_CONF1_SPEC>;
#[doc = ""]
pub mod lsch0_conf1;
#[doc = "LSCH0_DUTY_R register accessor: an alias for `Reg<LSCH0_DUTY_R_SPEC>`"]
pub type LSCH0_DUTY_R = crate::Reg<lsch0_duty_r::LSCH0_DUTY_R_SPEC>;
#[doc = ""]
pub mod lsch0_duty_r;
#[doc = "LSCH1_CONF0 register accessor: an alias for `Reg<LSCH1_CONF0_SPEC>`"]
pub type LSCH1_CONF0 = crate::Reg<lsch1_conf0::LSCH1_CONF0_SPEC>;
#[doc = ""]
pub mod lsch1_conf0;
#[doc = "LSCH1_HPOINT register accessor: an alias for `Reg<LSCH1_HPOINT_SPEC>`"]
pub type LSCH1_HPOINT = crate::Reg<lsch1_hpoint::LSCH1_HPOINT_SPEC>;
#[doc = ""]
pub mod lsch1_hpoint;
#[doc = "LSCH1_DUTY register accessor: an alias for `Reg<LSCH1_DUTY_SPEC>`"]
pub type LSCH1_DUTY = crate::Reg<lsch1_duty::LSCH1_DUTY_SPEC>;
#[doc = ""]
pub mod lsch1_duty;
#[doc = "LSCH1_CONF1 register accessor: an alias for `Reg<LSCH1_CONF1_SPEC>`"]
pub type LSCH1_CONF1 = crate::Reg<lsch1_conf1::LSCH1_CONF1_SPEC>;
#[doc = ""]
pub mod lsch1_conf1;
#[doc = "LSCH1_DUTY_R register accessor: an alias for `Reg<LSCH1_DUTY_R_SPEC>`"]
pub type LSCH1_DUTY_R = crate::Reg<lsch1_duty_r::LSCH1_DUTY_R_SPEC>;
#[doc = ""]
pub mod lsch1_duty_r;
#[doc = "LSCH2_CONF0 register accessor: an alias for `Reg<LSCH2_CONF0_SPEC>`"]
pub type LSCH2_CONF0 = crate::Reg<lsch2_conf0::LSCH2_CONF0_SPEC>;
#[doc = ""]
pub mod lsch2_conf0;
#[doc = "LSCH2_HPOINT register accessor: an alias for `Reg<LSCH2_HPOINT_SPEC>`"]
pub type LSCH2_HPOINT = crate::Reg<lsch2_hpoint::LSCH2_HPOINT_SPEC>;
#[doc = ""]
pub mod lsch2_hpoint;
#[doc = "LSCH2_DUTY register accessor: an alias for `Reg<LSCH2_DUTY_SPEC>`"]
pub type LSCH2_DUTY = crate::Reg<lsch2_duty::LSCH2_DUTY_SPEC>;
#[doc = ""]
pub mod lsch2_duty;
#[doc = "LSCH2_CONF1 register accessor: an alias for `Reg<LSCH2_CONF1_SPEC>`"]
pub type LSCH2_CONF1 = crate::Reg<lsch2_conf1::LSCH2_CONF1_SPEC>;
#[doc = ""]
pub mod lsch2_conf1;
#[doc = "LSCH2_DUTY_R register accessor: an alias for `Reg<LSCH2_DUTY_R_SPEC>`"]
pub type LSCH2_DUTY_R = crate::Reg<lsch2_duty_r::LSCH2_DUTY_R_SPEC>;
#[doc = ""]
pub mod lsch2_duty_r;
#[doc = "LSCH3_CONF0 register accessor: an alias for `Reg<LSCH3_CONF0_SPEC>`"]
pub type LSCH3_CONF0 = crate::Reg<lsch3_conf0::LSCH3_CONF0_SPEC>;
#[doc = ""]
pub mod lsch3_conf0;
#[doc = "LSCH3_HPOINT register accessor: an alias for `Reg<LSCH3_HPOINT_SPEC>`"]
pub type LSCH3_HPOINT = crate::Reg<lsch3_hpoint::LSCH3_HPOINT_SPEC>;
#[doc = ""]
pub mod lsch3_hpoint;
#[doc = "LSCH3_DUTY register accessor: an alias for `Reg<LSCH3_DUTY_SPEC>`"]
pub type LSCH3_DUTY = crate::Reg<lsch3_duty::LSCH3_DUTY_SPEC>;
#[doc = ""]
pub mod lsch3_duty;
#[doc = "LSCH3_CONF1 register accessor: an alias for `Reg<LSCH3_CONF1_SPEC>`"]
pub type LSCH3_CONF1 = crate::Reg<lsch3_conf1::LSCH3_CONF1_SPEC>;
#[doc = ""]
pub mod lsch3_conf1;
#[doc = "LSCH3_DUTY_R register accessor: an alias for `Reg<LSCH3_DUTY_R_SPEC>`"]
pub type LSCH3_DUTY_R = crate::Reg<lsch3_duty_r::LSCH3_DUTY_R_SPEC>;
#[doc = ""]
pub mod lsch3_duty_r;
#[doc = "LSCH4_CONF0 register accessor: an alias for `Reg<LSCH4_CONF0_SPEC>`"]
pub type LSCH4_CONF0 = crate::Reg<lsch4_conf0::LSCH4_CONF0_SPEC>;
#[doc = ""]
pub mod lsch4_conf0;
#[doc = "LSCH4_HPOINT register accessor: an alias for `Reg<LSCH4_HPOINT_SPEC>`"]
pub type LSCH4_HPOINT = crate::Reg<lsch4_hpoint::LSCH4_HPOINT_SPEC>;
#[doc = ""]
pub mod lsch4_hpoint;
#[doc = "LSCH4_DUTY register accessor: an alias for `Reg<LSCH4_DUTY_SPEC>`"]
pub type LSCH4_DUTY = crate::Reg<lsch4_duty::LSCH4_DUTY_SPEC>;
#[doc = ""]
pub mod lsch4_duty;
#[doc = "LSCH4_CONF1 register accessor: an alias for `Reg<LSCH4_CONF1_SPEC>`"]
pub type LSCH4_CONF1 = crate::Reg<lsch4_conf1::LSCH4_CONF1_SPEC>;
#[doc = ""]
pub mod lsch4_conf1;
#[doc = "LSCH4_DUTY_R register accessor: an alias for `Reg<LSCH4_DUTY_R_SPEC>`"]
pub type LSCH4_DUTY_R = crate::Reg<lsch4_duty_r::LSCH4_DUTY_R_SPEC>;
#[doc = ""]
pub mod lsch4_duty_r;
#[doc = "LSCH5_CONF0 register accessor: an alias for `Reg<LSCH5_CONF0_SPEC>`"]
pub type LSCH5_CONF0 = crate::Reg<lsch5_conf0::LSCH5_CONF0_SPEC>;
#[doc = ""]
pub mod lsch5_conf0;
#[doc = "LSCH5_HPOINT register accessor: an alias for `Reg<LSCH5_HPOINT_SPEC>`"]
pub type LSCH5_HPOINT = crate::Reg<lsch5_hpoint::LSCH5_HPOINT_SPEC>;
#[doc = ""]
pub mod lsch5_hpoint;
#[doc = "LSCH5_DUTY register accessor: an alias for `Reg<LSCH5_DUTY_SPEC>`"]
pub type LSCH5_DUTY = crate::Reg<lsch5_duty::LSCH5_DUTY_SPEC>;
#[doc = ""]
pub mod lsch5_duty;
#[doc = "LSCH5_CONF1 register accessor: an alias for `Reg<LSCH5_CONF1_SPEC>`"]
pub type LSCH5_CONF1 = crate::Reg<lsch5_conf1::LSCH5_CONF1_SPEC>;
#[doc = ""]
pub mod lsch5_conf1;
#[doc = "LSCH5_DUTY_R register accessor: an alias for `Reg<LSCH5_DUTY_R_SPEC>`"]
pub type LSCH5_DUTY_R = crate::Reg<lsch5_duty_r::LSCH5_DUTY_R_SPEC>;
#[doc = ""]
pub mod lsch5_duty_r;
#[doc = "LSCH6_CONF0 register accessor: an alias for `Reg<LSCH6_CONF0_SPEC>`"]
pub type LSCH6_CONF0 = crate::Reg<lsch6_conf0::LSCH6_CONF0_SPEC>;
#[doc = ""]
pub mod lsch6_conf0;
#[doc = "LSCH6_HPOINT register accessor: an alias for `Reg<LSCH6_HPOINT_SPEC>`"]
pub type LSCH6_HPOINT = crate::Reg<lsch6_hpoint::LSCH6_HPOINT_SPEC>;
#[doc = ""]
pub mod lsch6_hpoint;
#[doc = "LSCH6_DUTY register accessor: an alias for `Reg<LSCH6_DUTY_SPEC>`"]
pub type LSCH6_DUTY = crate::Reg<lsch6_duty::LSCH6_DUTY_SPEC>;
#[doc = ""]
pub mod lsch6_duty;
#[doc = "LSCH6_CONF1 register accessor: an alias for `Reg<LSCH6_CONF1_SPEC>`"]
pub type LSCH6_CONF1 = crate::Reg<lsch6_conf1::LSCH6_CONF1_SPEC>;
#[doc = ""]
pub mod lsch6_conf1;
#[doc = "LSCH6_DUTY_R register accessor: an alias for `Reg<LSCH6_DUTY_R_SPEC>`"]
pub type LSCH6_DUTY_R = crate::Reg<lsch6_duty_r::LSCH6_DUTY_R_SPEC>;
#[doc = ""]
pub mod lsch6_duty_r;
#[doc = "LSCH7_CONF0 register accessor: an alias for `Reg<LSCH7_CONF0_SPEC>`"]
pub type LSCH7_CONF0 = crate::Reg<lsch7_conf0::LSCH7_CONF0_SPEC>;
#[doc = ""]
pub mod lsch7_conf0;
#[doc = "LSCH7_HPOINT register accessor: an alias for `Reg<LSCH7_HPOINT_SPEC>`"]
pub type LSCH7_HPOINT = crate::Reg<lsch7_hpoint::LSCH7_HPOINT_SPEC>;
#[doc = ""]
pub mod lsch7_hpoint;
#[doc = "LSCH7_DUTY register accessor: an alias for `Reg<LSCH7_DUTY_SPEC>`"]
pub type LSCH7_DUTY = crate::Reg<lsch7_duty::LSCH7_DUTY_SPEC>;
#[doc = ""]
pub mod lsch7_duty;
#[doc = "LSCH7_CONF1 register accessor: an alias for `Reg<LSCH7_CONF1_SPEC>`"]
pub type LSCH7_CONF1 = crate::Reg<lsch7_conf1::LSCH7_CONF1_SPEC>;
#[doc = ""]
pub mod lsch7_conf1;
#[doc = "LSCH7_DUTY_R register accessor: an alias for `Reg<LSCH7_DUTY_R_SPEC>`"]
pub type LSCH7_DUTY_R = crate::Reg<lsch7_duty_r::LSCH7_DUTY_R_SPEC>;
#[doc = ""]
pub mod lsch7_duty_r;
#[doc = "HSTIMER0_CONF register accessor: an alias for `Reg<HSTIMER0_CONF_SPEC>`"]
pub type HSTIMER0_CONF = crate::Reg<hstimer0_conf::HSTIMER0_CONF_SPEC>;
#[doc = ""]
pub mod hstimer0_conf;
#[doc = "HSTIMER0_VALUE register accessor: an alias for `Reg<HSTIMER0_VALUE_SPEC>`"]
pub type HSTIMER0_VALUE = crate::Reg<hstimer0_value::HSTIMER0_VALUE_SPEC>;
#[doc = ""]
pub mod hstimer0_value;
#[doc = "HSTIMER1_CONF register accessor: an alias for `Reg<HSTIMER1_CONF_SPEC>`"]
pub type HSTIMER1_CONF = crate::Reg<hstimer1_conf::HSTIMER1_CONF_SPEC>;
#[doc = ""]
pub mod hstimer1_conf;
#[doc = "HSTIMER1_VALUE register accessor: an alias for `Reg<HSTIMER1_VALUE_SPEC>`"]
pub type HSTIMER1_VALUE = crate::Reg<hstimer1_value::HSTIMER1_VALUE_SPEC>;
#[doc = ""]
pub mod hstimer1_value;
#[doc = "HSTIMER2_CONF register accessor: an alias for `Reg<HSTIMER2_CONF_SPEC>`"]
pub type HSTIMER2_CONF = crate::Reg<hstimer2_conf::HSTIMER2_CONF_SPEC>;
#[doc = ""]
pub mod hstimer2_conf;
#[doc = "HSTIMER2_VALUE register accessor: an alias for `Reg<HSTIMER2_VALUE_SPEC>`"]
pub type HSTIMER2_VALUE = crate::Reg<hstimer2_value::HSTIMER2_VALUE_SPEC>;
#[doc = ""]
pub mod hstimer2_value;
#[doc = "HSTIMER3_CONF register accessor: an alias for `Reg<HSTIMER3_CONF_SPEC>`"]
pub type HSTIMER3_CONF = crate::Reg<hstimer3_conf::HSTIMER3_CONF_SPEC>;
#[doc = ""]
pub mod hstimer3_conf;
#[doc = "HSTIMER3_VALUE register accessor: an alias for `Reg<HSTIMER3_VALUE_SPEC>`"]
pub type HSTIMER3_VALUE = crate::Reg<hstimer3_value::HSTIMER3_VALUE_SPEC>;
#[doc = ""]
pub mod hstimer3_value;
#[doc = "LSTIMER0_CONF register accessor: an alias for `Reg<LSTIMER0_CONF_SPEC>`"]
pub type LSTIMER0_CONF = crate::Reg<lstimer0_conf::LSTIMER0_CONF_SPEC>;
#[doc = ""]
pub mod lstimer0_conf;
#[doc = "LSTIMER0_VALUE register accessor: an alias for `Reg<LSTIMER0_VALUE_SPEC>`"]
pub type LSTIMER0_VALUE = crate::Reg<lstimer0_value::LSTIMER0_VALUE_SPEC>;
#[doc = ""]
pub mod lstimer0_value;
#[doc = "LSTIMER1_CONF register accessor: an alias for `Reg<LSTIMER1_CONF_SPEC>`"]
pub type LSTIMER1_CONF = crate::Reg<lstimer1_conf::LSTIMER1_CONF_SPEC>;
#[doc = ""]
pub mod lstimer1_conf;
#[doc = "LSTIMER1_VALUE register accessor: an alias for `Reg<LSTIMER1_VALUE_SPEC>`"]
pub type LSTIMER1_VALUE = crate::Reg<lstimer1_value::LSTIMER1_VALUE_SPEC>;
#[doc = ""]
pub mod lstimer1_value;
#[doc = "LSTIMER2_CONF register accessor: an alias for `Reg<LSTIMER2_CONF_SPEC>`"]
pub type LSTIMER2_CONF = crate::Reg<lstimer2_conf::LSTIMER2_CONF_SPEC>;
#[doc = ""]
pub mod lstimer2_conf;
#[doc = "LSTIMER2_VALUE register accessor: an alias for `Reg<LSTIMER2_VALUE_SPEC>`"]
pub type LSTIMER2_VALUE = crate::Reg<lstimer2_value::LSTIMER2_VALUE_SPEC>;
#[doc = ""]
pub mod lstimer2_value;
#[doc = "LSTIMER3_CONF register accessor: an alias for `Reg<LSTIMER3_CONF_SPEC>`"]
pub type LSTIMER3_CONF = crate::Reg<lstimer3_conf::LSTIMER3_CONF_SPEC>;
#[doc = ""]
pub mod lstimer3_conf;
#[doc = "LSTIMER3_VALUE register accessor: an alias for `Reg<LSTIMER3_VALUE_SPEC>`"]
pub type LSTIMER3_VALUE = crate::Reg<lstimer3_value::LSTIMER3_VALUE_SPEC>;
#[doc = ""]
pub mod lstimer3_value;
#[doc = "INT_RAW register accessor: an alias for `Reg<INT_RAW_SPEC>`"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = ""]
pub mod int_raw;
#[doc = "INT_ST register accessor: an alias for `Reg<INT_ST_SPEC>`"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = ""]
pub mod int_st;
#[doc = "INT_ENA register accessor: an alias for `Reg<INT_ENA_SPEC>`"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = ""]
pub mod int_ena;
#[doc = "INT_CLR register accessor: an alias for `Reg<INT_CLR_SPEC>`"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = ""]
pub mod int_clr;
#[doc = "CONF register accessor: an alias for `Reg<CONF_SPEC>`"]
pub type CONF = crate::Reg<conf::CONF_SPEC>;
#[doc = ""]
pub mod conf;
#[doc = "DATE register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = ""]
pub mod date;
