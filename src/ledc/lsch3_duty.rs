#[doc = "Register `LSCH3_DUTY` reader"]
pub struct R(crate::R<LSCH3_DUTY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LSCH3_DUTY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LSCH3_DUTY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LSCH3_DUTY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DUTY_LSCH3` reader - This register represents the current duty of the output signal for low speed channel3."]
pub struct DUTY_LSCH3_R(crate::FieldReader<u32, u32>);
impl DUTY_LSCH3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        DUTY_LSCH3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_LSCH3_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:24 - This register represents the current duty of the output signal for low speed channel3."]
    #[inline(always)]
    pub fn duty_lsch3(&self) -> DUTY_LSCH3_R {
        DUTY_LSCH3_R::new((self.bits & 0x01ff_ffff) as u32)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lsch3_duty](index.html) module"]
pub struct LSCH3_DUTY_SPEC;
impl crate::RegisterSpec for LSCH3_DUTY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lsch3_duty::R](R) reader structure"]
impl crate::Readable for LSCH3_DUTY_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LSCH3_DUTY to value 0"]
impl crate::Resettable for LSCH3_DUTY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
