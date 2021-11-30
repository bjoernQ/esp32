#[doc = "Register `APP_CPU_RECORD_PDEBUGSTATUS` reader"]
pub struct R(crate::R<APP_CPU_RECORD_PDEBUGSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APP_CPU_RECORD_PDEBUGSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APP_CPU_RECORD_PDEBUGSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APP_CPU_RECORD_PDEBUGSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RECORD_APP_PDEBUGSTATUS` reader - "]
pub struct RECORD_APP_PDEBUGSTATUS_R(crate::FieldReader<u8, u8>);
impl RECORD_APP_PDEBUGSTATUS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RECORD_APP_PDEBUGSTATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RECORD_APP_PDEBUGSTATUS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn record_app_pdebugstatus(&self) -> RECORD_APP_PDEBUGSTATUS_R {
        RECORD_APP_PDEBUGSTATUS_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [app_cpu_record_pdebugstatus](index.html) module"]
pub struct APP_CPU_RECORD_PDEBUGSTATUS_SPEC;
impl crate::RegisterSpec for APP_CPU_RECORD_PDEBUGSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [app_cpu_record_pdebugstatus::R](R) reader structure"]
impl crate::Readable for APP_CPU_RECORD_PDEBUGSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets APP_CPU_RECORD_PDEBUGSTATUS to value 0"]
impl crate::Resettable for APP_CPU_RECORD_PDEBUGSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
