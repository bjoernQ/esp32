#[doc = "Register `XTAL_32K_PAD` reader"]
pub struct R(crate::R<XTAL_32K_PAD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XTAL_32K_PAD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XTAL_32K_PAD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XTAL_32K_PAD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XTAL_32K_PAD` writer"]
pub struct W(crate::W<XTAL_32K_PAD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XTAL_32K_PAD_SPEC>;
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
impl From<crate::W<XTAL_32K_PAD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XTAL_32K_PAD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBIAS_XTAL_32K` reader - 32K XTAL self-bias reference control."]
pub struct DBIAS_XTAL_32K_R(crate::FieldReader<u8, u8>);
impl DBIAS_XTAL_32K_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DBIAS_XTAL_32K_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBIAS_XTAL_32K_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBIAS_XTAL_32K` writer - 32K XTAL self-bias reference control."]
pub struct DBIAS_XTAL_32K_W<'a> {
    w: &'a mut W,
}
impl<'a> DBIAS_XTAL_32K_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | ((value as u32 & 0x03) << 1);
        self.w
    }
}
#[doc = "Field `DRES_XTAL_32K` reader - 32K XTAL resistor bias control."]
pub struct DRES_XTAL_32K_R(crate::FieldReader<u8, u8>);
impl DRES_XTAL_32K_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DRES_XTAL_32K_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DRES_XTAL_32K_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DRES_XTAL_32K` writer - 32K XTAL resistor bias control."]
pub struct DRES_XTAL_32K_W<'a> {
    w: &'a mut W,
}
impl<'a> DRES_XTAL_32K_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | ((value as u32 & 0x03) << 3);
        self.w
    }
}
#[doc = "Field `X32P_FUN_IE` reader - the input enable of the pad"]
pub struct X32P_FUN_IE_R(crate::FieldReader<bool, bool>);
impl X32P_FUN_IE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        X32P_FUN_IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for X32P_FUN_IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `X32P_FUN_IE` writer - the input enable of the pad"]
pub struct X32P_FUN_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> X32P_FUN_IE_W<'a> {
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
#[doc = "Field `X32P_SLP_OE` reader - the output enable of the pad in sleep status"]
pub struct X32P_SLP_OE_R(crate::FieldReader<bool, bool>);
impl X32P_SLP_OE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        X32P_SLP_OE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for X32P_SLP_OE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `X32P_SLP_OE` writer - the output enable of the pad in sleep status"]
pub struct X32P_SLP_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> X32P_SLP_OE_W<'a> {
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
#[doc = "Field `X32P_SLP_IE` reader - the input enable of the pad in sleep status"]
pub struct X32P_SLP_IE_R(crate::FieldReader<bool, bool>);
impl X32P_SLP_IE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        X32P_SLP_IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for X32P_SLP_IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `X32P_SLP_IE` writer - the input enable of the pad in sleep status"]
pub struct X32P_SLP_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> X32P_SLP_IE_W<'a> {
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
#[doc = "Field `X32P_SLP_SEL` reader - the sleep status selection signal of the pad"]
pub struct X32P_SLP_SEL_R(crate::FieldReader<bool, bool>);
impl X32P_SLP_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        X32P_SLP_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for X32P_SLP_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `X32P_SLP_SEL` writer - the sleep status selection signal of the pad"]
pub struct X32P_SLP_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> X32P_SLP_SEL_W<'a> {
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
#[doc = "Field `X32P_FUN_SEL` reader - the functional selection signal of the pad"]
pub struct X32P_FUN_SEL_R(crate::FieldReader<u8, u8>);
impl X32P_FUN_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        X32P_FUN_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for X32P_FUN_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `X32P_FUN_SEL` writer - the functional selection signal of the pad"]
pub struct X32P_FUN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> X32P_FUN_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | ((value as u32 & 0x03) << 9);
        self.w
    }
}
#[doc = "Field `X32N_FUN_IE` reader - the input enable of the pad"]
pub struct X32N_FUN_IE_R(crate::FieldReader<bool, bool>);
impl X32N_FUN_IE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        X32N_FUN_IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for X32N_FUN_IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `X32N_FUN_IE` writer - the input enable of the pad"]
pub struct X32N_FUN_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> X32N_FUN_IE_W<'a> {
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
#[doc = "Field `X32N_SLP_OE` reader - the output enable of the pad in sleep status"]
pub struct X32N_SLP_OE_R(crate::FieldReader<bool, bool>);
impl X32N_SLP_OE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        X32N_SLP_OE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for X32N_SLP_OE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `X32N_SLP_OE` writer - the output enable of the pad in sleep status"]
pub struct X32N_SLP_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> X32N_SLP_OE_W<'a> {
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
#[doc = "Field `X32N_SLP_IE` reader - the input enable of the pad in sleep status"]
pub struct X32N_SLP_IE_R(crate::FieldReader<bool, bool>);
impl X32N_SLP_IE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        X32N_SLP_IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for X32N_SLP_IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `X32N_SLP_IE` writer - the input enable of the pad in sleep status"]
pub struct X32N_SLP_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> X32N_SLP_IE_W<'a> {
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
#[doc = "Field `X32N_SLP_SEL` reader - the sleep status selection signal of the pad"]
pub struct X32N_SLP_SEL_R(crate::FieldReader<bool, bool>);
impl X32N_SLP_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        X32N_SLP_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for X32N_SLP_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `X32N_SLP_SEL` writer - the sleep status selection signal of the pad"]
pub struct X32N_SLP_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> X32N_SLP_SEL_W<'a> {
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
#[doc = "Field `X32N_FUN_SEL` reader - the functional selection signal of the pad"]
pub struct X32N_FUN_SEL_R(crate::FieldReader<u8, u8>);
impl X32N_FUN_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        X32N_FUN_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for X32N_FUN_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `X32N_FUN_SEL` writer - the functional selection signal of the pad"]
pub struct X32N_FUN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> X32N_FUN_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 15)) | ((value as u32 & 0x03) << 15);
        self.w
    }
}
#[doc = "Field `X32P_MUX_SEL` reader - Ò1Ó select the digital function Ó0Óslection the rtc function"]
pub struct X32P_MUX_SEL_R(crate::FieldReader<bool, bool>);
impl X32P_MUX_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        X32P_MUX_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for X32P_MUX_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `X32P_MUX_SEL` writer - Ò1Ó select the digital function Ó0Óslection the rtc function"]
pub struct X32P_MUX_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> X32P_MUX_SEL_W<'a> {
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
#[doc = "Field `X32N_MUX_SEL` reader - Ò1Ó select the digital function Ó0Óslection the rtc function"]
pub struct X32N_MUX_SEL_R(crate::FieldReader<bool, bool>);
impl X32N_MUX_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        X32N_MUX_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for X32N_MUX_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `X32N_MUX_SEL` writer - Ò1Ó select the digital function Ó0Óslection the rtc function"]
pub struct X32N_MUX_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> X32N_MUX_SEL_W<'a> {
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
#[doc = "Field `XPD_XTAL_32K` reader - Power up 32kHz crystal oscillator"]
pub struct XPD_XTAL_32K_R(crate::FieldReader<bool, bool>);
impl XPD_XTAL_32K_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XPD_XTAL_32K_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XPD_XTAL_32K_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XPD_XTAL_32K` writer - Power up 32kHz crystal oscillator"]
pub struct XPD_XTAL_32K_W<'a> {
    w: &'a mut W,
}
impl<'a> XPD_XTAL_32K_W<'a> {
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
#[doc = "Field `DAC_XTAL_32K` reader - 32K XTAL bias current DAC."]
pub struct DAC_XTAL_32K_R(crate::FieldReader<u8, u8>);
impl DAC_XTAL_32K_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DAC_XTAL_32K_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAC_XTAL_32K_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAC_XTAL_32K` writer - 32K XTAL bias current DAC."]
pub struct DAC_XTAL_32K_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_XTAL_32K_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Field `X32P_RUE` reader - the pull up enable of the pad"]
pub struct X32P_RUE_R(crate::FieldReader<bool, bool>);
impl X32P_RUE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        X32P_RUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for X32P_RUE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `X32P_RUE` writer - the pull up enable of the pad"]
pub struct X32P_RUE_W<'a> {
    w: &'a mut W,
}
impl<'a> X32P_RUE_W<'a> {
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
#[doc = "Field `X32P_RDE` reader - the pull down enable of the pad"]
pub struct X32P_RDE_R(crate::FieldReader<bool, bool>);
impl X32P_RDE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        X32P_RDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for X32P_RDE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `X32P_RDE` writer - the pull down enable of the pad"]
pub struct X32P_RDE_W<'a> {
    w: &'a mut W,
}
impl<'a> X32P_RDE_W<'a> {
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
#[doc = "Field `X32P_HOLD` reader - hold the current value of the output when setting the hold to Ò1Ó"]
pub struct X32P_HOLD_R(crate::FieldReader<bool, bool>);
impl X32P_HOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        X32P_HOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for X32P_HOLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `X32P_HOLD` writer - hold the current value of the output when setting the hold to Ò1Ó"]
pub struct X32P_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> X32P_HOLD_W<'a> {
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
#[doc = "Field `X32P_DRV` reader - the driver strength of the pad"]
pub struct X32P_DRV_R(crate::FieldReader<u8, u8>);
impl X32P_DRV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        X32P_DRV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for X32P_DRV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `X32P_DRV` writer - the driver strength of the pad"]
pub struct X32P_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> X32P_DRV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 25)) | ((value as u32 & 0x03) << 25);
        self.w
    }
}
#[doc = "Field `X32N_RUE` reader - the pull up enable of the pad"]
pub struct X32N_RUE_R(crate::FieldReader<bool, bool>);
impl X32N_RUE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        X32N_RUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for X32N_RUE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `X32N_RUE` writer - the pull up enable of the pad"]
pub struct X32N_RUE_W<'a> {
    w: &'a mut W,
}
impl<'a> X32N_RUE_W<'a> {
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
#[doc = "Field `X32N_RDE` reader - the pull down enable of the pad"]
pub struct X32N_RDE_R(crate::FieldReader<bool, bool>);
impl X32N_RDE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        X32N_RDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for X32N_RDE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `X32N_RDE` writer - the pull down enable of the pad"]
pub struct X32N_RDE_W<'a> {
    w: &'a mut W,
}
impl<'a> X32N_RDE_W<'a> {
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
#[doc = "Field `X32N_HOLD` reader - hold the current value of the output when setting the hold to Ò1Ó"]
pub struct X32N_HOLD_R(crate::FieldReader<bool, bool>);
impl X32N_HOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        X32N_HOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for X32N_HOLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `X32N_HOLD` writer - hold the current value of the output when setting the hold to Ò1Ó"]
pub struct X32N_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> X32N_HOLD_W<'a> {
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
#[doc = "Field `X32N_DRV` reader - the driver strength of the pad"]
pub struct X32N_DRV_R(crate::FieldReader<u8, u8>);
impl X32N_DRV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        X32N_DRV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for X32N_DRV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `X32N_DRV` writer - the driver strength of the pad"]
pub struct X32N_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> X32N_DRV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 1:2 - 32K XTAL self-bias reference control."]
    #[inline(always)]
    pub fn dbias_xtal_32k(&self) -> DBIAS_XTAL_32K_R {
        DBIAS_XTAL_32K_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bits 3:4 - 32K XTAL resistor bias control."]
    #[inline(always)]
    pub fn dres_xtal_32k(&self) -> DRES_XTAL_32K_R {
        DRES_XTAL_32K_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 5 - the input enable of the pad"]
    #[inline(always)]
    pub fn x32p_fun_ie(&self) -> X32P_FUN_IE_R {
        X32P_FUN_IE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - the output enable of the pad in sleep status"]
    #[inline(always)]
    pub fn x32p_slp_oe(&self) -> X32P_SLP_OE_R {
        X32P_SLP_OE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - the input enable of the pad in sleep status"]
    #[inline(always)]
    pub fn x32p_slp_ie(&self) -> X32P_SLP_IE_R {
        X32P_SLP_IE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - the sleep status selection signal of the pad"]
    #[inline(always)]
    pub fn x32p_slp_sel(&self) -> X32P_SLP_SEL_R {
        X32P_SLP_SEL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 9:10 - the functional selection signal of the pad"]
    #[inline(always)]
    pub fn x32p_fun_sel(&self) -> X32P_FUN_SEL_R {
        X32P_FUN_SEL_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bit 11 - the input enable of the pad"]
    #[inline(always)]
    pub fn x32n_fun_ie(&self) -> X32N_FUN_IE_R {
        X32N_FUN_IE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - the output enable of the pad in sleep status"]
    #[inline(always)]
    pub fn x32n_slp_oe(&self) -> X32N_SLP_OE_R {
        X32N_SLP_OE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - the input enable of the pad in sleep status"]
    #[inline(always)]
    pub fn x32n_slp_ie(&self) -> X32N_SLP_IE_R {
        X32N_SLP_IE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - the sleep status selection signal of the pad"]
    #[inline(always)]
    pub fn x32n_slp_sel(&self) -> X32N_SLP_SEL_R {
        X32N_SLP_SEL_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 15:16 - the functional selection signal of the pad"]
    #[inline(always)]
    pub fn x32n_fun_sel(&self) -> X32N_FUN_SEL_R {
        X32N_FUN_SEL_R::new(((self.bits >> 15) & 0x03) as u8)
    }
    #[doc = "Bit 17 - Ò1Ó select the digital function Ó0Óslection the rtc function"]
    #[inline(always)]
    pub fn x32p_mux_sel(&self) -> X32P_MUX_SEL_R {
        X32P_MUX_SEL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Ò1Ó select the digital function Ó0Óslection the rtc function"]
    #[inline(always)]
    pub fn x32n_mux_sel(&self) -> X32N_MUX_SEL_R {
        X32N_MUX_SEL_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Power up 32kHz crystal oscillator"]
    #[inline(always)]
    pub fn xpd_xtal_32k(&self) -> XPD_XTAL_32K_R {
        XPD_XTAL_32K_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 20:21 - 32K XTAL bias current DAC."]
    #[inline(always)]
    pub fn dac_xtal_32k(&self) -> DAC_XTAL_32K_R {
        DAC_XTAL_32K_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bit 22 - the pull up enable of the pad"]
    #[inline(always)]
    pub fn x32p_rue(&self) -> X32P_RUE_R {
        X32P_RUE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - the pull down enable of the pad"]
    #[inline(always)]
    pub fn x32p_rde(&self) -> X32P_RDE_R {
        X32P_RDE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - hold the current value of the output when setting the hold to Ò1Ó"]
    #[inline(always)]
    pub fn x32p_hold(&self) -> X32P_HOLD_R {
        X32P_HOLD_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 25:26 - the driver strength of the pad"]
    #[inline(always)]
    pub fn x32p_drv(&self) -> X32P_DRV_R {
        X32P_DRV_R::new(((self.bits >> 25) & 0x03) as u8)
    }
    #[doc = "Bit 27 - the pull up enable of the pad"]
    #[inline(always)]
    pub fn x32n_rue(&self) -> X32N_RUE_R {
        X32N_RUE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - the pull down enable of the pad"]
    #[inline(always)]
    pub fn x32n_rde(&self) -> X32N_RDE_R {
        X32N_RDE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - hold the current value of the output when setting the hold to Ò1Ó"]
    #[inline(always)]
    pub fn x32n_hold(&self) -> X32N_HOLD_R {
        X32N_HOLD_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 30:31 - the driver strength of the pad"]
    #[inline(always)]
    pub fn x32n_drv(&self) -> X32N_DRV_R {
        X32N_DRV_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 1:2 - 32K XTAL self-bias reference control."]
    #[inline(always)]
    pub fn dbias_xtal_32k(&mut self) -> DBIAS_XTAL_32K_W {
        DBIAS_XTAL_32K_W { w: self }
    }
    #[doc = "Bits 3:4 - 32K XTAL resistor bias control."]
    #[inline(always)]
    pub fn dres_xtal_32k(&mut self) -> DRES_XTAL_32K_W {
        DRES_XTAL_32K_W { w: self }
    }
    #[doc = "Bit 5 - the input enable of the pad"]
    #[inline(always)]
    pub fn x32p_fun_ie(&mut self) -> X32P_FUN_IE_W {
        X32P_FUN_IE_W { w: self }
    }
    #[doc = "Bit 6 - the output enable of the pad in sleep status"]
    #[inline(always)]
    pub fn x32p_slp_oe(&mut self) -> X32P_SLP_OE_W {
        X32P_SLP_OE_W { w: self }
    }
    #[doc = "Bit 7 - the input enable of the pad in sleep status"]
    #[inline(always)]
    pub fn x32p_slp_ie(&mut self) -> X32P_SLP_IE_W {
        X32P_SLP_IE_W { w: self }
    }
    #[doc = "Bit 8 - the sleep status selection signal of the pad"]
    #[inline(always)]
    pub fn x32p_slp_sel(&mut self) -> X32P_SLP_SEL_W {
        X32P_SLP_SEL_W { w: self }
    }
    #[doc = "Bits 9:10 - the functional selection signal of the pad"]
    #[inline(always)]
    pub fn x32p_fun_sel(&mut self) -> X32P_FUN_SEL_W {
        X32P_FUN_SEL_W { w: self }
    }
    #[doc = "Bit 11 - the input enable of the pad"]
    #[inline(always)]
    pub fn x32n_fun_ie(&mut self) -> X32N_FUN_IE_W {
        X32N_FUN_IE_W { w: self }
    }
    #[doc = "Bit 12 - the output enable of the pad in sleep status"]
    #[inline(always)]
    pub fn x32n_slp_oe(&mut self) -> X32N_SLP_OE_W {
        X32N_SLP_OE_W { w: self }
    }
    #[doc = "Bit 13 - the input enable of the pad in sleep status"]
    #[inline(always)]
    pub fn x32n_slp_ie(&mut self) -> X32N_SLP_IE_W {
        X32N_SLP_IE_W { w: self }
    }
    #[doc = "Bit 14 - the sleep status selection signal of the pad"]
    #[inline(always)]
    pub fn x32n_slp_sel(&mut self) -> X32N_SLP_SEL_W {
        X32N_SLP_SEL_W { w: self }
    }
    #[doc = "Bits 15:16 - the functional selection signal of the pad"]
    #[inline(always)]
    pub fn x32n_fun_sel(&mut self) -> X32N_FUN_SEL_W {
        X32N_FUN_SEL_W { w: self }
    }
    #[doc = "Bit 17 - Ò1Ó select the digital function Ó0Óslection the rtc function"]
    #[inline(always)]
    pub fn x32p_mux_sel(&mut self) -> X32P_MUX_SEL_W {
        X32P_MUX_SEL_W { w: self }
    }
    #[doc = "Bit 18 - Ò1Ó select the digital function Ó0Óslection the rtc function"]
    #[inline(always)]
    pub fn x32n_mux_sel(&mut self) -> X32N_MUX_SEL_W {
        X32N_MUX_SEL_W { w: self }
    }
    #[doc = "Bit 19 - Power up 32kHz crystal oscillator"]
    #[inline(always)]
    pub fn xpd_xtal_32k(&mut self) -> XPD_XTAL_32K_W {
        XPD_XTAL_32K_W { w: self }
    }
    #[doc = "Bits 20:21 - 32K XTAL bias current DAC."]
    #[inline(always)]
    pub fn dac_xtal_32k(&mut self) -> DAC_XTAL_32K_W {
        DAC_XTAL_32K_W { w: self }
    }
    #[doc = "Bit 22 - the pull up enable of the pad"]
    #[inline(always)]
    pub fn x32p_rue(&mut self) -> X32P_RUE_W {
        X32P_RUE_W { w: self }
    }
    #[doc = "Bit 23 - the pull down enable of the pad"]
    #[inline(always)]
    pub fn x32p_rde(&mut self) -> X32P_RDE_W {
        X32P_RDE_W { w: self }
    }
    #[doc = "Bit 24 - hold the current value of the output when setting the hold to Ò1Ó"]
    #[inline(always)]
    pub fn x32p_hold(&mut self) -> X32P_HOLD_W {
        X32P_HOLD_W { w: self }
    }
    #[doc = "Bits 25:26 - the driver strength of the pad"]
    #[inline(always)]
    pub fn x32p_drv(&mut self) -> X32P_DRV_W {
        X32P_DRV_W { w: self }
    }
    #[doc = "Bit 27 - the pull up enable of the pad"]
    #[inline(always)]
    pub fn x32n_rue(&mut self) -> X32N_RUE_W {
        X32N_RUE_W { w: self }
    }
    #[doc = "Bit 28 - the pull down enable of the pad"]
    #[inline(always)]
    pub fn x32n_rde(&mut self) -> X32N_RDE_W {
        X32N_RDE_W { w: self }
    }
    #[doc = "Bit 29 - hold the current value of the output when setting the hold to Ò1Ó"]
    #[inline(always)]
    pub fn x32n_hold(&mut self) -> X32N_HOLD_W {
        X32N_HOLD_W { w: self }
    }
    #[doc = "Bits 30:31 - the driver strength of the pad"]
    #[inline(always)]
    pub fn x32n_drv(&mut self) -> X32N_DRV_W {
        X32N_DRV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xtal_32k_pad](index.html) module"]
pub struct XTAL_32K_PAD_SPEC;
impl crate::RegisterSpec for XTAL_32K_PAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xtal_32k_pad::R](R) reader structure"]
impl crate::Readable for XTAL_32K_PAD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xtal_32k_pad::W](W) writer structure"]
impl crate::Writable for XTAL_32K_PAD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets XTAL_32K_PAD to value 0x8410_0010"]
impl crate::Resettable for XTAL_32K_PAD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8410_0010
    }
}
