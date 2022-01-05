#[doc = "Register `_1_STATE1` reader"]
pub struct R(crate::R<_1_STATE1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<_1_STATE1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<_1_STATE1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<_1_STATE1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SLC1_STATE1` reader - "]
pub struct SLC1_STATE1_R(crate::FieldReader<u32, u32>);
impl SLC1_STATE1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        SLC1_STATE1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC1_STATE1_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn slc1_state1(&self) -> SLC1_STATE1_R {
        SLC1_STATE1_R::new(self.bits as u32)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_1_state1](index.html) module"]
pub struct _1_STATE1_SPEC;
impl crate::RegisterSpec for _1_STATE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [_1_state1::R](R) reader structure"]
impl crate::Readable for _1_STATE1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets _1_STATE1 to value 0"]
impl crate::Resettable for _1_STATE1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}