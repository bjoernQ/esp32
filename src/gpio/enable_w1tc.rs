#[doc = "Register `ENABLE_W1TC` reader"]
pub struct R(crate::R<ENABLE_W1TC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENABLE_W1TC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ENABLE_W1TC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ENABLE_W1TC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ENABLE_W1TC` writer"]
pub struct W(crate::W<ENABLE_W1TC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ENABLE_W1TC_SPEC>;
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
impl From<crate::W<ENABLE_W1TC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ENABLE_W1TC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE_DATA_W1TC` reader - GPIO0~31 output enable write 1 to clear"]
pub struct ENABLE_DATA_W1TC_R(crate::FieldReader<u32, u32>);
impl ENABLE_DATA_W1TC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        ENABLE_DATA_W1TC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLE_DATA_W1TC_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLE_DATA_W1TC` writer - GPIO0~31 output enable write 1 to clear"]
pub struct ENABLE_DATA_W1TC_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_DATA_W1TC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - GPIO0~31 output enable write 1 to clear"]
    #[inline(always)]
    pub fn enable_data_w1tc(&self) -> ENABLE_DATA_W1TC_R {
        ENABLE_DATA_W1TC_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPIO0~31 output enable write 1 to clear"]
    #[inline(always)]
    pub fn enable_data_w1tc(&mut self) -> ENABLE_DATA_W1TC_W {
        ENABLE_DATA_W1TC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enable_w1tc](index.html) module"]
pub struct ENABLE_W1TC_SPEC;
impl crate::RegisterSpec for ENABLE_W1TC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [enable_w1tc::R](R) reader structure"]
impl crate::Readable for ENABLE_W1TC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [enable_w1tc::W](W) writer structure"]
impl crate::Writable for ENABLE_W1TC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ENABLE_W1TC to value 0"]
impl crate::Resettable for ENABLE_W1TC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
