#[doc = "Register `LSCH4_DUTY` reader"]
pub struct R(crate::R<LSCH4_DUTY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LSCH4_DUTY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LSCH4_DUTY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LSCH4_DUTY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DUTY_LSCH4` reader - This register represents the current duty of the output signal for low speed channel4."]
pub struct DUTY_LSCH4_R(crate::FieldReader<u32, u32>);
impl DUTY_LSCH4_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        DUTY_LSCH4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_LSCH4_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:24 - This register represents the current duty of the output signal for low speed channel4."]
    #[inline(always)]
    pub fn duty_lsch4(&self) -> DUTY_LSCH4_R {
        DUTY_LSCH4_R::new((self.bits & 0x01ff_ffff) as u32)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lsch4_duty](index.html) module"]
pub struct LSCH4_DUTY_SPEC;
impl crate::RegisterSpec for LSCH4_DUTY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lsch4_duty::R](R) reader structure"]
impl crate::Readable for LSCH4_DUTY_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LSCH4_DUTY to value 0"]
impl crate::Resettable for LSCH4_DUTY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
