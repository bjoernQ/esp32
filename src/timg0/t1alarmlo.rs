#[doc = "Register `T1ALARMLO` reader"]
pub struct R(crate::R<T1ALARMLO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<T1ALARMLO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<T1ALARMLO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<T1ALARMLO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `T1ALARMLO` writer"]
pub struct W(crate::W<T1ALARMLO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<T1ALARMLO_SPEC>;
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
impl From<crate::W<T1ALARMLO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<T1ALARMLO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T1_ALARM_LO` reader - Timer 1 time-base counter value lower 32 bits that will trigger the alarm"]
pub struct T1_ALARM_LO_R(crate::FieldReader<u32, u32>);
impl T1_ALARM_LO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        T1_ALARM_LO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T1_ALARM_LO_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T1_ALARM_LO` writer - Timer 1 time-base counter value lower 32 bits that will trigger the alarm"]
pub struct T1_ALARM_LO_W<'a> {
    w: &'a mut W,
}
impl<'a> T1_ALARM_LO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Timer 1 time-base counter value lower 32 bits that will trigger the alarm"]
    #[inline(always)]
    pub fn t1_alarm_lo(&self) -> T1_ALARM_LO_R {
        T1_ALARM_LO_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Timer 1 time-base counter value lower 32 bits that will trigger the alarm"]
    #[inline(always)]
    pub fn t1_alarm_lo(&mut self) -> T1_ALARM_LO_W {
        T1_ALARM_LO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t1alarmlo](index.html) module"]
pub struct T1ALARMLO_SPEC;
impl crate::RegisterSpec for T1ALARMLO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [t1alarmlo::R](R) reader structure"]
impl crate::Readable for T1ALARMLO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [t1alarmlo::W](W) writer structure"]
impl crate::Writable for T1ALARMLO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets T1ALARMLO to value 0"]
impl crate::Resettable for T1ALARMLO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
