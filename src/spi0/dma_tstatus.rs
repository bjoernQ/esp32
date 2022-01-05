#[doc = "Register `DMA_TSTATUS` reader"]
pub struct R(crate::R<DMA_TSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_TSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_TSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_TSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DMA_IN_STATUS` reader - spi dma write data to memory status."]
pub struct DMA_IN_STATUS_R(crate::FieldReader<u32, u32>);
impl DMA_IN_STATUS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        DMA_IN_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_IN_STATUS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - spi dma write data to memory status."]
    #[inline(always)]
    pub fn dma_in_status(&self) -> DMA_IN_STATUS_R {
        DMA_IN_STATUS_R::new(self.bits as u32)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_tstatus](index.html) module"]
pub struct DMA_TSTATUS_SPEC;
impl crate::RegisterSpec for DMA_TSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_tstatus::R](R) reader structure"]
impl crate::Readable for DMA_TSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMA_TSTATUS to value 0"]
impl crate::Resettable for DMA_TSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}