#[doc = "Register `W5` reader"]
pub struct R(crate::R<W5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<W5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<W5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<W5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `W5` writer"]
pub struct W(crate::W<W5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<W5_SPEC>;
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
impl From<crate::W<W5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<W5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUF5` reader - data buffer"]
pub struct BUF5_R(crate::FieldReader<u32, u32>);
impl BUF5_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        BUF5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUF5_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUF5` writer - data buffer"]
pub struct BUF5_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF5_W<'a> {
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
    pub fn buf5(&self) -> BUF5_R {
        BUF5_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn buf5(&mut self) -> BUF5_W {
        BUF5_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [w5](index.html) module"]
pub struct W5_SPEC;
impl crate::RegisterSpec for W5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [w5::R](R) reader structure"]
impl crate::Readable for W5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [w5::W](W) writer structure"]
impl crate::Writable for W5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets W5 to value 0"]
impl crate::Resettable for W5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
