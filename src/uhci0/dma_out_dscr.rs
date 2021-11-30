#[doc = "Register `DMA_OUT_DSCR` reader"]
pub struct R(crate::R<DMA_OUT_DSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_OUT_DSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_OUT_DSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_OUT_DSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OUTLINK_DSCR` reader - The content of current out link descriptor's third dword"]
pub struct OUTLINK_DSCR_R(crate::FieldReader<u32, u32>);
impl OUTLINK_DSCR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        OUTLINK_DSCR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTLINK_DSCR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - The content of current out link descriptor's third dword"]
    #[inline(always)]
    pub fn outlink_dscr(&self) -> OUTLINK_DSCR_R {
        OUTLINK_DSCR_R::new(self.bits as u32)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_out_dscr](index.html) module"]
pub struct DMA_OUT_DSCR_SPEC;
impl crate::RegisterSpec for DMA_OUT_DSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_out_dscr::R](R) reader structure"]
impl crate::Readable for DMA_OUT_DSCR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMA_OUT_DSCR to value 0"]
impl crate::Resettable for DMA_OUT_DSCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
