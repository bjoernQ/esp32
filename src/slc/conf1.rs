#[doc = "Register `CONF1` reader"]
pub struct R(crate::R<CONF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONF1` writer"]
pub struct W(crate::W<CONF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONF1_SPEC>;
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
impl From<crate::W<CONF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLC0_CHECK_OWNER` reader - "]
pub struct SLC0_CHECK_OWNER_R(crate::FieldReader<bool, bool>);
impl SLC0_CHECK_OWNER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC0_CHECK_OWNER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC0_CHECK_OWNER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC0_CHECK_OWNER` writer - "]
pub struct SLC0_CHECK_OWNER_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_CHECK_OWNER_W<'a> {
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
#[doc = "Field `SLC0_TX_CHECK_SUM_EN` reader - "]
pub struct SLC0_TX_CHECK_SUM_EN_R(crate::FieldReader<bool, bool>);
impl SLC0_TX_CHECK_SUM_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC0_TX_CHECK_SUM_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC0_TX_CHECK_SUM_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC0_TX_CHECK_SUM_EN` writer - "]
pub struct SLC0_TX_CHECK_SUM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_TX_CHECK_SUM_EN_W<'a> {
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
#[doc = "Field `SLC0_RX_CHECK_SUM_EN` reader - "]
pub struct SLC0_RX_CHECK_SUM_EN_R(crate::FieldReader<bool, bool>);
impl SLC0_RX_CHECK_SUM_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC0_RX_CHECK_SUM_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC0_RX_CHECK_SUM_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC0_RX_CHECK_SUM_EN` writer - "]
pub struct SLC0_RX_CHECK_SUM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_RX_CHECK_SUM_EN_W<'a> {
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
#[doc = "Field `CMD_HOLD_EN` reader - "]
pub struct CMD_HOLD_EN_R(crate::FieldReader<bool, bool>);
impl CMD_HOLD_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMD_HOLD_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMD_HOLD_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMD_HOLD_EN` writer - "]
pub struct CMD_HOLD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_HOLD_EN_W<'a> {
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
#[doc = "Field `SLC0_LEN_AUTO_CLR` reader - "]
pub struct SLC0_LEN_AUTO_CLR_R(crate::FieldReader<bool, bool>);
impl SLC0_LEN_AUTO_CLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC0_LEN_AUTO_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC0_LEN_AUTO_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC0_LEN_AUTO_CLR` writer - "]
pub struct SLC0_LEN_AUTO_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_LEN_AUTO_CLR_W<'a> {
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
#[doc = "Field `SLC0_TX_STITCH_EN` reader - "]
pub struct SLC0_TX_STITCH_EN_R(crate::FieldReader<bool, bool>);
impl SLC0_TX_STITCH_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC0_TX_STITCH_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC0_TX_STITCH_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC0_TX_STITCH_EN` writer - "]
pub struct SLC0_TX_STITCH_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_TX_STITCH_EN_W<'a> {
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
#[doc = "Field `SLC0_RX_STITCH_EN` reader - "]
pub struct SLC0_RX_STITCH_EN_R(crate::FieldReader<bool, bool>);
impl SLC0_RX_STITCH_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC0_RX_STITCH_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC0_RX_STITCH_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC0_RX_STITCH_EN` writer - "]
pub struct SLC0_RX_STITCH_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_RX_STITCH_EN_W<'a> {
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
#[doc = "Field `SLC1_CHECK_OWNER` reader - "]
pub struct SLC1_CHECK_OWNER_R(crate::FieldReader<bool, bool>);
impl SLC1_CHECK_OWNER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC1_CHECK_OWNER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC1_CHECK_OWNER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC1_CHECK_OWNER` writer - "]
pub struct SLC1_CHECK_OWNER_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC1_CHECK_OWNER_W<'a> {
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
#[doc = "Field `SLC1_TX_CHECK_SUM_EN` reader - "]
pub struct SLC1_TX_CHECK_SUM_EN_R(crate::FieldReader<bool, bool>);
impl SLC1_TX_CHECK_SUM_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC1_TX_CHECK_SUM_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC1_TX_CHECK_SUM_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC1_TX_CHECK_SUM_EN` writer - "]
pub struct SLC1_TX_CHECK_SUM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC1_TX_CHECK_SUM_EN_W<'a> {
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
#[doc = "Field `SLC1_RX_CHECK_SUM_EN` reader - "]
pub struct SLC1_RX_CHECK_SUM_EN_R(crate::FieldReader<bool, bool>);
impl SLC1_RX_CHECK_SUM_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC1_RX_CHECK_SUM_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC1_RX_CHECK_SUM_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC1_RX_CHECK_SUM_EN` writer - "]
pub struct SLC1_RX_CHECK_SUM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC1_RX_CHECK_SUM_EN_W<'a> {
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
#[doc = "Field `HOST_INT_LEVEL_SEL` reader - "]
pub struct HOST_INT_LEVEL_SEL_R(crate::FieldReader<bool, bool>);
impl HOST_INT_LEVEL_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HOST_INT_LEVEL_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOST_INT_LEVEL_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOST_INT_LEVEL_SEL` writer - "]
pub struct HOST_INT_LEVEL_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_INT_LEVEL_SEL_W<'a> {
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
#[doc = "Field `SLC1_TX_STITCH_EN` reader - "]
pub struct SLC1_TX_STITCH_EN_R(crate::FieldReader<bool, bool>);
impl SLC1_TX_STITCH_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC1_TX_STITCH_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC1_TX_STITCH_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC1_TX_STITCH_EN` writer - "]
pub struct SLC1_TX_STITCH_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC1_TX_STITCH_EN_W<'a> {
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
#[doc = "Field `SLC1_RX_STITCH_EN` reader - "]
pub struct SLC1_RX_STITCH_EN_R(crate::FieldReader<bool, bool>);
impl SLC1_RX_STITCH_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC1_RX_STITCH_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC1_RX_STITCH_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC1_RX_STITCH_EN` writer - "]
pub struct SLC1_RX_STITCH_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC1_RX_STITCH_EN_W<'a> {
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
#[doc = "Field `CLK_EN` reader - "]
pub struct CLK_EN_R(crate::FieldReader<bool, bool>);
impl CLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK_EN` writer - "]
pub struct CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_EN_W<'a> {
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
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn slc0_check_owner(&self) -> SLC0_CHECK_OWNER_R {
        SLC0_CHECK_OWNER_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn slc0_tx_check_sum_en(&self) -> SLC0_TX_CHECK_SUM_EN_R {
        SLC0_TX_CHECK_SUM_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn slc0_rx_check_sum_en(&self) -> SLC0_RX_CHECK_SUM_EN_R {
        SLC0_RX_CHECK_SUM_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cmd_hold_en(&self) -> CMD_HOLD_EN_R {
        CMD_HOLD_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn slc0_len_auto_clr(&self) -> SLC0_LEN_AUTO_CLR_R {
        SLC0_LEN_AUTO_CLR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn slc0_tx_stitch_en(&self) -> SLC0_TX_STITCH_EN_R {
        SLC0_TX_STITCH_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn slc0_rx_stitch_en(&self) -> SLC0_RX_STITCH_EN_R {
        SLC0_RX_STITCH_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slc1_check_owner(&self) -> SLC1_CHECK_OWNER_R {
        SLC1_CHECK_OWNER_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn slc1_tx_check_sum_en(&self) -> SLC1_TX_CHECK_SUM_EN_R {
        SLC1_TX_CHECK_SUM_EN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn slc1_rx_check_sum_en(&self) -> SLC1_RX_CHECK_SUM_EN_R {
        SLC1_RX_CHECK_SUM_EN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn host_int_level_sel(&self) -> HOST_INT_LEVEL_SEL_R {
        HOST_INT_LEVEL_SEL_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn slc1_tx_stitch_en(&self) -> SLC1_TX_STITCH_EN_R {
        SLC1_TX_STITCH_EN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn slc1_rx_stitch_en(&self) -> SLC1_RX_STITCH_EN_R {
        SLC1_RX_STITCH_EN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn slc0_check_owner(&mut self) -> SLC0_CHECK_OWNER_W {
        SLC0_CHECK_OWNER_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn slc0_tx_check_sum_en(&mut self) -> SLC0_TX_CHECK_SUM_EN_W {
        SLC0_TX_CHECK_SUM_EN_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn slc0_rx_check_sum_en(&mut self) -> SLC0_RX_CHECK_SUM_EN_W {
        SLC0_RX_CHECK_SUM_EN_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cmd_hold_en(&mut self) -> CMD_HOLD_EN_W {
        CMD_HOLD_EN_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn slc0_len_auto_clr(&mut self) -> SLC0_LEN_AUTO_CLR_W {
        SLC0_LEN_AUTO_CLR_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn slc0_tx_stitch_en(&mut self) -> SLC0_TX_STITCH_EN_W {
        SLC0_TX_STITCH_EN_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn slc0_rx_stitch_en(&mut self) -> SLC0_RX_STITCH_EN_W {
        SLC0_RX_STITCH_EN_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slc1_check_owner(&mut self) -> SLC1_CHECK_OWNER_W {
        SLC1_CHECK_OWNER_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn slc1_tx_check_sum_en(&mut self) -> SLC1_TX_CHECK_SUM_EN_W {
        SLC1_TX_CHECK_SUM_EN_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn slc1_rx_check_sum_en(&mut self) -> SLC1_RX_CHECK_SUM_EN_W {
        SLC1_RX_CHECK_SUM_EN_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn host_int_level_sel(&mut self) -> HOST_INT_LEVEL_SEL_W {
        HOST_INT_LEVEL_SEL_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn slc1_tx_stitch_en(&mut self) -> SLC1_TX_STITCH_EN_W {
        SLC1_TX_STITCH_EN_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn slc1_rx_stitch_en(&mut self) -> SLC1_RX_STITCH_EN_W {
        SLC1_RX_STITCH_EN_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W {
        CLK_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conf1](index.html) module"]
pub struct CONF1_SPEC;
impl crate::RegisterSpec for CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conf1::R](R) reader structure"]
impl crate::Readable for CONF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conf1::W](W) writer structure"]
impl crate::Writable for CONF1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONF1 to value 0x0030_0078"]
impl crate::Resettable for CONF1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0030_0078
    }
}
