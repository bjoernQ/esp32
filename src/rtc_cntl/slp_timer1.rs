#[doc = "Register `SLP_TIMER1` reader"]
pub struct R(crate::R<SLP_TIMER1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLP_TIMER1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLP_TIMER1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLP_TIMER1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLP_TIMER1` writer"]
pub struct W(crate::W<SLP_TIMER1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLP_TIMER1_SPEC>;
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
impl From<crate::W<SLP_TIMER1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLP_TIMER1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLP_VAL_HI` reader - RTC sleep timer high 16 bits"]
pub struct SLP_VAL_HI_R(crate::FieldReader<u16, u16>);
impl SLP_VAL_HI_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        SLP_VAL_HI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLP_VAL_HI_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLP_VAL_HI` writer - RTC sleep timer high 16 bits"]
pub struct SLP_VAL_HI_W<'a> {
    w: &'a mut W,
}
impl<'a> SLP_VAL_HI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `MAIN_TIMER_ALARM_EN` reader - timer alarm enable bit"]
pub struct MAIN_TIMER_ALARM_EN_R(crate::FieldReader<bool, bool>);
impl MAIN_TIMER_ALARM_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MAIN_TIMER_ALARM_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAIN_TIMER_ALARM_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAIN_TIMER_ALARM_EN` writer - timer alarm enable bit"]
pub struct MAIN_TIMER_ALARM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> MAIN_TIMER_ALARM_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - RTC sleep timer high 16 bits"]
    #[inline(always)]
    pub fn slp_val_hi(&self) -> SLP_VAL_HI_R {
        SLP_VAL_HI_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - timer alarm enable bit"]
    #[inline(always)]
    pub fn main_timer_alarm_en(&self) -> MAIN_TIMER_ALARM_EN_R {
        MAIN_TIMER_ALARM_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - RTC sleep timer high 16 bits"]
    #[inline(always)]
    pub fn slp_val_hi(&mut self) -> SLP_VAL_HI_W {
        SLP_VAL_HI_W { w: self }
    }
    #[doc = "Bit 16 - timer alarm enable bit"]
    #[inline(always)]
    pub fn main_timer_alarm_en(&mut self) -> MAIN_TIMER_ALARM_EN_W {
        MAIN_TIMER_ALARM_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slp_timer1](index.html) module"]
pub struct SLP_TIMER1_SPEC;
impl crate::RegisterSpec for SLP_TIMER1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slp_timer1::R](R) reader structure"]
impl crate::Readable for SLP_TIMER1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slp_timer1::W](W) writer structure"]
impl crate::Writable for SLP_TIMER1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SLP_TIMER1 to value 0"]
impl crate::Resettable for SLP_TIMER1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
