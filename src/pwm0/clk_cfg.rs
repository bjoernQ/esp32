#[doc = "Register `CLK_CFG` reader"]
pub struct R(crate::R<CLK_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_CFG` writer"]
pub struct W(crate::W<CLK_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_CFG_SPEC>;
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
impl From<crate::W<CLK_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLK_PRESCALE` reader - "]
pub struct CLK_PRESCALE_R(crate::FieldReader<u8, u8>);
impl CLK_PRESCALE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CLK_PRESCALE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_PRESCALE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK_PRESCALE` writer - "]
pub struct CLK_PRESCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_PRESCALE_W<'a> {
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
    pub fn clk_prescale(&self) -> CLK_PRESCALE_R {
        CLK_PRESCALE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn clk_prescale(&mut self) -> CLK_PRESCALE_W {
        CLK_PRESCALE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_cfg](index.html) module"]
pub struct CLK_CFG_SPEC;
impl crate::RegisterSpec for CLK_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_cfg::R](R) reader structure"]
impl crate::Readable for CLK_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_cfg::W](W) writer structure"]
impl crate::Writable for CLK_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_CFG to value 0"]
impl crate::Resettable for CLK_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
