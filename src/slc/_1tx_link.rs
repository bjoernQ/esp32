#[doc = "Register `_1TX_LINK` reader"]
pub struct R(crate::R<_1TX_LINK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<_1TX_LINK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<_1TX_LINK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<_1TX_LINK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `_1TX_LINK` writer"]
pub struct W(crate::W<_1TX_LINK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<_1TX_LINK_SPEC>;
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
impl From<crate::W<_1TX_LINK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<_1TX_LINK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLC1_TXLINK_ADDR` reader - "]
pub struct SLC1_TXLINK_ADDR_R(crate::FieldReader<u32, u32>);
impl SLC1_TXLINK_ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        SLC1_TXLINK_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC1_TXLINK_ADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC1_TXLINK_ADDR` writer - "]
pub struct SLC1_TXLINK_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC1_TXLINK_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | (value as u32 & 0x000f_ffff);
        self.w
    }
}
#[doc = "Field `SLC1_TXLINK_STOP` reader - "]
pub struct SLC1_TXLINK_STOP_R(crate::FieldReader<bool, bool>);
impl SLC1_TXLINK_STOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC1_TXLINK_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC1_TXLINK_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC1_TXLINK_STOP` writer - "]
pub struct SLC1_TXLINK_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC1_TXLINK_STOP_W<'a> {
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
#[doc = "Field `SLC1_TXLINK_START` reader - "]
pub struct SLC1_TXLINK_START_R(crate::FieldReader<bool, bool>);
impl SLC1_TXLINK_START_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC1_TXLINK_START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC1_TXLINK_START_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC1_TXLINK_START` writer - "]
pub struct SLC1_TXLINK_START_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC1_TXLINK_START_W<'a> {
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
#[doc = "Field `SLC1_TXLINK_RESTART` reader - "]
pub struct SLC1_TXLINK_RESTART_R(crate::FieldReader<bool, bool>);
impl SLC1_TXLINK_RESTART_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC1_TXLINK_RESTART_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC1_TXLINK_RESTART_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC1_TXLINK_RESTART` writer - "]
pub struct SLC1_TXLINK_RESTART_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC1_TXLINK_RESTART_W<'a> {
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
#[doc = "Field `SLC1_TXLINK_PARK` reader - "]
pub struct SLC1_TXLINK_PARK_R(crate::FieldReader<bool, bool>);
impl SLC1_TXLINK_PARK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC1_TXLINK_PARK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC1_TXLINK_PARK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn slc1_txlink_addr(&self) -> SLC1_TXLINK_ADDR_R {
        SLC1_TXLINK_ADDR_R::new((self.bits & 0x000f_ffff) as u32)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn slc1_txlink_stop(&self) -> SLC1_TXLINK_STOP_R {
        SLC1_TXLINK_STOP_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn slc1_txlink_start(&self) -> SLC1_TXLINK_START_R {
        SLC1_TXLINK_START_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn slc1_txlink_restart(&self) -> SLC1_TXLINK_RESTART_R {
        SLC1_TXLINK_RESTART_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn slc1_txlink_park(&self) -> SLC1_TXLINK_PARK_R {
        SLC1_TXLINK_PARK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn slc1_txlink_addr(&mut self) -> SLC1_TXLINK_ADDR_W {
        SLC1_TXLINK_ADDR_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn slc1_txlink_stop(&mut self) -> SLC1_TXLINK_STOP_W {
        SLC1_TXLINK_STOP_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn slc1_txlink_start(&mut self) -> SLC1_TXLINK_START_W {
        SLC1_TXLINK_START_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn slc1_txlink_restart(&mut self) -> SLC1_TXLINK_RESTART_W {
        SLC1_TXLINK_RESTART_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_1tx_link](index.html) module"]
pub struct _1TX_LINK_SPEC;
impl crate::RegisterSpec for _1TX_LINK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [_1tx_link::R](R) reader structure"]
impl crate::Readable for _1TX_LINK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [_1tx_link::W](W) writer structure"]
impl crate::Writable for _1TX_LINK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets _1TX_LINK to value 0"]
impl crate::Resettable for _1TX_LINK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
