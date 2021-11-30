#[doc = "Register `BRIDGE_CONF` reader"]
pub struct R(crate::R<BRIDGE_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BRIDGE_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BRIDGE_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BRIDGE_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BRIDGE_CONF` writer"]
pub struct W(crate::W<BRIDGE_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BRIDGE_CONF_SPEC>;
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
impl From<crate::W<BRIDGE_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BRIDGE_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXEOF_ENA` reader - "]
pub struct TXEOF_ENA_R(crate::FieldReader<u8, u8>);
impl TXEOF_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TXEOF_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXEOF_ENA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXEOF_ENA` writer - "]
pub struct TXEOF_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TXEOF_ENA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Field `FIFO_MAP_ENA` reader - "]
pub struct FIFO_MAP_ENA_R(crate::FieldReader<u8, u8>);
impl FIFO_MAP_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FIFO_MAP_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFO_MAP_ENA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFO_MAP_ENA` writer - "]
pub struct FIFO_MAP_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_MAP_ENA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `SLC0_TX_DUMMY_MODE` reader - "]
pub struct SLC0_TX_DUMMY_MODE_R(crate::FieldReader<bool, bool>);
impl SLC0_TX_DUMMY_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC0_TX_DUMMY_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC0_TX_DUMMY_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC0_TX_DUMMY_MODE` writer - "]
pub struct SLC0_TX_DUMMY_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_TX_DUMMY_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `HDA_MAP_128K` reader - "]
pub struct HDA_MAP_128K_R(crate::FieldReader<bool, bool>);
impl HDA_MAP_128K_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HDA_MAP_128K_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HDA_MAP_128K_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HDA_MAP_128K` writer - "]
pub struct HDA_MAP_128K_W<'a> {
    w: &'a mut W,
}
impl<'a> HDA_MAP_128K_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `SLC1_TX_DUMMY_MODE` reader - "]
pub struct SLC1_TX_DUMMY_MODE_R(crate::FieldReader<bool, bool>);
impl SLC1_TX_DUMMY_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC1_TX_DUMMY_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC1_TX_DUMMY_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC1_TX_DUMMY_MODE` writer - "]
pub struct SLC1_TX_DUMMY_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC1_TX_DUMMY_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `TX_PUSH_IDLE_NUM` reader - "]
pub struct TX_PUSH_IDLE_NUM_R(crate::FieldReader<u16, u16>);
impl TX_PUSH_IDLE_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TX_PUSH_IDLE_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_PUSH_IDLE_NUM_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_PUSH_IDLE_NUM` writer - "]
pub struct TX_PUSH_IDLE_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_PUSH_IDLE_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn txeof_ena(&self) -> TXEOF_ENA_R {
        TXEOF_ENA_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn fifo_map_ena(&self) -> FIFO_MAP_ENA_R {
        FIFO_MAP_ENA_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn slc0_tx_dummy_mode(&self) -> SLC0_TX_DUMMY_MODE_R {
        SLC0_TX_DUMMY_MODE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn hda_map_128k(&self) -> HDA_MAP_128K_R {
        HDA_MAP_128K_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn slc1_tx_dummy_mode(&self) -> SLC1_TX_DUMMY_MODE_R {
        SLC1_TX_DUMMY_MODE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn tx_push_idle_num(&self) -> TX_PUSH_IDLE_NUM_R {
        TX_PUSH_IDLE_NUM_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn txeof_ena(&mut self) -> TXEOF_ENA_W {
        TXEOF_ENA_W { w: self }
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn fifo_map_ena(&mut self) -> FIFO_MAP_ENA_W {
        FIFO_MAP_ENA_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn slc0_tx_dummy_mode(&mut self) -> SLC0_TX_DUMMY_MODE_W {
        SLC0_TX_DUMMY_MODE_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn hda_map_128k(&mut self) -> HDA_MAP_128K_W {
        HDA_MAP_128K_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn slc1_tx_dummy_mode(&mut self) -> SLC1_TX_DUMMY_MODE_W {
        SLC1_TX_DUMMY_MODE_W { w: self }
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn tx_push_idle_num(&mut self) -> TX_PUSH_IDLE_NUM_W {
        TX_PUSH_IDLE_NUM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bridge_conf](index.html) module"]
pub struct BRIDGE_CONF_SPEC;
impl crate::RegisterSpec for BRIDGE_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bridge_conf::R](R) reader structure"]
impl crate::Readable for BRIDGE_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bridge_conf::W](W) writer structure"]
impl crate::Writable for BRIDGE_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BRIDGE_CONF to value 0x000a_7720"]
impl crate::Resettable for BRIDGE_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x000a_7720
    }
}
