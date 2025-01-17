#[doc = "Register `HSCH2_DUTY` reader"]
pub struct R(crate::R<HSCH2_DUTY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSCH2_DUTY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSCH2_DUTY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSCH2_DUTY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DUTY_HSCH2` reader - This register represents the current duty of the output signal for high speed channel2."]
pub struct DUTY_HSCH2_R(crate::FieldReader<u32, u32>);
impl DUTY_HSCH2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        DUTY_HSCH2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_HSCH2_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:24 - This register represents the current duty of the output signal for high speed channel2."]
    #[inline(always)]
    pub fn duty_hsch2(&self) -> DUTY_HSCH2_R {
        DUTY_HSCH2_R::new((self.bits & 0x01ff_ffff) as u32)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsch2_duty](index.html) module"]
pub struct HSCH2_DUTY_SPEC;
impl crate::RegisterSpec for HSCH2_DUTY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hsch2_duty::R](R) reader structure"]
impl crate::Readable for HSCH2_DUTY_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HSCH2_DUTY to value 0"]
impl crate::Resettable for HSCH2_DUTY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
