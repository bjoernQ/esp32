#[doc = "Register `PERIP_CLK_EN` reader"]
pub struct R(crate::R<PERIP_CLK_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERIP_CLK_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERIP_CLK_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERIP_CLK_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERIP_CLK_EN` writer"]
pub struct W(crate::W<PERIP_CLK_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERIP_CLK_EN_SPEC>;
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
impl From<crate::W<PERIP_CLK_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERIP_CLK_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERIP_CLK_EN` reader - "]
pub struct PERIP_CLK_EN_R(crate::FieldReader<u32, u32>);
impl PERIP_CLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PERIP_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PERIP_CLK_EN_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PERIP_CLK_EN` writer - "]
pub struct PERIP_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PERIP_CLK_EN_W<'a> {
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
    pub fn perip_clk_en(&self) -> PERIP_CLK_EN_R {
        PERIP_CLK_EN_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn perip_clk_en(&mut self) -> PERIP_CLK_EN_W {
        PERIP_CLK_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perip_clk_en](index.html) module"]
pub struct PERIP_CLK_EN_SPEC;
impl crate::RegisterSpec for PERIP_CLK_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [perip_clk_en::R](R) reader structure"]
impl crate::Readable for PERIP_CLK_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [perip_clk_en::W](W) writer structure"]
impl crate::Writable for PERIP_CLK_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PERIP_CLK_EN to value 0xf9c1_e06f"]
impl crate::Resettable for PERIP_CLK_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xf9c1_e06f
    }
}
