#[doc = "Register `BLK0_WDATA4` reader"]
pub struct R(crate::R<BLK0_WDATA4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLK0_WDATA4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLK0_WDATA4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLK0_WDATA4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLK0_WDATA4` writer"]
pub struct W(crate::W<BLK0_WDATA4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLK0_WDATA4_SPEC>;
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
impl From<crate::W<BLK0_WDATA4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLK0_WDATA4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CK8M_FREQ` reader - "]
pub struct CK8M_FREQ_R(crate::FieldReader<u8, u8>);
impl CK8M_FREQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CK8M_FREQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CK8M_FREQ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CK8M_FREQ` writer - "]
pub struct CK8M_FREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> CK8M_FREQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `ADC_VREF` reader - True ADC reference voltage"]
pub struct ADC_VREF_R(crate::FieldReader<u8, u8>);
impl ADC_VREF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ADC_VREF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_VREF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_VREF` writer - True ADC reference voltage"]
pub struct ADC_VREF_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_VREF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | ((value as u32 & 0x1f) << 8);
        self.w
    }
}
#[doc = "Field `SDIO_DREFH` reader - "]
pub struct SDIO_DREFH_R(crate::FieldReader<u8, u8>);
impl SDIO_DREFH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SDIO_DREFH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIO_DREFH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIO_DREFH` writer - "]
pub struct SDIO_DREFH_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_DREFH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `SDIO_DREFM` reader - "]
pub struct SDIO_DREFM_R(crate::FieldReader<u8, u8>);
impl SDIO_DREFM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SDIO_DREFM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIO_DREFM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIO_DREFM` writer - "]
pub struct SDIO_DREFM_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_DREFM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Field `SDIO_DREFL` reader - "]
pub struct SDIO_DREFL_R(crate::FieldReader<u8, u8>);
impl SDIO_DREFL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SDIO_DREFL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIO_DREFL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIO_DREFL` writer - "]
pub struct SDIO_DREFL_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_DREFL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `XPD_SDIO` reader - program for XPD_SDIO_REG"]
pub struct XPD_SDIO_R(crate::FieldReader<bool, bool>);
impl XPD_SDIO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XPD_SDIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XPD_SDIO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XPD_SDIO` writer - program for XPD_SDIO_REG"]
pub struct XPD_SDIO_W<'a> {
    w: &'a mut W,
}
impl<'a> XPD_SDIO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `SDIO_TIEH` reader - program for SDIO_TIEH"]
pub struct SDIO_TIEH_R(crate::FieldReader<bool, bool>);
impl SDIO_TIEH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDIO_TIEH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIO_TIEH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIO_TIEH` writer - program for SDIO_TIEH"]
pub struct SDIO_TIEH_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_TIEH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `SDIO_FORCE` reader - program for sdio_force"]
pub struct SDIO_FORCE_R(crate::FieldReader<bool, bool>);
impl SDIO_FORCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDIO_FORCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIO_FORCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIO_FORCE` writer - program for sdio_force"]
pub struct SDIO_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_FORCE_W<'a> {
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
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn ck8m_freq(&self) -> CK8M_FREQ_R {
        CK8M_FREQ_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:12 - True ADC reference voltage"]
    #[inline(always)]
    pub fn adc_vref(&self) -> ADC_VREF_R {
        ADC_VREF_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn sdio_drefh(&self) -> SDIO_DREFH_R {
        SDIO_DREFH_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn sdio_drefm(&self) -> SDIO_DREFM_R {
        SDIO_DREFM_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn sdio_drefl(&self) -> SDIO_DREFL_R {
        SDIO_DREFL_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 14 - program for XPD_SDIO_REG"]
    #[inline(always)]
    pub fn xpd_sdio(&self) -> XPD_SDIO_R {
        XPD_SDIO_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - program for SDIO_TIEH"]
    #[inline(always)]
    pub fn sdio_tieh(&self) -> SDIO_TIEH_R {
        SDIO_TIEH_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - program for sdio_force"]
    #[inline(always)]
    pub fn sdio_force(&self) -> SDIO_FORCE_R {
        SDIO_FORCE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn ck8m_freq(&mut self) -> CK8M_FREQ_W {
        CK8M_FREQ_W { w: self }
    }
    #[doc = "Bits 8:12 - True ADC reference voltage"]
    #[inline(always)]
    pub fn adc_vref(&mut self) -> ADC_VREF_W {
        ADC_VREF_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn sdio_drefh(&mut self) -> SDIO_DREFH_W {
        SDIO_DREFH_W { w: self }
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn sdio_drefm(&mut self) -> SDIO_DREFM_W {
        SDIO_DREFM_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn sdio_drefl(&mut self) -> SDIO_DREFL_W {
        SDIO_DREFL_W { w: self }
    }
    #[doc = "Bit 14 - program for XPD_SDIO_REG"]
    #[inline(always)]
    pub fn xpd_sdio(&mut self) -> XPD_SDIO_W {
        XPD_SDIO_W { w: self }
    }
    #[doc = "Bit 15 - program for SDIO_TIEH"]
    #[inline(always)]
    pub fn sdio_tieh(&mut self) -> SDIO_TIEH_W {
        SDIO_TIEH_W { w: self }
    }
    #[doc = "Bit 16 - program for sdio_force"]
    #[inline(always)]
    pub fn sdio_force(&mut self) -> SDIO_FORCE_W {
        SDIO_FORCE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk0_wdata4](index.html) module"]
pub struct BLK0_WDATA4_SPEC;
impl crate::RegisterSpec for BLK0_WDATA4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blk0_wdata4::R](R) reader structure"]
impl crate::Readable for BLK0_WDATA4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [blk0_wdata4::W](W) writer structure"]
impl crate::Writable for BLK0_WDATA4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BLK0_WDATA4 to value 0"]
impl crate::Resettable for BLK0_WDATA4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
