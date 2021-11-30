#[doc = "Register `DMA_OUT_PUSH` reader"]
pub struct R(crate::R<DMA_OUT_PUSH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_OUT_PUSH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_OUT_PUSH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_OUT_PUSH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_OUT_PUSH` writer"]
pub struct W(crate::W<DMA_OUT_PUSH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_OUT_PUSH_SPEC>;
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
impl From<crate::W<DMA_OUT_PUSH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_OUT_PUSH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUTFIFO_WDATA` reader - This is the data need to be pushed into out link descriptor's fifo."]
pub struct OUTFIFO_WDATA_R(crate::FieldReader<u16, u16>);
impl OUTFIFO_WDATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        OUTFIFO_WDATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTFIFO_WDATA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTFIFO_WDATA` writer - This is the data need to be pushed into out link descriptor's fifo."]
pub struct OUTFIFO_WDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTFIFO_WDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | (value as u32 & 0x01ff);
        self.w
    }
}
#[doc = "Field `OUTFIFO_PUSH` reader - Set this bit to push data in out link descriptor's fifo."]
pub struct OUTFIFO_PUSH_R(crate::FieldReader<bool, bool>);
impl OUTFIFO_PUSH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUTFIFO_PUSH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTFIFO_PUSH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTFIFO_PUSH` writer - Set this bit to push data in out link descriptor's fifo."]
pub struct OUTFIFO_PUSH_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTFIFO_PUSH_W<'a> {
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
    #[doc = "Bits 0:8 - This is the data need to be pushed into out link descriptor's fifo."]
    #[inline(always)]
    pub fn outfifo_wdata(&self) -> OUTFIFO_WDATA_R {
        OUTFIFO_WDATA_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 16 - Set this bit to push data in out link descriptor's fifo."]
    #[inline(always)]
    pub fn outfifo_push(&self) -> OUTFIFO_PUSH_R {
        OUTFIFO_PUSH_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - This is the data need to be pushed into out link descriptor's fifo."]
    #[inline(always)]
    pub fn outfifo_wdata(&mut self) -> OUTFIFO_WDATA_W {
        OUTFIFO_WDATA_W { w: self }
    }
    #[doc = "Bit 16 - Set this bit to push data in out link descriptor's fifo."]
    #[inline(always)]
    pub fn outfifo_push(&mut self) -> OUTFIFO_PUSH_W {
        OUTFIFO_PUSH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_out_push](index.html) module"]
pub struct DMA_OUT_PUSH_SPEC;
impl crate::RegisterSpec for DMA_OUT_PUSH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_out_push::R](R) reader structure"]
impl crate::Readable for DMA_OUT_PUSH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_out_push::W](W) writer structure"]
impl crate::Writable for DMA_OUT_PUSH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_OUT_PUSH to value 0"]
impl crate::Resettable for DMA_OUT_PUSH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
