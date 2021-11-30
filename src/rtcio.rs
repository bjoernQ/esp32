#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - "]
    pub out: crate::Reg<out::OUT_SPEC>,
    #[doc = "0x04 - "]
    pub out_w1ts: crate::Reg<out_w1ts::OUT_W1TS_SPEC>,
    #[doc = "0x08 - "]
    pub out_w1tc: crate::Reg<out_w1tc::OUT_W1TC_SPEC>,
    #[doc = "0x0c - "]
    pub enable: crate::Reg<enable::ENABLE_SPEC>,
    #[doc = "0x10 - "]
    pub enable_w1ts: crate::Reg<enable_w1ts::ENABLE_W1TS_SPEC>,
    #[doc = "0x14 - "]
    pub enable_w1tc: crate::Reg<enable_w1tc::ENABLE_W1TC_SPEC>,
    #[doc = "0x18 - "]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x1c - "]
    pub status_w1ts: crate::Reg<status_w1ts::STATUS_W1TS_SPEC>,
    #[doc = "0x20 - "]
    pub status_w1tc: crate::Reg<status_w1tc::STATUS_W1TC_SPEC>,
    #[doc = "0x24 - "]
    pub in_: crate::Reg<in_::IN_SPEC>,
    #[doc = "0x28..0x70 - "]
    pub pin: [crate::Reg<pin::PIN_SPEC>; 18],
    #[doc = "0x70 - "]
    pub rtc_debug_sel: crate::Reg<rtc_debug_sel::RTC_DEBUG_SEL_SPEC>,
    #[doc = "0x74 - "]
    pub dig_pad_hold: crate::Reg<dig_pad_hold::DIG_PAD_HOLD_SPEC>,
    #[doc = "0x78 - "]
    pub hall_sens: crate::Reg<hall_sens::HALL_SENS_SPEC>,
    #[doc = "0x7c - "]
    pub sensor_pads: crate::Reg<sensor_pads::SENSOR_PADS_SPEC>,
    #[doc = "0x80 - "]
    pub adc_pad: crate::Reg<adc_pad::ADC_PAD_SPEC>,
    #[doc = "0x84 - "]
    pub pad_dac1: crate::Reg<pad_dac1::PAD_DAC1_SPEC>,
    #[doc = "0x88 - "]
    pub pad_dac2: crate::Reg<pad_dac2::PAD_DAC2_SPEC>,
    #[doc = "0x8c - "]
    pub xtal_32k_pad: crate::Reg<xtal_32k_pad::XTAL_32K_PAD_SPEC>,
    #[doc = "0x90 - "]
    pub touch_cfg: crate::Reg<touch_cfg::TOUCH_CFG_SPEC>,
    #[doc = "0x94 - "]
    pub touch_pad0: crate::Reg<touch_pad0::TOUCH_PAD0_SPEC>,
    #[doc = "0x98 - "]
    pub touch_pad1: crate::Reg<touch_pad1::TOUCH_PAD1_SPEC>,
    #[doc = "0x9c - "]
    pub touch_pad2: crate::Reg<touch_pad2::TOUCH_PAD2_SPEC>,
    #[doc = "0xa0 - "]
    pub touch_pad3: crate::Reg<touch_pad3::TOUCH_PAD3_SPEC>,
    #[doc = "0xa4 - "]
    pub touch_pad4: crate::Reg<touch_pad4::TOUCH_PAD4_SPEC>,
    #[doc = "0xa8 - "]
    pub touch_pad5: crate::Reg<touch_pad5::TOUCH_PAD5_SPEC>,
    #[doc = "0xac - "]
    pub touch_pad6: crate::Reg<touch_pad6::TOUCH_PAD6_SPEC>,
    #[doc = "0xb0 - "]
    pub touch_pad7: crate::Reg<touch_pad7::TOUCH_PAD7_SPEC>,
    #[doc = "0xb4 - "]
    pub touch_pad8: crate::Reg<touch_pad8::TOUCH_PAD8_SPEC>,
    #[doc = "0xb8 - "]
    pub touch_pad9: crate::Reg<touch_pad9::TOUCH_PAD9_SPEC>,
    #[doc = "0xbc - "]
    pub ext_wakeup0: crate::Reg<ext_wakeup0::EXT_WAKEUP0_SPEC>,
    #[doc = "0xc0 - "]
    pub xtl_ext_ctr: crate::Reg<xtl_ext_ctr::XTL_EXT_CTR_SPEC>,
    #[doc = "0xc4 - "]
    pub sar_i2c_io: crate::Reg<sar_i2c_io::SAR_I2C_IO_SPEC>,
    #[doc = "0xc8 - "]
    pub date: crate::Reg<date::DATE_SPEC>,
}
#[doc = "OUT register accessor: an alias for `Reg<OUT_SPEC>`"]
pub type OUT = crate::Reg<out::OUT_SPEC>;
#[doc = ""]
pub mod out;
#[doc = "OUT_W1TS register accessor: an alias for `Reg<OUT_W1TS_SPEC>`"]
pub type OUT_W1TS = crate::Reg<out_w1ts::OUT_W1TS_SPEC>;
#[doc = ""]
pub mod out_w1ts;
#[doc = "OUT_W1TC register accessor: an alias for `Reg<OUT_W1TC_SPEC>`"]
pub type OUT_W1TC = crate::Reg<out_w1tc::OUT_W1TC_SPEC>;
#[doc = ""]
pub mod out_w1tc;
#[doc = "ENABLE register accessor: an alias for `Reg<ENABLE_SPEC>`"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = ""]
pub mod enable;
#[doc = "ENABLE_W1TS register accessor: an alias for `Reg<ENABLE_W1TS_SPEC>`"]
pub type ENABLE_W1TS = crate::Reg<enable_w1ts::ENABLE_W1TS_SPEC>;
#[doc = ""]
pub mod enable_w1ts;
#[doc = "ENABLE_W1TC register accessor: an alias for `Reg<ENABLE_W1TC_SPEC>`"]
pub type ENABLE_W1TC = crate::Reg<enable_w1tc::ENABLE_W1TC_SPEC>;
#[doc = ""]
pub mod enable_w1tc;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = ""]
pub mod status;
#[doc = "STATUS_W1TS register accessor: an alias for `Reg<STATUS_W1TS_SPEC>`"]
pub type STATUS_W1TS = crate::Reg<status_w1ts::STATUS_W1TS_SPEC>;
#[doc = ""]
pub mod status_w1ts;
#[doc = "STATUS_W1TC register accessor: an alias for `Reg<STATUS_W1TC_SPEC>`"]
pub type STATUS_W1TC = crate::Reg<status_w1tc::STATUS_W1TC_SPEC>;
#[doc = ""]
pub mod status_w1tc;
#[doc = "IN register accessor: an alias for `Reg<IN_SPEC>`"]
pub type IN = crate::Reg<in_::IN_SPEC>;
#[doc = ""]
pub mod in_;
#[doc = "PIN register accessor: an alias for `Reg<PIN_SPEC>`"]
pub type PIN = crate::Reg<pin::PIN_SPEC>;
#[doc = ""]
pub mod pin;
#[doc = "RTC_DEBUG_SEL register accessor: an alias for `Reg<RTC_DEBUG_SEL_SPEC>`"]
pub type RTC_DEBUG_SEL = crate::Reg<rtc_debug_sel::RTC_DEBUG_SEL_SPEC>;
#[doc = ""]
pub mod rtc_debug_sel;
#[doc = "DIG_PAD_HOLD register accessor: an alias for `Reg<DIG_PAD_HOLD_SPEC>`"]
pub type DIG_PAD_HOLD = crate::Reg<dig_pad_hold::DIG_PAD_HOLD_SPEC>;
#[doc = ""]
pub mod dig_pad_hold;
#[doc = "HALL_SENS register accessor: an alias for `Reg<HALL_SENS_SPEC>`"]
pub type HALL_SENS = crate::Reg<hall_sens::HALL_SENS_SPEC>;
#[doc = ""]
pub mod hall_sens;
#[doc = "SENSOR_PADS register accessor: an alias for `Reg<SENSOR_PADS_SPEC>`"]
pub type SENSOR_PADS = crate::Reg<sensor_pads::SENSOR_PADS_SPEC>;
#[doc = ""]
pub mod sensor_pads;
#[doc = "ADC_PAD register accessor: an alias for `Reg<ADC_PAD_SPEC>`"]
pub type ADC_PAD = crate::Reg<adc_pad::ADC_PAD_SPEC>;
#[doc = ""]
pub mod adc_pad;
#[doc = "PAD_DAC1 register accessor: an alias for `Reg<PAD_DAC1_SPEC>`"]
pub type PAD_DAC1 = crate::Reg<pad_dac1::PAD_DAC1_SPEC>;
#[doc = ""]
pub mod pad_dac1;
#[doc = "PAD_DAC2 register accessor: an alias for `Reg<PAD_DAC2_SPEC>`"]
pub type PAD_DAC2 = crate::Reg<pad_dac2::PAD_DAC2_SPEC>;
#[doc = ""]
pub mod pad_dac2;
#[doc = "XTAL_32K_PAD register accessor: an alias for `Reg<XTAL_32K_PAD_SPEC>`"]
pub type XTAL_32K_PAD = crate::Reg<xtal_32k_pad::XTAL_32K_PAD_SPEC>;
#[doc = ""]
pub mod xtal_32k_pad;
#[doc = "TOUCH_CFG register accessor: an alias for `Reg<TOUCH_CFG_SPEC>`"]
pub type TOUCH_CFG = crate::Reg<touch_cfg::TOUCH_CFG_SPEC>;
#[doc = ""]
pub mod touch_cfg;
#[doc = "TOUCH_PAD0 register accessor: an alias for `Reg<TOUCH_PAD0_SPEC>`"]
pub type TOUCH_PAD0 = crate::Reg<touch_pad0::TOUCH_PAD0_SPEC>;
#[doc = ""]
pub mod touch_pad0;
#[doc = "TOUCH_PAD1 register accessor: an alias for `Reg<TOUCH_PAD1_SPEC>`"]
pub type TOUCH_PAD1 = crate::Reg<touch_pad1::TOUCH_PAD1_SPEC>;
#[doc = ""]
pub mod touch_pad1;
#[doc = "TOUCH_PAD2 register accessor: an alias for `Reg<TOUCH_PAD2_SPEC>`"]
pub type TOUCH_PAD2 = crate::Reg<touch_pad2::TOUCH_PAD2_SPEC>;
#[doc = ""]
pub mod touch_pad2;
#[doc = "TOUCH_PAD3 register accessor: an alias for `Reg<TOUCH_PAD3_SPEC>`"]
pub type TOUCH_PAD3 = crate::Reg<touch_pad3::TOUCH_PAD3_SPEC>;
#[doc = ""]
pub mod touch_pad3;
#[doc = "TOUCH_PAD4 register accessor: an alias for `Reg<TOUCH_PAD4_SPEC>`"]
pub type TOUCH_PAD4 = crate::Reg<touch_pad4::TOUCH_PAD4_SPEC>;
#[doc = ""]
pub mod touch_pad4;
#[doc = "TOUCH_PAD5 register accessor: an alias for `Reg<TOUCH_PAD5_SPEC>`"]
pub type TOUCH_PAD5 = crate::Reg<touch_pad5::TOUCH_PAD5_SPEC>;
#[doc = ""]
pub mod touch_pad5;
#[doc = "TOUCH_PAD6 register accessor: an alias for `Reg<TOUCH_PAD6_SPEC>`"]
pub type TOUCH_PAD6 = crate::Reg<touch_pad6::TOUCH_PAD6_SPEC>;
#[doc = ""]
pub mod touch_pad6;
#[doc = "TOUCH_PAD7 register accessor: an alias for `Reg<TOUCH_PAD7_SPEC>`"]
pub type TOUCH_PAD7 = crate::Reg<touch_pad7::TOUCH_PAD7_SPEC>;
#[doc = ""]
pub mod touch_pad7;
#[doc = "TOUCH_PAD8 register accessor: an alias for `Reg<TOUCH_PAD8_SPEC>`"]
pub type TOUCH_PAD8 = crate::Reg<touch_pad8::TOUCH_PAD8_SPEC>;
#[doc = ""]
pub mod touch_pad8;
#[doc = "TOUCH_PAD9 register accessor: an alias for `Reg<TOUCH_PAD9_SPEC>`"]
pub type TOUCH_PAD9 = crate::Reg<touch_pad9::TOUCH_PAD9_SPEC>;
#[doc = ""]
pub mod touch_pad9;
#[doc = "EXT_WAKEUP0 register accessor: an alias for `Reg<EXT_WAKEUP0_SPEC>`"]
pub type EXT_WAKEUP0 = crate::Reg<ext_wakeup0::EXT_WAKEUP0_SPEC>;
#[doc = ""]
pub mod ext_wakeup0;
#[doc = "XTL_EXT_CTR register accessor: an alias for `Reg<XTL_EXT_CTR_SPEC>`"]
pub type XTL_EXT_CTR = crate::Reg<xtl_ext_ctr::XTL_EXT_CTR_SPEC>;
#[doc = ""]
pub mod xtl_ext_ctr;
#[doc = "SAR_I2C_IO register accessor: an alias for `Reg<SAR_I2C_IO_SPEC>`"]
pub type SAR_I2C_IO = crate::Reg<sar_i2c_io::SAR_I2C_IO_SPEC>;
#[doc = ""]
pub mod sar_i2c_io;
#[doc = "DATE register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = ""]
pub mod date;
