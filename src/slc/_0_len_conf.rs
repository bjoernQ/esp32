#[doc = "Register `_0_LEN_CONF` reader"]
pub struct R(crate::R<_0_LEN_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<_0_LEN_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<_0_LEN_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<_0_LEN_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `_0_LEN_CONF` writer"]
pub struct W(crate::W<_0_LEN_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<_0_LEN_CONF_SPEC>;
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
impl From<crate::W<_0_LEN_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<_0_LEN_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLC0_LEN_WDATA` writer - "]
pub struct SLC0_LEN_WDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_LEN_WDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | (value as u32 & 0x000f_ffff);
        self.w
    }
}
#[doc = "Field `SLC0_LEN_WR` writer - "]
pub struct SLC0_LEN_WR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_LEN_WR_W<'a> {
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
#[doc = "Field `SLC0_LEN_INC` writer - "]
pub struct SLC0_LEN_INC_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_LEN_INC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `SLC0_LEN_INC_MORE` writer - "]
pub struct SLC0_LEN_INC_MORE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_LEN_INC_MORE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `SLC0_RX_PACKET_LOAD_EN` reader - "]
pub struct SLC0_RX_PACKET_LOAD_EN_R(crate::FieldReader<bool, bool>);
impl SLC0_RX_PACKET_LOAD_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC0_RX_PACKET_LOAD_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC0_RX_PACKET_LOAD_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC0_RX_PACKET_LOAD_EN` writer - "]
pub struct SLC0_RX_PACKET_LOAD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_RX_PACKET_LOAD_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `SLC0_TX_PACKET_LOAD_EN` reader - "]
pub struct SLC0_TX_PACKET_LOAD_EN_R(crate::FieldReader<bool, bool>);
impl SLC0_TX_PACKET_LOAD_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC0_TX_PACKET_LOAD_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC0_TX_PACKET_LOAD_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC0_TX_PACKET_LOAD_EN` writer - "]
pub struct SLC0_TX_PACKET_LOAD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_TX_PACKET_LOAD_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `SLC0_RX_GET_USED_DSCR` writer - "]
pub struct SLC0_RX_GET_USED_DSCR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_RX_GET_USED_DSCR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `SLC0_TX_GET_USED_DSCR` writer - "]
pub struct SLC0_TX_GET_USED_DSCR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_TX_GET_USED_DSCR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `SLC0_RX_NEW_PKT_IND` reader - "]
pub struct SLC0_RX_NEW_PKT_IND_R(crate::FieldReader<bool, bool>);
impl SLC0_RX_NEW_PKT_IND_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC0_RX_NEW_PKT_IND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC0_RX_NEW_PKT_IND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC0_TX_NEW_PKT_IND` reader - "]
pub struct SLC0_TX_NEW_PKT_IND_R(crate::FieldReader<bool, bool>);
impl SLC0_TX_NEW_PKT_IND_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC0_TX_NEW_PKT_IND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC0_TX_NEW_PKT_IND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn slc0_rx_packet_load_en(&self) -> SLC0_RX_PACKET_LOAD_EN_R {
        SLC0_RX_PACKET_LOAD_EN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn slc0_tx_packet_load_en(&self) -> SLC0_TX_PACKET_LOAD_EN_R {
        SLC0_TX_PACKET_LOAD_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn slc0_rx_new_pkt_ind(&self) -> SLC0_RX_NEW_PKT_IND_R {
        SLC0_RX_NEW_PKT_IND_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn slc0_tx_new_pkt_ind(&self) -> SLC0_TX_NEW_PKT_IND_R {
        SLC0_TX_NEW_PKT_IND_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn slc0_len_wdata(&mut self) -> SLC0_LEN_WDATA_W {
        SLC0_LEN_WDATA_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn slc0_len_wr(&mut self) -> SLC0_LEN_WR_W {
        SLC0_LEN_WR_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn slc0_len_inc(&mut self) -> SLC0_LEN_INC_W {
        SLC0_LEN_INC_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn slc0_len_inc_more(&mut self) -> SLC0_LEN_INC_MORE_W {
        SLC0_LEN_INC_MORE_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn slc0_rx_packet_load_en(&mut self) -> SLC0_RX_PACKET_LOAD_EN_W {
        SLC0_RX_PACKET_LOAD_EN_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn slc0_tx_packet_load_en(&mut self) -> SLC0_TX_PACKET_LOAD_EN_W {
        SLC0_TX_PACKET_LOAD_EN_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn slc0_rx_get_used_dscr(&mut self) -> SLC0_RX_GET_USED_DSCR_W {
        SLC0_RX_GET_USED_DSCR_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn slc0_tx_get_used_dscr(&mut self) -> SLC0_TX_GET_USED_DSCR_W {
        SLC0_TX_GET_USED_DSCR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_0_len_conf](index.html) module"]
pub struct _0_LEN_CONF_SPEC;
impl crate::RegisterSpec for _0_LEN_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [_0_len_conf::R](R) reader structure"]
impl crate::Readable for _0_LEN_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [_0_len_conf::W](W) writer structure"]
impl crate::Writable for _0_LEN_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets _0_LEN_CONF to value 0"]
impl crate::Resettable for _0_LEN_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
