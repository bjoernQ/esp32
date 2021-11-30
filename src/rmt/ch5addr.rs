#[doc = "Register `CH5ADDR` reader"]
pub struct R(crate::R<CH5ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH5ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH5ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH5ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `APB_MEM_ADDR_CH5` reader - The ram relative address in channel5 by apb fifo access"]
pub struct APB_MEM_ADDR_CH5_R(crate::FieldReader<u32, u32>);
impl APB_MEM_ADDR_CH5_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        APB_MEM_ADDR_CH5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB_MEM_ADDR_CH5_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - The ram relative address in channel5 by apb fifo access"]
    #[inline(always)]
    pub fn apb_mem_addr_ch5(&self) -> APB_MEM_ADDR_CH5_R {
        APB_MEM_ADDR_CH5_R::new(self.bits as u32)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch5addr](index.html) module"]
pub struct CH5ADDR_SPEC;
impl crate::RegisterSpec for CH5ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch5addr::R](R) reader structure"]
impl crate::Readable for CH5ADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CH5ADDR to value 0"]
impl crate::Resettable for CH5ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
