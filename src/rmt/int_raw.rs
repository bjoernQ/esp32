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
#[doc = "Field `CH0_TX_END_INT_RAW` reader - The interrupt raw bit for channel 0 turns to high level when the transmit process is done."]
pub struct CH0_TX_END_INT_RAW_R(crate::FieldReader<bool, bool>);
impl CH0_TX_END_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH0_TX_END_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH0_TX_END_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH0_RX_END_INT_RAW` reader - The interrupt raw bit for channel 0 turns to high level when the receive process is done."]
pub struct CH0_RX_END_INT_RAW_R(crate::FieldReader<bool, bool>);
impl CH0_RX_END_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH0_RX_END_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH0_RX_END_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH0_ERR_INT_RAW` reader - The interrupt raw bit for channel 0 turns to high level when channle 0 detects some errors."]
pub struct CH0_ERR_INT_RAW_R(crate::FieldReader<bool, bool>);
impl CH0_ERR_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH0_ERR_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH0_ERR_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH1_TX_END_INT_RAW` reader - The interrupt raw bit for channel 1 turns to high level when the transmit process is done."]
pub struct CH1_TX_END_INT_RAW_R(crate::FieldReader<bool, bool>);
impl CH1_TX_END_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH1_TX_END_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH1_TX_END_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH1_RX_END_INT_RAW` reader - The interrupt raw bit for channel 1 turns to high level when the receive process is done."]
pub struct CH1_RX_END_INT_RAW_R(crate::FieldReader<bool, bool>);
impl CH1_RX_END_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH1_RX_END_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH1_RX_END_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH1_ERR_INT_RAW` reader - The interrupt raw bit for channel 1 turns to high level when channle 1 detects some errors."]
pub struct CH1_ERR_INT_RAW_R(crate::FieldReader<bool, bool>);
impl CH1_ERR_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH1_ERR_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH1_ERR_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH2_TX_END_INT_RAW` reader - The interrupt raw bit for channel 2 turns to high level when the transmit process is done."]
pub struct CH2_TX_END_INT_RAW_R(crate::FieldReader<bool, bool>);
impl CH2_TX_END_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH2_TX_END_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH2_TX_END_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH2_RX_END_INT_RAW` reader - The interrupt raw bit for channel 2 turns to high level when the receive process is done."]
pub struct CH2_RX_END_INT_RAW_R(crate::FieldReader<bool, bool>);
impl CH2_RX_END_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH2_RX_END_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH2_RX_END_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH2_ERR_INT_RAW` reader - The interrupt raw bit for channel 2 turns to high level when channle 2 detects some errors."]
pub struct CH2_ERR_INT_RAW_R(crate::FieldReader<bool, bool>);
impl CH2_ERR_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH2_ERR_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH2_ERR_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH3_TX_END_INT_RAW` reader - The interrupt raw bit for channel 3 turns to high level when the transmit process is done."]
pub struct CH3_TX_END_INT_RAW_R(crate::FieldReader<bool, bool>);
impl CH3_TX_END_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH3_TX_END_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH3_TX_END_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH3_RX_END_INT_RAW` reader - The interrupt raw bit for channel 3 turns to high level when the receive process is done."]
pub struct CH3_RX_END_INT_RAW_R(crate::FieldReader<bool, bool>);
impl CH3_RX_END_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH3_RX_END_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH3_RX_END_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH3_ERR_INT_RAW` reader - The interrupt raw bit for channel 3 turns to high level when channle 3 detects some errors."]
pub struct CH3_ERR_INT_RAW_R(crate::FieldReader<bool, bool>);
impl CH3_ERR_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH3_ERR_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH3_ERR_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH4_TX_END_INT_RAW` reader - The interrupt raw bit for channel 4 turns to high level when the transmit process is done."]
pub struct CH4_TX_END_INT_RAW_R(crate::FieldReader<bool, bool>);
impl CH4_TX_END_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH4_TX_END_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH4_TX_END_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH4_RX_END_INT_RAW` reader - The interrupt raw bit for channel 4 turns to high level when the receive process is done."]
pub struct CH4_RX_END_INT_RAW_R(crate::FieldReader<bool, bool>);
impl CH4_RX_END_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH4_RX_END_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH4_RX_END_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH4_ERR_INT_RAW` reader - The interrupt raw bit for channel 4 turns to high level when channle 4 detects some errors."]
pub struct CH4_ERR_INT_RAW_R(crate::FieldReader<bool, bool>);
impl CH4_ERR_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH4_ERR_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH4_ERR_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH5_TX_END_INT_RAW` reader - The interrupt raw bit for channel 5 turns to high level when the transmit process is done."]
pub struct CH5_TX_END_INT_RAW_R(crate::FieldReader<bool, bool>);
impl CH5_TX_END_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH5_TX_END_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH5_TX_END_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH5_RX_END_INT_RAW` reader - The interrupt raw bit for channel 5 turns to high level when the receive process is done."]
pub struct CH5_RX_END_INT_RAW_R(crate::FieldReader<bool, bool>);
impl CH5_RX_END_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH5_RX_END_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH5_RX_END_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH5_ERR_INT_RAW` reader - The interrupt raw bit for channel 5 turns to high level when channle 5 detects some errors."]
pub struct CH5_ERR_INT_RAW_R(crate::FieldReader<bool, bool>);
impl CH5_ERR_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH5_ERR_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH5_ERR_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH6_TX_END_INT_RAW` reader - The interrupt raw bit for channel 6 turns to high level when the transmit process is done."]
pub struct CH6_TX_END_INT_RAW_R(crate::FieldReader<bool, bool>);
impl CH6_TX_END_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH6_TX_END_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH6_TX_END_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH6_RX_END_INT_RAW` reader - The interrupt raw bit for channel 6 turns to high level when the receive process is done."]
pub struct CH6_RX_END_INT_RAW_R(crate::FieldReader<bool, bool>);
impl CH6_RX_END_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH6_RX_END_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH6_RX_END_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH6_ERR_INT_RAW` reader - The interrupt raw bit for channel 6 turns to high level when channle 6 detects some errors."]
pub struct CH6_ERR_INT_RAW_R(crate::FieldReader<bool, bool>);
impl CH6_ERR_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH6_ERR_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH6_ERR_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH7_TX_END_INT_RAW` reader - The interrupt raw bit for channel 7 turns to high level when the transmit process is done."]
pub struct CH7_TX_END_INT_RAW_R(crate::FieldReader<bool, bool>);
impl CH7_TX_END_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH7_TX_END_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH7_TX_END_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH7_RX_END_INT_RAW` reader - The interrupt raw bit for channel 7 turns to high level when the receive process is done."]
pub struct CH7_RX_END_INT_RAW_R(crate::FieldReader<bool, bool>);
impl CH7_RX_END_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH7_RX_END_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH7_RX_END_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH7_ERR_INT_RAW` reader - The interrupt raw bit for channel 7 turns to high level when channle 7 detects some errors."]
pub struct CH7_ERR_INT_RAW_R(crate::FieldReader<bool, bool>);
impl CH7_ERR_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH7_ERR_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH7_ERR_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH0_TX_THR_EVENT_INT_RAW` reader - The interrupt raw bit for channel 0 turns to high level when transmitter in channle0 have send datas more than reg_rmt_tx_lim_ch0 after detecting this interrupt software can updata the old datas with new datas."]
pub struct CH0_TX_THR_EVENT_INT_RAW_R(crate::FieldReader<bool, bool>);
impl CH0_TX_THR_EVENT_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH0_TX_THR_EVENT_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH0_TX_THR_EVENT_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH1_TX_THR_EVENT_INT_RAW` reader - The interrupt raw bit for channel 1 turns to high level when transmitter in channle1 have send datas more than reg_rmt_tx_lim_ch1 after detecting this interrupt software can updata the old datas with new datas."]
pub struct CH1_TX_THR_EVENT_INT_RAW_R(crate::FieldReader<bool, bool>);
impl CH1_TX_THR_EVENT_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH1_TX_THR_EVENT_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH1_TX_THR_EVENT_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH2_TX_THR_EVENT_INT_RAW` reader - The interrupt raw bit for channel 2 turns to high level when transmitter in channle2 have send datas more than reg_rmt_tx_lim_ch2 after detecting this interrupt software can updata the old datas with new datas."]
pub struct CH2_TX_THR_EVENT_INT_RAW_R(crate::FieldReader<bool, bool>);
impl CH2_TX_THR_EVENT_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH2_TX_THR_EVENT_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH2_TX_THR_EVENT_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH3_TX_THR_EVENT_INT_RAW` reader - The interrupt raw bit for channel 3 turns to high level when transmitter in channle3 have send datas more than reg_rmt_tx_lim_ch3 after detecting this interrupt software can updata the old datas with new datas."]
pub struct CH3_TX_THR_EVENT_INT_RAW_R(crate::FieldReader<bool, bool>);
impl CH3_TX_THR_EVENT_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH3_TX_THR_EVENT_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH3_TX_THR_EVENT_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH4_TX_THR_EVENT_INT_RAW` reader - The interrupt raw bit for channel 4 turns to high level when transmitter in channle4 have send datas more than reg_rmt_tx_lim_ch4 after detecting this interrupt software can updata the old datas with new datas."]
pub struct CH4_TX_THR_EVENT_INT_RAW_R(crate::FieldReader<bool, bool>);
impl CH4_TX_THR_EVENT_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH4_TX_THR_EVENT_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH4_TX_THR_EVENT_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH5_TX_THR_EVENT_INT_RAW` reader - The interrupt raw bit for channel 5 turns to high level when transmitter in channle5 have send datas more than reg_rmt_tx_lim_ch5 after detecting this interrupt software can updata the old datas with new datas."]
pub struct CH5_TX_THR_EVENT_INT_RAW_R(crate::FieldReader<bool, bool>);
impl CH5_TX_THR_EVENT_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH5_TX_THR_EVENT_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH5_TX_THR_EVENT_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH6_TX_THR_EVENT_INT_RAW` reader - The interrupt raw bit for channel 6 turns to high level when transmitter in channle6 have send datas more than reg_rmt_tx_lim_ch6 after detecting this interrupt software can updata the old datas with new datas."]
pub struct CH6_TX_THR_EVENT_INT_RAW_R(crate::FieldReader<bool, bool>);
impl CH6_TX_THR_EVENT_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH6_TX_THR_EVENT_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH6_TX_THR_EVENT_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH7_TX_THR_EVENT_INT_RAW` reader - The interrupt raw bit for channel7 turns to high level when transmitter in channle 7 have send datas more than reg_rmt_tx_lim_ch7 after detecting this interrupt software can updata the old datas with new datas."]
pub struct CH7_TX_THR_EVENT_INT_RAW_R(crate::FieldReader<bool, bool>);
impl CH7_TX_THR_EVENT_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH7_TX_THR_EVENT_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH7_TX_THR_EVENT_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - The interrupt raw bit for channel 0 turns to high level when the transmit process is done."]
    #[inline(always)]
    pub fn ch0_tx_end_int_raw(&self) -> CH0_TX_END_INT_RAW_R {
        CH0_TX_END_INT_RAW_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - The interrupt raw bit for channel 0 turns to high level when the receive process is done."]
    #[inline(always)]
    pub fn ch0_rx_end_int_raw(&self) -> CH0_RX_END_INT_RAW_R {
        CH0_RX_END_INT_RAW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - The interrupt raw bit for channel 0 turns to high level when channle 0 detects some errors."]
    #[inline(always)]
    pub fn ch0_err_int_raw(&self) -> CH0_ERR_INT_RAW_R {
        CH0_ERR_INT_RAW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - The interrupt raw bit for channel 1 turns to high level when the transmit process is done."]
    #[inline(always)]
    pub fn ch1_tx_end_int_raw(&self) -> CH1_TX_END_INT_RAW_R {
        CH1_TX_END_INT_RAW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - The interrupt raw bit for channel 1 turns to high level when the receive process is done."]
    #[inline(always)]
    pub fn ch1_rx_end_int_raw(&self) -> CH1_RX_END_INT_RAW_R {
        CH1_RX_END_INT_RAW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - The interrupt raw bit for channel 1 turns to high level when channle 1 detects some errors."]
    #[inline(always)]
    pub fn ch1_err_int_raw(&self) -> CH1_ERR_INT_RAW_R {
        CH1_ERR_INT_RAW_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - The interrupt raw bit for channel 2 turns to high level when the transmit process is done."]
    #[inline(always)]
    pub fn ch2_tx_end_int_raw(&self) -> CH2_TX_END_INT_RAW_R {
        CH2_TX_END_INT_RAW_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - The interrupt raw bit for channel 2 turns to high level when the receive process is done."]
    #[inline(always)]
    pub fn ch2_rx_end_int_raw(&self) -> CH2_RX_END_INT_RAW_R {
        CH2_RX_END_INT_RAW_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - The interrupt raw bit for channel 2 turns to high level when channle 2 detects some errors."]
    #[inline(always)]
    pub fn ch2_err_int_raw(&self) -> CH2_ERR_INT_RAW_R {
        CH2_ERR_INT_RAW_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - The interrupt raw bit for channel 3 turns to high level when the transmit process is done."]
    #[inline(always)]
    pub fn ch3_tx_end_int_raw(&self) -> CH3_TX_END_INT_RAW_R {
        CH3_TX_END_INT_RAW_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - The interrupt raw bit for channel 3 turns to high level when the receive process is done."]
    #[inline(always)]
    pub fn ch3_rx_end_int_raw(&self) -> CH3_RX_END_INT_RAW_R {
        CH3_RX_END_INT_RAW_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - The interrupt raw bit for channel 3 turns to high level when channle 3 detects some errors."]
    #[inline(always)]
    pub fn ch3_err_int_raw(&self) -> CH3_ERR_INT_RAW_R {
        CH3_ERR_INT_RAW_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - The interrupt raw bit for channel 4 turns to high level when the transmit process is done."]
    #[inline(always)]
    pub fn ch4_tx_end_int_raw(&self) -> CH4_TX_END_INT_RAW_R {
        CH4_TX_END_INT_RAW_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - The interrupt raw bit for channel 4 turns to high level when the receive process is done."]
    #[inline(always)]
    pub fn ch4_rx_end_int_raw(&self) -> CH4_RX_END_INT_RAW_R {
        CH4_RX_END_INT_RAW_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - The interrupt raw bit for channel 4 turns to high level when channle 4 detects some errors."]
    #[inline(always)]
    pub fn ch4_err_int_raw(&self) -> CH4_ERR_INT_RAW_R {
        CH4_ERR_INT_RAW_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - The interrupt raw bit for channel 5 turns to high level when the transmit process is done."]
    #[inline(always)]
    pub fn ch5_tx_end_int_raw(&self) -> CH5_TX_END_INT_RAW_R {
        CH5_TX_END_INT_RAW_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - The interrupt raw bit for channel 5 turns to high level when the receive process is done."]
    #[inline(always)]
    pub fn ch5_rx_end_int_raw(&self) -> CH5_RX_END_INT_RAW_R {
        CH5_RX_END_INT_RAW_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - The interrupt raw bit for channel 5 turns to high level when channle 5 detects some errors."]
    #[inline(always)]
    pub fn ch5_err_int_raw(&self) -> CH5_ERR_INT_RAW_R {
        CH5_ERR_INT_RAW_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - The interrupt raw bit for channel 6 turns to high level when the transmit process is done."]
    #[inline(always)]
    pub fn ch6_tx_end_int_raw(&self) -> CH6_TX_END_INT_RAW_R {
        CH6_TX_END_INT_RAW_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - The interrupt raw bit for channel 6 turns to high level when the receive process is done."]
    #[inline(always)]
    pub fn ch6_rx_end_int_raw(&self) -> CH6_RX_END_INT_RAW_R {
        CH6_RX_END_INT_RAW_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - The interrupt raw bit for channel 6 turns to high level when channle 6 detects some errors."]
    #[inline(always)]
    pub fn ch6_err_int_raw(&self) -> CH6_ERR_INT_RAW_R {
        CH6_ERR_INT_RAW_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - The interrupt raw bit for channel 7 turns to high level when the transmit process is done."]
    #[inline(always)]
    pub fn ch7_tx_end_int_raw(&self) -> CH7_TX_END_INT_RAW_R {
        CH7_TX_END_INT_RAW_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - The interrupt raw bit for channel 7 turns to high level when the receive process is done."]
    #[inline(always)]
    pub fn ch7_rx_end_int_raw(&self) -> CH7_RX_END_INT_RAW_R {
        CH7_RX_END_INT_RAW_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - The interrupt raw bit for channel 7 turns to high level when channle 7 detects some errors."]
    #[inline(always)]
    pub fn ch7_err_int_raw(&self) -> CH7_ERR_INT_RAW_R {
        CH7_ERR_INT_RAW_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - The interrupt raw bit for channel 0 turns to high level when transmitter in channle0 have send datas more than reg_rmt_tx_lim_ch0 after detecting this interrupt software can updata the old datas with new datas."]
    #[inline(always)]
    pub fn ch0_tx_thr_event_int_raw(&self) -> CH0_TX_THR_EVENT_INT_RAW_R {
        CH0_TX_THR_EVENT_INT_RAW_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - The interrupt raw bit for channel 1 turns to high level when transmitter in channle1 have send datas more than reg_rmt_tx_lim_ch1 after detecting this interrupt software can updata the old datas with new datas."]
    #[inline(always)]
    pub fn ch1_tx_thr_event_int_raw(&self) -> CH1_TX_THR_EVENT_INT_RAW_R {
        CH1_TX_THR_EVENT_INT_RAW_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - The interrupt raw bit for channel 2 turns to high level when transmitter in channle2 have send datas more than reg_rmt_tx_lim_ch2 after detecting this interrupt software can updata the old datas with new datas."]
    #[inline(always)]
    pub fn ch2_tx_thr_event_int_raw(&self) -> CH2_TX_THR_EVENT_INT_RAW_R {
        CH2_TX_THR_EVENT_INT_RAW_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - The interrupt raw bit for channel 3 turns to high level when transmitter in channle3 have send datas more than reg_rmt_tx_lim_ch3 after detecting this interrupt software can updata the old datas with new datas."]
    #[inline(always)]
    pub fn ch3_tx_thr_event_int_raw(&self) -> CH3_TX_THR_EVENT_INT_RAW_R {
        CH3_TX_THR_EVENT_INT_RAW_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - The interrupt raw bit for channel 4 turns to high level when transmitter in channle4 have send datas more than reg_rmt_tx_lim_ch4 after detecting this interrupt software can updata the old datas with new datas."]
    #[inline(always)]
    pub fn ch4_tx_thr_event_int_raw(&self) -> CH4_TX_THR_EVENT_INT_RAW_R {
        CH4_TX_THR_EVENT_INT_RAW_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - The interrupt raw bit for channel 5 turns to high level when transmitter in channle5 have send datas more than reg_rmt_tx_lim_ch5 after detecting this interrupt software can updata the old datas with new datas."]
    #[inline(always)]
    pub fn ch5_tx_thr_event_int_raw(&self) -> CH5_TX_THR_EVENT_INT_RAW_R {
        CH5_TX_THR_EVENT_INT_RAW_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - The interrupt raw bit for channel 6 turns to high level when transmitter in channle6 have send datas more than reg_rmt_tx_lim_ch6 after detecting this interrupt software can updata the old datas with new datas."]
    #[inline(always)]
    pub fn ch6_tx_thr_event_int_raw(&self) -> CH6_TX_THR_EVENT_INT_RAW_R {
        CH6_TX_THR_EVENT_INT_RAW_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - The interrupt raw bit for channel7 turns to high level when transmitter in channle 7 have send datas more than reg_rmt_tx_lim_ch7 after detecting this interrupt software can updata the old datas with new datas."]
    #[inline(always)]
    pub fn ch7_tx_thr_event_int_raw(&self) -> CH7_TX_THR_EVENT_INT_RAW_R {
        CH7_TX_THR_EVENT_INT_RAW_R::new(((self.bits >> 31) & 0x01) != 0)
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
