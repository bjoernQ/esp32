#[doc = "Register `T1ALARMHI` reader"]
pub struct R(crate::R<T1ALARMHI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<T1ALARMHI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<T1ALARMHI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<T1ALARMHI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `T1ALARMHI` writer"]
pub struct W(crate::W<T1ALARMHI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<T1ALARMHI_SPEC>;
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
impl From<crate::W<T1ALARMHI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<T1ALARMHI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T1_ALARM_HI` reader - Timer 1 time-base counter value higher 32 bits that will trigger the alarm"]
pub struct T1_ALARM_HI_R(crate::FieldReader<u32, u32>);
impl T1_ALARM_HI_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        T1_ALARM_HI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T1_ALARM_HI_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T1_ALARM_HI` writer - Timer 1 time-base counter value higher 32 bits that will trigger the alarm"]
pub struct T1_ALARM_HI_W<'a> {
    w: &'a mut W,
}
impl<'a> T1_ALARM_HI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Timer 1 time-base counter value higher 32 bits that will trigger the alarm"]
    #[inline(always)]
    pub fn t1_alarm_hi(&self) -> T1_ALARM_HI_R {
        T1_ALARM_HI_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Timer 1 time-base counter value higher 32 bits that will trigger the alarm"]
    #[inline(always)]
    pub fn t1_alarm_hi(&mut self) -> T1_ALARM_HI_W {
        T1_ALARM_HI_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t1alarmhi](index.html) module"]
pub struct T1ALARMHI_SPEC;
impl crate::RegisterSpec for T1ALARMHI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [t1alarmhi::R](R) reader structure"]
impl crate::Readable for T1ALARMHI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [t1alarmhi::W](W) writer structure"]
impl crate::Writable for T1ALARMHI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets T1ALARMHI to value 0"]
impl crate::Resettable for T1ALARMHI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
