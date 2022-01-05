#[doc = "Register `SHA512_BUSY` reader"]
pub struct R(crate::R<SHA512_BUSY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHA512_BUSY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHA512_BUSY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHA512_BUSY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SHA512_BUSY` reader - SHA-512 operation status: 1 if the SHA accelerator is processing data, 0 if it is idle."]
pub struct SHA512_BUSY_R(crate::FieldReader<bool, bool>);
impl SHA512_BUSY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SHA512_BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SHA512_BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - SHA-512 operation status: 1 if the SHA accelerator is processing data, 0 if it is idle."]
    #[inline(always)]
    pub fn sha512_busy(&self) -> SHA512_BUSY_R {
        SHA512_BUSY_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sha512_busy](index.html) module"]
pub struct SHA512_BUSY_SPEC;
impl crate::RegisterSpec for SHA512_BUSY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sha512_busy::R](R) reader structure"]
impl crate::Readable for SHA512_BUSY_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SHA512_BUSY to value 0"]
impl crate::Resettable for SHA512_BUSY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}