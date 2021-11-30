#[doc = "Register `HSCH6_HPOINT` reader"]
pub struct R(crate::R<HSCH6_HPOINT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSCH6_HPOINT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSCH6_HPOINT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSCH6_HPOINT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSCH6_HPOINT` writer"]
pub struct W(crate::W<HSCH6_HPOINT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSCH6_HPOINT_SPEC>;
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
impl From<crate::W<HSCH6_HPOINT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSCH6_HPOINT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HPOINT_HSCH6` reader - The output value changes to high when htimerx(x=\\[0 3\\]) selected by high speed channel6 has reached reg_hpoint_hsch6\\[19:0\\]"]
pub struct HPOINT_HSCH6_R(crate::FieldReader<u32, u32>);
impl HPOINT_HSCH6_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        HPOINT_HSCH6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HPOINT_HSCH6_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HPOINT_HSCH6` writer - The output value changes to high when htimerx(x=\\[0 3\\]) selected by high speed channel6 has reached reg_hpoint_hsch6\\[19:0\\]"]
pub struct HPOINT_HSCH6_W<'a> {
    w: &'a mut W,
}
impl<'a> HPOINT_HSCH6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | (value as u32 & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:19 - The output value changes to high when htimerx(x=\\[0 3\\]) selected by high speed channel6 has reached reg_hpoint_hsch6\\[19:0\\]"]
    #[inline(always)]
    pub fn hpoint_hsch6(&self) -> HPOINT_HSCH6_R {
        HPOINT_HSCH6_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19 - The output value changes to high when htimerx(x=\\[0 3\\]) selected by high speed channel6 has reached reg_hpoint_hsch6\\[19:0\\]"]
    #[inline(always)]
    pub fn hpoint_hsch6(&mut self) -> HPOINT_HSCH6_W {
        HPOINT_HSCH6_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsch6_hpoint](index.html) module"]
pub struct HSCH6_HPOINT_SPEC;
impl crate::RegisterSpec for HSCH6_HPOINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hsch6_hpoint::R](R) reader structure"]
impl crate::Readable for HSCH6_HPOINT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hsch6_hpoint::W](W) writer structure"]
impl crate::Writable for HSCH6_HPOINT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HSCH6_HPOINT to value 0"]
impl crate::Resettable for HSCH6_HPOINT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
