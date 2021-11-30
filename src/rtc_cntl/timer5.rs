#[doc = "Register `TIMER5` reader"]
pub struct R(crate::R<TIMER5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER5` writer"]
pub struct W(crate::W<TIMER5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER5_SPEC>;
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
impl From<crate::W<TIMER5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ULP_CP_SUBTIMER_PREDIV` reader - "]
pub struct ULP_CP_SUBTIMER_PREDIV_R(crate::FieldReader<u8, u8>);
impl ULP_CP_SUBTIMER_PREDIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ULP_CP_SUBTIMER_PREDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ULP_CP_SUBTIMER_PREDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ULP_CP_SUBTIMER_PREDIV` writer - "]
pub struct ULP_CP_SUBTIMER_PREDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> ULP_CP_SUBTIMER_PREDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `MIN_SLP_VAL` reader - minimal sleep cycles in slow_clk_rtc"]
pub struct MIN_SLP_VAL_R(crate::FieldReader<u8, u8>);
impl MIN_SLP_VAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MIN_SLP_VAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MIN_SLP_VAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MIN_SLP_VAL` writer - minimal sleep cycles in slow_clk_rtc"]
pub struct MIN_SLP_VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> MIN_SLP_VAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `RTCMEM_WAIT_TIMER` reader - "]
pub struct RTCMEM_WAIT_TIMER_R(crate::FieldReader<u16, u16>);
impl RTCMEM_WAIT_TIMER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        RTCMEM_WAIT_TIMER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCMEM_WAIT_TIMER_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCMEM_WAIT_TIMER` writer - "]
pub struct RTCMEM_WAIT_TIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCMEM_WAIT_TIMER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 16)) | ((value as u32 & 0x01ff) << 16);
        self.w
    }
}
#[doc = "Field `RTCMEM_POWERUP_TIMER` reader - "]
pub struct RTCMEM_POWERUP_TIMER_R(crate::FieldReader<u8, u8>);
impl RTCMEM_POWERUP_TIMER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RTCMEM_POWERUP_TIMER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCMEM_POWERUP_TIMER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCMEM_POWERUP_TIMER` writer - "]
pub struct RTCMEM_POWERUP_TIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCMEM_POWERUP_TIMER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 25)) | ((value as u32 & 0x7f) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn ulp_cp_subtimer_prediv(&self) -> ULP_CP_SUBTIMER_PREDIV_R {
        ULP_CP_SUBTIMER_PREDIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - minimal sleep cycles in slow_clk_rtc"]
    #[inline(always)]
    pub fn min_slp_val(&self) -> MIN_SLP_VAL_R {
        MIN_SLP_VAL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:24"]
    #[inline(always)]
    pub fn rtcmem_wait_timer(&self) -> RTCMEM_WAIT_TIMER_R {
        RTCMEM_WAIT_TIMER_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bits 25:31"]
    #[inline(always)]
    pub fn rtcmem_powerup_timer(&self) -> RTCMEM_POWERUP_TIMER_R {
        RTCMEM_POWERUP_TIMER_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn ulp_cp_subtimer_prediv(&mut self) -> ULP_CP_SUBTIMER_PREDIV_W {
        ULP_CP_SUBTIMER_PREDIV_W { w: self }
    }
    #[doc = "Bits 8:15 - minimal sleep cycles in slow_clk_rtc"]
    #[inline(always)]
    pub fn min_slp_val(&mut self) -> MIN_SLP_VAL_W {
        MIN_SLP_VAL_W { w: self }
    }
    #[doc = "Bits 16:24"]
    #[inline(always)]
    pub fn rtcmem_wait_timer(&mut self) -> RTCMEM_WAIT_TIMER_W {
        RTCMEM_WAIT_TIMER_W { w: self }
    }
    #[doc = "Bits 25:31"]
    #[inline(always)]
    pub fn rtcmem_powerup_timer(&mut self) -> RTCMEM_POWERUP_TIMER_W {
        RTCMEM_POWERUP_TIMER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer5](index.html) module"]
pub struct TIMER5_SPEC;
impl crate::RegisterSpec for TIMER5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer5::R](R) reader structure"]
impl crate::Readable for TIMER5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer5::W](W) writer structure"]
impl crate::Writable for TIMER5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMER5 to value 0x1214_8001"]
impl crate::Resettable for TIMER5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1214_8001
    }
}
