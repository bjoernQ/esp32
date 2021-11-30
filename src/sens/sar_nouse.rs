#[doc = "Register `SAR_NOUSE` reader"]
pub struct R(crate::R<SAR_NOUSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_NOUSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_NOUSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_NOUSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_NOUSE` writer"]
pub struct W(crate::W<SAR_NOUSE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_NOUSE_SPEC>;
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
impl From<crate::W<SAR_NOUSE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_NOUSE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAR_NOUSE` reader - "]
pub struct SAR_NOUSE_R(crate::FieldReader<u32, u32>);
impl SAR_NOUSE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        SAR_NOUSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR_NOUSE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR_NOUSE` writer - "]
pub struct SAR_NOUSE_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR_NOUSE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sar_nouse(&self) -> SAR_NOUSE_R {
        SAR_NOUSE_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sar_nouse(&mut self) -> SAR_NOUSE_W {
        SAR_NOUSE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_nouse](index.html) module"]
pub struct SAR_NOUSE_SPEC;
impl crate::RegisterSpec for SAR_NOUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_nouse::R](R) reader structure"]
impl crate::Readable for SAR_NOUSE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_nouse::W](W) writer structure"]
impl crate::Writable for SAR_NOUSE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAR_NOUSE to value 0"]
impl crate::Resettable for SAR_NOUSE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
