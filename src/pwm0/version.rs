#[doc = "Register `VERSION` reader"]
pub struct R(crate::R<VERSION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VERSION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VERSION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VERSION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VERSION` writer"]
pub struct W(crate::W<VERSION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VERSION_SPEC>;
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
impl From<crate::W<VERSION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VERSION_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATE` reader - "]
pub struct DATE_R(crate::FieldReader<u32, u32>);
impl DATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        DATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATE` writer - "]
pub struct DATE_W<'a> {
    w: &'a mut W,
}
impl<'a> DATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff_ffff) | (value as u32 & 0x0fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:27"]
    #[inline(always)]
    pub fn date(&self) -> DATE_R {
        DATE_R::new((self.bits & 0x0fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:27"]
    #[inline(always)]
    pub fn date(&mut self) -> DATE_W {
        DATE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [version](index.html) module"]
pub struct VERSION_SPEC;
impl crate::RegisterSpec for VERSION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [version::R](R) reader structure"]
impl crate::Readable for VERSION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [version::W](W) writer structure"]
impl crate::Writable for VERSION_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VERSION to value 0x0210_7230"]
impl crate::Resettable for VERSION_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0210_7230
    }
}
