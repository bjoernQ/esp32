#[doc = "Register `DMA_INT_CLR` reader"]
pub struct R(crate::R<DMA_INT_CLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_INT_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_INT_CLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_INT_CLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_INT_CLR` writer"]
pub struct W(crate::W<DMA_INT_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_INT_CLR_SPEC>;
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
impl From<crate::W<DMA_INT_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_INT_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INLINK_DSCR_EMPTY_INT_CLR` reader - The clear bit for lack of enough inlink descriptors."]
pub struct INLINK_DSCR_EMPTY_INT_CLR_R(crate::FieldReader<bool, bool>);
impl INLINK_DSCR_EMPTY_INT_CLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INLINK_DSCR_EMPTY_INT_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INLINK_DSCR_EMPTY_INT_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INLINK_DSCR_EMPTY_INT_CLR` writer - The clear bit for lack of enough inlink descriptors."]
pub struct INLINK_DSCR_EMPTY_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> INLINK_DSCR_EMPTY_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `OUTLINK_DSCR_ERROR_INT_CLR` reader - The clear bit for outlink descriptor error."]
pub struct OUTLINK_DSCR_ERROR_INT_CLR_R(crate::FieldReader<bool, bool>);
impl OUTLINK_DSCR_ERROR_INT_CLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUTLINK_DSCR_ERROR_INT_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTLINK_DSCR_ERROR_INT_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTLINK_DSCR_ERROR_INT_CLR` writer - The clear bit for outlink descriptor error."]
pub struct OUTLINK_DSCR_ERROR_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTLINK_DSCR_ERROR_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `INLINK_DSCR_ERROR_INT_CLR` reader - The clear bit for inlink descriptor error."]
pub struct INLINK_DSCR_ERROR_INT_CLR_R(crate::FieldReader<bool, bool>);
impl INLINK_DSCR_ERROR_INT_CLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INLINK_DSCR_ERROR_INT_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INLINK_DSCR_ERROR_INT_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INLINK_DSCR_ERROR_INT_CLR` writer - The clear bit for inlink descriptor error."]
pub struct INLINK_DSCR_ERROR_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> INLINK_DSCR_ERROR_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `IN_DONE_INT_CLR` reader - The clear bit for completing usage of a inlink descriptor."]
pub struct IN_DONE_INT_CLR_R(crate::FieldReader<bool, bool>);
impl IN_DONE_INT_CLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IN_DONE_INT_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN_DONE_INT_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN_DONE_INT_CLR` writer - The clear bit for completing usage of a inlink descriptor."]
pub struct IN_DONE_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_DONE_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `IN_ERR_EOF_INT_CLR` reader - The clear bit for receiving error."]
pub struct IN_ERR_EOF_INT_CLR_R(crate::FieldReader<bool, bool>);
impl IN_ERR_EOF_INT_CLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IN_ERR_EOF_INT_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN_ERR_EOF_INT_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN_ERR_EOF_INT_CLR` writer - The clear bit for receiving error."]
pub struct IN_ERR_EOF_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_ERR_EOF_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `IN_SUC_EOF_INT_CLR` reader - The clear bit for completing receiving all the packets from host."]
pub struct IN_SUC_EOF_INT_CLR_R(crate::FieldReader<bool, bool>);
impl IN_SUC_EOF_INT_CLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IN_SUC_EOF_INT_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN_SUC_EOF_INT_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN_SUC_EOF_INT_CLR` writer - The clear bit for completing receiving all the packets from host."]
pub struct IN_SUC_EOF_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_SUC_EOF_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `OUT_DONE_INT_CLR` reader - The clear bit for completing usage of a outlink descriptor."]
pub struct OUT_DONE_INT_CLR_R(crate::FieldReader<bool, bool>);
impl OUT_DONE_INT_CLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUT_DONE_INT_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_DONE_INT_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT_DONE_INT_CLR` writer - The clear bit for completing usage of a outlink descriptor."]
pub struct OUT_DONE_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_DONE_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `OUT_EOF_INT_CLR` reader - The clear bit for sending a packet to host done."]
pub struct OUT_EOF_INT_CLR_R(crate::FieldReader<bool, bool>);
impl OUT_EOF_INT_CLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUT_EOF_INT_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_EOF_INT_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT_EOF_INT_CLR` writer - The clear bit for sending a packet to host done."]
pub struct OUT_EOF_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_EOF_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `OUT_TOTAL_EOF_INT_CLR` reader - The clear bit for sending all the packets to host done."]
pub struct OUT_TOTAL_EOF_INT_CLR_R(crate::FieldReader<bool, bool>);
impl OUT_TOTAL_EOF_INT_CLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUT_TOTAL_EOF_INT_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_TOTAL_EOF_INT_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT_TOTAL_EOF_INT_CLR` writer - The clear bit for sending all the packets to host done."]
pub struct OUT_TOTAL_EOF_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_TOTAL_EOF_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - The clear bit for lack of enough inlink descriptors."]
    #[inline(always)]
    pub fn inlink_dscr_empty_int_clr(&self) -> INLINK_DSCR_EMPTY_INT_CLR_R {
        INLINK_DSCR_EMPTY_INT_CLR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - The clear bit for outlink descriptor error."]
    #[inline(always)]
    pub fn outlink_dscr_error_int_clr(&self) -> OUTLINK_DSCR_ERROR_INT_CLR_R {
        OUTLINK_DSCR_ERROR_INT_CLR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - The clear bit for inlink descriptor error."]
    #[inline(always)]
    pub fn inlink_dscr_error_int_clr(&self) -> INLINK_DSCR_ERROR_INT_CLR_R {
        INLINK_DSCR_ERROR_INT_CLR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - The clear bit for completing usage of a inlink descriptor."]
    #[inline(always)]
    pub fn in_done_int_clr(&self) -> IN_DONE_INT_CLR_R {
        IN_DONE_INT_CLR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - The clear bit for receiving error."]
    #[inline(always)]
    pub fn in_err_eof_int_clr(&self) -> IN_ERR_EOF_INT_CLR_R {
        IN_ERR_EOF_INT_CLR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - The clear bit for completing receiving all the packets from host."]
    #[inline(always)]
    pub fn in_suc_eof_int_clr(&self) -> IN_SUC_EOF_INT_CLR_R {
        IN_SUC_EOF_INT_CLR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - The clear bit for completing usage of a outlink descriptor."]
    #[inline(always)]
    pub fn out_done_int_clr(&self) -> OUT_DONE_INT_CLR_R {
        OUT_DONE_INT_CLR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - The clear bit for sending a packet to host done."]
    #[inline(always)]
    pub fn out_eof_int_clr(&self) -> OUT_EOF_INT_CLR_R {
        OUT_EOF_INT_CLR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - The clear bit for sending all the packets to host done."]
    #[inline(always)]
    pub fn out_total_eof_int_clr(&self) -> OUT_TOTAL_EOF_INT_CLR_R {
        OUT_TOTAL_EOF_INT_CLR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The clear bit for lack of enough inlink descriptors."]
    #[inline(always)]
    pub fn inlink_dscr_empty_int_clr(&mut self) -> INLINK_DSCR_EMPTY_INT_CLR_W {
        INLINK_DSCR_EMPTY_INT_CLR_W { w: self }
    }
    #[doc = "Bit 1 - The clear bit for outlink descriptor error."]
    #[inline(always)]
    pub fn outlink_dscr_error_int_clr(&mut self) -> OUTLINK_DSCR_ERROR_INT_CLR_W {
        OUTLINK_DSCR_ERROR_INT_CLR_W { w: self }
    }
    #[doc = "Bit 2 - The clear bit for inlink descriptor error."]
    #[inline(always)]
    pub fn inlink_dscr_error_int_clr(&mut self) -> INLINK_DSCR_ERROR_INT_CLR_W {
        INLINK_DSCR_ERROR_INT_CLR_W { w: self }
    }
    #[doc = "Bit 3 - The clear bit for completing usage of a inlink descriptor."]
    #[inline(always)]
    pub fn in_done_int_clr(&mut self) -> IN_DONE_INT_CLR_W {
        IN_DONE_INT_CLR_W { w: self }
    }
    #[doc = "Bit 4 - The clear bit for receiving error."]
    #[inline(always)]
    pub fn in_err_eof_int_clr(&mut self) -> IN_ERR_EOF_INT_CLR_W {
        IN_ERR_EOF_INT_CLR_W { w: self }
    }
    #[doc = "Bit 5 - The clear bit for completing receiving all the packets from host."]
    #[inline(always)]
    pub fn in_suc_eof_int_clr(&mut self) -> IN_SUC_EOF_INT_CLR_W {
        IN_SUC_EOF_INT_CLR_W { w: self }
    }
    #[doc = "Bit 6 - The clear bit for completing usage of a outlink descriptor."]
    #[inline(always)]
    pub fn out_done_int_clr(&mut self) -> OUT_DONE_INT_CLR_W {
        OUT_DONE_INT_CLR_W { w: self }
    }
    #[doc = "Bit 7 - The clear bit for sending a packet to host done."]
    #[inline(always)]
    pub fn out_eof_int_clr(&mut self) -> OUT_EOF_INT_CLR_W {
        OUT_EOF_INT_CLR_W { w: self }
    }
    #[doc = "Bit 8 - The clear bit for sending all the packets to host done."]
    #[inline(always)]
    pub fn out_total_eof_int_clr(&mut self) -> OUT_TOTAL_EOF_INT_CLR_W {
        OUT_TOTAL_EOF_INT_CLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_int_clr](index.html) module"]
pub struct DMA_INT_CLR_SPEC;
impl crate::RegisterSpec for DMA_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_int_clr::R](R) reader structure"]
impl crate::Readable for DMA_INT_CLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_int_clr::W](W) writer structure"]
impl crate::Writable for DMA_INT_CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_INT_CLR to value 0"]
impl crate::Resettable for DMA_INT_CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
