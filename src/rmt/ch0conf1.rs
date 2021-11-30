#[doc = "Register `CH0CONF1` reader"]
pub struct R(crate::R<CH0CONF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH0CONF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH0CONF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH0CONF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH0CONF1` writer"]
pub struct W(crate::W<CH0CONF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH0CONF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CH0CONF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH0CONF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_START_CH0` reader - Set this bit to start sending data for channel0."]
pub struct TX_START_CH0_R(crate::FieldReader<bool, bool>);
impl TX_START_CH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_START_CH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_START_CH0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_START_CH0` writer - Set this bit to start sending data for channel0."]
pub struct TX_START_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_START_CH0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `RX_EN_CH0` reader - Set this bit to enbale receving data for channel0."]
pub struct RX_EN_CH0_R(crate::FieldReader<bool, bool>);
impl RX_EN_CH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_EN_CH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_EN_CH0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_EN_CH0` writer - Set this bit to enbale receving data for channel0."]
pub struct RX_EN_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_EN_CH0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `MEM_WR_RST_CH0` reader - Set this bit to reset write ram address for channel0 by receiver access."]
pub struct MEM_WR_RST_CH0_R(crate::FieldReader<bool, bool>);
impl MEM_WR_RST_CH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEM_WR_RST_CH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM_WR_RST_CH0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM_WR_RST_CH0` writer - Set this bit to reset write ram address for channel0 by receiver access."]
pub struct MEM_WR_RST_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_WR_RST_CH0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `MEM_RD_RST_CH0` reader - Set this bit to reset read ram address for channel0 by transmitter access."]
pub struct MEM_RD_RST_CH0_R(crate::FieldReader<bool, bool>);
impl MEM_RD_RST_CH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEM_RD_RST_CH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM_RD_RST_CH0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM_RD_RST_CH0` writer - Set this bit to reset read ram address for channel0 by transmitter access."]
pub struct MEM_RD_RST_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_RD_RST_CH0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `APB_MEM_RST_CH0` reader - Set this bit to reset W/R ram address for channel0 by apb fifo access"]
pub struct APB_MEM_RST_CH0_R(crate::FieldReader<bool, bool>);
impl APB_MEM_RST_CH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APB_MEM_RST_CH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB_MEM_RST_CH0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APB_MEM_RST_CH0` writer - Set this bit to reset W/R ram address for channel0 by apb fifo access"]
pub struct APB_MEM_RST_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_MEM_RST_CH0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `MEM_OWNER_CH0` reader - This is the mark of channel0's ram usage right.1'b1：receiver uses the ram 0：transmitter uses the ram"]
pub struct MEM_OWNER_CH0_R(crate::FieldReader<bool, bool>);
impl MEM_OWNER_CH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEM_OWNER_CH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM_OWNER_CH0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM_OWNER_CH0` writer - This is the mark of channel0's ram usage right.1'b1：receiver uses the ram 0：transmitter uses the ram"]
pub struct MEM_OWNER_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_OWNER_CH0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `TX_CONTI_MODE_CH0` reader - Set this bit to continue sending from the first data to the last data in channel0 again and again."]
pub struct TX_CONTI_MODE_CH0_R(crate::FieldReader<bool, bool>);
impl TX_CONTI_MODE_CH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_CONTI_MODE_CH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_CONTI_MODE_CH0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_CONTI_MODE_CH0` writer - Set this bit to continue sending from the first data to the last data in channel0 again and again."]
pub struct TX_CONTI_MODE_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_CONTI_MODE_CH0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `RX_FILTER_EN_CH0` reader - This is the receive filter enable bit for channel0."]
pub struct RX_FILTER_EN_CH0_R(crate::FieldReader<bool, bool>);
impl RX_FILTER_EN_CH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_FILTER_EN_CH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_FILTER_EN_CH0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_FILTER_EN_CH0` writer - This is the receive filter enable bit for channel0."]
pub struct RX_FILTER_EN_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FILTER_EN_CH0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `RX_FILTER_THRES_CH0` reader - in receive mode channel0 ignore input pulse when the pulse width is smaller then this value."]
pub struct RX_FILTER_THRES_CH0_R(crate::FieldReader<u8, u8>);
impl RX_FILTER_THRES_CH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RX_FILTER_THRES_CH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_FILTER_THRES_CH0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_FILTER_THRES_CH0` writer - in receive mode channel0 ignore input pulse when the pulse width is smaller then this value."]
pub struct RX_FILTER_THRES_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FILTER_THRES_CH0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `REF_CNT_RST_CH0` reader - This bit is used to reset divider in channel0."]
pub struct REF_CNT_RST_CH0_R(crate::FieldReader<bool, bool>);
impl REF_CNT_RST_CH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REF_CNT_RST_CH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REF_CNT_RST_CH0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REF_CNT_RST_CH0` writer - This bit is used to reset divider in channel0."]
pub struct REF_CNT_RST_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> REF_CNT_RST_CH0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `REF_ALWAYS_ON_CH0` reader - This bit is used to select base clock. 1'b1:clk_apb 1'b0:clk_ref"]
pub struct REF_ALWAYS_ON_CH0_R(crate::FieldReader<bool, bool>);
impl REF_ALWAYS_ON_CH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REF_ALWAYS_ON_CH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REF_ALWAYS_ON_CH0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REF_ALWAYS_ON_CH0` writer - This bit is used to select base clock. 1'b1:clk_apb 1'b0:clk_ref"]
pub struct REF_ALWAYS_ON_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> REF_ALWAYS_ON_CH0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `IDLE_OUT_LV_CH0` reader - This bit configures the output signal's level for channel0 in IDLE state."]
pub struct IDLE_OUT_LV_CH0_R(crate::FieldReader<bool, bool>);
impl IDLE_OUT_LV_CH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IDLE_OUT_LV_CH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDLE_OUT_LV_CH0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDLE_OUT_LV_CH0` writer - This bit configures the output signal's level for channel0 in IDLE state."]
pub struct IDLE_OUT_LV_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLE_OUT_LV_CH0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `IDLE_OUT_EN_CH0` reader - This is the output enable control bit for channel0 in IDLE state."]
pub struct IDLE_OUT_EN_CH0_R(crate::FieldReader<bool, bool>);
impl IDLE_OUT_EN_CH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IDLE_OUT_EN_CH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDLE_OUT_EN_CH0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDLE_OUT_EN_CH0` writer - This is the output enable control bit for channel0 in IDLE state."]
pub struct IDLE_OUT_EN_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLE_OUT_EN_CH0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Set this bit to start sending data for channel0."]
    #[inline(always)]
    pub fn tx_start_ch0(&self) -> TX_START_CH0_R {
        TX_START_CH0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Set this bit to enbale receving data for channel0."]
    #[inline(always)]
    pub fn rx_en_ch0(&self) -> RX_EN_CH0_R {
        RX_EN_CH0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Set this bit to reset write ram address for channel0 by receiver access."]
    #[inline(always)]
    pub fn mem_wr_rst_ch0(&self) -> MEM_WR_RST_CH0_R {
        MEM_WR_RST_CH0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Set this bit to reset read ram address for channel0 by transmitter access."]
    #[inline(always)]
    pub fn mem_rd_rst_ch0(&self) -> MEM_RD_RST_CH0_R {
        MEM_RD_RST_CH0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Set this bit to reset W/R ram address for channel0 by apb fifo access"]
    #[inline(always)]
    pub fn apb_mem_rst_ch0(&self) -> APB_MEM_RST_CH0_R {
        APB_MEM_RST_CH0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - This is the mark of channel0's ram usage right.1'b1：receiver uses the ram 0：transmitter uses the ram"]
    #[inline(always)]
    pub fn mem_owner_ch0(&self) -> MEM_OWNER_CH0_R {
        MEM_OWNER_CH0_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Set this bit to continue sending from the first data to the last data in channel0 again and again."]
    #[inline(always)]
    pub fn tx_conti_mode_ch0(&self) -> TX_CONTI_MODE_CH0_R {
        TX_CONTI_MODE_CH0_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - This is the receive filter enable bit for channel0."]
    #[inline(always)]
    pub fn rx_filter_en_ch0(&self) -> RX_FILTER_EN_CH0_R {
        RX_FILTER_EN_CH0_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - in receive mode channel0 ignore input pulse when the pulse width is smaller then this value."]
    #[inline(always)]
    pub fn rx_filter_thres_ch0(&self) -> RX_FILTER_THRES_CH0_R {
        RX_FILTER_THRES_CH0_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - This bit is used to reset divider in channel0."]
    #[inline(always)]
    pub fn ref_cnt_rst_ch0(&self) -> REF_CNT_RST_CH0_R {
        REF_CNT_RST_CH0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - This bit is used to select base clock. 1'b1:clk_apb 1'b0:clk_ref"]
    #[inline(always)]
    pub fn ref_always_on_ch0(&self) -> REF_ALWAYS_ON_CH0_R {
        REF_ALWAYS_ON_CH0_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - This bit configures the output signal's level for channel0 in IDLE state."]
    #[inline(always)]
    pub fn idle_out_lv_ch0(&self) -> IDLE_OUT_LV_CH0_R {
        IDLE_OUT_LV_CH0_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - This is the output enable control bit for channel0 in IDLE state."]
    #[inline(always)]
    pub fn idle_out_en_ch0(&self) -> IDLE_OUT_EN_CH0_R {
        IDLE_OUT_EN_CH0_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to start sending data for channel0."]
    #[inline(always)]
    pub fn tx_start_ch0(&mut self) -> TX_START_CH0_W {
        TX_START_CH0_W { w: self }
    }
    #[doc = "Bit 1 - Set this bit to enbale receving data for channel0."]
    #[inline(always)]
    pub fn rx_en_ch0(&mut self) -> RX_EN_CH0_W {
        RX_EN_CH0_W { w: self }
    }
    #[doc = "Bit 2 - Set this bit to reset write ram address for channel0 by receiver access."]
    #[inline(always)]
    pub fn mem_wr_rst_ch0(&mut self) -> MEM_WR_RST_CH0_W {
        MEM_WR_RST_CH0_W { w: self }
    }
    #[doc = "Bit 3 - Set this bit to reset read ram address for channel0 by transmitter access."]
    #[inline(always)]
    pub fn mem_rd_rst_ch0(&mut self) -> MEM_RD_RST_CH0_W {
        MEM_RD_RST_CH0_W { w: self }
    }
    #[doc = "Bit 4 - Set this bit to reset W/R ram address for channel0 by apb fifo access"]
    #[inline(always)]
    pub fn apb_mem_rst_ch0(&mut self) -> APB_MEM_RST_CH0_W {
        APB_MEM_RST_CH0_W { w: self }
    }
    #[doc = "Bit 5 - This is the mark of channel0's ram usage right.1'b1：receiver uses the ram 0：transmitter uses the ram"]
    #[inline(always)]
    pub fn mem_owner_ch0(&mut self) -> MEM_OWNER_CH0_W {
        MEM_OWNER_CH0_W { w: self }
    }
    #[doc = "Bit 6 - Set this bit to continue sending from the first data to the last data in channel0 again and again."]
    #[inline(always)]
    pub fn tx_conti_mode_ch0(&mut self) -> TX_CONTI_MODE_CH0_W {
        TX_CONTI_MODE_CH0_W { w: self }
    }
    #[doc = "Bit 7 - This is the receive filter enable bit for channel0."]
    #[inline(always)]
    pub fn rx_filter_en_ch0(&mut self) -> RX_FILTER_EN_CH0_W {
        RX_FILTER_EN_CH0_W { w: self }
    }
    #[doc = "Bits 8:15 - in receive mode channel0 ignore input pulse when the pulse width is smaller then this value."]
    #[inline(always)]
    pub fn rx_filter_thres_ch0(&mut self) -> RX_FILTER_THRES_CH0_W {
        RX_FILTER_THRES_CH0_W { w: self }
    }
    #[doc = "Bit 16 - This bit is used to reset divider in channel0."]
    #[inline(always)]
    pub fn ref_cnt_rst_ch0(&mut self) -> REF_CNT_RST_CH0_W {
        REF_CNT_RST_CH0_W { w: self }
    }
    #[doc = "Bit 17 - This bit is used to select base clock. 1'b1:clk_apb 1'b0:clk_ref"]
    #[inline(always)]
    pub fn ref_always_on_ch0(&mut self) -> REF_ALWAYS_ON_CH0_W {
        REF_ALWAYS_ON_CH0_W { w: self }
    }
    #[doc = "Bit 18 - This bit configures the output signal's level for channel0 in IDLE state."]
    #[inline(always)]
    pub fn idle_out_lv_ch0(&mut self) -> IDLE_OUT_LV_CH0_W {
        IDLE_OUT_LV_CH0_W { w: self }
    }
    #[doc = "Bit 19 - This is the output enable control bit for channel0 in IDLE state."]
    #[inline(always)]
    pub fn idle_out_en_ch0(&mut self) -> IDLE_OUT_EN_CH0_W {
        IDLE_OUT_EN_CH0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0conf1](index.html) module"]
pub struct CH0CONF1_SPEC;
impl crate::RegisterSpec for CH0CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch0conf1::R](R) reader structure"]
impl crate::Readable for CH0CONF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch0conf1::W](W) writer structure"]
impl crate::Writable for CH0CONF1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CH0CONF1 to value 0x0f20"]
impl crate::Resettable for CH0CONF1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f20
    }
}
