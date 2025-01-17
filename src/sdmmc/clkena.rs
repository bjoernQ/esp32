#[doc = "Register `CLKENA` reader"]
pub struct R(crate::R<CLKENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKENA` writer"]
pub struct W(crate::W<CLKENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKENA_SPEC>;
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
impl From<crate::W<CLKENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCLK_ENABLE` reader - Clock-enable control for two SD card clocks and one MMC card clock is supported. One bit per card. 0: Clock disabled; 1: Clock enabled."]
pub struct CCLK_ENABLE_R(crate::FieldReader<u8, u8>);
impl CCLK_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CCLK_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCLK_ENABLE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCLK_ENABLE` writer - Clock-enable control for two SD card clocks and one MMC card clock is supported. One bit per card. 0: Clock disabled; 1: Clock enabled."]
pub struct CCLK_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCLK_ENABLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `LP_ENABLE` reader - Disable clock when the card is in IDLE state. One bit per card. 0: clock disabled; 1: clock enabled."]
pub struct LP_ENABLE_R(crate::FieldReader<u8, u8>);
impl LP_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LP_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LP_ENABLE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LP_ENABLE` writer - Disable clock when the card is in IDLE state. One bit per card. 0: clock disabled; 1: clock enabled."]
pub struct LP_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> LP_ENABLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Clock-enable control for two SD card clocks and one MMC card clock is supported. One bit per card. 0: Clock disabled; 1: Clock enabled."]
    #[inline(always)]
    pub fn cclk_enable(&self) -> CCLK_ENABLE_R {
        CCLK_ENABLE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Disable clock when the card is in IDLE state. One bit per card. 0: clock disabled; 1: clock enabled."]
    #[inline(always)]
    pub fn lp_enable(&self) -> LP_ENABLE_R {
        LP_ENABLE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Clock-enable control for two SD card clocks and one MMC card clock is supported. One bit per card. 0: Clock disabled; 1: Clock enabled."]
    #[inline(always)]
    pub fn cclk_enable(&mut self) -> CCLK_ENABLE_W {
        CCLK_ENABLE_W { w: self }
    }
    #[doc = "Bits 16:17 - Disable clock when the card is in IDLE state. One bit per card. 0: clock disabled; 1: clock enabled."]
    #[inline(always)]
    pub fn lp_enable(&mut self) -> LP_ENABLE_W {
        LP_ENABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkena](index.html) module"]
pub struct CLKENA_SPEC;
impl crate::RegisterSpec for CLKENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkena::R](R) reader structure"]
impl crate::Readable for CLKENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkena::W](W) writer structure"]
impl crate::Writable for CLKENA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLKENA to value 0"]
impl crate::Resettable for CLKENA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
