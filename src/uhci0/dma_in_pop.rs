#[doc = "Register `DMA_IN_POP` reader"]
pub struct R(crate::R<DMA_IN_POP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_IN_POP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_IN_POP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_IN_POP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_IN_POP` writer"]
pub struct W(crate::W<DMA_IN_POP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_IN_POP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DMA_IN_POP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_IN_POP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INFIFO_RDATA` reader - This register stores the data pop from in link descriptor's fifo."]
pub struct INFIFO_RDATA_R(crate::FieldReader<u16, u16>);
impl INFIFO_RDATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        INFIFO_RDATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INFIFO_RDATA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INFIFO_POP` reader - Set this bit to pop data in in link descriptor's fifo."]
pub struct INFIFO_POP_R(crate::FieldReader<bool, bool>);
impl INFIFO_POP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INFIFO_POP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INFIFO_POP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INFIFO_POP` writer - Set this bit to pop data in in link descriptor's fifo."]
pub struct INFIFO_POP_W<'a> {
    w: &'a mut W,
}
impl<'a> INFIFO_POP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - This register stores the data pop from in link descriptor's fifo."]
    #[inline(always)]
    pub fn infifo_rdata(&self) -> INFIFO_RDATA_R {
        INFIFO_RDATA_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 16 - Set this bit to pop data in in link descriptor's fifo."]
    #[inline(always)]
    pub fn infifo_pop(&self) -> INFIFO_POP_R {
        INFIFO_POP_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Set this bit to pop data in in link descriptor's fifo."]
    #[inline(always)]
    pub fn infifo_pop(&mut self) -> INFIFO_POP_W {
        INFIFO_POP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_in_pop](index.html) module"]
pub struct DMA_IN_POP_SPEC;
impl crate::RegisterSpec for DMA_IN_POP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_in_pop::R](R) reader structure"]
impl crate::Readable for DMA_IN_POP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_in_pop::W](W) writer structure"]
impl crate::Writable for DMA_IN_POP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_IN_POP to value 0"]
impl crate::Resettable for DMA_IN_POP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
