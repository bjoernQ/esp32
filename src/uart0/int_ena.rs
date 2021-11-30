#[doc = "Register `INT_ENA` reader"]
pub struct R(crate::R<INT_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_ENA` writer"]
pub struct W(crate::W<INT_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_ENA_SPEC>;
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
impl From<crate::W<INT_ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXFIFO_FULL_INT_ENA` reader - This is the enable bit for rxfifo_full_int_st register."]
pub struct RXFIFO_FULL_INT_ENA_R(crate::FieldReader<bool, bool>);
impl RXFIFO_FULL_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXFIFO_FULL_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFIFO_FULL_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFIFO_FULL_INT_ENA` writer - This is the enable bit for rxfifo_full_int_st register."]
pub struct RXFIFO_FULL_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFO_FULL_INT_ENA_W<'a> {
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
#[doc = "Field `TXFIFO_EMPTY_INT_ENA` reader - This is the enable bit for rxfifo_full_int_st register."]
pub struct TXFIFO_EMPTY_INT_ENA_R(crate::FieldReader<bool, bool>);
impl TXFIFO_EMPTY_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXFIFO_EMPTY_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFIFO_EMPTY_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFIFO_EMPTY_INT_ENA` writer - This is the enable bit for rxfifo_full_int_st register."]
pub struct TXFIFO_EMPTY_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFIFO_EMPTY_INT_ENA_W<'a> {
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
#[doc = "Field `PARITY_ERR_INT_ENA` reader - This is the enable bit for parity_err_int_st register."]
pub struct PARITY_ERR_INT_ENA_R(crate::FieldReader<bool, bool>);
impl PARITY_ERR_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PARITY_ERR_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PARITY_ERR_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PARITY_ERR_INT_ENA` writer - This is the enable bit for parity_err_int_st register."]
pub struct PARITY_ERR_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> PARITY_ERR_INT_ENA_W<'a> {
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
#[doc = "Field `FRM_ERR_INT_ENA` reader - This is the enable bit for frm_err_int_st register."]
pub struct FRM_ERR_INT_ENA_R(crate::FieldReader<bool, bool>);
impl FRM_ERR_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FRM_ERR_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRM_ERR_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRM_ERR_INT_ENA` writer - This is the enable bit for frm_err_int_st register."]
pub struct FRM_ERR_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> FRM_ERR_INT_ENA_W<'a> {
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
#[doc = "Field `RXFIFO_OVF_INT_ENA` reader - This is the enable bit for rxfifo_ovf_int_st register."]
pub struct RXFIFO_OVF_INT_ENA_R(crate::FieldReader<bool, bool>);
impl RXFIFO_OVF_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXFIFO_OVF_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFIFO_OVF_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFIFO_OVF_INT_ENA` writer - This is the enable bit for rxfifo_ovf_int_st register."]
pub struct RXFIFO_OVF_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFO_OVF_INT_ENA_W<'a> {
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
#[doc = "Field `DSR_CHG_INT_ENA` reader - This is the enable bit for dsr_chg_int_st register."]
pub struct DSR_CHG_INT_ENA_R(crate::FieldReader<bool, bool>);
impl DSR_CHG_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DSR_CHG_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSR_CHG_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSR_CHG_INT_ENA` writer - This is the enable bit for dsr_chg_int_st register."]
pub struct DSR_CHG_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DSR_CHG_INT_ENA_W<'a> {
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
#[doc = "Field `CTS_CHG_INT_ENA` reader - This is the enable bit for cts_chg_int_st register."]
pub struct CTS_CHG_INT_ENA_R(crate::FieldReader<bool, bool>);
impl CTS_CHG_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CTS_CHG_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTS_CHG_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTS_CHG_INT_ENA` writer - This is the enable bit for cts_chg_int_st register."]
pub struct CTS_CHG_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CTS_CHG_INT_ENA_W<'a> {
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
#[doc = "Field `BRK_DET_INT_ENA` reader - This is the enable bit for brk_det_int_st register."]
pub struct BRK_DET_INT_ENA_R(crate::FieldReader<bool, bool>);
impl BRK_DET_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BRK_DET_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BRK_DET_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRK_DET_INT_ENA` writer - This is the enable bit for brk_det_int_st register."]
pub struct BRK_DET_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> BRK_DET_INT_ENA_W<'a> {
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
#[doc = "Field `RXFIFO_TOUT_INT_ENA` reader - This is the enable bit for rxfifo_tout_int_st register."]
pub struct RXFIFO_TOUT_INT_ENA_R(crate::FieldReader<bool, bool>);
impl RXFIFO_TOUT_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXFIFO_TOUT_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFIFO_TOUT_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFIFO_TOUT_INT_ENA` writer - This is the enable bit for rxfifo_tout_int_st register."]
pub struct RXFIFO_TOUT_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFO_TOUT_INT_ENA_W<'a> {
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
#[doc = "Field `SW_XON_INT_ENA` reader - This is the enable bit for sw_xon_int_st register."]
pub struct SW_XON_INT_ENA_R(crate::FieldReader<bool, bool>);
impl SW_XON_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SW_XON_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_XON_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_XON_INT_ENA` writer - This is the enable bit for sw_xon_int_st register."]
pub struct SW_XON_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_XON_INT_ENA_W<'a> {
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
#[doc = "Field `SW_XOFF_INT_ENA` reader - This is the enable bit for sw_xoff_int_st register."]
pub struct SW_XOFF_INT_ENA_R(crate::FieldReader<bool, bool>);
impl SW_XOFF_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SW_XOFF_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_XOFF_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_XOFF_INT_ENA` writer - This is the enable bit for sw_xoff_int_st register."]
pub struct SW_XOFF_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_XOFF_INT_ENA_W<'a> {
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
#[doc = "Field `GLITCH_DET_INT_ENA` reader - This is the enable bit for glitch_det_int_st register."]
pub struct GLITCH_DET_INT_ENA_R(crate::FieldReader<bool, bool>);
impl GLITCH_DET_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GLITCH_DET_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GLITCH_DET_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GLITCH_DET_INT_ENA` writer - This is the enable bit for glitch_det_int_st register."]
pub struct GLITCH_DET_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> GLITCH_DET_INT_ENA_W<'a> {
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
#[doc = "Field `TX_BRK_DONE_INT_ENA` reader - This is the enable bit for tx_brk_done_int_st register."]
pub struct TX_BRK_DONE_INT_ENA_R(crate::FieldReader<bool, bool>);
impl TX_BRK_DONE_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_BRK_DONE_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_BRK_DONE_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_BRK_DONE_INT_ENA` writer - This is the enable bit for tx_brk_done_int_st register."]
pub struct TX_BRK_DONE_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_BRK_DONE_INT_ENA_W<'a> {
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
#[doc = "Field `TX_BRK_IDLE_DONE_INT_ENA` reader - This is the enable bit for tx_brk_idle_done_int_st register."]
pub struct TX_BRK_IDLE_DONE_INT_ENA_R(crate::FieldReader<bool, bool>);
impl TX_BRK_IDLE_DONE_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_BRK_IDLE_DONE_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_BRK_IDLE_DONE_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_BRK_IDLE_DONE_INT_ENA` writer - This is the enable bit for tx_brk_idle_done_int_st register."]
pub struct TX_BRK_IDLE_DONE_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_BRK_IDLE_DONE_INT_ENA_W<'a> {
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
#[doc = "Field `TX_DONE_INT_ENA` reader - This is the enable bit for tx_done_int_st register."]
pub struct TX_DONE_INT_ENA_R(crate::FieldReader<bool, bool>);
impl TX_DONE_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_DONE_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_DONE_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_DONE_INT_ENA` writer - This is the enable bit for tx_done_int_st register."]
pub struct TX_DONE_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DONE_INT_ENA_W<'a> {
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
#[doc = "Field `RS485_PARITY_ERR_INT_ENA` reader - This is the enable bit for rs485_parity_err_int_st register."]
pub struct RS485_PARITY_ERR_INT_ENA_R(crate::FieldReader<bool, bool>);
impl RS485_PARITY_ERR_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RS485_PARITY_ERR_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RS485_PARITY_ERR_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RS485_PARITY_ERR_INT_ENA` writer - This is the enable bit for rs485_parity_err_int_st register."]
pub struct RS485_PARITY_ERR_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> RS485_PARITY_ERR_INT_ENA_W<'a> {
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
#[doc = "Field `RS485_FRM_ERR_INT_ENA` reader - This is the enable bit for rs485_parity_err_int_st register."]
pub struct RS485_FRM_ERR_INT_ENA_R(crate::FieldReader<bool, bool>);
impl RS485_FRM_ERR_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RS485_FRM_ERR_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RS485_FRM_ERR_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RS485_FRM_ERR_INT_ENA` writer - This is the enable bit for rs485_parity_err_int_st register."]
pub struct RS485_FRM_ERR_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> RS485_FRM_ERR_INT_ENA_W<'a> {
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
#[doc = "Field `RS485_CLASH_INT_ENA` reader - This is the enable bit for rs485_clash_int_st register."]
pub struct RS485_CLASH_INT_ENA_R(crate::FieldReader<bool, bool>);
impl RS485_CLASH_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RS485_CLASH_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RS485_CLASH_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RS485_CLASH_INT_ENA` writer - This is the enable bit for rs485_clash_int_st register."]
pub struct RS485_CLASH_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> RS485_CLASH_INT_ENA_W<'a> {
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
#[doc = "Field `AT_CMD_CHAR_DET_INT_ENA` reader - This is the enable bit for at_cmd_char_det_int_st register."]
pub struct AT_CMD_CHAR_DET_INT_ENA_R(crate::FieldReader<bool, bool>);
impl AT_CMD_CHAR_DET_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AT_CMD_CHAR_DET_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AT_CMD_CHAR_DET_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AT_CMD_CHAR_DET_INT_ENA` writer - This is the enable bit for at_cmd_char_det_int_st register."]
pub struct AT_CMD_CHAR_DET_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> AT_CMD_CHAR_DET_INT_ENA_W<'a> {
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
impl R {
    #[doc = "Bit 0 - This is the enable bit for rxfifo_full_int_st register."]
    #[inline(always)]
    pub fn rxfifo_full_int_ena(&self) -> RXFIFO_FULL_INT_ENA_R {
        RXFIFO_FULL_INT_ENA_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - This is the enable bit for rxfifo_full_int_st register."]
    #[inline(always)]
    pub fn txfifo_empty_int_ena(&self) -> TXFIFO_EMPTY_INT_ENA_R {
        TXFIFO_EMPTY_INT_ENA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - This is the enable bit for parity_err_int_st register."]
    #[inline(always)]
    pub fn parity_err_int_ena(&self) -> PARITY_ERR_INT_ENA_R {
        PARITY_ERR_INT_ENA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - This is the enable bit for frm_err_int_st register."]
    #[inline(always)]
    pub fn frm_err_int_ena(&self) -> FRM_ERR_INT_ENA_R {
        FRM_ERR_INT_ENA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - This is the enable bit for rxfifo_ovf_int_st register."]
    #[inline(always)]
    pub fn rxfifo_ovf_int_ena(&self) -> RXFIFO_OVF_INT_ENA_R {
        RXFIFO_OVF_INT_ENA_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - This is the enable bit for dsr_chg_int_st register."]
    #[inline(always)]
    pub fn dsr_chg_int_ena(&self) -> DSR_CHG_INT_ENA_R {
        DSR_CHG_INT_ENA_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - This is the enable bit for cts_chg_int_st register."]
    #[inline(always)]
    pub fn cts_chg_int_ena(&self) -> CTS_CHG_INT_ENA_R {
        CTS_CHG_INT_ENA_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - This is the enable bit for brk_det_int_st register."]
    #[inline(always)]
    pub fn brk_det_int_ena(&self) -> BRK_DET_INT_ENA_R {
        BRK_DET_INT_ENA_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - This is the enable bit for rxfifo_tout_int_st register."]
    #[inline(always)]
    pub fn rxfifo_tout_int_ena(&self) -> RXFIFO_TOUT_INT_ENA_R {
        RXFIFO_TOUT_INT_ENA_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - This is the enable bit for sw_xon_int_st register."]
    #[inline(always)]
    pub fn sw_xon_int_ena(&self) -> SW_XON_INT_ENA_R {
        SW_XON_INT_ENA_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - This is the enable bit for sw_xoff_int_st register."]
    #[inline(always)]
    pub fn sw_xoff_int_ena(&self) -> SW_XOFF_INT_ENA_R {
        SW_XOFF_INT_ENA_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - This is the enable bit for glitch_det_int_st register."]
    #[inline(always)]
    pub fn glitch_det_int_ena(&self) -> GLITCH_DET_INT_ENA_R {
        GLITCH_DET_INT_ENA_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - This is the enable bit for tx_brk_done_int_st register."]
    #[inline(always)]
    pub fn tx_brk_done_int_ena(&self) -> TX_BRK_DONE_INT_ENA_R {
        TX_BRK_DONE_INT_ENA_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - This is the enable bit for tx_brk_idle_done_int_st register."]
    #[inline(always)]
    pub fn tx_brk_idle_done_int_ena(&self) -> TX_BRK_IDLE_DONE_INT_ENA_R {
        TX_BRK_IDLE_DONE_INT_ENA_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - This is the enable bit for tx_done_int_st register."]
    #[inline(always)]
    pub fn tx_done_int_ena(&self) -> TX_DONE_INT_ENA_R {
        TX_DONE_INT_ENA_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - This is the enable bit for rs485_parity_err_int_st register."]
    #[inline(always)]
    pub fn rs485_parity_err_int_ena(&self) -> RS485_PARITY_ERR_INT_ENA_R {
        RS485_PARITY_ERR_INT_ENA_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - This is the enable bit for rs485_parity_err_int_st register."]
    #[inline(always)]
    pub fn rs485_frm_err_int_ena(&self) -> RS485_FRM_ERR_INT_ENA_R {
        RS485_FRM_ERR_INT_ENA_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - This is the enable bit for rs485_clash_int_st register."]
    #[inline(always)]
    pub fn rs485_clash_int_ena(&self) -> RS485_CLASH_INT_ENA_R {
        RS485_CLASH_INT_ENA_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - This is the enable bit for at_cmd_char_det_int_st register."]
    #[inline(always)]
    pub fn at_cmd_char_det_int_ena(&self) -> AT_CMD_CHAR_DET_INT_ENA_R {
        AT_CMD_CHAR_DET_INT_ENA_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This is the enable bit for rxfifo_full_int_st register."]
    #[inline(always)]
    pub fn rxfifo_full_int_ena(&mut self) -> RXFIFO_FULL_INT_ENA_W {
        RXFIFO_FULL_INT_ENA_W { w: self }
    }
    #[doc = "Bit 1 - This is the enable bit for rxfifo_full_int_st register."]
    #[inline(always)]
    pub fn txfifo_empty_int_ena(&mut self) -> TXFIFO_EMPTY_INT_ENA_W {
        TXFIFO_EMPTY_INT_ENA_W { w: self }
    }
    #[doc = "Bit 2 - This is the enable bit for parity_err_int_st register."]
    #[inline(always)]
    pub fn parity_err_int_ena(&mut self) -> PARITY_ERR_INT_ENA_W {
        PARITY_ERR_INT_ENA_W { w: self }
    }
    #[doc = "Bit 3 - This is the enable bit for frm_err_int_st register."]
    #[inline(always)]
    pub fn frm_err_int_ena(&mut self) -> FRM_ERR_INT_ENA_W {
        FRM_ERR_INT_ENA_W { w: self }
    }
    #[doc = "Bit 4 - This is the enable bit for rxfifo_ovf_int_st register."]
    #[inline(always)]
    pub fn rxfifo_ovf_int_ena(&mut self) -> RXFIFO_OVF_INT_ENA_W {
        RXFIFO_OVF_INT_ENA_W { w: self }
    }
    #[doc = "Bit 5 - This is the enable bit for dsr_chg_int_st register."]
    #[inline(always)]
    pub fn dsr_chg_int_ena(&mut self) -> DSR_CHG_INT_ENA_W {
        DSR_CHG_INT_ENA_W { w: self }
    }
    #[doc = "Bit 6 - This is the enable bit for cts_chg_int_st register."]
    #[inline(always)]
    pub fn cts_chg_int_ena(&mut self) -> CTS_CHG_INT_ENA_W {
        CTS_CHG_INT_ENA_W { w: self }
    }
    #[doc = "Bit 7 - This is the enable bit for brk_det_int_st register."]
    #[inline(always)]
    pub fn brk_det_int_ena(&mut self) -> BRK_DET_INT_ENA_W {
        BRK_DET_INT_ENA_W { w: self }
    }
    #[doc = "Bit 8 - This is the enable bit for rxfifo_tout_int_st register."]
    #[inline(always)]
    pub fn rxfifo_tout_int_ena(&mut self) -> RXFIFO_TOUT_INT_ENA_W {
        RXFIFO_TOUT_INT_ENA_W { w: self }
    }
    #[doc = "Bit 9 - This is the enable bit for sw_xon_int_st register."]
    #[inline(always)]
    pub fn sw_xon_int_ena(&mut self) -> SW_XON_INT_ENA_W {
        SW_XON_INT_ENA_W { w: self }
    }
    #[doc = "Bit 10 - This is the enable bit for sw_xoff_int_st register."]
    #[inline(always)]
    pub fn sw_xoff_int_ena(&mut self) -> SW_XOFF_INT_ENA_W {
        SW_XOFF_INT_ENA_W { w: self }
    }
    #[doc = "Bit 11 - This is the enable bit for glitch_det_int_st register."]
    #[inline(always)]
    pub fn glitch_det_int_ena(&mut self) -> GLITCH_DET_INT_ENA_W {
        GLITCH_DET_INT_ENA_W { w: self }
    }
    #[doc = "Bit 12 - This is the enable bit for tx_brk_done_int_st register."]
    #[inline(always)]
    pub fn tx_brk_done_int_ena(&mut self) -> TX_BRK_DONE_INT_ENA_W {
        TX_BRK_DONE_INT_ENA_W { w: self }
    }
    #[doc = "Bit 13 - This is the enable bit for tx_brk_idle_done_int_st register."]
    #[inline(always)]
    pub fn tx_brk_idle_done_int_ena(&mut self) -> TX_BRK_IDLE_DONE_INT_ENA_W {
        TX_BRK_IDLE_DONE_INT_ENA_W { w: self }
    }
    #[doc = "Bit 14 - This is the enable bit for tx_done_int_st register."]
    #[inline(always)]
    pub fn tx_done_int_ena(&mut self) -> TX_DONE_INT_ENA_W {
        TX_DONE_INT_ENA_W { w: self }
    }
    #[doc = "Bit 15 - This is the enable bit for rs485_parity_err_int_st register."]
    #[inline(always)]
    pub fn rs485_parity_err_int_ena(&mut self) -> RS485_PARITY_ERR_INT_ENA_W {
        RS485_PARITY_ERR_INT_ENA_W { w: self }
    }
    #[doc = "Bit 16 - This is the enable bit for rs485_parity_err_int_st register."]
    #[inline(always)]
    pub fn rs485_frm_err_int_ena(&mut self) -> RS485_FRM_ERR_INT_ENA_W {
        RS485_FRM_ERR_INT_ENA_W { w: self }
    }
    #[doc = "Bit 17 - This is the enable bit for rs485_clash_int_st register."]
    #[inline(always)]
    pub fn rs485_clash_int_ena(&mut self) -> RS485_CLASH_INT_ENA_W {
        RS485_CLASH_INT_ENA_W { w: self }
    }
    #[doc = "Bit 18 - This is the enable bit for at_cmd_char_det_int_st register."]
    #[inline(always)]
    pub fn at_cmd_char_det_int_ena(&mut self) -> AT_CMD_CHAR_DET_INT_ENA_W {
        AT_CMD_CHAR_DET_INT_ENA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_ena](index.html) module"]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_ena::R](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_ena::W](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
