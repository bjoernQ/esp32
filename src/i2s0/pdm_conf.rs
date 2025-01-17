#[doc = "Register `PDM_CONF` reader"]
pub struct R(crate::R<PDM_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDM_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDM_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDM_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDM_CONF` writer"]
pub struct W(crate::W<PDM_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDM_CONF_SPEC>;
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
impl From<crate::W<PDM_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDM_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_PDM_EN` reader - "]
pub struct TX_PDM_EN_R(crate::FieldReader<bool, bool>);
impl TX_PDM_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_PDM_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_PDM_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_PDM_EN` writer - "]
pub struct TX_PDM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_PDM_EN_W<'a> {
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
#[doc = "Field `RX_PDM_EN` reader - "]
pub struct RX_PDM_EN_R(crate::FieldReader<bool, bool>);
impl RX_PDM_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_PDM_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_PDM_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_PDM_EN` writer - "]
pub struct RX_PDM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_PDM_EN_W<'a> {
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
#[doc = "Field `PCM2PDM_CONV_EN` reader - "]
pub struct PCM2PDM_CONV_EN_R(crate::FieldReader<bool, bool>);
impl PCM2PDM_CONV_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PCM2PDM_CONV_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCM2PDM_CONV_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCM2PDM_CONV_EN` writer - "]
pub struct PCM2PDM_CONV_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PCM2PDM_CONV_EN_W<'a> {
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
#[doc = "Field `PDM2PCM_CONV_EN` reader - "]
pub struct PDM2PCM_CONV_EN_R(crate::FieldReader<bool, bool>);
impl PDM2PCM_CONV_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDM2PCM_CONV_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDM2PCM_CONV_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDM2PCM_CONV_EN` writer - "]
pub struct PDM2PCM_CONV_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PDM2PCM_CONV_EN_W<'a> {
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
#[doc = "Field `TX_PDM_SINC_OSR2` reader - "]
pub struct TX_PDM_SINC_OSR2_R(crate::FieldReader<u8, u8>);
impl TX_PDM_SINC_OSR2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TX_PDM_SINC_OSR2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_PDM_SINC_OSR2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_PDM_SINC_OSR2` writer - "]
pub struct TX_PDM_SINC_OSR2_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_PDM_SINC_OSR2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `TX_PDM_PRESCALE` reader - "]
pub struct TX_PDM_PRESCALE_R(crate::FieldReader<u8, u8>);
impl TX_PDM_PRESCALE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TX_PDM_PRESCALE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_PDM_PRESCALE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_PDM_PRESCALE` writer - "]
pub struct TX_PDM_PRESCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_PDM_PRESCALE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `TX_PDM_HP_IN_SHIFT` reader - "]
pub struct TX_PDM_HP_IN_SHIFT_R(crate::FieldReader<u8, u8>);
impl TX_PDM_HP_IN_SHIFT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TX_PDM_HP_IN_SHIFT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_PDM_HP_IN_SHIFT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_PDM_HP_IN_SHIFT` writer - "]
pub struct TX_PDM_HP_IN_SHIFT_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_PDM_HP_IN_SHIFT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `TX_PDM_LP_IN_SHIFT` reader - "]
pub struct TX_PDM_LP_IN_SHIFT_R(crate::FieldReader<u8, u8>);
impl TX_PDM_LP_IN_SHIFT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TX_PDM_LP_IN_SHIFT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_PDM_LP_IN_SHIFT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_PDM_LP_IN_SHIFT` writer - "]
pub struct TX_PDM_LP_IN_SHIFT_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_PDM_LP_IN_SHIFT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
#[doc = "Field `TX_PDM_SINC_IN_SHIFT` reader - "]
pub struct TX_PDM_SINC_IN_SHIFT_R(crate::FieldReader<u8, u8>);
impl TX_PDM_SINC_IN_SHIFT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TX_PDM_SINC_IN_SHIFT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_PDM_SINC_IN_SHIFT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_PDM_SINC_IN_SHIFT` writer - "]
pub struct TX_PDM_SINC_IN_SHIFT_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_PDM_SINC_IN_SHIFT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Field `TX_PDM_SIGMADELTA_IN_SHIFT` reader - "]
pub struct TX_PDM_SIGMADELTA_IN_SHIFT_R(crate::FieldReader<u8, u8>);
impl TX_PDM_SIGMADELTA_IN_SHIFT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TX_PDM_SIGMADELTA_IN_SHIFT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_PDM_SIGMADELTA_IN_SHIFT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_PDM_SIGMADELTA_IN_SHIFT` writer - "]
pub struct TX_PDM_SIGMADELTA_IN_SHIFT_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_PDM_SIGMADELTA_IN_SHIFT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
#[doc = "Field `RX_PDM_SINC_DSR_16_EN` reader - "]
pub struct RX_PDM_SINC_DSR_16_EN_R(crate::FieldReader<bool, bool>);
impl RX_PDM_SINC_DSR_16_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_PDM_SINC_DSR_16_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_PDM_SINC_DSR_16_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_PDM_SINC_DSR_16_EN` writer - "]
pub struct RX_PDM_SINC_DSR_16_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_PDM_SINC_DSR_16_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `TX_PDM_HP_BYPASS` reader - "]
pub struct TX_PDM_HP_BYPASS_R(crate::FieldReader<bool, bool>);
impl TX_PDM_HP_BYPASS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_PDM_HP_BYPASS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_PDM_HP_BYPASS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_PDM_HP_BYPASS` writer - "]
pub struct TX_PDM_HP_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_PDM_HP_BYPASS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tx_pdm_en(&self) -> TX_PDM_EN_R {
        TX_PDM_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rx_pdm_en(&self) -> RX_PDM_EN_R {
        RX_PDM_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pcm2pdm_conv_en(&self) -> PCM2PDM_CONV_EN_R {
        PCM2PDM_CONV_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pdm2pcm_conv_en(&self) -> PDM2PCM_CONV_EN_R {
        PDM2PCM_CONV_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn tx_pdm_sinc_osr2(&self) -> TX_PDM_SINC_OSR2_R {
        TX_PDM_SINC_OSR2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn tx_pdm_prescale(&self) -> TX_PDM_PRESCALE_R {
        TX_PDM_PRESCALE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn tx_pdm_hp_in_shift(&self) -> TX_PDM_HP_IN_SHIFT_R {
        TX_PDM_HP_IN_SHIFT_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn tx_pdm_lp_in_shift(&self) -> TX_PDM_LP_IN_SHIFT_R {
        TX_PDM_LP_IN_SHIFT_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn tx_pdm_sinc_in_shift(&self) -> TX_PDM_SINC_IN_SHIFT_R {
        TX_PDM_SINC_IN_SHIFT_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn tx_pdm_sigmadelta_in_shift(&self) -> TX_PDM_SIGMADELTA_IN_SHIFT_R {
        TX_PDM_SIGMADELTA_IN_SHIFT_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn rx_pdm_sinc_dsr_16_en(&self) -> RX_PDM_SINC_DSR_16_EN_R {
        RX_PDM_SINC_DSR_16_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn tx_pdm_hp_bypass(&self) -> TX_PDM_HP_BYPASS_R {
        TX_PDM_HP_BYPASS_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tx_pdm_en(&mut self) -> TX_PDM_EN_W {
        TX_PDM_EN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rx_pdm_en(&mut self) -> RX_PDM_EN_W {
        RX_PDM_EN_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pcm2pdm_conv_en(&mut self) -> PCM2PDM_CONV_EN_W {
        PCM2PDM_CONV_EN_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pdm2pcm_conv_en(&mut self) -> PDM2PCM_CONV_EN_W {
        PDM2PCM_CONV_EN_W { w: self }
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn tx_pdm_sinc_osr2(&mut self) -> TX_PDM_SINC_OSR2_W {
        TX_PDM_SINC_OSR2_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn tx_pdm_prescale(&mut self) -> TX_PDM_PRESCALE_W {
        TX_PDM_PRESCALE_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn tx_pdm_hp_in_shift(&mut self) -> TX_PDM_HP_IN_SHIFT_W {
        TX_PDM_HP_IN_SHIFT_W { w: self }
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn tx_pdm_lp_in_shift(&mut self) -> TX_PDM_LP_IN_SHIFT_W {
        TX_PDM_LP_IN_SHIFT_W { w: self }
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn tx_pdm_sinc_in_shift(&mut self) -> TX_PDM_SINC_IN_SHIFT_W {
        TX_PDM_SINC_IN_SHIFT_W { w: self }
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn tx_pdm_sigmadelta_in_shift(&mut self) -> TX_PDM_SIGMADELTA_IN_SHIFT_W {
        TX_PDM_SIGMADELTA_IN_SHIFT_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn rx_pdm_sinc_dsr_16_en(&mut self) -> RX_PDM_SINC_DSR_16_EN_W {
        RX_PDM_SINC_DSR_16_EN_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn tx_pdm_hp_bypass(&mut self) -> TX_PDM_HP_BYPASS_W {
        TX_PDM_HP_BYPASS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdm_conf](index.html) module"]
pub struct PDM_CONF_SPEC;
impl crate::RegisterSpec for PDM_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdm_conf::R](R) reader structure"]
impl crate::Readable for PDM_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdm_conf::W](W) writer structure"]
impl crate::Writable for PDM_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDM_CONF to value 0x0155_0020"]
impl crate::Resettable for PDM_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0155_0020
    }
}
