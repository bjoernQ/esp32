#[doc = "Register `GEN2_TSTMP_B` reader"]
pub struct R(crate::R<GEN2_TSTMP_B_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GEN2_TSTMP_B_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GEN2_TSTMP_B_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GEN2_TSTMP_B_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GEN2_TSTMP_B` writer"]
pub struct W(crate::W<GEN2_TSTMP_B_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GEN2_TSTMP_B_SPEC>;
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
impl From<crate::W<GEN2_TSTMP_B_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GEN2_TSTMP_B_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GEN2_B` reader - "]
pub struct GEN2_B_R(crate::FieldReader<u16, u16>);
impl GEN2_B_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        GEN2_B_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GEN2_B_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GEN2_B` writer - "]
pub struct GEN2_B_W<'a> {
    w: &'a mut W,
}
impl<'a> GEN2_B_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn gen2_b(&self) -> GEN2_B_R {
        GEN2_B_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn gen2_b(&mut self) -> GEN2_B_W {
        GEN2_B_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gen2_tstmp_b](index.html) module"]
pub struct GEN2_TSTMP_B_SPEC;
impl crate::RegisterSpec for GEN2_TSTMP_B_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gen2_tstmp_b::R](R) reader structure"]
impl crate::Readable for GEN2_TSTMP_B_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gen2_tstmp_b::W](W) writer structure"]
impl crate::Writable for GEN2_TSTMP_B_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GEN2_TSTMP_B to value 0"]
impl crate::Resettable for GEN2_TSTMP_B_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
