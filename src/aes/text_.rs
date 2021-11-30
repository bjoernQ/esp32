#[doc = "Register `TEXT_%s` reader"]
pub struct R(crate::R<TEXT__SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TEXT__SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TEXT__SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TEXT__SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TEXT_%s` writer"]
pub struct W(crate::W<TEXT__SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TEXT__SPEC>;
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
impl From<crate::W<TEXT__SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TEXT__SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TEXT` reader - Plaintext and ciphertext register."]
pub struct TEXT_R(crate::FieldReader<u8, u8>);
impl TEXT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TEXT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEXT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEXT` writer - Plaintext and ciphertext register."]
pub struct TEXT_W<'a> {
    w: &'a mut W,
}
impl<'a> TEXT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Plaintext and ciphertext register."]
    #[inline(always)]
    pub fn text(&self) -> TEXT_R {
        TEXT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Plaintext and ciphertext register."]
    #[inline(always)]
    pub fn text(&mut self) -> TEXT_W {
        TEXT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [text_](index.html) module"]
pub struct TEXT__SPEC;
impl crate::RegisterSpec for TEXT__SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [text_::R](R) reader structure"]
impl crate::Readable for TEXT__SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [text_::W](W) writer structure"]
impl crate::Writable for TEXT__SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TEXT_%s to value 0"]
impl crate::Resettable for TEXT__SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
