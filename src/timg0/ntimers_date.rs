#[doc = "Register `NTIMERS_DATE` reader"]
pub struct R(crate::R<NTIMERS_DATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NTIMERS_DATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NTIMERS_DATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NTIMERS_DATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NTIMERS_DATE` writer"]
pub struct W(crate::W<NTIMERS_DATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NTIMERS_DATE_SPEC>;
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
impl From<crate::W<NTIMERS_DATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NTIMERS_DATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NTIMERS_DATE` reader - Version of this regfile"]
pub struct NTIMERS_DATE_R(crate::FieldReader<u32, u32>);
impl NTIMERS_DATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        NTIMERS_DATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NTIMERS_DATE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NTIMERS_DATE` writer - Version of this regfile"]
pub struct NTIMERS_DATE_W<'a> {
    w: &'a mut W,
}
impl<'a> NTIMERS_DATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff_ffff) | (value as u32 & 0x0fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:27 - Version of this regfile"]
    #[inline(always)]
    pub fn ntimers_date(&self) -> NTIMERS_DATE_R {
        NTIMERS_DATE_R::new((self.bits & 0x0fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:27 - Version of this regfile"]
    #[inline(always)]
    pub fn ntimers_date(&mut self) -> NTIMERS_DATE_W {
        NTIMERS_DATE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ntimers_date](index.html) module"]
pub struct NTIMERS_DATE_SPEC;
impl crate::RegisterSpec for NTIMERS_DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ntimers_date::R](R) reader structure"]
impl crate::Readable for NTIMERS_DATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ntimers_date::W](W) writer structure"]
impl crate::Writable for NTIMERS_DATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NTIMERS_DATE to value 0x0160_4290"]
impl crate::Resettable for NTIMERS_DATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0160_4290
    }
}
