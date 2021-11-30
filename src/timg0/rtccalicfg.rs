#[doc = "Register `RTCCALICFG` reader"]
pub struct R(crate::R<RTCCALICFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCCALICFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCCALICFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCCALICFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCCALICFG` writer"]
pub struct W(crate::W<RTCCALICFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCCALICFG_SPEC>;
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
impl From<crate::W<RTCCALICFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCCALICFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_CALI_START_CYCLING` reader - "]
pub struct RTC_CALI_START_CYCLING_R(crate::FieldReader<bool, bool>);
impl RTC_CALI_START_CYCLING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_CALI_START_CYCLING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_CALI_START_CYCLING_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_CALI_START_CYCLING` writer - "]
pub struct RTC_CALI_START_CYCLING_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CALI_START_CYCLING_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `RTC_CALI_CLK_SEL` reader - "]
pub struct RTC_CALI_CLK_SEL_R(crate::FieldReader<u8, u8>);
impl RTC_CALI_CLK_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RTC_CALI_CLK_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_CALI_CLK_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_CALI_CLK_SEL` writer - "]
pub struct RTC_CALI_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CALI_CLK_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | ((value as u32 & 0x03) << 13);
        self.w
    }
}
#[doc = "Field `RTC_CALI_RDY` reader - "]
pub struct RTC_CALI_RDY_R(crate::FieldReader<bool, bool>);
impl RTC_CALI_RDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_CALI_RDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_CALI_RDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_CALI_MAX` reader - "]
pub struct RTC_CALI_MAX_R(crate::FieldReader<u16, u16>);
impl RTC_CALI_MAX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        RTC_CALI_MAX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_CALI_MAX_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_CALI_MAX` writer - "]
pub struct RTC_CALI_MAX_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CALI_MAX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff << 16)) | ((value as u32 & 0x7fff) << 16);
        self.w
    }
}
#[doc = "Field `RTC_CALI_START` reader - "]
pub struct RTC_CALI_START_R(crate::FieldReader<bool, bool>);
impl RTC_CALI_START_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_CALI_START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_CALI_START_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_CALI_START` writer - "]
pub struct RTC_CALI_START_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CALI_START_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rtc_cali_start_cycling(&self) -> RTC_CALI_START_CYCLING_R {
        RTC_CALI_START_CYCLING_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn rtc_cali_clk_sel(&self) -> RTC_CALI_CLK_SEL_R {
        RTC_CALI_CLK_SEL_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn rtc_cali_rdy(&self) -> RTC_CALI_RDY_R {
        RTC_CALI_RDY_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:30"]
    #[inline(always)]
    pub fn rtc_cali_max(&self) -> RTC_CALI_MAX_R {
        RTC_CALI_MAX_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rtc_cali_start(&self) -> RTC_CALI_START_R {
        RTC_CALI_START_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rtc_cali_start_cycling(&mut self) -> RTC_CALI_START_CYCLING_W {
        RTC_CALI_START_CYCLING_W { w: self }
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn rtc_cali_clk_sel(&mut self) -> RTC_CALI_CLK_SEL_W {
        RTC_CALI_CLK_SEL_W { w: self }
    }
    #[doc = "Bits 16:30"]
    #[inline(always)]
    pub fn rtc_cali_max(&mut self) -> RTC_CALI_MAX_W {
        RTC_CALI_MAX_W { w: self }
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rtc_cali_start(&mut self) -> RTC_CALI_START_W {
        RTC_CALI_START_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtccalicfg](index.html) module"]
pub struct RTCCALICFG_SPEC;
impl crate::RegisterSpec for RTCCALICFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtccalicfg::R](R) reader structure"]
impl crate::Readable for RTCCALICFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtccalicfg::W](W) writer structure"]
impl crate::Writable for RTCCALICFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTCCALICFG to value 0x0001_3000"]
impl crate::Resettable for RTCCALICFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0001_3000
    }
}
