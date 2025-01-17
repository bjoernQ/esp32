#[doc = "Register `TIMER_ALARM` reader"]
pub struct R(crate::R<TIMER_ALARM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER_ALARM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER_ALARM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER_ALARM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER_ALARM` writer"]
pub struct W(crate::W<TIMER_ALARM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER_ALARM_SPEC>;
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
impl From<crate::W<TIMER_ALARM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER_ALARM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER_ALARM` reader - "]
pub struct TIMER_ALARM_R(crate::FieldReader<u8, u8>);
impl TIMER_ALARM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TIMER_ALARM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER_ALARM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER_ALARM` writer - "]
pub struct TIMER_ALARM_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_ALARM_W<'a> {
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
    pub fn timer_alarm(&self) -> TIMER_ALARM_R {
        TIMER_ALARM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn timer_alarm(&mut self) -> TIMER_ALARM_W {
        TIMER_ALARM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer_alarm](index.html) module"]
pub struct TIMER_ALARM_SPEC;
impl crate::RegisterSpec for TIMER_ALARM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer_alarm::R](R) reader structure"]
impl crate::Readable for TIMER_ALARM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer_alarm::W](W) writer structure"]
impl crate::Writable for TIMER_ALARM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMER_ALARM to value 0"]
impl crate::Resettable for TIMER_ALARM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
