#[doc = "Register `BBPD_CTRL` reader"]
pub struct R(crate::R<BBPD_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BBPD_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BBPD_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BBPD_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BBPD_CTRL` writer"]
pub struct W(crate::W<BBPD_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BBPD_CTRL_SPEC>;
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
impl From<crate::W<BBPD_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BBPD_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DC_EST_FORCE_PD` reader - "]
pub struct DC_EST_FORCE_PD_R(crate::FieldReader<bool, bool>);
impl DC_EST_FORCE_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DC_EST_FORCE_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DC_EST_FORCE_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DC_EST_FORCE_PD` writer - "]
pub struct DC_EST_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> DC_EST_FORCE_PD_W<'a> {
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
#[doc = "Field `DC_EST_FORCE_PU` reader - "]
pub struct DC_EST_FORCE_PU_R(crate::FieldReader<bool, bool>);
impl DC_EST_FORCE_PU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DC_EST_FORCE_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DC_EST_FORCE_PU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DC_EST_FORCE_PU` writer - "]
pub struct DC_EST_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> DC_EST_FORCE_PU_W<'a> {
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
#[doc = "Field `FFT_FORCE_PD` reader - "]
pub struct FFT_FORCE_PD_R(crate::FieldReader<bool, bool>);
impl FFT_FORCE_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FFT_FORCE_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FFT_FORCE_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FFT_FORCE_PD` writer - "]
pub struct FFT_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> FFT_FORCE_PD_W<'a> {
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
#[doc = "Field `FFT_FORCE_PU` reader - "]
pub struct FFT_FORCE_PU_R(crate::FieldReader<bool, bool>);
impl FFT_FORCE_PU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FFT_FORCE_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FFT_FORCE_PU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FFT_FORCE_PU` writer - "]
pub struct FFT_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> FFT_FORCE_PU_W<'a> {
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
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dc_est_force_pd(&self) -> DC_EST_FORCE_PD_R {
        DC_EST_FORCE_PD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dc_est_force_pu(&self) -> DC_EST_FORCE_PU_R {
        DC_EST_FORCE_PU_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn fft_force_pd(&self) -> FFT_FORCE_PD_R {
        FFT_FORCE_PD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn fft_force_pu(&self) -> FFT_FORCE_PU_R {
        FFT_FORCE_PU_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dc_est_force_pd(&mut self) -> DC_EST_FORCE_PD_W {
        DC_EST_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dc_est_force_pu(&mut self) -> DC_EST_FORCE_PU_W {
        DC_EST_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn fft_force_pd(&mut self) -> FFT_FORCE_PD_W {
        FFT_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn fft_force_pu(&mut self) -> FFT_FORCE_PU_W {
        FFT_FORCE_PU_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Baseband control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bbpd_ctrl](index.html) module"]
pub struct BBPD_CTRL_SPEC;
impl crate::RegisterSpec for BBPD_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bbpd_ctrl::R](R) reader structure"]
impl crate::Readable for BBPD_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bbpd_ctrl::W](W) writer structure"]
impl crate::Writable for BBPD_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BBPD_CTRL to value 0"]
impl crate::Resettable for BBPD_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
