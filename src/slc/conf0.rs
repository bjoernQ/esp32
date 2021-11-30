#[doc = "Register `CONF0` reader"]
pub struct R(crate::R<CONF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONF0` writer"]
pub struct W(crate::W<CONF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONF0_SPEC>;
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
impl From<crate::W<CONF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLC0_TX_RST` reader - "]
pub struct SLC0_TX_RST_R(crate::FieldReader<bool, bool>);
impl SLC0_TX_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC0_TX_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC0_TX_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC0_TX_RST` writer - "]
pub struct SLC0_TX_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_TX_RST_W<'a> {
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
#[doc = "Field `SLC0_RX_RST` reader - "]
pub struct SLC0_RX_RST_R(crate::FieldReader<bool, bool>);
impl SLC0_RX_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC0_RX_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC0_RX_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC0_RX_RST` writer - "]
pub struct SLC0_RX_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_RX_RST_W<'a> {
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
#[doc = "Field `AHBM_FIFO_RST` reader - "]
pub struct AHBM_FIFO_RST_R(crate::FieldReader<bool, bool>);
impl AHBM_FIFO_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AHBM_FIFO_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AHBM_FIFO_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AHBM_FIFO_RST` writer - "]
pub struct AHBM_FIFO_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> AHBM_FIFO_RST_W<'a> {
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
#[doc = "Field `AHBM_RST` reader - "]
pub struct AHBM_RST_R(crate::FieldReader<bool, bool>);
impl AHBM_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AHBM_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AHBM_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AHBM_RST` writer - "]
pub struct AHBM_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> AHBM_RST_W<'a> {
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
#[doc = "Field `SLC0_TX_LOOP_TEST` reader - "]
pub struct SLC0_TX_LOOP_TEST_R(crate::FieldReader<bool, bool>);
impl SLC0_TX_LOOP_TEST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC0_TX_LOOP_TEST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC0_TX_LOOP_TEST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC0_TX_LOOP_TEST` writer - "]
pub struct SLC0_TX_LOOP_TEST_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_TX_LOOP_TEST_W<'a> {
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
#[doc = "Field `SLC0_RX_LOOP_TEST` reader - "]
pub struct SLC0_RX_LOOP_TEST_R(crate::FieldReader<bool, bool>);
impl SLC0_RX_LOOP_TEST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC0_RX_LOOP_TEST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC0_RX_LOOP_TEST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC0_RX_LOOP_TEST` writer - "]
pub struct SLC0_RX_LOOP_TEST_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_RX_LOOP_TEST_W<'a> {
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
#[doc = "Field `SLC0_RX_AUTO_WRBACK` reader - "]
pub struct SLC0_RX_AUTO_WRBACK_R(crate::FieldReader<bool, bool>);
impl SLC0_RX_AUTO_WRBACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC0_RX_AUTO_WRBACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC0_RX_AUTO_WRBACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC0_RX_AUTO_WRBACK` writer - "]
pub struct SLC0_RX_AUTO_WRBACK_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_RX_AUTO_WRBACK_W<'a> {
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
#[doc = "Field `SLC0_RX_NO_RESTART_CLR` reader - "]
pub struct SLC0_RX_NO_RESTART_CLR_R(crate::FieldReader<bool, bool>);
impl SLC0_RX_NO_RESTART_CLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC0_RX_NO_RESTART_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC0_RX_NO_RESTART_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC0_RX_NO_RESTART_CLR` writer - "]
pub struct SLC0_RX_NO_RESTART_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_RX_NO_RESTART_CLR_W<'a> {
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
#[doc = "Field `SLC0_RXDSCR_BURST_EN` reader - "]
pub struct SLC0_RXDSCR_BURST_EN_R(crate::FieldReader<bool, bool>);
impl SLC0_RXDSCR_BURST_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC0_RXDSCR_BURST_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC0_RXDSCR_BURST_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC0_RXDSCR_BURST_EN` writer - "]
pub struct SLC0_RXDSCR_BURST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_RXDSCR_BURST_EN_W<'a> {
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
#[doc = "Field `SLC0_RXDATA_BURST_EN` reader - "]
pub struct SLC0_RXDATA_BURST_EN_R(crate::FieldReader<bool, bool>);
impl SLC0_RXDATA_BURST_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC0_RXDATA_BURST_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC0_RXDATA_BURST_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC0_RXDATA_BURST_EN` writer - "]
pub struct SLC0_RXDATA_BURST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_RXDATA_BURST_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `SLC0_RXLINK_AUTO_RET` reader - "]
pub struct SLC0_RXLINK_AUTO_RET_R(crate::FieldReader<bool, bool>);
impl SLC0_RXLINK_AUTO_RET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC0_RXLINK_AUTO_RET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC0_RXLINK_AUTO_RET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC0_RXLINK_AUTO_RET` writer - "]
pub struct SLC0_RXLINK_AUTO_RET_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_RXLINK_AUTO_RET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `SLC0_TXLINK_AUTO_RET` reader - "]
pub struct SLC0_TXLINK_AUTO_RET_R(crate::FieldReader<bool, bool>);
impl SLC0_TXLINK_AUTO_RET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC0_TXLINK_AUTO_RET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC0_TXLINK_AUTO_RET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC0_TXLINK_AUTO_RET` writer - "]
pub struct SLC0_TXLINK_AUTO_RET_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_TXLINK_AUTO_RET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `SLC0_TXDSCR_BURST_EN` reader - "]
pub struct SLC0_TXDSCR_BURST_EN_R(crate::FieldReader<bool, bool>);
impl SLC0_TXDSCR_BURST_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC0_TXDSCR_BURST_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC0_TXDSCR_BURST_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC0_TXDSCR_BURST_EN` writer - "]
pub struct SLC0_TXDSCR_BURST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_TXDSCR_BURST_EN_W<'a> {
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
#[doc = "Field `SLC0_TXDATA_BURST_EN` reader - "]
pub struct SLC0_TXDATA_BURST_EN_R(crate::FieldReader<bool, bool>);
impl SLC0_TXDATA_BURST_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC0_TXDATA_BURST_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC0_TXDATA_BURST_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC0_TXDATA_BURST_EN` writer - "]
pub struct SLC0_TXDATA_BURST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_TXDATA_BURST_EN_W<'a> {
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
#[doc = "Field `SLC0_TOKEN_AUTO_CLR` reader - "]
pub struct SLC0_TOKEN_AUTO_CLR_R(crate::FieldReader<bool, bool>);
impl SLC0_TOKEN_AUTO_CLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC0_TOKEN_AUTO_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC0_TOKEN_AUTO_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC0_TOKEN_AUTO_CLR` writer - "]
pub struct SLC0_TOKEN_AUTO_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_TOKEN_AUTO_CLR_W<'a> {
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
#[doc = "Field `SLC0_TOKEN_SEL` reader - "]
pub struct SLC0_TOKEN_SEL_R(crate::FieldReader<bool, bool>);
impl SLC0_TOKEN_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC0_TOKEN_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC0_TOKEN_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC0_TOKEN_SEL` writer - "]
pub struct SLC0_TOKEN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_TOKEN_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `SLC1_TX_RST` reader - "]
pub struct SLC1_TX_RST_R(crate::FieldReader<bool, bool>);
impl SLC1_TX_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC1_TX_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC1_TX_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC1_TX_RST` writer - "]
pub struct SLC1_TX_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC1_TX_RST_W<'a> {
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
#[doc = "Field `SLC1_RX_RST` reader - "]
pub struct SLC1_RX_RST_R(crate::FieldReader<bool, bool>);
impl SLC1_RX_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC1_RX_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC1_RX_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC1_RX_RST` writer - "]
pub struct SLC1_RX_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC1_RX_RST_W<'a> {
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
#[doc = "Field `SLC0_WR_RETRY_MASK_EN` reader - "]
pub struct SLC0_WR_RETRY_MASK_EN_R(crate::FieldReader<bool, bool>);
impl SLC0_WR_RETRY_MASK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC0_WR_RETRY_MASK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC0_WR_RETRY_MASK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC0_WR_RETRY_MASK_EN` writer - "]
pub struct SLC0_WR_RETRY_MASK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_WR_RETRY_MASK_EN_W<'a> {
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
#[doc = "Field `SLC1_WR_RETRY_MASK_EN` reader - "]
pub struct SLC1_WR_RETRY_MASK_EN_R(crate::FieldReader<bool, bool>);
impl SLC1_WR_RETRY_MASK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC1_WR_RETRY_MASK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC1_WR_RETRY_MASK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC1_WR_RETRY_MASK_EN` writer - "]
pub struct SLC1_WR_RETRY_MASK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC1_WR_RETRY_MASK_EN_W<'a> {
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
#[doc = "Field `SLC1_TX_LOOP_TEST` reader - "]
pub struct SLC1_TX_LOOP_TEST_R(crate::FieldReader<bool, bool>);
impl SLC1_TX_LOOP_TEST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC1_TX_LOOP_TEST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC1_TX_LOOP_TEST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC1_TX_LOOP_TEST` writer - "]
pub struct SLC1_TX_LOOP_TEST_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC1_TX_LOOP_TEST_W<'a> {
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
#[doc = "Field `SLC1_RX_LOOP_TEST` reader - "]
pub struct SLC1_RX_LOOP_TEST_R(crate::FieldReader<bool, bool>);
impl SLC1_RX_LOOP_TEST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC1_RX_LOOP_TEST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC1_RX_LOOP_TEST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC1_RX_LOOP_TEST` writer - "]
pub struct SLC1_RX_LOOP_TEST_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC1_RX_LOOP_TEST_W<'a> {
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
#[doc = "Field `SLC1_RX_AUTO_WRBACK` reader - "]
pub struct SLC1_RX_AUTO_WRBACK_R(crate::FieldReader<bool, bool>);
impl SLC1_RX_AUTO_WRBACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC1_RX_AUTO_WRBACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC1_RX_AUTO_WRBACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC1_RX_AUTO_WRBACK` writer - "]
pub struct SLC1_RX_AUTO_WRBACK_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC1_RX_AUTO_WRBACK_W<'a> {
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
#[doc = "Field `SLC1_RX_NO_RESTART_CLR` reader - "]
pub struct SLC1_RX_NO_RESTART_CLR_R(crate::FieldReader<bool, bool>);
impl SLC1_RX_NO_RESTART_CLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC1_RX_NO_RESTART_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC1_RX_NO_RESTART_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC1_RX_NO_RESTART_CLR` writer - "]
pub struct SLC1_RX_NO_RESTART_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC1_RX_NO_RESTART_CLR_W<'a> {
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
#[doc = "Field `SLC1_RXDSCR_BURST_EN` reader - "]
pub struct SLC1_RXDSCR_BURST_EN_R(crate::FieldReader<bool, bool>);
impl SLC1_RXDSCR_BURST_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC1_RXDSCR_BURST_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC1_RXDSCR_BURST_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC1_RXDSCR_BURST_EN` writer - "]
pub struct SLC1_RXDSCR_BURST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC1_RXDSCR_BURST_EN_W<'a> {
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
#[doc = "Field `SLC1_RXDATA_BURST_EN` reader - "]
pub struct SLC1_RXDATA_BURST_EN_R(crate::FieldReader<bool, bool>);
impl SLC1_RXDATA_BURST_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC1_RXDATA_BURST_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC1_RXDATA_BURST_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC1_RXDATA_BURST_EN` writer - "]
pub struct SLC1_RXDATA_BURST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC1_RXDATA_BURST_EN_W<'a> {
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
#[doc = "Field `SLC1_RXLINK_AUTO_RET` reader - "]
pub struct SLC1_RXLINK_AUTO_RET_R(crate::FieldReader<bool, bool>);
impl SLC1_RXLINK_AUTO_RET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC1_RXLINK_AUTO_RET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC1_RXLINK_AUTO_RET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC1_RXLINK_AUTO_RET` writer - "]
pub struct SLC1_RXLINK_AUTO_RET_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC1_RXLINK_AUTO_RET_W<'a> {
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
#[doc = "Field `SLC1_TXLINK_AUTO_RET` reader - "]
pub struct SLC1_TXLINK_AUTO_RET_R(crate::FieldReader<bool, bool>);
impl SLC1_TXLINK_AUTO_RET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC1_TXLINK_AUTO_RET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC1_TXLINK_AUTO_RET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC1_TXLINK_AUTO_RET` writer - "]
pub struct SLC1_TXLINK_AUTO_RET_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC1_TXLINK_AUTO_RET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `SLC1_TXDSCR_BURST_EN` reader - "]
pub struct SLC1_TXDSCR_BURST_EN_R(crate::FieldReader<bool, bool>);
impl SLC1_TXDSCR_BURST_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC1_TXDSCR_BURST_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC1_TXDSCR_BURST_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC1_TXDSCR_BURST_EN` writer - "]
pub struct SLC1_TXDSCR_BURST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC1_TXDSCR_BURST_EN_W<'a> {
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
#[doc = "Field `SLC1_TXDATA_BURST_EN` reader - "]
pub struct SLC1_TXDATA_BURST_EN_R(crate::FieldReader<bool, bool>);
impl SLC1_TXDATA_BURST_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC1_TXDATA_BURST_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC1_TXDATA_BURST_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC1_TXDATA_BURST_EN` writer - "]
pub struct SLC1_TXDATA_BURST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC1_TXDATA_BURST_EN_W<'a> {
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
#[doc = "Field `SLC1_TOKEN_AUTO_CLR` reader - "]
pub struct SLC1_TOKEN_AUTO_CLR_R(crate::FieldReader<bool, bool>);
impl SLC1_TOKEN_AUTO_CLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC1_TOKEN_AUTO_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC1_TOKEN_AUTO_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC1_TOKEN_AUTO_CLR` writer - "]
pub struct SLC1_TOKEN_AUTO_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC1_TOKEN_AUTO_CLR_W<'a> {
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
#[doc = "Field `SLC1_TOKEN_SEL` reader - "]
pub struct SLC1_TOKEN_SEL_R(crate::FieldReader<bool, bool>);
impl SLC1_TOKEN_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC1_TOKEN_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC1_TOKEN_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC1_TOKEN_SEL` writer - "]
pub struct SLC1_TOKEN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC1_TOKEN_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn slc0_tx_rst(&self) -> SLC0_TX_RST_R {
        SLC0_TX_RST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn slc0_rx_rst(&self) -> SLC0_RX_RST_R {
        SLC0_RX_RST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ahbm_fifo_rst(&self) -> AHBM_FIFO_RST_R {
        AHBM_FIFO_RST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ahbm_rst(&self) -> AHBM_RST_R {
        AHBM_RST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn slc0_tx_loop_test(&self) -> SLC0_TX_LOOP_TEST_R {
        SLC0_TX_LOOP_TEST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn slc0_rx_loop_test(&self) -> SLC0_RX_LOOP_TEST_R {
        SLC0_RX_LOOP_TEST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn slc0_rx_auto_wrback(&self) -> SLC0_RX_AUTO_WRBACK_R {
        SLC0_RX_AUTO_WRBACK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn slc0_rx_no_restart_clr(&self) -> SLC0_RX_NO_RESTART_CLR_R {
        SLC0_RX_NO_RESTART_CLR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn slc0_rxdscr_burst_en(&self) -> SLC0_RXDSCR_BURST_EN_R {
        SLC0_RXDSCR_BURST_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn slc0_rxdata_burst_en(&self) -> SLC0_RXDATA_BURST_EN_R {
        SLC0_RXDATA_BURST_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn slc0_rxlink_auto_ret(&self) -> SLC0_RXLINK_AUTO_RET_R {
        SLC0_RXLINK_AUTO_RET_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn slc0_txlink_auto_ret(&self) -> SLC0_TXLINK_AUTO_RET_R {
        SLC0_TXLINK_AUTO_RET_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn slc0_txdscr_burst_en(&self) -> SLC0_TXDSCR_BURST_EN_R {
        SLC0_TXDSCR_BURST_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn slc0_txdata_burst_en(&self) -> SLC0_TXDATA_BURST_EN_R {
        SLC0_TXDATA_BURST_EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn slc0_token_auto_clr(&self) -> SLC0_TOKEN_AUTO_CLR_R {
        SLC0_TOKEN_AUTO_CLR_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn slc0_token_sel(&self) -> SLC0_TOKEN_SEL_R {
        SLC0_TOKEN_SEL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slc1_tx_rst(&self) -> SLC1_TX_RST_R {
        SLC1_TX_RST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn slc1_rx_rst(&self) -> SLC1_RX_RST_R {
        SLC1_RX_RST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn slc0_wr_retry_mask_en(&self) -> SLC0_WR_RETRY_MASK_EN_R {
        SLC0_WR_RETRY_MASK_EN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn slc1_wr_retry_mask_en(&self) -> SLC1_WR_RETRY_MASK_EN_R {
        SLC1_WR_RETRY_MASK_EN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn slc1_tx_loop_test(&self) -> SLC1_TX_LOOP_TEST_R {
        SLC1_TX_LOOP_TEST_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn slc1_rx_loop_test(&self) -> SLC1_RX_LOOP_TEST_R {
        SLC1_RX_LOOP_TEST_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn slc1_rx_auto_wrback(&self) -> SLC1_RX_AUTO_WRBACK_R {
        SLC1_RX_AUTO_WRBACK_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn slc1_rx_no_restart_clr(&self) -> SLC1_RX_NO_RESTART_CLR_R {
        SLC1_RX_NO_RESTART_CLR_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn slc1_rxdscr_burst_en(&self) -> SLC1_RXDSCR_BURST_EN_R {
        SLC1_RXDSCR_BURST_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn slc1_rxdata_burst_en(&self) -> SLC1_RXDATA_BURST_EN_R {
        SLC1_RXDATA_BURST_EN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn slc1_rxlink_auto_ret(&self) -> SLC1_RXLINK_AUTO_RET_R {
        SLC1_RXLINK_AUTO_RET_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn slc1_txlink_auto_ret(&self) -> SLC1_TXLINK_AUTO_RET_R {
        SLC1_TXLINK_AUTO_RET_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn slc1_txdscr_burst_en(&self) -> SLC1_TXDSCR_BURST_EN_R {
        SLC1_TXDSCR_BURST_EN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn slc1_txdata_burst_en(&self) -> SLC1_TXDATA_BURST_EN_R {
        SLC1_TXDATA_BURST_EN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn slc1_token_auto_clr(&self) -> SLC1_TOKEN_AUTO_CLR_R {
        SLC1_TOKEN_AUTO_CLR_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn slc1_token_sel(&self) -> SLC1_TOKEN_SEL_R {
        SLC1_TOKEN_SEL_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn slc0_tx_rst(&mut self) -> SLC0_TX_RST_W {
        SLC0_TX_RST_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn slc0_rx_rst(&mut self) -> SLC0_RX_RST_W {
        SLC0_RX_RST_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ahbm_fifo_rst(&mut self) -> AHBM_FIFO_RST_W {
        AHBM_FIFO_RST_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ahbm_rst(&mut self) -> AHBM_RST_W {
        AHBM_RST_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn slc0_tx_loop_test(&mut self) -> SLC0_TX_LOOP_TEST_W {
        SLC0_TX_LOOP_TEST_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn slc0_rx_loop_test(&mut self) -> SLC0_RX_LOOP_TEST_W {
        SLC0_RX_LOOP_TEST_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn slc0_rx_auto_wrback(&mut self) -> SLC0_RX_AUTO_WRBACK_W {
        SLC0_RX_AUTO_WRBACK_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn slc0_rx_no_restart_clr(&mut self) -> SLC0_RX_NO_RESTART_CLR_W {
        SLC0_RX_NO_RESTART_CLR_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn slc0_rxdscr_burst_en(&mut self) -> SLC0_RXDSCR_BURST_EN_W {
        SLC0_RXDSCR_BURST_EN_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn slc0_rxdata_burst_en(&mut self) -> SLC0_RXDATA_BURST_EN_W {
        SLC0_RXDATA_BURST_EN_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn slc0_rxlink_auto_ret(&mut self) -> SLC0_RXLINK_AUTO_RET_W {
        SLC0_RXLINK_AUTO_RET_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn slc0_txlink_auto_ret(&mut self) -> SLC0_TXLINK_AUTO_RET_W {
        SLC0_TXLINK_AUTO_RET_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn slc0_txdscr_burst_en(&mut self) -> SLC0_TXDSCR_BURST_EN_W {
        SLC0_TXDSCR_BURST_EN_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn slc0_txdata_burst_en(&mut self) -> SLC0_TXDATA_BURST_EN_W {
        SLC0_TXDATA_BURST_EN_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn slc0_token_auto_clr(&mut self) -> SLC0_TOKEN_AUTO_CLR_W {
        SLC0_TOKEN_AUTO_CLR_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn slc0_token_sel(&mut self) -> SLC0_TOKEN_SEL_W {
        SLC0_TOKEN_SEL_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slc1_tx_rst(&mut self) -> SLC1_TX_RST_W {
        SLC1_TX_RST_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn slc1_rx_rst(&mut self) -> SLC1_RX_RST_W {
        SLC1_RX_RST_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn slc0_wr_retry_mask_en(&mut self) -> SLC0_WR_RETRY_MASK_EN_W {
        SLC0_WR_RETRY_MASK_EN_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn slc1_wr_retry_mask_en(&mut self) -> SLC1_WR_RETRY_MASK_EN_W {
        SLC1_WR_RETRY_MASK_EN_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn slc1_tx_loop_test(&mut self) -> SLC1_TX_LOOP_TEST_W {
        SLC1_TX_LOOP_TEST_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn slc1_rx_loop_test(&mut self) -> SLC1_RX_LOOP_TEST_W {
        SLC1_RX_LOOP_TEST_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn slc1_rx_auto_wrback(&mut self) -> SLC1_RX_AUTO_WRBACK_W {
        SLC1_RX_AUTO_WRBACK_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn slc1_rx_no_restart_clr(&mut self) -> SLC1_RX_NO_RESTART_CLR_W {
        SLC1_RX_NO_RESTART_CLR_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn slc1_rxdscr_burst_en(&mut self) -> SLC1_RXDSCR_BURST_EN_W {
        SLC1_RXDSCR_BURST_EN_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn slc1_rxdata_burst_en(&mut self) -> SLC1_RXDATA_BURST_EN_W {
        SLC1_RXDATA_BURST_EN_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn slc1_rxlink_auto_ret(&mut self) -> SLC1_RXLINK_AUTO_RET_W {
        SLC1_RXLINK_AUTO_RET_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn slc1_txlink_auto_ret(&mut self) -> SLC1_TXLINK_AUTO_RET_W {
        SLC1_TXLINK_AUTO_RET_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn slc1_txdscr_burst_en(&mut self) -> SLC1_TXDSCR_BURST_EN_W {
        SLC1_TXDSCR_BURST_EN_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn slc1_txdata_burst_en(&mut self) -> SLC1_TXDATA_BURST_EN_W {
        SLC1_TXDATA_BURST_EN_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn slc1_token_auto_clr(&mut self) -> SLC1_TOKEN_AUTO_CLR_W {
        SLC1_TOKEN_AUTO_CLR_W { w: self }
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn slc1_token_sel(&mut self) -> SLC1_TOKEN_SEL_W {
        SLC1_TOKEN_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conf0](index.html) module"]
pub struct CONF0_SPEC;
impl crate::RegisterSpec for CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conf0::R](R) reader structure"]
impl crate::Readable for CONF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conf0::W](W) writer structure"]
impl crate::Writable for CONF0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONF0 to value 0xff3c_ff30"]
impl crate::Resettable for CONF0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xff3c_ff30
    }
}
