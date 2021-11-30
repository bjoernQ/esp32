#[doc = "Register `TIMING` reader"]
pub struct R(crate::R<TIMING_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMING_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMING_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMING_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMING` writer"]
pub struct W(crate::W<TIMING_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMING_SPEC>;
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
impl From<crate::W<TIMING_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMING_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_BCK_IN_DELAY` reader - "]
pub struct TX_BCK_IN_DELAY_R(crate::FieldReader<u8, u8>);
impl TX_BCK_IN_DELAY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TX_BCK_IN_DELAY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_BCK_IN_DELAY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_BCK_IN_DELAY` writer - "]
pub struct TX_BCK_IN_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_BCK_IN_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `TX_WS_IN_DELAY` reader - "]
pub struct TX_WS_IN_DELAY_R(crate::FieldReader<u8, u8>);
impl TX_WS_IN_DELAY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TX_WS_IN_DELAY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_WS_IN_DELAY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_WS_IN_DELAY` writer - "]
pub struct TX_WS_IN_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_WS_IN_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `RX_BCK_IN_DELAY` reader - "]
pub struct RX_BCK_IN_DELAY_R(crate::FieldReader<u8, u8>);
impl RX_BCK_IN_DELAY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RX_BCK_IN_DELAY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_BCK_IN_DELAY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_BCK_IN_DELAY` writer - "]
pub struct RX_BCK_IN_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_BCK_IN_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `RX_WS_IN_DELAY` reader - "]
pub struct RX_WS_IN_DELAY_R(crate::FieldReader<u8, u8>);
impl RX_WS_IN_DELAY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RX_WS_IN_DELAY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_WS_IN_DELAY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_WS_IN_DELAY` writer - "]
pub struct RX_WS_IN_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_WS_IN_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `RX_SD_IN_DELAY` reader - "]
pub struct RX_SD_IN_DELAY_R(crate::FieldReader<u8, u8>);
impl RX_SD_IN_DELAY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RX_SD_IN_DELAY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_SD_IN_DELAY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_SD_IN_DELAY` writer - "]
pub struct RX_SD_IN_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_SD_IN_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `TX_BCK_OUT_DELAY` reader - "]
pub struct TX_BCK_OUT_DELAY_R(crate::FieldReader<u8, u8>);
impl TX_BCK_OUT_DELAY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TX_BCK_OUT_DELAY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_BCK_OUT_DELAY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_BCK_OUT_DELAY` writer - "]
pub struct TX_BCK_OUT_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_BCK_OUT_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Field `TX_WS_OUT_DELAY` reader - "]
pub struct TX_WS_OUT_DELAY_R(crate::FieldReader<u8, u8>);
impl TX_WS_OUT_DELAY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TX_WS_OUT_DELAY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_WS_OUT_DELAY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_WS_OUT_DELAY` writer - "]
pub struct TX_WS_OUT_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_WS_OUT_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `TX_SD_OUT_DELAY` reader - "]
pub struct TX_SD_OUT_DELAY_R(crate::FieldReader<u8, u8>);
impl TX_SD_OUT_DELAY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TX_SD_OUT_DELAY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_SD_OUT_DELAY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_SD_OUT_DELAY` writer - "]
pub struct TX_SD_OUT_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_SD_OUT_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Field `RX_WS_OUT_DELAY` reader - "]
pub struct RX_WS_OUT_DELAY_R(crate::FieldReader<u8, u8>);
impl RX_WS_OUT_DELAY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RX_WS_OUT_DELAY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_WS_OUT_DELAY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_WS_OUT_DELAY` writer - "]
pub struct RX_WS_OUT_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_WS_OUT_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `RX_BCK_OUT_DELAY` reader - "]
pub struct RX_BCK_OUT_DELAY_R(crate::FieldReader<u8, u8>);
impl RX_BCK_OUT_DELAY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RX_BCK_OUT_DELAY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_BCK_OUT_DELAY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_BCK_OUT_DELAY` writer - "]
pub struct RX_BCK_OUT_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_BCK_OUT_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
#[doc = "Field `TX_DSYNC_SW` reader - "]
pub struct TX_DSYNC_SW_R(crate::FieldReader<bool, bool>);
impl TX_DSYNC_SW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_DSYNC_SW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_DSYNC_SW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_DSYNC_SW` writer - "]
pub struct TX_DSYNC_SW_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DSYNC_SW_W<'a> {
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
#[doc = "Field `RX_DSYNC_SW` reader - "]
pub struct RX_DSYNC_SW_R(crate::FieldReader<bool, bool>);
impl RX_DSYNC_SW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_DSYNC_SW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_DSYNC_SW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_DSYNC_SW` writer - "]
pub struct RX_DSYNC_SW_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_DSYNC_SW_W<'a> {
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
#[doc = "Field `DATA_ENABLE_DELAY` reader - "]
pub struct DATA_ENABLE_DELAY_R(crate::FieldReader<u8, u8>);
impl DATA_ENABLE_DELAY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DATA_ENABLE_DELAY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_ENABLE_DELAY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA_ENABLE_DELAY` writer - "]
pub struct DATA_ENABLE_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_ENABLE_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
#[doc = "Field `TX_BCK_IN_INV` reader - "]
pub struct TX_BCK_IN_INV_R(crate::FieldReader<bool, bool>);
impl TX_BCK_IN_INV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_BCK_IN_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_BCK_IN_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_BCK_IN_INV` writer - "]
pub struct TX_BCK_IN_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_BCK_IN_INV_W<'a> {
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
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn tx_bck_in_delay(&self) -> TX_BCK_IN_DELAY_R {
        TX_BCK_IN_DELAY_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn tx_ws_in_delay(&self) -> TX_WS_IN_DELAY_R {
        TX_WS_IN_DELAY_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn rx_bck_in_delay(&self) -> RX_BCK_IN_DELAY_R {
        RX_BCK_IN_DELAY_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn rx_ws_in_delay(&self) -> RX_WS_IN_DELAY_R {
        RX_WS_IN_DELAY_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn rx_sd_in_delay(&self) -> RX_SD_IN_DELAY_R {
        RX_SD_IN_DELAY_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn tx_bck_out_delay(&self) -> TX_BCK_OUT_DELAY_R {
        TX_BCK_OUT_DELAY_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn tx_ws_out_delay(&self) -> TX_WS_OUT_DELAY_R {
        TX_WS_OUT_DELAY_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn tx_sd_out_delay(&self) -> TX_SD_OUT_DELAY_R {
        TX_SD_OUT_DELAY_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn rx_ws_out_delay(&self) -> RX_WS_OUT_DELAY_R {
        RX_WS_OUT_DELAY_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn rx_bck_out_delay(&self) -> RX_BCK_OUT_DELAY_R {
        RX_BCK_OUT_DELAY_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn tx_dsync_sw(&self) -> TX_DSYNC_SW_R {
        TX_DSYNC_SW_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn rx_dsync_sw(&self) -> RX_DSYNC_SW_R {
        RX_DSYNC_SW_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn data_enable_delay(&self) -> DATA_ENABLE_DELAY_R {
        DATA_ENABLE_DELAY_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn tx_bck_in_inv(&self) -> TX_BCK_IN_INV_R {
        TX_BCK_IN_INV_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn tx_bck_in_delay(&mut self) -> TX_BCK_IN_DELAY_W {
        TX_BCK_IN_DELAY_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn tx_ws_in_delay(&mut self) -> TX_WS_IN_DELAY_W {
        TX_WS_IN_DELAY_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn rx_bck_in_delay(&mut self) -> RX_BCK_IN_DELAY_W {
        RX_BCK_IN_DELAY_W { w: self }
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn rx_ws_in_delay(&mut self) -> RX_WS_IN_DELAY_W {
        RX_WS_IN_DELAY_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn rx_sd_in_delay(&mut self) -> RX_SD_IN_DELAY_W {
        RX_SD_IN_DELAY_W { w: self }
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn tx_bck_out_delay(&mut self) -> TX_BCK_OUT_DELAY_W {
        TX_BCK_OUT_DELAY_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn tx_ws_out_delay(&mut self) -> TX_WS_OUT_DELAY_W {
        TX_WS_OUT_DELAY_W { w: self }
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn tx_sd_out_delay(&mut self) -> TX_SD_OUT_DELAY_W {
        TX_SD_OUT_DELAY_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn rx_ws_out_delay(&mut self) -> RX_WS_OUT_DELAY_W {
        RX_WS_OUT_DELAY_W { w: self }
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn rx_bck_out_delay(&mut self) -> RX_BCK_OUT_DELAY_W {
        RX_BCK_OUT_DELAY_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn tx_dsync_sw(&mut self) -> TX_DSYNC_SW_W {
        TX_DSYNC_SW_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn rx_dsync_sw(&mut self) -> RX_DSYNC_SW_W {
        RX_DSYNC_SW_W { w: self }
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn data_enable_delay(&mut self) -> DATA_ENABLE_DELAY_W {
        DATA_ENABLE_DELAY_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn tx_bck_in_inv(&mut self) -> TX_BCK_IN_INV_W {
        TX_BCK_IN_INV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timing](index.html) module"]
pub struct TIMING_SPEC;
impl crate::RegisterSpec for TIMING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timing::R](R) reader structure"]
impl crate::Readable for TIMING_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timing::W](W) writer structure"]
impl crate::Writable for TIMING_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMING to value 0"]
impl crate::Resettable for TIMING_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
