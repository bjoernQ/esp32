#[doc = "Register `INT_RAW` reader"]
pub struct R(crate::R<INT_RAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_RAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_RAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_RAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXFIFO_FULL_INT_RAW` reader - This interrupt raw bit turns to high level when receiver receives more data than (rx_flow_thrhd_h3 rx_flow_thrhd)."]
pub struct RXFIFO_FULL_INT_RAW_R(crate::FieldReader<bool, bool>);
impl RXFIFO_FULL_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXFIFO_FULL_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFIFO_FULL_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFIFO_EMPTY_INT_RAW` reader - This interrupt raw bit turns to high level when the amount of data in transmitter's fifo is less than ((tx_mem_cnttxfifo_cnt) ."]
pub struct TXFIFO_EMPTY_INT_RAW_R(crate::FieldReader<bool, bool>);
impl TXFIFO_EMPTY_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXFIFO_EMPTY_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFIFO_EMPTY_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PARITY_ERR_INT_RAW` reader - This interrupt raw bit turns to high level when receiver detects the parity error of data."]
pub struct PARITY_ERR_INT_RAW_R(crate::FieldReader<bool, bool>);
impl PARITY_ERR_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PARITY_ERR_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PARITY_ERR_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRM_ERR_INT_RAW` reader - This interrupt raw bit turns to high level when receiver detects data's frame error ."]
pub struct FRM_ERR_INT_RAW_R(crate::FieldReader<bool, bool>);
impl FRM_ERR_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FRM_ERR_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRM_ERR_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFIFO_OVF_INT_RAW` reader - This interrupt raw bit turns to high level when receiver receives more data than the fifo can store."]
pub struct RXFIFO_OVF_INT_RAW_R(crate::FieldReader<bool, bool>);
impl RXFIFO_OVF_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXFIFO_OVF_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFIFO_OVF_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSR_CHG_INT_RAW` reader - This interrupt raw bit turns to high level when receiver detects the edge change of dsrn signal."]
pub struct DSR_CHG_INT_RAW_R(crate::FieldReader<bool, bool>);
impl DSR_CHG_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DSR_CHG_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSR_CHG_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTS_CHG_INT_RAW` reader - This interrupt raw bit turns to high level when receiver detects the edge change of ctsn signal."]
pub struct CTS_CHG_INT_RAW_R(crate::FieldReader<bool, bool>);
impl CTS_CHG_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CTS_CHG_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTS_CHG_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRK_DET_INT_RAW` reader - This interrupt raw bit turns to high level when receiver detects the 0 after the stop bit."]
pub struct BRK_DET_INT_RAW_R(crate::FieldReader<bool, bool>);
impl BRK_DET_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BRK_DET_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BRK_DET_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFIFO_TOUT_INT_RAW` reader - This interrupt raw bit turns to high level when receiver takes more time than rx_tout_thrhd to receive a byte."]
pub struct RXFIFO_TOUT_INT_RAW_R(crate::FieldReader<bool, bool>);
impl RXFIFO_TOUT_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXFIFO_TOUT_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFIFO_TOUT_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_XON_INT_RAW` reader - This interrupt raw bit turns to high level when receiver receives xoff char with uart_sw_flow_con_en is set to 1."]
pub struct SW_XON_INT_RAW_R(crate::FieldReader<bool, bool>);
impl SW_XON_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SW_XON_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_XON_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_XOFF_INT_RAW` reader - This interrupt raw bit turns to high level when receiver receives xon char with uart_sw_flow_con_en is set to 1."]
pub struct SW_XOFF_INT_RAW_R(crate::FieldReader<bool, bool>);
impl SW_XOFF_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SW_XOFF_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_XOFF_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GLITCH_DET_INT_RAW` reader - This interrupt raw bit turns to high level when receiver detects the start bit."]
pub struct GLITCH_DET_INT_RAW_R(crate::FieldReader<bool, bool>);
impl GLITCH_DET_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GLITCH_DET_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GLITCH_DET_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_BRK_DONE_INT_RAW` reader - This interrupt raw bit turns to high level when transmitter completes sendding 0 after all the datas in transmitter's fifo are send."]
pub struct TX_BRK_DONE_INT_RAW_R(crate::FieldReader<bool, bool>);
impl TX_BRK_DONE_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_BRK_DONE_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_BRK_DONE_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_BRK_IDLE_DONE_INT_RAW` reader - This interrupt raw bit turns to high level when transmitter has kept the shortest duration after the last data has been send."]
pub struct TX_BRK_IDLE_DONE_INT_RAW_R(crate::FieldReader<bool, bool>);
impl TX_BRK_IDLE_DONE_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_BRK_IDLE_DONE_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_BRK_IDLE_DONE_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_DONE_INT_RAW` reader - This interrupt raw bit turns to high level when transmitter has send all the data in fifo."]
pub struct TX_DONE_INT_RAW_R(crate::FieldReader<bool, bool>);
impl TX_DONE_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_DONE_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_DONE_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RS485_PARITY_ERR_INT_RAW` reader - This interrupt raw bit turns to high level when rs485 detects the parity error."]
pub struct RS485_PARITY_ERR_INT_RAW_R(crate::FieldReader<bool, bool>);
impl RS485_PARITY_ERR_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RS485_PARITY_ERR_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RS485_PARITY_ERR_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RS485_FRM_ERR_INT_RAW` reader - This interrupt raw bit turns to high level when rs485 detects the data frame error."]
pub struct RS485_FRM_ERR_INT_RAW_R(crate::FieldReader<bool, bool>);
impl RS485_FRM_ERR_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RS485_FRM_ERR_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RS485_FRM_ERR_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RS485_CLASH_INT_RAW` reader - This interrupt raw bit turns to high level when rs485 detects the clash between transmitter and receiver."]
pub struct RS485_CLASH_INT_RAW_R(crate::FieldReader<bool, bool>);
impl RS485_CLASH_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RS485_CLASH_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RS485_CLASH_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AT_CMD_CHAR_DET_INT_RAW` reader - This interrupt raw bit turns to high level when receiver detects the configured at_cmd chars."]
pub struct AT_CMD_CHAR_DET_INT_RAW_R(crate::FieldReader<bool, bool>);
impl AT_CMD_CHAR_DET_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AT_CMD_CHAR_DET_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AT_CMD_CHAR_DET_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - This interrupt raw bit turns to high level when receiver receives more data than (rx_flow_thrhd_h3 rx_flow_thrhd)."]
    #[inline(always)]
    pub fn rxfifo_full_int_raw(&self) -> RXFIFO_FULL_INT_RAW_R {
        RXFIFO_FULL_INT_RAW_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - This interrupt raw bit turns to high level when the amount of data in transmitter's fifo is less than ((tx_mem_cnttxfifo_cnt) ."]
    #[inline(always)]
    pub fn txfifo_empty_int_raw(&self) -> TXFIFO_EMPTY_INT_RAW_R {
        TXFIFO_EMPTY_INT_RAW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - This interrupt raw bit turns to high level when receiver detects the parity error of data."]
    #[inline(always)]
    pub fn parity_err_int_raw(&self) -> PARITY_ERR_INT_RAW_R {
        PARITY_ERR_INT_RAW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - This interrupt raw bit turns to high level when receiver detects data's frame error ."]
    #[inline(always)]
    pub fn frm_err_int_raw(&self) -> FRM_ERR_INT_RAW_R {
        FRM_ERR_INT_RAW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - This interrupt raw bit turns to high level when receiver receives more data than the fifo can store."]
    #[inline(always)]
    pub fn rxfifo_ovf_int_raw(&self) -> RXFIFO_OVF_INT_RAW_R {
        RXFIFO_OVF_INT_RAW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - This interrupt raw bit turns to high level when receiver detects the edge change of dsrn signal."]
    #[inline(always)]
    pub fn dsr_chg_int_raw(&self) -> DSR_CHG_INT_RAW_R {
        DSR_CHG_INT_RAW_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - This interrupt raw bit turns to high level when receiver detects the edge change of ctsn signal."]
    #[inline(always)]
    pub fn cts_chg_int_raw(&self) -> CTS_CHG_INT_RAW_R {
        CTS_CHG_INT_RAW_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - This interrupt raw bit turns to high level when receiver detects the 0 after the stop bit."]
    #[inline(always)]
    pub fn brk_det_int_raw(&self) -> BRK_DET_INT_RAW_R {
        BRK_DET_INT_RAW_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - This interrupt raw bit turns to high level when receiver takes more time than rx_tout_thrhd to receive a byte."]
    #[inline(always)]
    pub fn rxfifo_tout_int_raw(&self) -> RXFIFO_TOUT_INT_RAW_R {
        RXFIFO_TOUT_INT_RAW_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - This interrupt raw bit turns to high level when receiver receives xoff char with uart_sw_flow_con_en is set to 1."]
    #[inline(always)]
    pub fn sw_xon_int_raw(&self) -> SW_XON_INT_RAW_R {
        SW_XON_INT_RAW_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - This interrupt raw bit turns to high level when receiver receives xon char with uart_sw_flow_con_en is set to 1."]
    #[inline(always)]
    pub fn sw_xoff_int_raw(&self) -> SW_XOFF_INT_RAW_R {
        SW_XOFF_INT_RAW_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - This interrupt raw bit turns to high level when receiver detects the start bit."]
    #[inline(always)]
    pub fn glitch_det_int_raw(&self) -> GLITCH_DET_INT_RAW_R {
        GLITCH_DET_INT_RAW_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - This interrupt raw bit turns to high level when transmitter completes sendding 0 after all the datas in transmitter's fifo are send."]
    #[inline(always)]
    pub fn tx_brk_done_int_raw(&self) -> TX_BRK_DONE_INT_RAW_R {
        TX_BRK_DONE_INT_RAW_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - This interrupt raw bit turns to high level when transmitter has kept the shortest duration after the last data has been send."]
    #[inline(always)]
    pub fn tx_brk_idle_done_int_raw(&self) -> TX_BRK_IDLE_DONE_INT_RAW_R {
        TX_BRK_IDLE_DONE_INT_RAW_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - This interrupt raw bit turns to high level when transmitter has send all the data in fifo."]
    #[inline(always)]
    pub fn tx_done_int_raw(&self) -> TX_DONE_INT_RAW_R {
        TX_DONE_INT_RAW_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - This interrupt raw bit turns to high level when rs485 detects the parity error."]
    #[inline(always)]
    pub fn rs485_parity_err_int_raw(&self) -> RS485_PARITY_ERR_INT_RAW_R {
        RS485_PARITY_ERR_INT_RAW_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - This interrupt raw bit turns to high level when rs485 detects the data frame error."]
    #[inline(always)]
    pub fn rs485_frm_err_int_raw(&self) -> RS485_FRM_ERR_INT_RAW_R {
        RS485_FRM_ERR_INT_RAW_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - This interrupt raw bit turns to high level when rs485 detects the clash between transmitter and receiver."]
    #[inline(always)]
    pub fn rs485_clash_int_raw(&self) -> RS485_CLASH_INT_RAW_R {
        RS485_CLASH_INT_RAW_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - This interrupt raw bit turns to high level when receiver detects the configured at_cmd chars."]
    #[inline(always)]
    pub fn at_cmd_char_det_int_raw(&self) -> AT_CMD_CHAR_DET_INT_RAW_R {
        AT_CMD_CHAR_DET_INT_RAW_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_raw](index.html) module"]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_raw::R](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
