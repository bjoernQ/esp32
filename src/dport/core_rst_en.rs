#[doc = "Register `CORE_RST_EN` reader"]
pub struct R(crate::R<CORE_RST_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_RST_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_RST_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_RST_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE_RST_EN` writer"]
pub struct W(crate::W<CORE_RST_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_RST_EN_SPEC>;
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
impl From<crate::W<CORE_RST_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_RST_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_RST` reader - "]
pub struct CORE_RST_R(crate::FieldReader<u8, u8>);
impl CORE_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_RST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_RST` writer - "]
pub struct CORE_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_RST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn core_rst(&self) -> CORE_RST_R {
        CORE_RST_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn core_rst(&mut self) -> CORE_RST_W {
        CORE_RST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_rst_en](index.html) module"]
pub struct CORE_RST_EN_SPEC;
impl crate::RegisterSpec for CORE_RST_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_rst_en::R](R) reader structure"]
impl crate::Readable for CORE_RST_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_rst_en::W](W) writer structure"]
impl crate::Writable for CORE_RST_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CORE_RST_EN to value 0"]
impl crate::Resettable for CORE_RST_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
