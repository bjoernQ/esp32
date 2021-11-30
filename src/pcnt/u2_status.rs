#[doc = "Register `U2_STATUS` reader"]
pub struct R(crate::R<U2_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<U2_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<U2_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<U2_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CORE_STATUS_U2` reader - "]
pub struct CORE_STATUS_U2_R(crate::FieldReader<u32, u32>);
impl CORE_STATUS_U2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CORE_STATUS_U2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_STATUS_U2_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn core_status_u2(&self) -> CORE_STATUS_U2_R {
        CORE_STATUS_U2_R::new(self.bits as u32)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u2_status](index.html) module"]
pub struct U2_STATUS_SPEC;
impl crate::RegisterSpec for U2_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [u2_status::R](R) reader structure"]
impl crate::Readable for U2_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets U2_STATUS to value 0"]
impl crate::Resettable for U2_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
