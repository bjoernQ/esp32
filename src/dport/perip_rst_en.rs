#[doc = "Register `PERIP_RST_EN` reader"]
pub struct R(crate::R<PERIP_RST_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERIP_RST_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERIP_RST_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERIP_RST_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERIP_RST_EN` writer"]
pub struct W(crate::W<PERIP_RST_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERIP_RST_EN_SPEC>;
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
impl From<crate::W<PERIP_RST_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERIP_RST_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERIP_RST` reader - "]
pub struct PERIP_RST_R(crate::FieldReader<u32, u32>);
impl PERIP_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PERIP_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PERIP_RST_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PERIP_RST` writer - "]
pub struct PERIP_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> PERIP_RST_W<'a> {
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
    pub fn perip_rst(&self) -> PERIP_RST_R {
        PERIP_RST_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn perip_rst(&mut self) -> PERIP_RST_W {
        PERIP_RST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perip_rst_en](index.html) module"]
pub struct PERIP_RST_EN_SPEC;
impl crate::RegisterSpec for PERIP_RST_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [perip_rst_en::R](R) reader structure"]
impl crate::Readable for PERIP_RST_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [perip_rst_en::W](W) writer structure"]
impl crate::Writable for PERIP_RST_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PERIP_RST_EN to value 0"]
impl crate::Resettable for PERIP_RST_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
