#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - "]
    pub fifo: crate::Reg<fifo::FIFO_SPEC>,
    #[doc = "0x04 - "]
    pub int_raw: crate::Reg<int_raw::INT_RAW_SPEC>,
    #[doc = "0x08 - "]
    pub int_st: crate::Reg<int_st::INT_ST_SPEC>,
    #[doc = "0x0c - "]
    pub int_ena: crate::Reg<int_ena::INT_ENA_SPEC>,
    #[doc = "0x10 - "]
    pub int_clr: crate::Reg<int_clr::INT_CLR_SPEC>,
    #[doc = "0x14 - "]
    pub clkdiv: crate::Reg<clkdiv::CLKDIV_SPEC>,
    #[doc = "0x18 - "]
    pub autobaud: crate::Reg<autobaud::AUTOBAUD_SPEC>,
    #[doc = "0x1c - "]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x20 - "]
    pub conf0: crate::Reg<conf0::CONF0_SPEC>,
    #[doc = "0x24 - "]
    pub conf1: crate::Reg<conf1::CONF1_SPEC>,
    #[doc = "0x28 - "]
    pub lowpulse: crate::Reg<lowpulse::LOWPULSE_SPEC>,
    #[doc = "0x2c - "]
    pub highpulse: crate::Reg<highpulse::HIGHPULSE_SPEC>,
    #[doc = "0x30 - "]
    pub rxd_cnt: crate::Reg<rxd_cnt::RXD_CNT_SPEC>,
    #[doc = "0x34 - "]
    pub flow_conf: crate::Reg<flow_conf::FLOW_CONF_SPEC>,
    #[doc = "0x38 - "]
    pub sleep_conf: crate::Reg<sleep_conf::SLEEP_CONF_SPEC>,
    #[doc = "0x3c - "]
    pub swfc_conf: crate::Reg<swfc_conf::SWFC_CONF_SPEC>,
    #[doc = "0x40 - "]
    pub idle_conf: crate::Reg<idle_conf::IDLE_CONF_SPEC>,
    #[doc = "0x44 - "]
    pub rs485_conf: crate::Reg<rs485_conf::RS485_CONF_SPEC>,
    #[doc = "0x48 - "]
    pub at_cmd_precnt: crate::Reg<at_cmd_precnt::AT_CMD_PRECNT_SPEC>,
    #[doc = "0x4c - "]
    pub at_cmd_postcnt: crate::Reg<at_cmd_postcnt::AT_CMD_POSTCNT_SPEC>,
    #[doc = "0x50 - "]
    pub at_cmd_gaptout: crate::Reg<at_cmd_gaptout::AT_CMD_GAPTOUT_SPEC>,
    #[doc = "0x54 - "]
    pub at_cmd_char: crate::Reg<at_cmd_char::AT_CMD_CHAR_SPEC>,
    #[doc = "0x58 - "]
    pub mem_conf: crate::Reg<mem_conf::MEM_CONF_SPEC>,
    #[doc = "0x5c - "]
    pub mem_tx_status: crate::Reg<mem_tx_status::MEM_TX_STATUS_SPEC>,
    #[doc = "0x60 - "]
    pub mem_rx_status: crate::Reg<mem_rx_status::MEM_RX_STATUS_SPEC>,
    #[doc = "0x64 - "]
    pub mem_cnt_status: crate::Reg<mem_cnt_status::MEM_CNT_STATUS_SPEC>,
    #[doc = "0x68 - "]
    pub pospulse: crate::Reg<pospulse::POSPULSE_SPEC>,
    #[doc = "0x6c - "]
    pub negpulse: crate::Reg<negpulse::NEGPULSE_SPEC>,
    _reserved28: [u8; 0x08],
    #[doc = "0x78 - "]
    pub date: crate::Reg<date::DATE_SPEC>,
    #[doc = "0x7c - "]
    pub id: crate::Reg<id::ID_SPEC>,
}
#[doc = "FIFO register accessor: an alias for `Reg<FIFO_SPEC>`"]
pub type FIFO = crate::Reg<fifo::FIFO_SPEC>;
#[doc = ""]
pub mod fifo;
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
#[doc = "CLKDIV register accessor: an alias for `Reg<CLKDIV_SPEC>`"]
pub type CLKDIV = crate::Reg<clkdiv::CLKDIV_SPEC>;
#[doc = ""]
pub mod clkdiv;
#[doc = "AUTOBAUD register accessor: an alias for `Reg<AUTOBAUD_SPEC>`"]
pub type AUTOBAUD = crate::Reg<autobaud::AUTOBAUD_SPEC>;
#[doc = ""]
pub mod autobaud;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = ""]
pub mod status;
#[doc = "CONF0 register accessor: an alias for `Reg<CONF0_SPEC>`"]
pub type CONF0 = crate::Reg<conf0::CONF0_SPEC>;
#[doc = ""]
pub mod conf0;
#[doc = "CONF1 register accessor: an alias for `Reg<CONF1_SPEC>`"]
pub type CONF1 = crate::Reg<conf1::CONF1_SPEC>;
#[doc = ""]
pub mod conf1;
#[doc = "LOWPULSE register accessor: an alias for `Reg<LOWPULSE_SPEC>`"]
pub type LOWPULSE = crate::Reg<lowpulse::LOWPULSE_SPEC>;
#[doc = ""]
pub mod lowpulse;
#[doc = "HIGHPULSE register accessor: an alias for `Reg<HIGHPULSE_SPEC>`"]
pub type HIGHPULSE = crate::Reg<highpulse::HIGHPULSE_SPEC>;
#[doc = ""]
pub mod highpulse;
#[doc = "RXD_CNT register accessor: an alias for `Reg<RXD_CNT_SPEC>`"]
pub type RXD_CNT = crate::Reg<rxd_cnt::RXD_CNT_SPEC>;
#[doc = ""]
pub mod rxd_cnt;
#[doc = "FLOW_CONF register accessor: an alias for `Reg<FLOW_CONF_SPEC>`"]
pub type FLOW_CONF = crate::Reg<flow_conf::FLOW_CONF_SPEC>;
#[doc = ""]
pub mod flow_conf;
#[doc = "SLEEP_CONF register accessor: an alias for `Reg<SLEEP_CONF_SPEC>`"]
pub type SLEEP_CONF = crate::Reg<sleep_conf::SLEEP_CONF_SPEC>;
#[doc = ""]
pub mod sleep_conf;
#[doc = "SWFC_CONF register accessor: an alias for `Reg<SWFC_CONF_SPEC>`"]
pub type SWFC_CONF = crate::Reg<swfc_conf::SWFC_CONF_SPEC>;
#[doc = ""]
pub mod swfc_conf;
#[doc = "IDLE_CONF register accessor: an alias for `Reg<IDLE_CONF_SPEC>`"]
pub type IDLE_CONF = crate::Reg<idle_conf::IDLE_CONF_SPEC>;
#[doc = ""]
pub mod idle_conf;
#[doc = "RS485_CONF register accessor: an alias for `Reg<RS485_CONF_SPEC>`"]
pub type RS485_CONF = crate::Reg<rs485_conf::RS485_CONF_SPEC>;
#[doc = ""]
pub mod rs485_conf;
#[doc = "AT_CMD_PRECNT register accessor: an alias for `Reg<AT_CMD_PRECNT_SPEC>`"]
pub type AT_CMD_PRECNT = crate::Reg<at_cmd_precnt::AT_CMD_PRECNT_SPEC>;
#[doc = ""]
pub mod at_cmd_precnt;
#[doc = "AT_CMD_POSTCNT register accessor: an alias for `Reg<AT_CMD_POSTCNT_SPEC>`"]
pub type AT_CMD_POSTCNT = crate::Reg<at_cmd_postcnt::AT_CMD_POSTCNT_SPEC>;
#[doc = ""]
pub mod at_cmd_postcnt;
#[doc = "AT_CMD_GAPTOUT register accessor: an alias for `Reg<AT_CMD_GAPTOUT_SPEC>`"]
pub type AT_CMD_GAPTOUT = crate::Reg<at_cmd_gaptout::AT_CMD_GAPTOUT_SPEC>;
#[doc = ""]
pub mod at_cmd_gaptout;
#[doc = "AT_CMD_CHAR register accessor: an alias for `Reg<AT_CMD_CHAR_SPEC>`"]
pub type AT_CMD_CHAR = crate::Reg<at_cmd_char::AT_CMD_CHAR_SPEC>;
#[doc = ""]
pub mod at_cmd_char;
#[doc = "MEM_CONF register accessor: an alias for `Reg<MEM_CONF_SPEC>`"]
pub type MEM_CONF = crate::Reg<mem_conf::MEM_CONF_SPEC>;
#[doc = ""]
pub mod mem_conf;
#[doc = "MEM_TX_STATUS register accessor: an alias for `Reg<MEM_TX_STATUS_SPEC>`"]
pub type MEM_TX_STATUS = crate::Reg<mem_tx_status::MEM_TX_STATUS_SPEC>;
#[doc = ""]
pub mod mem_tx_status;
#[doc = "MEM_RX_STATUS register accessor: an alias for `Reg<MEM_RX_STATUS_SPEC>`"]
pub type MEM_RX_STATUS = crate::Reg<mem_rx_status::MEM_RX_STATUS_SPEC>;
#[doc = ""]
pub mod mem_rx_status;
#[doc = "MEM_CNT_STATUS register accessor: an alias for `Reg<MEM_CNT_STATUS_SPEC>`"]
pub type MEM_CNT_STATUS = crate::Reg<mem_cnt_status::MEM_CNT_STATUS_SPEC>;
#[doc = ""]
pub mod mem_cnt_status;
#[doc = "POSPULSE register accessor: an alias for `Reg<POSPULSE_SPEC>`"]
pub type POSPULSE = crate::Reg<pospulse::POSPULSE_SPEC>;
#[doc = ""]
pub mod pospulse;
#[doc = "NEGPULSE register accessor: an alias for `Reg<NEGPULSE_SPEC>`"]
pub type NEGPULSE = crate::Reg<negpulse::NEGPULSE_SPEC>;
#[doc = ""]
pub mod negpulse;
#[doc = "DATE register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = ""]
pub mod date;
#[doc = "ID register accessor: an alias for `Reg<ID_SPEC>`"]
pub type ID = crate::Reg<id::ID_SPEC>;
#[doc = ""]
pub mod id;
