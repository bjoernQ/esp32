#[doc = "Register `RX_HEAD` reader"]
pub struct R(crate::R<RX_HEAD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_HEAD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_HEAD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_HEAD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RX_HEAD` reader - This register stores the packet header received by DMA"]
pub struct RX_HEAD_R(crate::FieldReader<u32, u32>);
impl RX_HEAD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        RX_HEAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_HEAD_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - This register stores the packet header received by DMA"]
    #[inline(always)]
    pub fn rx_head(&self) -> RX_HEAD_R {
        RX_HEAD_R::new(self.bits as u32)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_head](index.html) module"]
pub struct RX_HEAD_SPEC;
impl crate::RegisterSpec for RX_HEAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_head::R](R) reader structure"]
impl crate::Readable for RX_HEAD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RX_HEAD to value 0"]
impl crate::Resettable for RX_HEAD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
