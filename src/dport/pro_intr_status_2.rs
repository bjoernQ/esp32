#[doc = "Register `PRO_INTR_STATUS_2` reader"]
pub struct R(crate::R<PRO_INTR_STATUS_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_INTR_STATUS_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_INTR_STATUS_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_INTR_STATUS_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PRO_INTR_STATUS_2` reader - "]
pub struct PRO_INTR_STATUS_2_R(crate::FieldReader<u32, u32>);
impl PRO_INTR_STATUS_2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PRO_INTR_STATUS_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_INTR_STATUS_2_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn pro_intr_status_2(&self) -> PRO_INTR_STATUS_2_R {
        PRO_INTR_STATUS_2_R::new(self.bits as u32)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_intr_status_2](index.html) module"]
pub struct PRO_INTR_STATUS_2_SPEC;
impl crate::RegisterSpec for PRO_INTR_STATUS_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_intr_status_2::R](R) reader structure"]
impl crate::Readable for PRO_INTR_STATUS_2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PRO_INTR_STATUS_2 to value 0"]
impl crate::Resettable for PRO_INTR_STATUS_2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
