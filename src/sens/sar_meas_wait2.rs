#[doc = "Register `SAR_MEAS_WAIT2` reader"]
pub struct R(crate::R<SAR_MEAS_WAIT2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_MEAS_WAIT2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_MEAS_WAIT2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_MEAS_WAIT2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_MEAS_WAIT2` writer"]
pub struct W(crate::W<SAR_MEAS_WAIT2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_MEAS_WAIT2_SPEC>;
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
impl From<crate::W<SAR_MEAS_WAIT2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_MEAS_WAIT2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FORCE_XPD_SAR_SW` reader - "]
pub struct FORCE_XPD_SAR_SW_R(crate::FieldReader<bool, bool>);
impl FORCE_XPD_SAR_SW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FORCE_XPD_SAR_SW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FORCE_XPD_SAR_SW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FORCE_XPD_SAR_SW` writer - "]
pub struct FORCE_XPD_SAR_SW_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_XPD_SAR_SW_W<'a> {
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
#[doc = "Field `SAR_AMP_WAIT3` reader - "]
pub struct SAR_AMP_WAIT3_R(crate::FieldReader<u16, u16>);
impl SAR_AMP_WAIT3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        SAR_AMP_WAIT3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR_AMP_WAIT3_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR_AMP_WAIT3` writer - "]
pub struct SAR_AMP_WAIT3_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR_AMP_WAIT3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `FORCE_XPD_AMP` reader - "]
pub struct FORCE_XPD_AMP_R(crate::FieldReader<u8, u8>);
impl FORCE_XPD_AMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FORCE_XPD_AMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FORCE_XPD_AMP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FORCE_XPD_AMP` writer - "]
pub struct FORCE_XPD_AMP_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_XPD_AMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `FORCE_XPD_SAR` reader - "]
pub struct FORCE_XPD_SAR_R(crate::FieldReader<u8, u8>);
impl FORCE_XPD_SAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FORCE_XPD_SAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FORCE_XPD_SAR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FORCE_XPD_SAR` writer - "]
pub struct FORCE_XPD_SAR_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_XPD_SAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
#[doc = "Field `SAR2_RSTB_WAIT` reader - "]
pub struct SAR2_RSTB_WAIT_R(crate::FieldReader<u8, u8>);
impl SAR2_RSTB_WAIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SAR2_RSTB_WAIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR2_RSTB_WAIT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR2_RSTB_WAIT` writer - "]
pub struct SAR2_RSTB_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR2_RSTB_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 20)) | ((value as u32 & 0xff) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn force_xpd_sar_sw(&self) -> FORCE_XPD_SAR_SW_R {
        FORCE_XPD_SAR_SW_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn sar_amp_wait3(&self) -> SAR_AMP_WAIT3_R {
        SAR_AMP_WAIT3_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn force_xpd_amp(&self) -> FORCE_XPD_AMP_R {
        FORCE_XPD_AMP_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn force_xpd_sar(&self) -> FORCE_XPD_SAR_R {
        FORCE_XPD_SAR_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:27"]
    #[inline(always)]
    pub fn sar2_rstb_wait(&self) -> SAR2_RSTB_WAIT_R {
        SAR2_RSTB_WAIT_R::new(((self.bits >> 20) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn force_xpd_sar_sw(&mut self) -> FORCE_XPD_SAR_SW_W {
        FORCE_XPD_SAR_SW_W { w: self }
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn sar_amp_wait3(&mut self) -> SAR_AMP_WAIT3_W {
        SAR_AMP_WAIT3_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn force_xpd_amp(&mut self) -> FORCE_XPD_AMP_W {
        FORCE_XPD_AMP_W { w: self }
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn force_xpd_sar(&mut self) -> FORCE_XPD_SAR_W {
        FORCE_XPD_SAR_W { w: self }
    }
    #[doc = "Bits 20:27"]
    #[inline(always)]
    pub fn sar2_rstb_wait(&mut self) -> SAR2_RSTB_WAIT_W {
        SAR2_RSTB_WAIT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_meas_wait2](index.html) module"]
pub struct SAR_MEAS_WAIT2_SPEC;
impl crate::RegisterSpec for SAR_MEAS_WAIT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_meas_wait2::R](R) reader structure"]
impl crate::Readable for SAR_MEAS_WAIT2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_meas_wait2::W](W) writer structure"]
impl crate::Writable for SAR_MEAS_WAIT2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAR_MEAS_WAIT2 to value 0x0020_000a"]
impl crate::Resettable for SAR_MEAS_WAIT2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0020_000a
    }
}
