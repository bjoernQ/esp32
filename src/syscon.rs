#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - "]
    pub sysclk_conf: crate::Reg<sysclk_conf::SYSCLK_CONF_SPEC>,
    #[doc = "0x04 - "]
    pub xtal_tick_conf: crate::Reg<xtal_tick_conf::XTAL_TICK_CONF_SPEC>,
    #[doc = "0x08 - "]
    pub pll_tick_conf: crate::Reg<pll_tick_conf::PLL_TICK_CONF_SPEC>,
    #[doc = "0x0c - "]
    pub ck8m_tick_conf: crate::Reg<ck8m_tick_conf::CK8M_TICK_CONF_SPEC>,
    #[doc = "0x10 - "]
    pub saradc_ctrl: crate::Reg<saradc_ctrl::SARADC_CTRL_SPEC>,
    #[doc = "0x14 - "]
    pub saradc_ctrl2: crate::Reg<saradc_ctrl2::SARADC_CTRL2_SPEC>,
    #[doc = "0x18 - "]
    pub saradc_fsm: crate::Reg<saradc_fsm::SARADC_FSM_SPEC>,
    #[doc = "0x1c - "]
    pub saradc_sar1_patt_tab1: crate::Reg<saradc_sar1_patt_tab1::SARADC_SAR1_PATT_TAB1_SPEC>,
    #[doc = "0x20 - "]
    pub saradc_sar1_patt_tab2: crate::Reg<saradc_sar1_patt_tab2::SARADC_SAR1_PATT_TAB2_SPEC>,
    #[doc = "0x24 - "]
    pub saradc_sar1_patt_tab3: crate::Reg<saradc_sar1_patt_tab3::SARADC_SAR1_PATT_TAB3_SPEC>,
    #[doc = "0x28 - "]
    pub saradc_sar1_patt_tab4: crate::Reg<saradc_sar1_patt_tab4::SARADC_SAR1_PATT_TAB4_SPEC>,
    #[doc = "0x2c - "]
    pub saradc_sar2_patt_tab1: crate::Reg<saradc_sar2_patt_tab1::SARADC_SAR2_PATT_TAB1_SPEC>,
    #[doc = "0x30 - "]
    pub saradc_sar2_patt_tab2: crate::Reg<saradc_sar2_patt_tab2::SARADC_SAR2_PATT_TAB2_SPEC>,
    #[doc = "0x34 - "]
    pub saradc_sar2_patt_tab3: crate::Reg<saradc_sar2_patt_tab3::SARADC_SAR2_PATT_TAB3_SPEC>,
    #[doc = "0x38 - "]
    pub saradc_sar2_patt_tab4: crate::Reg<saradc_sar2_patt_tab4::SARADC_SAR2_PATT_TAB4_SPEC>,
    #[doc = "0x3c - "]
    pub apll_tick_conf: crate::Reg<apll_tick_conf::APLL_TICK_CONF_SPEC>,
    _reserved16: [u8; 0x3c],
    #[doc = "0x7c - "]
    pub date: crate::Reg<date::DATE_SPEC>,
}
#[doc = "SYSCLK_CONF register accessor: an alias for `Reg<SYSCLK_CONF_SPEC>`"]
pub type SYSCLK_CONF = crate::Reg<sysclk_conf::SYSCLK_CONF_SPEC>;
#[doc = ""]
pub mod sysclk_conf;
#[doc = "XTAL_TICK_CONF register accessor: an alias for `Reg<XTAL_TICK_CONF_SPEC>`"]
pub type XTAL_TICK_CONF = crate::Reg<xtal_tick_conf::XTAL_TICK_CONF_SPEC>;
#[doc = ""]
pub mod xtal_tick_conf;
#[doc = "PLL_TICK_CONF register accessor: an alias for `Reg<PLL_TICK_CONF_SPEC>`"]
pub type PLL_TICK_CONF = crate::Reg<pll_tick_conf::PLL_TICK_CONF_SPEC>;
#[doc = ""]
pub mod pll_tick_conf;
#[doc = "CK8M_TICK_CONF register accessor: an alias for `Reg<CK8M_TICK_CONF_SPEC>`"]
pub type CK8M_TICK_CONF = crate::Reg<ck8m_tick_conf::CK8M_TICK_CONF_SPEC>;
#[doc = ""]
pub mod ck8m_tick_conf;
#[doc = "SARADC_CTRL register accessor: an alias for `Reg<SARADC_CTRL_SPEC>`"]
pub type SARADC_CTRL = crate::Reg<saradc_ctrl::SARADC_CTRL_SPEC>;
#[doc = ""]
pub mod saradc_ctrl;
#[doc = "SARADC_CTRL2 register accessor: an alias for `Reg<SARADC_CTRL2_SPEC>`"]
pub type SARADC_CTRL2 = crate::Reg<saradc_ctrl2::SARADC_CTRL2_SPEC>;
#[doc = ""]
pub mod saradc_ctrl2;
#[doc = "SARADC_FSM register accessor: an alias for `Reg<SARADC_FSM_SPEC>`"]
pub type SARADC_FSM = crate::Reg<saradc_fsm::SARADC_FSM_SPEC>;
#[doc = ""]
pub mod saradc_fsm;
#[doc = "SARADC_SAR1_PATT_TAB1 register accessor: an alias for `Reg<SARADC_SAR1_PATT_TAB1_SPEC>`"]
pub type SARADC_SAR1_PATT_TAB1 = crate::Reg<saradc_sar1_patt_tab1::SARADC_SAR1_PATT_TAB1_SPEC>;
#[doc = ""]
pub mod saradc_sar1_patt_tab1;
#[doc = "SARADC_SAR1_PATT_TAB2 register accessor: an alias for `Reg<SARADC_SAR1_PATT_TAB2_SPEC>`"]
pub type SARADC_SAR1_PATT_TAB2 = crate::Reg<saradc_sar1_patt_tab2::SARADC_SAR1_PATT_TAB2_SPEC>;
#[doc = ""]
pub mod saradc_sar1_patt_tab2;
#[doc = "SARADC_SAR1_PATT_TAB3 register accessor: an alias for `Reg<SARADC_SAR1_PATT_TAB3_SPEC>`"]
pub type SARADC_SAR1_PATT_TAB3 = crate::Reg<saradc_sar1_patt_tab3::SARADC_SAR1_PATT_TAB3_SPEC>;
#[doc = ""]
pub mod saradc_sar1_patt_tab3;
#[doc = "SARADC_SAR1_PATT_TAB4 register accessor: an alias for `Reg<SARADC_SAR1_PATT_TAB4_SPEC>`"]
pub type SARADC_SAR1_PATT_TAB4 = crate::Reg<saradc_sar1_patt_tab4::SARADC_SAR1_PATT_TAB4_SPEC>;
#[doc = ""]
pub mod saradc_sar1_patt_tab4;
#[doc = "SARADC_SAR2_PATT_TAB1 register accessor: an alias for `Reg<SARADC_SAR2_PATT_TAB1_SPEC>`"]
pub type SARADC_SAR2_PATT_TAB1 = crate::Reg<saradc_sar2_patt_tab1::SARADC_SAR2_PATT_TAB1_SPEC>;
#[doc = ""]
pub mod saradc_sar2_patt_tab1;
#[doc = "SARADC_SAR2_PATT_TAB2 register accessor: an alias for `Reg<SARADC_SAR2_PATT_TAB2_SPEC>`"]
pub type SARADC_SAR2_PATT_TAB2 = crate::Reg<saradc_sar2_patt_tab2::SARADC_SAR2_PATT_TAB2_SPEC>;
#[doc = ""]
pub mod saradc_sar2_patt_tab2;
#[doc = "SARADC_SAR2_PATT_TAB3 register accessor: an alias for `Reg<SARADC_SAR2_PATT_TAB3_SPEC>`"]
pub type SARADC_SAR2_PATT_TAB3 = crate::Reg<saradc_sar2_patt_tab3::SARADC_SAR2_PATT_TAB3_SPEC>;
#[doc = ""]
pub mod saradc_sar2_patt_tab3;
#[doc = "SARADC_SAR2_PATT_TAB4 register accessor: an alias for `Reg<SARADC_SAR2_PATT_TAB4_SPEC>`"]
pub type SARADC_SAR2_PATT_TAB4 = crate::Reg<saradc_sar2_patt_tab4::SARADC_SAR2_PATT_TAB4_SPEC>;
#[doc = ""]
pub mod saradc_sar2_patt_tab4;
#[doc = "APLL_TICK_CONF register accessor: an alias for `Reg<APLL_TICK_CONF_SPEC>`"]
pub type APLL_TICK_CONF = crate::Reg<apll_tick_conf::APLL_TICK_CONF_SPEC>;
#[doc = ""]
pub mod apll_tick_conf;
#[doc = "DATE register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = ""]
pub mod date;
