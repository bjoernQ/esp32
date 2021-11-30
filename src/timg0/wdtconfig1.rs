#[doc = "Register `WDTCONFIG1` reader"]
pub struct R(crate::R<WDTCONFIG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDTCONFIG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDTCONFIG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDTCONFIG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDTCONFIG1` writer"]
pub struct W(crate::W<WDTCONFIG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDTCONFIG1_SPEC>;
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
impl From<crate::W<WDTCONFIG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDTCONFIG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDT_CLK_PRESCALE` reader - SWDT clock prescale value. Period = 12.5ns * value stored in this register"]
pub struct WDT_CLK_PRESCALE_R(crate::FieldReader<u16, u16>);
impl WDT_CLK_PRESCALE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        WDT_CLK_PRESCALE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDT_CLK_PRESCALE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDT_CLK_PRESCALE` writer - SWDT clock prescale value. Period = 12.5ns * value stored in this register"]
pub struct WDT_CLK_PRESCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_CLK_PRESCALE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - SWDT clock prescale value. Period = 12.5ns * value stored in this register"]
    #[inline(always)]
    pub fn wdt_clk_prescale(&self) -> WDT_CLK_PRESCALE_R {
        WDT_CLK_PRESCALE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - SWDT clock prescale value. Period = 12.5ns * value stored in this register"]
    #[inline(always)]
    pub fn wdt_clk_prescale(&mut self) -> WDT_CLK_PRESCALE_W {
        WDT_CLK_PRESCALE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtconfig1](index.html) module"]
pub struct WDTCONFIG1_SPEC;
impl crate::RegisterSpec for WDTCONFIG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdtconfig1::R](R) reader structure"]
impl crate::Readable for WDTCONFIG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdtconfig1::W](W) writer structure"]
impl crate::Writable for WDTCONFIG1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WDTCONFIG1 to value 0x0001_0000"]
impl crate::Resettable for WDTCONFIG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0001_0000
    }
}
