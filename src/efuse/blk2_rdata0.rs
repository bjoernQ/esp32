#[doc = "Register `BLK2_RDATA0` reader"]
pub struct R(crate::R<BLK2_RDATA0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLK2_RDATA0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLK2_RDATA0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLK2_RDATA0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BLK2_DOUT0` reader - read for BLOCK2"]
pub struct BLK2_DOUT0_R(crate::FieldReader<u32, u32>);
impl BLK2_DOUT0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        BLK2_DOUT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLK2_DOUT0_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - read for BLOCK2"]
    #[inline(always)]
    pub fn blk2_dout0(&self) -> BLK2_DOUT0_R {
        BLK2_DOUT0_R::new(self.bits as u32)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk2_rdata0](index.html) module"]
pub struct BLK2_RDATA0_SPEC;
impl crate::RegisterSpec for BLK2_RDATA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blk2_rdata0::R](R) reader structure"]
impl crate::Readable for BLK2_RDATA0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BLK2_RDATA0 to value 0"]
impl crate::Resettable for BLK2_RDATA0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
