#[doc = "Register `CACHE_MUX_MODE` reader"]
pub struct R(crate::R<CACHE_MUX_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACHE_MUX_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACHE_MUX_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACHE_MUX_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CACHE_MUX_MODE` writer"]
pub struct W(crate::W<CACHE_MUX_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CACHE_MUX_MODE_SPEC>;
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
impl From<crate::W<CACHE_MUX_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CACHE_MUX_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CACHE_MUX_MODE` reader - "]
pub struct CACHE_MUX_MODE_R(crate::FieldReader<u8, u8>);
impl CACHE_MUX_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CACHE_MUX_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CACHE_MUX_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CACHE_MUX_MODE` writer - "]
pub struct CACHE_MUX_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHE_MUX_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn cache_mux_mode(&self) -> CACHE_MUX_MODE_R {
        CACHE_MUX_MODE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn cache_mux_mode(&mut self) -> CACHE_MUX_MODE_W {
        CACHE_MUX_MODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cache_mux_mode](index.html) module"]
pub struct CACHE_MUX_MODE_SPEC;
impl crate::RegisterSpec for CACHE_MUX_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cache_mux_mode::R](R) reader structure"]
impl crate::Readable for CACHE_MUX_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cache_mux_mode::W](W) writer structure"]
impl crate::Writable for CACHE_MUX_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CACHE_MUX_MODE to value 0"]
impl crate::Resettable for CACHE_MUX_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
