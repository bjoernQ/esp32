#[doc = "Register `RX_DSCR_CONF` reader"]
pub struct R(crate::R<RX_DSCR_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_DSCR_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_DSCR_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_DSCR_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RX_DSCR_CONF` writer"]
pub struct W(crate::W<RX_DSCR_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX_DSCR_CONF_SPEC>;
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
impl From<crate::W<RX_DSCR_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX_DSCR_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLC0_TOKEN_NO_REPLACE` reader - "]
pub struct SLC0_TOKEN_NO_REPLACE_R(crate::FieldReader<bool, bool>);
impl SLC0_TOKEN_NO_REPLACE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC0_TOKEN_NO_REPLACE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC0_TOKEN_NO_REPLACE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC0_TOKEN_NO_REPLACE` writer - "]
pub struct SLC0_TOKEN_NO_REPLACE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_TOKEN_NO_REPLACE_W<'a> {
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
#[doc = "Field `SLC0_INFOR_NO_REPLACE` reader - "]
pub struct SLC0_INFOR_NO_REPLACE_R(crate::FieldReader<bool, bool>);
impl SLC0_INFOR_NO_REPLACE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC0_INFOR_NO_REPLACE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC0_INFOR_NO_REPLACE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC0_INFOR_NO_REPLACE` writer - "]
pub struct SLC0_INFOR_NO_REPLACE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_INFOR_NO_REPLACE_W<'a> {
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
#[doc = "Field `SLC0_RX_FILL_MODE` reader - "]
pub struct SLC0_RX_FILL_MODE_R(crate::FieldReader<bool, bool>);
impl SLC0_RX_FILL_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC0_RX_FILL_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC0_RX_FILL_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC0_RX_FILL_MODE` writer - "]
pub struct SLC0_RX_FILL_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_RX_FILL_MODE_W<'a> {
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
#[doc = "Field `SLC0_RX_EOF_MODE` reader - "]
pub struct SLC0_RX_EOF_MODE_R(crate::FieldReader<bool, bool>);
impl SLC0_RX_EOF_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC0_RX_EOF_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC0_RX_EOF_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC0_RX_EOF_MODE` writer - "]
pub struct SLC0_RX_EOF_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_RX_EOF_MODE_W<'a> {
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
#[doc = "Field `SLC0_RX_FILL_EN` reader - "]
pub struct SLC0_RX_FILL_EN_R(crate::FieldReader<bool, bool>);
impl SLC0_RX_FILL_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC0_RX_FILL_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC0_RX_FILL_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC0_RX_FILL_EN` writer - "]
pub struct SLC0_RX_FILL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_RX_FILL_EN_W<'a> {
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
#[doc = "Field `SLC0_RD_RETRY_THRESHOLD` reader - "]
pub struct SLC0_RD_RETRY_THRESHOLD_R(crate::FieldReader<u16, u16>);
impl SLC0_RD_RETRY_THRESHOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        SLC0_RD_RETRY_THRESHOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC0_RD_RETRY_THRESHOLD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC0_RD_RETRY_THRESHOLD` writer - "]
pub struct SLC0_RD_RETRY_THRESHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_RD_RETRY_THRESHOLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 5)) | ((value as u32 & 0x07ff) << 5);
        self.w
    }
}
#[doc = "Field `SLC1_TOKEN_NO_REPLACE` reader - "]
pub struct SLC1_TOKEN_NO_REPLACE_R(crate::FieldReader<bool, bool>);
impl SLC1_TOKEN_NO_REPLACE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC1_TOKEN_NO_REPLACE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC1_TOKEN_NO_REPLACE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC1_TOKEN_NO_REPLACE` writer - "]
pub struct SLC1_TOKEN_NO_REPLACE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC1_TOKEN_NO_REPLACE_W<'a> {
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
#[doc = "Field `SLC1_INFOR_NO_REPLACE` reader - "]
pub struct SLC1_INFOR_NO_REPLACE_R(crate::FieldReader<bool, bool>);
impl SLC1_INFOR_NO_REPLACE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC1_INFOR_NO_REPLACE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC1_INFOR_NO_REPLACE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC1_INFOR_NO_REPLACE` writer - "]
pub struct SLC1_INFOR_NO_REPLACE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC1_INFOR_NO_REPLACE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `SLC1_RX_FILL_MODE` reader - "]
pub struct SLC1_RX_FILL_MODE_R(crate::FieldReader<bool, bool>);
impl SLC1_RX_FILL_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC1_RX_FILL_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC1_RX_FILL_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC1_RX_FILL_MODE` writer - "]
pub struct SLC1_RX_FILL_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC1_RX_FILL_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `SLC1_RX_EOF_MODE` reader - "]
pub struct SLC1_RX_EOF_MODE_R(crate::FieldReader<bool, bool>);
impl SLC1_RX_EOF_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC1_RX_EOF_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC1_RX_EOF_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC1_RX_EOF_MODE` writer - "]
pub struct SLC1_RX_EOF_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC1_RX_EOF_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `SLC1_RX_FILL_EN` reader - "]
pub struct SLC1_RX_FILL_EN_R(crate::FieldReader<bool, bool>);
impl SLC1_RX_FILL_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC1_RX_FILL_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC1_RX_FILL_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC1_RX_FILL_EN` writer - "]
pub struct SLC1_RX_FILL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC1_RX_FILL_EN_W<'a> {
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
#[doc = "Field `SLC1_RD_RETRY_THRESHOLD` reader - "]
pub struct SLC1_RD_RETRY_THRESHOLD_R(crate::FieldReader<u16, u16>);
impl SLC1_RD_RETRY_THRESHOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        SLC1_RD_RETRY_THRESHOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC1_RD_RETRY_THRESHOLD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC1_RD_RETRY_THRESHOLD` writer - "]
pub struct SLC1_RD_RETRY_THRESHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC1_RD_RETRY_THRESHOLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 21)) | ((value as u32 & 0x07ff) << 21);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn slc0_token_no_replace(&self) -> SLC0_TOKEN_NO_REPLACE_R {
        SLC0_TOKEN_NO_REPLACE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn slc0_infor_no_replace(&self) -> SLC0_INFOR_NO_REPLACE_R {
        SLC0_INFOR_NO_REPLACE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn slc0_rx_fill_mode(&self) -> SLC0_RX_FILL_MODE_R {
        SLC0_RX_FILL_MODE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn slc0_rx_eof_mode(&self) -> SLC0_RX_EOF_MODE_R {
        SLC0_RX_EOF_MODE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn slc0_rx_fill_en(&self) -> SLC0_RX_FILL_EN_R {
        SLC0_RX_FILL_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:15"]
    #[inline(always)]
    pub fn slc0_rd_retry_threshold(&self) -> SLC0_RD_RETRY_THRESHOLD_R {
        SLC0_RD_RETRY_THRESHOLD_R::new(((self.bits >> 5) & 0x07ff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slc1_token_no_replace(&self) -> SLC1_TOKEN_NO_REPLACE_R {
        SLC1_TOKEN_NO_REPLACE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn slc1_infor_no_replace(&self) -> SLC1_INFOR_NO_REPLACE_R {
        SLC1_INFOR_NO_REPLACE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn slc1_rx_fill_mode(&self) -> SLC1_RX_FILL_MODE_R {
        SLC1_RX_FILL_MODE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn slc1_rx_eof_mode(&self) -> SLC1_RX_EOF_MODE_R {
        SLC1_RX_EOF_MODE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn slc1_rx_fill_en(&self) -> SLC1_RX_FILL_EN_R {
        SLC1_RX_FILL_EN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 21:31"]
    #[inline(always)]
    pub fn slc1_rd_retry_threshold(&self) -> SLC1_RD_RETRY_THRESHOLD_R {
        SLC1_RD_RETRY_THRESHOLD_R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn slc0_token_no_replace(&mut self) -> SLC0_TOKEN_NO_REPLACE_W {
        SLC0_TOKEN_NO_REPLACE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn slc0_infor_no_replace(&mut self) -> SLC0_INFOR_NO_REPLACE_W {
        SLC0_INFOR_NO_REPLACE_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn slc0_rx_fill_mode(&mut self) -> SLC0_RX_FILL_MODE_W {
        SLC0_RX_FILL_MODE_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn slc0_rx_eof_mode(&mut self) -> SLC0_RX_EOF_MODE_W {
        SLC0_RX_EOF_MODE_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn slc0_rx_fill_en(&mut self) -> SLC0_RX_FILL_EN_W {
        SLC0_RX_FILL_EN_W { w: self }
    }
    #[doc = "Bits 5:15"]
    #[inline(always)]
    pub fn slc0_rd_retry_threshold(&mut self) -> SLC0_RD_RETRY_THRESHOLD_W {
        SLC0_RD_RETRY_THRESHOLD_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slc1_token_no_replace(&mut self) -> SLC1_TOKEN_NO_REPLACE_W {
        SLC1_TOKEN_NO_REPLACE_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn slc1_infor_no_replace(&mut self) -> SLC1_INFOR_NO_REPLACE_W {
        SLC1_INFOR_NO_REPLACE_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn slc1_rx_fill_mode(&mut self) -> SLC1_RX_FILL_MODE_W {
        SLC1_RX_FILL_MODE_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn slc1_rx_eof_mode(&mut self) -> SLC1_RX_EOF_MODE_W {
        SLC1_RX_EOF_MODE_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn slc1_rx_fill_en(&mut self) -> SLC1_RX_FILL_EN_W {
        SLC1_RX_FILL_EN_W { w: self }
    }
    #[doc = "Bits 21:31"]
    #[inline(always)]
    pub fn slc1_rd_retry_threshold(&mut self) -> SLC1_RD_RETRY_THRESHOLD_W {
        SLC1_RD_RETRY_THRESHOLD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_dscr_conf](index.html) module"]
pub struct RX_DSCR_CONF_SPEC;
impl crate::RegisterSpec for RX_DSCR_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_dscr_conf::R](R) reader structure"]
impl crate::Readable for RX_DSCR_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx_dscr_conf::W](W) writer structure"]
impl crate::Writable for RX_DSCR_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RX_DSCR_CONF to value 0x101b_101a"]
impl crate::Resettable for RX_DSCR_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x101b_101a
    }
}
