#[doc = "Register `PAD_DAC2` reader"]
pub struct R(crate::R<PAD_DAC2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAD_DAC2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAD_DAC2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAD_DAC2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PAD_DAC2` writer"]
pub struct W(crate::W<PAD_DAC2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAD_DAC2_SPEC>;
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
impl From<crate::W<PAD_DAC2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAD_DAC2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDAC2_DAC_XPD_FORCE` reader - Power on DAC2. Usually we need to tristate PDAC2 if we power on the DAC i.e. IE=0 OE=0 RDE=0 RUE=0"]
pub struct PDAC2_DAC_XPD_FORCE_R(crate::FieldReader<bool, bool>);
impl PDAC2_DAC_XPD_FORCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDAC2_DAC_XPD_FORCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDAC2_DAC_XPD_FORCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDAC2_DAC_XPD_FORCE` writer - Power on DAC2. Usually we need to tristate PDAC2 if we power on the DAC i.e. IE=0 OE=0 RDE=0 RUE=0"]
pub struct PDAC2_DAC_XPD_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> PDAC2_DAC_XPD_FORCE_W<'a> {
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
#[doc = "Field `PDAC2_FUN_IE` reader - the input enable of the pad"]
pub struct PDAC2_FUN_IE_R(crate::FieldReader<bool, bool>);
impl PDAC2_FUN_IE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDAC2_FUN_IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDAC2_FUN_IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDAC2_FUN_IE` writer - the input enable of the pad"]
pub struct PDAC2_FUN_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> PDAC2_FUN_IE_W<'a> {
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
#[doc = "Field `PDAC2_SLP_OE` reader - the output enable of the pad in sleep status"]
pub struct PDAC2_SLP_OE_R(crate::FieldReader<bool, bool>);
impl PDAC2_SLP_OE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDAC2_SLP_OE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDAC2_SLP_OE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDAC2_SLP_OE` writer - the output enable of the pad in sleep status"]
pub struct PDAC2_SLP_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> PDAC2_SLP_OE_W<'a> {
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
#[doc = "Field `PDAC2_SLP_IE` reader - the input enable of the pad in sleep status"]
pub struct PDAC2_SLP_IE_R(crate::FieldReader<bool, bool>);
impl PDAC2_SLP_IE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDAC2_SLP_IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDAC2_SLP_IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDAC2_SLP_IE` writer - the input enable of the pad in sleep status"]
pub struct PDAC2_SLP_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> PDAC2_SLP_IE_W<'a> {
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
#[doc = "Field `PDAC2_SLP_SEL` reader - the sleep status selection signal of the pad"]
pub struct PDAC2_SLP_SEL_R(crate::FieldReader<bool, bool>);
impl PDAC2_SLP_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDAC2_SLP_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDAC2_SLP_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDAC2_SLP_SEL` writer - the sleep status selection signal of the pad"]
pub struct PDAC2_SLP_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PDAC2_SLP_SEL_W<'a> {
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
#[doc = "Field `PDAC2_FUN_SEL` reader - the functional selection signal of the pad"]
pub struct PDAC2_FUN_SEL_R(crate::FieldReader<u8, u8>);
impl PDAC2_FUN_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PDAC2_FUN_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDAC2_FUN_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDAC2_FUN_SEL` writer - the functional selection signal of the pad"]
pub struct PDAC2_FUN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PDAC2_FUN_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 15)) | ((value as u32 & 0x03) << 15);
        self.w
    }
}
#[doc = "Field `PDAC2_MUX_SEL` reader - Ò1Ó select the digital function Ó0Óslection the rtc function"]
pub struct PDAC2_MUX_SEL_R(crate::FieldReader<bool, bool>);
impl PDAC2_MUX_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDAC2_MUX_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDAC2_MUX_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDAC2_MUX_SEL` writer - Ò1Ó select the digital function Ó0Óslection the rtc function"]
pub struct PDAC2_MUX_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PDAC2_MUX_SEL_W<'a> {
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
#[doc = "Field `PDAC2_XPD_DAC` reader - Power on DAC2. Usually we need to tristate PDAC1 if we power on the DAC i.e. IE=0 OE=0 RDE=0 RUE=0"]
pub struct PDAC2_XPD_DAC_R(crate::FieldReader<bool, bool>);
impl PDAC2_XPD_DAC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDAC2_XPD_DAC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDAC2_XPD_DAC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDAC2_XPD_DAC` writer - Power on DAC2. Usually we need to tristate PDAC1 if we power on the DAC i.e. IE=0 OE=0 RDE=0 RUE=0"]
pub struct PDAC2_XPD_DAC_W<'a> {
    w: &'a mut W,
}
impl<'a> PDAC2_XPD_DAC_W<'a> {
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
#[doc = "Field `PDAC2_DAC` reader - PAD DAC2 control code."]
pub struct PDAC2_DAC_R(crate::FieldReader<u8, u8>);
impl PDAC2_DAC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PDAC2_DAC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDAC2_DAC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDAC2_DAC` writer - PAD DAC2 control code."]
pub struct PDAC2_DAC_W<'a> {
    w: &'a mut W,
}
impl<'a> PDAC2_DAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 19)) | ((value as u32 & 0xff) << 19);
        self.w
    }
}
#[doc = "Field `PDAC2_RUE` reader - the pull up enable of the pad"]
pub struct PDAC2_RUE_R(crate::FieldReader<bool, bool>);
impl PDAC2_RUE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDAC2_RUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDAC2_RUE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDAC2_RUE` writer - the pull up enable of the pad"]
pub struct PDAC2_RUE_W<'a> {
    w: &'a mut W,
}
impl<'a> PDAC2_RUE_W<'a> {
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
#[doc = "Field `PDAC2_RDE` reader - the pull down enable of the pad"]
pub struct PDAC2_RDE_R(crate::FieldReader<bool, bool>);
impl PDAC2_RDE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDAC2_RDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDAC2_RDE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDAC2_RDE` writer - the pull down enable of the pad"]
pub struct PDAC2_RDE_W<'a> {
    w: &'a mut W,
}
impl<'a> PDAC2_RDE_W<'a> {
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
#[doc = "Field `PDAC2_HOLD` reader - hold the current value of the output when setting the hold to Ò1Ó"]
pub struct PDAC2_HOLD_R(crate::FieldReader<bool, bool>);
impl PDAC2_HOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDAC2_HOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDAC2_HOLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDAC2_HOLD` writer - hold the current value of the output when setting the hold to Ò1Ó"]
pub struct PDAC2_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> PDAC2_HOLD_W<'a> {
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
#[doc = "Field `PDAC2_DRV` reader - the driver strength of the pad"]
pub struct PDAC2_DRV_R(crate::FieldReader<u8, u8>);
impl PDAC2_DRV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PDAC2_DRV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDAC2_DRV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDAC2_DRV` writer - the driver strength of the pad"]
pub struct PDAC2_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> PDAC2_DRV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bit 10 - Power on DAC2. Usually we need to tristate PDAC2 if we power on the DAC i.e. IE=0 OE=0 RDE=0 RUE=0"]
    #[inline(always)]
    pub fn pdac2_dac_xpd_force(&self) -> PDAC2_DAC_XPD_FORCE_R {
        PDAC2_DAC_XPD_FORCE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - the input enable of the pad"]
    #[inline(always)]
    pub fn pdac2_fun_ie(&self) -> PDAC2_FUN_IE_R {
        PDAC2_FUN_IE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - the output enable of the pad in sleep status"]
    #[inline(always)]
    pub fn pdac2_slp_oe(&self) -> PDAC2_SLP_OE_R {
        PDAC2_SLP_OE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - the input enable of the pad in sleep status"]
    #[inline(always)]
    pub fn pdac2_slp_ie(&self) -> PDAC2_SLP_IE_R {
        PDAC2_SLP_IE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - the sleep status selection signal of the pad"]
    #[inline(always)]
    pub fn pdac2_slp_sel(&self) -> PDAC2_SLP_SEL_R {
        PDAC2_SLP_SEL_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 15:16 - the functional selection signal of the pad"]
    #[inline(always)]
    pub fn pdac2_fun_sel(&self) -> PDAC2_FUN_SEL_R {
        PDAC2_FUN_SEL_R::new(((self.bits >> 15) & 0x03) as u8)
    }
    #[doc = "Bit 17 - Ò1Ó select the digital function Ó0Óslection the rtc function"]
    #[inline(always)]
    pub fn pdac2_mux_sel(&self) -> PDAC2_MUX_SEL_R {
        PDAC2_MUX_SEL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Power on DAC2. Usually we need to tristate PDAC1 if we power on the DAC i.e. IE=0 OE=0 RDE=0 RUE=0"]
    #[inline(always)]
    pub fn pdac2_xpd_dac(&self) -> PDAC2_XPD_DAC_R {
        PDAC2_XPD_DAC_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 19:26 - PAD DAC2 control code."]
    #[inline(always)]
    pub fn pdac2_dac(&self) -> PDAC2_DAC_R {
        PDAC2_DAC_R::new(((self.bits >> 19) & 0xff) as u8)
    }
    #[doc = "Bit 27 - the pull up enable of the pad"]
    #[inline(always)]
    pub fn pdac2_rue(&self) -> PDAC2_RUE_R {
        PDAC2_RUE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - the pull down enable of the pad"]
    #[inline(always)]
    pub fn pdac2_rde(&self) -> PDAC2_RDE_R {
        PDAC2_RDE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - hold the current value of the output when setting the hold to Ò1Ó"]
    #[inline(always)]
    pub fn pdac2_hold(&self) -> PDAC2_HOLD_R {
        PDAC2_HOLD_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 30:31 - the driver strength of the pad"]
    #[inline(always)]
    pub fn pdac2_drv(&self) -> PDAC2_DRV_R {
        PDAC2_DRV_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 10 - Power on DAC2. Usually we need to tristate PDAC2 if we power on the DAC i.e. IE=0 OE=0 RDE=0 RUE=0"]
    #[inline(always)]
    pub fn pdac2_dac_xpd_force(&mut self) -> PDAC2_DAC_XPD_FORCE_W {
        PDAC2_DAC_XPD_FORCE_W { w: self }
    }
    #[doc = "Bit 11 - the input enable of the pad"]
    #[inline(always)]
    pub fn pdac2_fun_ie(&mut self) -> PDAC2_FUN_IE_W {
        PDAC2_FUN_IE_W { w: self }
    }
    #[doc = "Bit 12 - the output enable of the pad in sleep status"]
    #[inline(always)]
    pub fn pdac2_slp_oe(&mut self) -> PDAC2_SLP_OE_W {
        PDAC2_SLP_OE_W { w: self }
    }
    #[doc = "Bit 13 - the input enable of the pad in sleep status"]
    #[inline(always)]
    pub fn pdac2_slp_ie(&mut self) -> PDAC2_SLP_IE_W {
        PDAC2_SLP_IE_W { w: self }
    }
    #[doc = "Bit 14 - the sleep status selection signal of the pad"]
    #[inline(always)]
    pub fn pdac2_slp_sel(&mut self) -> PDAC2_SLP_SEL_W {
        PDAC2_SLP_SEL_W { w: self }
    }
    #[doc = "Bits 15:16 - the functional selection signal of the pad"]
    #[inline(always)]
    pub fn pdac2_fun_sel(&mut self) -> PDAC2_FUN_SEL_W {
        PDAC2_FUN_SEL_W { w: self }
    }
    #[doc = "Bit 17 - Ò1Ó select the digital function Ó0Óslection the rtc function"]
    #[inline(always)]
    pub fn pdac2_mux_sel(&mut self) -> PDAC2_MUX_SEL_W {
        PDAC2_MUX_SEL_W { w: self }
    }
    #[doc = "Bit 18 - Power on DAC2. Usually we need to tristate PDAC1 if we power on the DAC i.e. IE=0 OE=0 RDE=0 RUE=0"]
    #[inline(always)]
    pub fn pdac2_xpd_dac(&mut self) -> PDAC2_XPD_DAC_W {
        PDAC2_XPD_DAC_W { w: self }
    }
    #[doc = "Bits 19:26 - PAD DAC2 control code."]
    #[inline(always)]
    pub fn pdac2_dac(&mut self) -> PDAC2_DAC_W {
        PDAC2_DAC_W { w: self }
    }
    #[doc = "Bit 27 - the pull up enable of the pad"]
    #[inline(always)]
    pub fn pdac2_rue(&mut self) -> PDAC2_RUE_W {
        PDAC2_RUE_W { w: self }
    }
    #[doc = "Bit 28 - the pull down enable of the pad"]
    #[inline(always)]
    pub fn pdac2_rde(&mut self) -> PDAC2_RDE_W {
        PDAC2_RDE_W { w: self }
    }
    #[doc = "Bit 29 - hold the current value of the output when setting the hold to Ò1Ó"]
    #[inline(always)]
    pub fn pdac2_hold(&mut self) -> PDAC2_HOLD_W {
        PDAC2_HOLD_W { w: self }
    }
    #[doc = "Bits 30:31 - the driver strength of the pad"]
    #[inline(always)]
    pub fn pdac2_drv(&mut self) -> PDAC2_DRV_W {
        PDAC2_DRV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pad_dac2](index.html) module"]
pub struct PAD_DAC2_SPEC;
impl crate::RegisterSpec for PAD_DAC2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pad_dac2::R](R) reader structure"]
impl crate::Readable for PAD_DAC2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pad_dac2::W](W) writer structure"]
impl crate::Writable for PAD_DAC2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PAD_DAC2 to value 0x8000_0000"]
impl crate::Resettable for PAD_DAC2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000_0000
    }
}
