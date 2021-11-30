#[doc = "Register `LACTLOADLO` reader"]
pub struct R(crate::R<LACTLOADLO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LACTLOADLO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LACTLOADLO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LACTLOADLO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LACTLOADLO` writer"]
pub struct W(crate::W<LACTLOADLO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LACTLOADLO_SPEC>;
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
impl From<crate::W<LACTLOADLO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LACTLOADLO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LACT_LOAD_LO` reader - "]
pub struct LACT_LOAD_LO_R(crate::FieldReader<u32, u32>);
impl LACT_LOAD_LO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        LACT_LOAD_LO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LACT_LOAD_LO_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LACT_LOAD_LO` writer - "]
pub struct LACT_LOAD_LO_W<'a> {
    w: &'a mut W,
}
impl<'a> LACT_LOAD_LO_W<'a> {
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
    pub fn lact_load_lo(&self) -> LACT_LOAD_LO_R {
        LACT_LOAD_LO_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn lact_load_lo(&mut self) -> LACT_LOAD_LO_W {
        LACT_LOAD_LO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lactloadlo](index.html) module"]
pub struct LACTLOADLO_SPEC;
impl crate::RegisterSpec for LACTLOADLO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lactloadlo::R](R) reader structure"]
impl crate::Readable for LACTLOADLO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lactloadlo::W](W) writer structure"]
impl crate::Writable for LACTLOADLO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LACTLOADLO to value 0"]
impl crate::Resettable for LACTLOADLO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
