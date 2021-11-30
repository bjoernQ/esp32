#[doc = "Register `STORE6` reader"]
pub struct R(crate::R<STORE6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STORE6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STORE6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STORE6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STORE6` writer"]
pub struct W(crate::W<STORE6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STORE6_SPEC>;
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
impl From<crate::W<STORE6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STORE6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCRATCH6` reader - 32-bit general purpose retention register"]
pub struct SCRATCH6_R(crate::FieldReader<u32, u32>);
impl SCRATCH6_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        SCRATCH6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCRATCH6_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCRATCH6` writer - 32-bit general purpose retention register"]
pub struct SCRATCH6_W<'a> {
    w: &'a mut W,
}
impl<'a> SCRATCH6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 32-bit general purpose retention register"]
    #[inline(always)]
    pub fn scratch6(&self) -> SCRATCH6_R {
        SCRATCH6_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 32-bit general purpose retention register"]
    #[inline(always)]
    pub fn scratch6(&mut self) -> SCRATCH6_W {
        SCRATCH6_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [store6](index.html) module"]
pub struct STORE6_SPEC;
impl crate::RegisterSpec for STORE6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [store6::R](R) reader structure"]
impl crate::Readable for STORE6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [store6::W](W) writer structure"]
impl crate::Writable for STORE6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STORE6 to value 0"]
impl crate::Resettable for STORE6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
