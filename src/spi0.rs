#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - "]
    pub cmd: crate::Reg<cmd::CMD_SPEC>,
    #[doc = "0x04 - "]
    pub addr: crate::Reg<addr::ADDR_SPEC>,
    #[doc = "0x08 - "]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x0c - "]
    pub ctrl1: crate::Reg<ctrl1::CTRL1_SPEC>,
    #[doc = "0x10 - "]
    pub rd_status: crate::Reg<rd_status::RD_STATUS_SPEC>,
    #[doc = "0x14 - "]
    pub ctrl2: crate::Reg<ctrl2::CTRL2_SPEC>,
    #[doc = "0x18 - "]
    pub clock: crate::Reg<clock::CLOCK_SPEC>,
    #[doc = "0x1c - "]
    pub user: crate::Reg<user::USER_SPEC>,
    #[doc = "0x20 - "]
    pub user1: crate::Reg<user1::USER1_SPEC>,
    #[doc = "0x24 - "]
    pub user2: crate::Reg<user2::USER2_SPEC>,
    #[doc = "0x28 - "]
    pub mosi_dlen: crate::Reg<mosi_dlen::MOSI_DLEN_SPEC>,
    #[doc = "0x2c - "]
    pub miso_dlen: crate::Reg<miso_dlen::MISO_DLEN_SPEC>,
    #[doc = "0x30 - "]
    pub slv_wr_status: crate::Reg<slv_wr_status::SLV_WR_STATUS_SPEC>,
    #[doc = "0x34 - "]
    pub pin: crate::Reg<pin::PIN_SPEC>,
    #[doc = "0x38 - "]
    pub slave: crate::Reg<slave::SLAVE_SPEC>,
    #[doc = "0x3c - "]
    pub slave1: crate::Reg<slave1::SLAVE1_SPEC>,
    #[doc = "0x40 - "]
    pub slave2: crate::Reg<slave2::SLAVE2_SPEC>,
    #[doc = "0x44 - "]
    pub slave3: crate::Reg<slave3::SLAVE3_SPEC>,
    #[doc = "0x48 - "]
    pub slv_wrbuf_dlen: crate::Reg<slv_wrbuf_dlen::SLV_WRBUF_DLEN_SPEC>,
    #[doc = "0x4c - "]
    pub slv_rdbuf_dlen: crate::Reg<slv_rdbuf_dlen::SLV_RDBUF_DLEN_SPEC>,
    #[doc = "0x50 - "]
    pub cache_fctrl: crate::Reg<cache_fctrl::CACHE_FCTRL_SPEC>,
    #[doc = "0x54 - "]
    pub cache_sctrl: crate::Reg<cache_sctrl::CACHE_SCTRL_SPEC>,
    #[doc = "0x58 - "]
    pub sram_cmd: crate::Reg<sram_cmd::SRAM_CMD_SPEC>,
    #[doc = "0x5c - "]
    pub sram_drd_cmd: crate::Reg<sram_drd_cmd::SRAM_DRD_CMD_SPEC>,
    #[doc = "0x60 - "]
    pub sram_dwr_cmd: crate::Reg<sram_dwr_cmd::SRAM_DWR_CMD_SPEC>,
    #[doc = "0x64 - "]
    pub slv_rd_bit: crate::Reg<slv_rd_bit::SLV_RD_BIT_SPEC>,
    _reserved26: [u8; 0x18],
    #[doc = "0x80 - "]
    pub w0: crate::Reg<w0::W0_SPEC>,
    #[doc = "0x84 - "]
    pub w1: crate::Reg<w1::W1_SPEC>,
    #[doc = "0x88 - "]
    pub w2: crate::Reg<w2::W2_SPEC>,
    #[doc = "0x8c - "]
    pub w3: crate::Reg<w3::W3_SPEC>,
    #[doc = "0x90 - "]
    pub w4: crate::Reg<w4::W4_SPEC>,
    #[doc = "0x94 - "]
    pub w5: crate::Reg<w5::W5_SPEC>,
    #[doc = "0x98 - "]
    pub w6: crate::Reg<w6::W6_SPEC>,
    #[doc = "0x9c - "]
    pub w7: crate::Reg<w7::W7_SPEC>,
    #[doc = "0xa0 - "]
    pub w8: crate::Reg<w8::W8_SPEC>,
    #[doc = "0xa4 - "]
    pub w9: crate::Reg<w9::W9_SPEC>,
    #[doc = "0xa8 - "]
    pub w10: crate::Reg<w10::W10_SPEC>,
    #[doc = "0xac - "]
    pub w11: crate::Reg<w11::W11_SPEC>,
    #[doc = "0xb0 - "]
    pub w12: crate::Reg<w12::W12_SPEC>,
    #[doc = "0xb4 - "]
    pub w13: crate::Reg<w13::W13_SPEC>,
    #[doc = "0xb8 - "]
    pub w14: crate::Reg<w14::W14_SPEC>,
    #[doc = "0xbc - "]
    pub w15: crate::Reg<w15::W15_SPEC>,
    #[doc = "0xc0 - "]
    pub tx_crc: crate::Reg<tx_crc::TX_CRC_SPEC>,
    _reserved43: [u8; 0x2c],
    #[doc = "0xf0 - "]
    pub ext0: crate::Reg<ext0::EXT0_SPEC>,
    #[doc = "0xf4 - "]
    pub ext1: crate::Reg<ext1::EXT1_SPEC>,
    #[doc = "0xf8 - "]
    pub ext2: crate::Reg<ext2::EXT2_SPEC>,
    #[doc = "0xfc - "]
    pub ext3: crate::Reg<ext3::EXT3_SPEC>,
    #[doc = "0x100 - "]
    pub dma_conf: crate::Reg<dma_conf::DMA_CONF_SPEC>,
    #[doc = "0x104 - "]
    pub dma_out_link: crate::Reg<dma_out_link::DMA_OUT_LINK_SPEC>,
    #[doc = "0x108 - "]
    pub dma_in_link: crate::Reg<dma_in_link::DMA_IN_LINK_SPEC>,
    #[doc = "0x10c - "]
    pub dma_status: crate::Reg<dma_status::DMA_STATUS_SPEC>,
    #[doc = "0x110 - "]
    pub dma_int_ena: crate::Reg<dma_int_ena::DMA_INT_ENA_SPEC>,
    #[doc = "0x114 - "]
    pub dma_int_raw: crate::Reg<dma_int_raw::DMA_INT_RAW_SPEC>,
    #[doc = "0x118 - "]
    pub dma_int_st: crate::Reg<dma_int_st::DMA_INT_ST_SPEC>,
    #[doc = "0x11c - "]
    pub dma_int_clr: crate::Reg<dma_int_clr::DMA_INT_CLR_SPEC>,
    #[doc = "0x120 - "]
    pub in_err_eof_des_addr: crate::Reg<in_err_eof_des_addr::IN_ERR_EOF_DES_ADDR_SPEC>,
    #[doc = "0x124 - "]
    pub in_suc_eof_des_addr: crate::Reg<in_suc_eof_des_addr::IN_SUC_EOF_DES_ADDR_SPEC>,
    #[doc = "0x128 - "]
    pub inlink_dscr: crate::Reg<inlink_dscr::INLINK_DSCR_SPEC>,
    #[doc = "0x12c - "]
    pub inlink_dscr_bf0: crate::Reg<inlink_dscr_bf0::INLINK_DSCR_BF0_SPEC>,
    #[doc = "0x130 - "]
    pub inlink_dscr_bf1: crate::Reg<inlink_dscr_bf1::INLINK_DSCR_BF1_SPEC>,
    #[doc = "0x134 - "]
    pub out_eof_bfr_des_addr: crate::Reg<out_eof_bfr_des_addr::OUT_EOF_BFR_DES_ADDR_SPEC>,
    #[doc = "0x138 - "]
    pub out_eof_des_addr: crate::Reg<out_eof_des_addr::OUT_EOF_DES_ADDR_SPEC>,
    #[doc = "0x13c - "]
    pub outlink_dscr: crate::Reg<outlink_dscr::OUTLINK_DSCR_SPEC>,
    #[doc = "0x140 - "]
    pub outlink_dscr_bf0: crate::Reg<outlink_dscr_bf0::OUTLINK_DSCR_BF0_SPEC>,
    #[doc = "0x144 - "]
    pub outlink_dscr_bf1: crate::Reg<outlink_dscr_bf1::OUTLINK_DSCR_BF1_SPEC>,
    #[doc = "0x148 - "]
    pub dma_rstatus: crate::Reg<dma_rstatus::DMA_RSTATUS_SPEC>,
    #[doc = "0x14c - "]
    pub dma_tstatus: crate::Reg<dma_tstatus::DMA_TSTATUS_SPEC>,
    _reserved67: [u8; 0x02ac],
    #[doc = "0x3fc - "]
    pub date: crate::Reg<date::DATE_SPEC>,
}
#[doc = "CMD register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = ""]
pub mod cmd;
#[doc = "ADDR register accessor: an alias for `Reg<ADDR_SPEC>`"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = ""]
pub mod addr;
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = ""]
pub mod ctrl;
#[doc = "CTRL1 register accessor: an alias for `Reg<CTRL1_SPEC>`"]
pub type CTRL1 = crate::Reg<ctrl1::CTRL1_SPEC>;
#[doc = ""]
pub mod ctrl1;
#[doc = "RD_STATUS register accessor: an alias for `Reg<RD_STATUS_SPEC>`"]
pub type RD_STATUS = crate::Reg<rd_status::RD_STATUS_SPEC>;
#[doc = ""]
pub mod rd_status;
#[doc = "CTRL2 register accessor: an alias for `Reg<CTRL2_SPEC>`"]
pub type CTRL2 = crate::Reg<ctrl2::CTRL2_SPEC>;
#[doc = ""]
pub mod ctrl2;
#[doc = "CLOCK register accessor: an alias for `Reg<CLOCK_SPEC>`"]
pub type CLOCK = crate::Reg<clock::CLOCK_SPEC>;
#[doc = ""]
pub mod clock;
#[doc = "USER register accessor: an alias for `Reg<USER_SPEC>`"]
pub type USER = crate::Reg<user::USER_SPEC>;
#[doc = ""]
pub mod user;
#[doc = "USER1 register accessor: an alias for `Reg<USER1_SPEC>`"]
pub type USER1 = crate::Reg<user1::USER1_SPEC>;
#[doc = ""]
pub mod user1;
#[doc = "USER2 register accessor: an alias for `Reg<USER2_SPEC>`"]
pub type USER2 = crate::Reg<user2::USER2_SPEC>;
#[doc = ""]
pub mod user2;
#[doc = "MOSI_DLEN register accessor: an alias for `Reg<MOSI_DLEN_SPEC>`"]
pub type MOSI_DLEN = crate::Reg<mosi_dlen::MOSI_DLEN_SPEC>;
#[doc = ""]
pub mod mosi_dlen;
#[doc = "MISO_DLEN register accessor: an alias for `Reg<MISO_DLEN_SPEC>`"]
pub type MISO_DLEN = crate::Reg<miso_dlen::MISO_DLEN_SPEC>;
#[doc = ""]
pub mod miso_dlen;
#[doc = "SLV_WR_STATUS register accessor: an alias for `Reg<SLV_WR_STATUS_SPEC>`"]
pub type SLV_WR_STATUS = crate::Reg<slv_wr_status::SLV_WR_STATUS_SPEC>;
#[doc = ""]
pub mod slv_wr_status;
#[doc = "PIN register accessor: an alias for `Reg<PIN_SPEC>`"]
pub type PIN = crate::Reg<pin::PIN_SPEC>;
#[doc = ""]
pub mod pin;
#[doc = "SLAVE register accessor: an alias for `Reg<SLAVE_SPEC>`"]
pub type SLAVE = crate::Reg<slave::SLAVE_SPEC>;
#[doc = ""]
pub mod slave;
#[doc = "SLAVE1 register accessor: an alias for `Reg<SLAVE1_SPEC>`"]
pub type SLAVE1 = crate::Reg<slave1::SLAVE1_SPEC>;
#[doc = ""]
pub mod slave1;
#[doc = "SLAVE2 register accessor: an alias for `Reg<SLAVE2_SPEC>`"]
pub type SLAVE2 = crate::Reg<slave2::SLAVE2_SPEC>;
#[doc = ""]
pub mod slave2;
#[doc = "SLAVE3 register accessor: an alias for `Reg<SLAVE3_SPEC>`"]
pub type SLAVE3 = crate::Reg<slave3::SLAVE3_SPEC>;
#[doc = ""]
pub mod slave3;
#[doc = "SLV_WRBUF_DLEN register accessor: an alias for `Reg<SLV_WRBUF_DLEN_SPEC>`"]
pub type SLV_WRBUF_DLEN = crate::Reg<slv_wrbuf_dlen::SLV_WRBUF_DLEN_SPEC>;
#[doc = ""]
pub mod slv_wrbuf_dlen;
#[doc = "SLV_RDBUF_DLEN register accessor: an alias for `Reg<SLV_RDBUF_DLEN_SPEC>`"]
pub type SLV_RDBUF_DLEN = crate::Reg<slv_rdbuf_dlen::SLV_RDBUF_DLEN_SPEC>;
#[doc = ""]
pub mod slv_rdbuf_dlen;
#[doc = "CACHE_FCTRL register accessor: an alias for `Reg<CACHE_FCTRL_SPEC>`"]
pub type CACHE_FCTRL = crate::Reg<cache_fctrl::CACHE_FCTRL_SPEC>;
#[doc = ""]
pub mod cache_fctrl;
#[doc = "CACHE_SCTRL register accessor: an alias for `Reg<CACHE_SCTRL_SPEC>`"]
pub type CACHE_SCTRL = crate::Reg<cache_sctrl::CACHE_SCTRL_SPEC>;
#[doc = ""]
pub mod cache_sctrl;
#[doc = "SRAM_CMD register accessor: an alias for `Reg<SRAM_CMD_SPEC>`"]
pub type SRAM_CMD = crate::Reg<sram_cmd::SRAM_CMD_SPEC>;
#[doc = ""]
pub mod sram_cmd;
#[doc = "SRAM_DRD_CMD register accessor: an alias for `Reg<SRAM_DRD_CMD_SPEC>`"]
pub type SRAM_DRD_CMD = crate::Reg<sram_drd_cmd::SRAM_DRD_CMD_SPEC>;
#[doc = ""]
pub mod sram_drd_cmd;
#[doc = "SRAM_DWR_CMD register accessor: an alias for `Reg<SRAM_DWR_CMD_SPEC>`"]
pub type SRAM_DWR_CMD = crate::Reg<sram_dwr_cmd::SRAM_DWR_CMD_SPEC>;
#[doc = ""]
pub mod sram_dwr_cmd;
#[doc = "SLV_RD_BIT register accessor: an alias for `Reg<SLV_RD_BIT_SPEC>`"]
pub type SLV_RD_BIT = crate::Reg<slv_rd_bit::SLV_RD_BIT_SPEC>;
#[doc = ""]
pub mod slv_rd_bit;
#[doc = "W0 register accessor: an alias for `Reg<W0_SPEC>`"]
pub type W0 = crate::Reg<w0::W0_SPEC>;
#[doc = ""]
pub mod w0;
#[doc = "W1 register accessor: an alias for `Reg<W1_SPEC>`"]
pub type W1 = crate::Reg<w1::W1_SPEC>;
#[doc = ""]
pub mod w1;
#[doc = "W2 register accessor: an alias for `Reg<W2_SPEC>`"]
pub type W2 = crate::Reg<w2::W2_SPEC>;
#[doc = ""]
pub mod w2;
#[doc = "W3 register accessor: an alias for `Reg<W3_SPEC>`"]
pub type W3 = crate::Reg<w3::W3_SPEC>;
#[doc = ""]
pub mod w3;
#[doc = "W4 register accessor: an alias for `Reg<W4_SPEC>`"]
pub type W4 = crate::Reg<w4::W4_SPEC>;
#[doc = ""]
pub mod w4;
#[doc = "W5 register accessor: an alias for `Reg<W5_SPEC>`"]
pub type W5 = crate::Reg<w5::W5_SPEC>;
#[doc = ""]
pub mod w5;
#[doc = "W6 register accessor: an alias for `Reg<W6_SPEC>`"]
pub type W6 = crate::Reg<w6::W6_SPEC>;
#[doc = ""]
pub mod w6;
#[doc = "W7 register accessor: an alias for `Reg<W7_SPEC>`"]
pub type W7 = crate::Reg<w7::W7_SPEC>;
#[doc = ""]
pub mod w7;
#[doc = "W8 register accessor: an alias for `Reg<W8_SPEC>`"]
pub type W8 = crate::Reg<w8::W8_SPEC>;
#[doc = ""]
pub mod w8;
#[doc = "W9 register accessor: an alias for `Reg<W9_SPEC>`"]
pub type W9 = crate::Reg<w9::W9_SPEC>;
#[doc = ""]
pub mod w9;
#[doc = "W10 register accessor: an alias for `Reg<W10_SPEC>`"]
pub type W10 = crate::Reg<w10::W10_SPEC>;
#[doc = ""]
pub mod w10;
#[doc = "W11 register accessor: an alias for `Reg<W11_SPEC>`"]
pub type W11 = crate::Reg<w11::W11_SPEC>;
#[doc = ""]
pub mod w11;
#[doc = "W12 register accessor: an alias for `Reg<W12_SPEC>`"]
pub type W12 = crate::Reg<w12::W12_SPEC>;
#[doc = ""]
pub mod w12;
#[doc = "W13 register accessor: an alias for `Reg<W13_SPEC>`"]
pub type W13 = crate::Reg<w13::W13_SPEC>;
#[doc = ""]
pub mod w13;
#[doc = "W14 register accessor: an alias for `Reg<W14_SPEC>`"]
pub type W14 = crate::Reg<w14::W14_SPEC>;
#[doc = ""]
pub mod w14;
#[doc = "W15 register accessor: an alias for `Reg<W15_SPEC>`"]
pub type W15 = crate::Reg<w15::W15_SPEC>;
#[doc = ""]
pub mod w15;
#[doc = "TX_CRC register accessor: an alias for `Reg<TX_CRC_SPEC>`"]
pub type TX_CRC = crate::Reg<tx_crc::TX_CRC_SPEC>;
#[doc = ""]
pub mod tx_crc;
#[doc = "EXT0 register accessor: an alias for `Reg<EXT0_SPEC>`"]
pub type EXT0 = crate::Reg<ext0::EXT0_SPEC>;
#[doc = ""]
pub mod ext0;
#[doc = "EXT1 register accessor: an alias for `Reg<EXT1_SPEC>`"]
pub type EXT1 = crate::Reg<ext1::EXT1_SPEC>;
#[doc = ""]
pub mod ext1;
#[doc = "EXT2 register accessor: an alias for `Reg<EXT2_SPEC>`"]
pub type EXT2 = crate::Reg<ext2::EXT2_SPEC>;
#[doc = ""]
pub mod ext2;
#[doc = "EXT3 register accessor: an alias for `Reg<EXT3_SPEC>`"]
pub type EXT3 = crate::Reg<ext3::EXT3_SPEC>;
#[doc = ""]
pub mod ext3;
#[doc = "DMA_CONF register accessor: an alias for `Reg<DMA_CONF_SPEC>`"]
pub type DMA_CONF = crate::Reg<dma_conf::DMA_CONF_SPEC>;
#[doc = ""]
pub mod dma_conf;
#[doc = "DMA_OUT_LINK register accessor: an alias for `Reg<DMA_OUT_LINK_SPEC>`"]
pub type DMA_OUT_LINK = crate::Reg<dma_out_link::DMA_OUT_LINK_SPEC>;
#[doc = ""]
pub mod dma_out_link;
#[doc = "DMA_IN_LINK register accessor: an alias for `Reg<DMA_IN_LINK_SPEC>`"]
pub type DMA_IN_LINK = crate::Reg<dma_in_link::DMA_IN_LINK_SPEC>;
#[doc = ""]
pub mod dma_in_link;
#[doc = "DMA_STATUS register accessor: an alias for `Reg<DMA_STATUS_SPEC>`"]
pub type DMA_STATUS = crate::Reg<dma_status::DMA_STATUS_SPEC>;
#[doc = ""]
pub mod dma_status;
#[doc = "DMA_INT_ENA register accessor: an alias for `Reg<DMA_INT_ENA_SPEC>`"]
pub type DMA_INT_ENA = crate::Reg<dma_int_ena::DMA_INT_ENA_SPEC>;
#[doc = ""]
pub mod dma_int_ena;
#[doc = "DMA_INT_RAW register accessor: an alias for `Reg<DMA_INT_RAW_SPEC>`"]
pub type DMA_INT_RAW = crate::Reg<dma_int_raw::DMA_INT_RAW_SPEC>;
#[doc = ""]
pub mod dma_int_raw;
#[doc = "DMA_INT_ST register accessor: an alias for `Reg<DMA_INT_ST_SPEC>`"]
pub type DMA_INT_ST = crate::Reg<dma_int_st::DMA_INT_ST_SPEC>;
#[doc = ""]
pub mod dma_int_st;
#[doc = "DMA_INT_CLR register accessor: an alias for `Reg<DMA_INT_CLR_SPEC>`"]
pub type DMA_INT_CLR = crate::Reg<dma_int_clr::DMA_INT_CLR_SPEC>;
#[doc = ""]
pub mod dma_int_clr;
#[doc = "IN_ERR_EOF_DES_ADDR register accessor: an alias for `Reg<IN_ERR_EOF_DES_ADDR_SPEC>`"]
pub type IN_ERR_EOF_DES_ADDR = crate::Reg<in_err_eof_des_addr::IN_ERR_EOF_DES_ADDR_SPEC>;
#[doc = ""]
pub mod in_err_eof_des_addr;
#[doc = "IN_SUC_EOF_DES_ADDR register accessor: an alias for `Reg<IN_SUC_EOF_DES_ADDR_SPEC>`"]
pub type IN_SUC_EOF_DES_ADDR = crate::Reg<in_suc_eof_des_addr::IN_SUC_EOF_DES_ADDR_SPEC>;
#[doc = ""]
pub mod in_suc_eof_des_addr;
#[doc = "INLINK_DSCR register accessor: an alias for `Reg<INLINK_DSCR_SPEC>`"]
pub type INLINK_DSCR = crate::Reg<inlink_dscr::INLINK_DSCR_SPEC>;
#[doc = ""]
pub mod inlink_dscr;
#[doc = "INLINK_DSCR_BF0 register accessor: an alias for `Reg<INLINK_DSCR_BF0_SPEC>`"]
pub type INLINK_DSCR_BF0 = crate::Reg<inlink_dscr_bf0::INLINK_DSCR_BF0_SPEC>;
#[doc = ""]
pub mod inlink_dscr_bf0;
#[doc = "INLINK_DSCR_BF1 register accessor: an alias for `Reg<INLINK_DSCR_BF1_SPEC>`"]
pub type INLINK_DSCR_BF1 = crate::Reg<inlink_dscr_bf1::INLINK_DSCR_BF1_SPEC>;
#[doc = ""]
pub mod inlink_dscr_bf1;
#[doc = "OUT_EOF_BFR_DES_ADDR register accessor: an alias for `Reg<OUT_EOF_BFR_DES_ADDR_SPEC>`"]
pub type OUT_EOF_BFR_DES_ADDR = crate::Reg<out_eof_bfr_des_addr::OUT_EOF_BFR_DES_ADDR_SPEC>;
#[doc = ""]
pub mod out_eof_bfr_des_addr;
#[doc = "OUT_EOF_DES_ADDR register accessor: an alias for `Reg<OUT_EOF_DES_ADDR_SPEC>`"]
pub type OUT_EOF_DES_ADDR = crate::Reg<out_eof_des_addr::OUT_EOF_DES_ADDR_SPEC>;
#[doc = ""]
pub mod out_eof_des_addr;
#[doc = "OUTLINK_DSCR register accessor: an alias for `Reg<OUTLINK_DSCR_SPEC>`"]
pub type OUTLINK_DSCR = crate::Reg<outlink_dscr::OUTLINK_DSCR_SPEC>;
#[doc = ""]
pub mod outlink_dscr;
#[doc = "OUTLINK_DSCR_BF0 register accessor: an alias for `Reg<OUTLINK_DSCR_BF0_SPEC>`"]
pub type OUTLINK_DSCR_BF0 = crate::Reg<outlink_dscr_bf0::OUTLINK_DSCR_BF0_SPEC>;
#[doc = ""]
pub mod outlink_dscr_bf0;
#[doc = "OUTLINK_DSCR_BF1 register accessor: an alias for `Reg<OUTLINK_DSCR_BF1_SPEC>`"]
pub type OUTLINK_DSCR_BF1 = crate::Reg<outlink_dscr_bf1::OUTLINK_DSCR_BF1_SPEC>;
#[doc = ""]
pub mod outlink_dscr_bf1;
#[doc = "DMA_RSTATUS register accessor: an alias for `Reg<DMA_RSTATUS_SPEC>`"]
pub type DMA_RSTATUS = crate::Reg<dma_rstatus::DMA_RSTATUS_SPEC>;
#[doc = ""]
pub mod dma_rstatus;
#[doc = "DMA_TSTATUS register accessor: an alias for `Reg<DMA_TSTATUS_SPEC>`"]
pub type DMA_TSTATUS = crate::Reg<dma_tstatus::DMA_TSTATUS_SPEC>;
#[doc = ""]
pub mod dma_tstatus;
#[doc = "DATE register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = ""]
pub mod date;
