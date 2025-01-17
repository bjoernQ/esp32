#[doc = "Register `CMDARG` reader"]
pub struct R(crate::R<CMDARG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMDARG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMDARG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMDARG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMDARG` writer"]
pub struct W(crate::W<CMDARG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMDARG_SPEC>;
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
impl From<crate::W<CMDARG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMDARG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMDARG` reader - Value indicates command argument to be passed to the card."]
pub struct CMDARG_R(crate::FieldReader<u32, u32>);
impl CMDARG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CMDARG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMDARG_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMDARG` writer - Value indicates command argument to be passed to the card."]
pub struct CMDARG_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDARG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Value indicates command argument to be passed to the card."]
    #[inline(always)]
    pub fn cmdarg(&self) -> CMDARG_R {
        CMDARG_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Value indicates command argument to be passed to the card."]
    #[inline(always)]
    pub fn cmdarg(&mut self) -> CMDARG_W {
        CMDARG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Command argument data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmdarg](index.html) module"]
pub struct CMDARG_SPEC;
impl crate::RegisterSpec for CMDARG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmdarg::R](R) reader structure"]
impl crate::Readable for CMDARG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmdarg::W](W) writer structure"]
impl crate::Writable for CMDARG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMDARG to value 0"]
impl crate::Resettable for CMDARG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
