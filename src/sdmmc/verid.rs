#[doc = "Register `VERID` reader"]
pub struct R(crate::R<VERID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VERID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VERID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VERID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VERSIONID` reader - Hardware version register. Can also be read by fireware."]
pub struct VERSIONID_R(crate::FieldReader<u32, u32>);
impl VERSIONID_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        VERSIONID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VERSIONID_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Hardware version register. Can also be read by fireware."]
    #[inline(always)]
    pub fn versionid(&self) -> VERSIONID_R {
        VERSIONID_R::new(self.bits as u32)
    }
}
#[doc = "Version ID (scratchpad) register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [verid](index.html) module"]
pub struct VERID_SPEC;
impl crate::RegisterSpec for VERID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [verid::R](R) reader structure"]
impl crate::Readable for VERID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets VERID to value 0x5432_270a"]
impl crate::Resettable for VERID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x5432_270a
    }
}
