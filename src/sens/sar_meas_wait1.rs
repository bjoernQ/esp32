#[doc = "Register `SAR_MEAS_WAIT1` reader"]
pub struct R(crate::R<SAR_MEAS_WAIT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_MEAS_WAIT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_MEAS_WAIT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_MEAS_WAIT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_MEAS_WAIT1` writer"]
pub struct W(crate::W<SAR_MEAS_WAIT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_MEAS_WAIT1_SPEC>;
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
impl From<crate::W<SAR_MEAS_WAIT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_MEAS_WAIT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAR_AMP_WAIT1` reader - "]
pub struct SAR_AMP_WAIT1_R(crate::FieldReader<u16, u16>);
impl SAR_AMP_WAIT1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        SAR_AMP_WAIT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR_AMP_WAIT1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR_AMP_WAIT1` writer - "]
pub struct SAR_AMP_WAIT1_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR_AMP_WAIT1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `SAR_AMP_WAIT2` reader - "]
pub struct SAR_AMP_WAIT2_R(crate::FieldReader<u16, u16>);
impl SAR_AMP_WAIT2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        SAR_AMP_WAIT2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR_AMP_WAIT2_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR_AMP_WAIT2` writer - "]
pub struct SAR_AMP_WAIT2_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR_AMP_WAIT2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn sar_amp_wait1(&self) -> SAR_AMP_WAIT1_R {
        SAR_AMP_WAIT1_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn sar_amp_wait2(&self) -> SAR_AMP_WAIT2_R {
        SAR_AMP_WAIT2_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn sar_amp_wait1(&mut self) -> SAR_AMP_WAIT1_W {
        SAR_AMP_WAIT1_W { w: self }
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn sar_amp_wait2(&mut self) -> SAR_AMP_WAIT2_W {
        SAR_AMP_WAIT2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_meas_wait1](index.html) module"]
pub struct SAR_MEAS_WAIT1_SPEC;
impl crate::RegisterSpec for SAR_MEAS_WAIT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_meas_wait1::R](R) reader structure"]
impl crate::Readable for SAR_MEAS_WAIT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_meas_wait1::W](W) writer structure"]
impl crate::Writable for SAR_MEAS_WAIT1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAR_MEAS_WAIT1 to value 0x000a_000a"]
impl crate::Resettable for SAR_MEAS_WAIT1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x000a_000a
    }
}
