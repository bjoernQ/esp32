#[doc = "Register `_0RX_LINK` reader"]
pub struct R(crate::R<_0RX_LINK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<_0RX_LINK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<_0RX_LINK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<_0RX_LINK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `_0RX_LINK` writer"]
pub struct W(crate::W<_0RX_LINK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<_0RX_LINK_SPEC>;
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
impl From<crate::W<_0RX_LINK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<_0RX_LINK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLC0_RXLINK_ADDR` reader - "]
pub struct SLC0_RXLINK_ADDR_R(crate::FieldReader<u32, u32>);
impl SLC0_RXLINK_ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        SLC0_RXLINK_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC0_RXLINK_ADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC0_RXLINK_ADDR` writer - "]
pub struct SLC0_RXLINK_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_RXLINK_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | (value as u32 & 0x000f_ffff);
        self.w
    }
}
#[doc = "Field `SLC0_RXLINK_STOP` reader - "]
pub struct SLC0_RXLINK_STOP_R(crate::FieldReader<bool, bool>);
impl SLC0_RXLINK_STOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC0_RXLINK_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC0_RXLINK_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC0_RXLINK_STOP` writer - "]
pub struct SLC0_RXLINK_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_RXLINK_STOP_W<'a> {
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
#[doc = "Field `SLC0_RXLINK_START` reader - "]
pub struct SLC0_RXLINK_START_R(crate::FieldReader<bool, bool>);
impl SLC0_RXLINK_START_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC0_RXLINK_START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC0_RXLINK_START_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC0_RXLINK_START` writer - "]
pub struct SLC0_RXLINK_START_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_RXLINK_START_W<'a> {
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
#[doc = "Field `SLC0_RXLINK_RESTART` reader - "]
pub struct SLC0_RXLINK_RESTART_R(crate::FieldReader<bool, bool>);
impl SLC0_RXLINK_RESTART_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC0_RXLINK_RESTART_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC0_RXLINK_RESTART_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC0_RXLINK_RESTART` writer - "]
pub struct SLC0_RXLINK_RESTART_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_RXLINK_RESTART_W<'a> {
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
#[doc = "Field `SLC0_RXLINK_PARK` reader - "]
pub struct SLC0_RXLINK_PARK_R(crate::FieldReader<bool, bool>);
impl SLC0_RXLINK_PARK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC0_RXLINK_PARK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC0_RXLINK_PARK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn slc0_rxlink_addr(&self) -> SLC0_RXLINK_ADDR_R {
        SLC0_RXLINK_ADDR_R::new((self.bits & 0x000f_ffff) as u32)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn slc0_rxlink_stop(&self) -> SLC0_RXLINK_STOP_R {
        SLC0_RXLINK_STOP_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn slc0_rxlink_start(&self) -> SLC0_RXLINK_START_R {
        SLC0_RXLINK_START_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn slc0_rxlink_restart(&self) -> SLC0_RXLINK_RESTART_R {
        SLC0_RXLINK_RESTART_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn slc0_rxlink_park(&self) -> SLC0_RXLINK_PARK_R {
        SLC0_RXLINK_PARK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn slc0_rxlink_addr(&mut self) -> SLC0_RXLINK_ADDR_W {
        SLC0_RXLINK_ADDR_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn slc0_rxlink_stop(&mut self) -> SLC0_RXLINK_STOP_W {
        SLC0_RXLINK_STOP_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn slc0_rxlink_start(&mut self) -> SLC0_RXLINK_START_W {
        SLC0_RXLINK_START_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn slc0_rxlink_restart(&mut self) -> SLC0_RXLINK_RESTART_W {
        SLC0_RXLINK_RESTART_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_0rx_link](index.html) module"]
pub struct _0RX_LINK_SPEC;
impl crate::RegisterSpec for _0RX_LINK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [_0rx_link::R](R) reader structure"]
impl crate::Readable for _0RX_LINK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [_0rx_link::W](W) writer structure"]
impl crate::Writable for _0RX_LINK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets _0RX_LINK to value 0"]
impl crate::Resettable for _0RX_LINK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
