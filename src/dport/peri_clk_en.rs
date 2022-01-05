#[doc = "Register `PERI_CLK_EN` reader"]
pub struct R(crate::R<PERI_CLK_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERI_CLK_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERI_CLK_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERI_CLK_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERI_CLK_EN` writer"]
pub struct W(crate::W<PERI_CLK_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERI_CLK_EN_SPEC>;
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
impl From<crate::W<PERI_CLK_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERI_CLK_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERI_CLK_EN` reader - "]
pub struct PERI_CLK_EN_R(crate::FieldReader<u32, u32>);
impl PERI_CLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PERI_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PERI_CLK_EN_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PERI_CLK_EN` writer - "]
pub struct PERI_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PERI_CLK_EN_W<'a> {
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
    pub fn peri_clk_en(&self) -> PERI_CLK_EN_R {
        PERI_CLK_EN_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn peri_clk_en(&mut self) -> PERI_CLK_EN_W {
        PERI_CLK_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peri_clk_en](index.html) module"]
pub struct PERI_CLK_EN_SPEC;
impl crate::RegisterSpec for PERI_CLK_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [peri_clk_en::R](R) reader structure"]
impl crate::Readable for PERI_CLK_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [peri_clk_en::W](W) writer structure"]
impl crate::Writable for PERI_CLK_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PERI_CLK_EN to value 0"]
impl crate::Resettable for PERI_CLK_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}