#[doc = "Register `MEM_CONF` reader"]
pub struct R(crate::R<MEM_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEM_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEM_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEM_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MEM_CONF` writer"]
pub struct W(crate::W<MEM_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MEM_CONF_SPEC>;
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
impl From<crate::W<MEM_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MEM_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MEM_PD` reader - Set this bit to power down mem.when reg_mem_pd registers in the 3 uarts are all set to 1 mem will enter low power mode."]
pub struct MEM_PD_R(crate::FieldReader<bool, bool>);
impl MEM_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEM_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM_PD` writer - Set this bit to power down mem.when reg_mem_pd registers in the 3 uarts are all set to 1 mem will enter low power mode."]
pub struct MEM_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_PD_W<'a> {
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
#[doc = "Field `RX_SIZE` reader - This register is used to configure the amount of mem allocated to receiver's fifo. the default byte num is 128."]
pub struct RX_SIZE_R(crate::FieldReader<u8, u8>);
impl RX_SIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RX_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_SIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_SIZE` writer - This register is used to configure the amount of mem allocated to receiver's fifo. the default byte num is 128."]
pub struct RX_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 3)) | ((value as u32 & 0x0f) << 3);
        self.w
    }
}
#[doc = "Field `TX_SIZE` reader - This register is used to configure the amount of mem allocated to transmitter's fifo.the default byte num is 128."]
pub struct TX_SIZE_R(crate::FieldReader<u8, u8>);
impl TX_SIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TX_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_SIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_SIZE` writer - This register is used to configure the amount of mem allocated to transmitter's fifo.the default byte num is 128."]
pub struct TX_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 7)) | ((value as u32 & 0x0f) << 7);
        self.w
    }
}
#[doc = "Field `RX_FLOW_THRHD_H3` reader - refer to the rx_flow_thrhd's describtion."]
pub struct RX_FLOW_THRHD_H3_R(crate::FieldReader<u8, u8>);
impl RX_FLOW_THRHD_H3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RX_FLOW_THRHD_H3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_FLOW_THRHD_H3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_FLOW_THRHD_H3` writer - refer to the rx_flow_thrhd's describtion."]
pub struct RX_FLOW_THRHD_H3_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FLOW_THRHD_H3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 15)) | ((value as u32 & 0x07) << 15);
        self.w
    }
}
#[doc = "Field `RX_TOUT_THRHD_H3` reader - refer to the rx_tout_thrhd's describtion."]
pub struct RX_TOUT_THRHD_H3_R(crate::FieldReader<u8, u8>);
impl RX_TOUT_THRHD_H3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RX_TOUT_THRHD_H3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_TOUT_THRHD_H3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_TOUT_THRHD_H3` writer - refer to the rx_tout_thrhd's describtion."]
pub struct RX_TOUT_THRHD_H3_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_TOUT_THRHD_H3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | ((value as u32 & 0x07) << 18);
        self.w
    }
}
#[doc = "Field `XON_THRESHOLD_H2` reader - refer to the uart_xon_threshold's describtion."]
pub struct XON_THRESHOLD_H2_R(crate::FieldReader<u8, u8>);
impl XON_THRESHOLD_H2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        XON_THRESHOLD_H2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XON_THRESHOLD_H2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XON_THRESHOLD_H2` writer - refer to the uart_xon_threshold's describtion."]
pub struct XON_THRESHOLD_H2_W<'a> {
    w: &'a mut W,
}
impl<'a> XON_THRESHOLD_H2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | ((value as u32 & 0x03) << 21);
        self.w
    }
}
#[doc = "Field `XOFF_THRESHOLD_H2` reader - refer to the uart_xoff_threshold's describtion."]
pub struct XOFF_THRESHOLD_H2_R(crate::FieldReader<u8, u8>);
impl XOFF_THRESHOLD_H2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        XOFF_THRESHOLD_H2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XOFF_THRESHOLD_H2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XOFF_THRESHOLD_H2` writer - refer to the uart_xoff_threshold's describtion."]
pub struct XOFF_THRESHOLD_H2_W<'a> {
    w: &'a mut W,
}
impl<'a> XOFF_THRESHOLD_H2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 23)) | ((value as u32 & 0x03) << 23);
        self.w
    }
}
#[doc = "Field `RX_MEM_FULL_THRHD` reader - refer to the rxfifo_full_thrhd's describtion."]
pub struct RX_MEM_FULL_THRHD_R(crate::FieldReader<u8, u8>);
impl RX_MEM_FULL_THRHD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RX_MEM_FULL_THRHD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_MEM_FULL_THRHD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_MEM_FULL_THRHD` writer - refer to the rxfifo_full_thrhd's describtion."]
pub struct RX_MEM_FULL_THRHD_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_MEM_FULL_THRHD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 25)) | ((value as u32 & 0x07) << 25);
        self.w
    }
}
#[doc = "Field `TX_MEM_EMPTY_THRHD` reader - refer to txfifo_empty_thrhd 's describtion."]
pub struct TX_MEM_EMPTY_THRHD_R(crate::FieldReader<u8, u8>);
impl TX_MEM_EMPTY_THRHD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TX_MEM_EMPTY_THRHD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_MEM_EMPTY_THRHD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_MEM_EMPTY_THRHD` writer - refer to txfifo_empty_thrhd 's describtion."]
pub struct TX_MEM_EMPTY_THRHD_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_MEM_EMPTY_THRHD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | ((value as u32 & 0x07) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Set this bit to power down mem.when reg_mem_pd registers in the 3 uarts are all set to 1 mem will enter low power mode."]
    #[inline(always)]
    pub fn mem_pd(&self) -> MEM_PD_R {
        MEM_PD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 3:6 - This register is used to configure the amount of mem allocated to receiver's fifo. the default byte num is 128."]
    #[inline(always)]
    pub fn rx_size(&self) -> RX_SIZE_R {
        RX_SIZE_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bits 7:10 - This register is used to configure the amount of mem allocated to transmitter's fifo.the default byte num is 128."]
    #[inline(always)]
    pub fn tx_size(&self) -> TX_SIZE_R {
        TX_SIZE_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bits 15:17 - refer to the rx_flow_thrhd's describtion."]
    #[inline(always)]
    pub fn rx_flow_thrhd_h3(&self) -> RX_FLOW_THRHD_H3_R {
        RX_FLOW_THRHD_H3_R::new(((self.bits >> 15) & 0x07) as u8)
    }
    #[doc = "Bits 18:20 - refer to the rx_tout_thrhd's describtion."]
    #[inline(always)]
    pub fn rx_tout_thrhd_h3(&self) -> RX_TOUT_THRHD_H3_R {
        RX_TOUT_THRHD_H3_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bits 21:22 - refer to the uart_xon_threshold's describtion."]
    #[inline(always)]
    pub fn xon_threshold_h2(&self) -> XON_THRESHOLD_H2_R {
        XON_THRESHOLD_H2_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bits 23:24 - refer to the uart_xoff_threshold's describtion."]
    #[inline(always)]
    pub fn xoff_threshold_h2(&self) -> XOFF_THRESHOLD_H2_R {
        XOFF_THRESHOLD_H2_R::new(((self.bits >> 23) & 0x03) as u8)
    }
    #[doc = "Bits 25:27 - refer to the rxfifo_full_thrhd's describtion."]
    #[inline(always)]
    pub fn rx_mem_full_thrhd(&self) -> RX_MEM_FULL_THRHD_R {
        RX_MEM_FULL_THRHD_R::new(((self.bits >> 25) & 0x07) as u8)
    }
    #[doc = "Bits 28:30 - refer to txfifo_empty_thrhd 's describtion."]
    #[inline(always)]
    pub fn tx_mem_empty_thrhd(&self) -> TX_MEM_EMPTY_THRHD_R {
        TX_MEM_EMPTY_THRHD_R::new(((self.bits >> 28) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to power down mem.when reg_mem_pd registers in the 3 uarts are all set to 1 mem will enter low power mode."]
    #[inline(always)]
    pub fn mem_pd(&mut self) -> MEM_PD_W {
        MEM_PD_W { w: self }
    }
    #[doc = "Bits 3:6 - This register is used to configure the amount of mem allocated to receiver's fifo. the default byte num is 128."]
    #[inline(always)]
    pub fn rx_size(&mut self) -> RX_SIZE_W {
        RX_SIZE_W { w: self }
    }
    #[doc = "Bits 7:10 - This register is used to configure the amount of mem allocated to transmitter's fifo.the default byte num is 128."]
    #[inline(always)]
    pub fn tx_size(&mut self) -> TX_SIZE_W {
        TX_SIZE_W { w: self }
    }
    #[doc = "Bits 15:17 - refer to the rx_flow_thrhd's describtion."]
    #[inline(always)]
    pub fn rx_flow_thrhd_h3(&mut self) -> RX_FLOW_THRHD_H3_W {
        RX_FLOW_THRHD_H3_W { w: self }
    }
    #[doc = "Bits 18:20 - refer to the rx_tout_thrhd's describtion."]
    #[inline(always)]
    pub fn rx_tout_thrhd_h3(&mut self) -> RX_TOUT_THRHD_H3_W {
        RX_TOUT_THRHD_H3_W { w: self }
    }
    #[doc = "Bits 21:22 - refer to the uart_xon_threshold's describtion."]
    #[inline(always)]
    pub fn xon_threshold_h2(&mut self) -> XON_THRESHOLD_H2_W {
        XON_THRESHOLD_H2_W { w: self }
    }
    #[doc = "Bits 23:24 - refer to the uart_xoff_threshold's describtion."]
    #[inline(always)]
    pub fn xoff_threshold_h2(&mut self) -> XOFF_THRESHOLD_H2_W {
        XOFF_THRESHOLD_H2_W { w: self }
    }
    #[doc = "Bits 25:27 - refer to the rxfifo_full_thrhd's describtion."]
    #[inline(always)]
    pub fn rx_mem_full_thrhd(&mut self) -> RX_MEM_FULL_THRHD_W {
        RX_MEM_FULL_THRHD_W { w: self }
    }
    #[doc = "Bits 28:30 - refer to txfifo_empty_thrhd 's describtion."]
    #[inline(always)]
    pub fn tx_mem_empty_thrhd(&mut self) -> TX_MEM_EMPTY_THRHD_W {
        TX_MEM_EMPTY_THRHD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_conf](index.html) module"]
pub struct MEM_CONF_SPEC;
impl crate::RegisterSpec for MEM_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mem_conf::R](R) reader structure"]
impl crate::Readable for MEM_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mem_conf::W](W) writer structure"]
impl crate::Writable for MEM_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MEM_CONF to value 0x88"]
impl crate::Resettable for MEM_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x88
    }
}
