#[doc = "Register `OUTLINK_DSCR_BF1` reader"]
pub struct R(crate::R<OUTLINK_DSCR_BF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUTLINK_DSCR_BF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUTLINK_DSCR_BF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUTLINK_DSCR_BF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DMA_OUTLINK_DSCR_BF1` reader - The content of current out descriptor data buffer pointer."]
pub struct DMA_OUTLINK_DSCR_BF1_R(crate::FieldReader<u32, u32>);
impl DMA_OUTLINK_DSCR_BF1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        DMA_OUTLINK_DSCR_BF1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_OUTLINK_DSCR_BF1_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - The content of current out descriptor data buffer pointer."]
    #[inline(always)]
    pub fn dma_outlink_dscr_bf1(&self) -> DMA_OUTLINK_DSCR_BF1_R {
        DMA_OUTLINK_DSCR_BF1_R::new(self.bits as u32)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outlink_dscr_bf1](index.html) module"]
pub struct OUTLINK_DSCR_BF1_SPEC;
impl crate::RegisterSpec for OUTLINK_DSCR_BF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [outlink_dscr_bf1::R](R) reader structure"]
impl crate::Readable for OUTLINK_DSCR_BF1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OUTLINK_DSCR_BF1 to value 0"]
impl crate::Resettable for OUTLINK_DSCR_BF1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
