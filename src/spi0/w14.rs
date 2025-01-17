#[doc = "Register `W14` reader"]
pub struct R(crate::R<W14_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<W14_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<W14_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<W14_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `W14` writer"]
pub struct W(crate::W<W14_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<W14_SPEC>;
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
impl From<crate::W<W14_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<W14_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUF14` reader - data buffer"]
pub struct BUF14_R(crate::FieldReader<u32, u32>);
impl BUF14_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        BUF14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUF14_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUF14` writer - data buffer"]
pub struct BUF14_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn buf14(&self) -> BUF14_R {
        BUF14_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn buf14(&mut self) -> BUF14_W {
        BUF14_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [w14](index.html) module"]
pub struct W14_SPEC;
impl crate::RegisterSpec for W14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [w14::R](R) reader structure"]
impl crate::Readable for W14_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [w14::W](W) writer structure"]
impl crate::Writable for W14_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets W14 to value 0"]
impl crate::Resettable for W14_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
