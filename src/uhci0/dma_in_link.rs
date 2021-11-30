#[doc = "Register `DMA_IN_LINK` reader"]
pub struct R(crate::R<DMA_IN_LINK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_IN_LINK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_IN_LINK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_IN_LINK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_IN_LINK` writer"]
pub struct W(crate::W<DMA_IN_LINK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_IN_LINK_SPEC>;
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
impl From<crate::W<DMA_IN_LINK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_IN_LINK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INLINK_ADDR` reader - This register stores the least 20 bits of the first in link descriptor's address."]
pub struct INLINK_ADDR_R(crate::FieldReader<u32, u32>);
impl INLINK_ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        INLINK_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INLINK_ADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INLINK_ADDR` writer - This register stores the least 20 bits of the first in link descriptor's address."]
pub struct INLINK_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> INLINK_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | (value as u32 & 0x000f_ffff);
        self.w
    }
}
#[doc = "Field `INLINK_AUTO_RET` reader - 1:when a packet is wrong in link descriptor returns to the descriptor which is lately used."]
pub struct INLINK_AUTO_RET_R(crate::FieldReader<bool, bool>);
impl INLINK_AUTO_RET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INLINK_AUTO_RET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INLINK_AUTO_RET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INLINK_AUTO_RET` writer - 1:when a packet is wrong in link descriptor returns to the descriptor which is lately used."]
pub struct INLINK_AUTO_RET_W<'a> {
    w: &'a mut W,
}
impl<'a> INLINK_AUTO_RET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `INLINK_STOP` reader - Set this bit to stop dealing with the in link descriptors."]
pub struct INLINK_STOP_R(crate::FieldReader<bool, bool>);
impl INLINK_STOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INLINK_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INLINK_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INLINK_STOP` writer - Set this bit to stop dealing with the in link descriptors."]
pub struct INLINK_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> INLINK_STOP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `INLINK_START` reader - Set this bit to start dealing with the in link descriptors."]
pub struct INLINK_START_R(crate::FieldReader<bool, bool>);
impl INLINK_START_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INLINK_START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INLINK_START_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INLINK_START` writer - Set this bit to start dealing with the in link descriptors."]
pub struct INLINK_START_W<'a> {
    w: &'a mut W,
}
impl<'a> INLINK_START_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `INLINK_RESTART` reader - Set this bit to mount on new in link descriptors"]
pub struct INLINK_RESTART_R(crate::FieldReader<bool, bool>);
impl INLINK_RESTART_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INLINK_RESTART_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INLINK_RESTART_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INLINK_RESTART` writer - Set this bit to mount on new in link descriptors"]
pub struct INLINK_RESTART_W<'a> {
    w: &'a mut W,
}
impl<'a> INLINK_RESTART_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `INLINK_PARK` reader - 1:the in link descriptor's fsm is in idle state. 0:the in link descriptor's fsm is working"]
pub struct INLINK_PARK_R(crate::FieldReader<bool, bool>);
impl INLINK_PARK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INLINK_PARK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INLINK_PARK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:19 - This register stores the least 20 bits of the first in link descriptor's address."]
    #[inline(always)]
    pub fn inlink_addr(&self) -> INLINK_ADDR_R {
        INLINK_ADDR_R::new((self.bits & 0x000f_ffff) as u32)
    }
    #[doc = "Bit 20 - 1:when a packet is wrong in link descriptor returns to the descriptor which is lately used."]
    #[inline(always)]
    pub fn inlink_auto_ret(&self) -> INLINK_AUTO_RET_R {
        INLINK_AUTO_RET_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Set this bit to stop dealing with the in link descriptors."]
    #[inline(always)]
    pub fn inlink_stop(&self) -> INLINK_STOP_R {
        INLINK_STOP_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Set this bit to start dealing with the in link descriptors."]
    #[inline(always)]
    pub fn inlink_start(&self) -> INLINK_START_R {
        INLINK_START_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Set this bit to mount on new in link descriptors"]
    #[inline(always)]
    pub fn inlink_restart(&self) -> INLINK_RESTART_R {
        INLINK_RESTART_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - 1:the in link descriptor's fsm is in idle state. 0:the in link descriptor's fsm is working"]
    #[inline(always)]
    pub fn inlink_park(&self) -> INLINK_PARK_R {
        INLINK_PARK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:19 - This register stores the least 20 bits of the first in link descriptor's address."]
    #[inline(always)]
    pub fn inlink_addr(&mut self) -> INLINK_ADDR_W {
        INLINK_ADDR_W { w: self }
    }
    #[doc = "Bit 20 - 1:when a packet is wrong in link descriptor returns to the descriptor which is lately used."]
    #[inline(always)]
    pub fn inlink_auto_ret(&mut self) -> INLINK_AUTO_RET_W {
        INLINK_AUTO_RET_W { w: self }
    }
    #[doc = "Bit 28 - Set this bit to stop dealing with the in link descriptors."]
    #[inline(always)]
    pub fn inlink_stop(&mut self) -> INLINK_STOP_W {
        INLINK_STOP_W { w: self }
    }
    #[doc = "Bit 29 - Set this bit to start dealing with the in link descriptors."]
    #[inline(always)]
    pub fn inlink_start(&mut self) -> INLINK_START_W {
        INLINK_START_W { w: self }
    }
    #[doc = "Bit 30 - Set this bit to mount on new in link descriptors"]
    #[inline(always)]
    pub fn inlink_restart(&mut self) -> INLINK_RESTART_W {
        INLINK_RESTART_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_in_link](index.html) module"]
pub struct DMA_IN_LINK_SPEC;
impl crate::RegisterSpec for DMA_IN_LINK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_in_link::R](R) reader structure"]
impl crate::Readable for DMA_IN_LINK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_in_link::W](W) writer structure"]
impl crate::Writable for DMA_IN_LINK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_IN_LINK to value 0x0010_0000"]
impl crate::Resettable for DMA_IN_LINK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0010_0000
    }
}
