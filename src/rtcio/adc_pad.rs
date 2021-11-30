#[doc = "Register `ADC_PAD` reader"]
pub struct R(crate::R<ADC_PAD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_PAD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_PAD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_PAD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_PAD` writer"]
pub struct W(crate::W<ADC_PAD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_PAD_SPEC>;
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
impl From<crate::W<ADC_PAD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_PAD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC2_FUN_IE` reader - the input enable of the pad"]
pub struct ADC2_FUN_IE_R(crate::FieldReader<bool, bool>);
impl ADC2_FUN_IE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC2_FUN_IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC2_FUN_IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC2_FUN_IE` writer - the input enable of the pad"]
pub struct ADC2_FUN_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC2_FUN_IE_W<'a> {
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
#[doc = "Field `ADC2_SLP_IE` reader - the input enable of the pad in sleep status"]
pub struct ADC2_SLP_IE_R(crate::FieldReader<bool, bool>);
impl ADC2_SLP_IE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC2_SLP_IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC2_SLP_IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC2_SLP_IE` writer - the input enable of the pad in sleep status"]
pub struct ADC2_SLP_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC2_SLP_IE_W<'a> {
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
#[doc = "Field `ADC2_SLP_SEL` reader - the sleep status selection signal of the pad"]
pub struct ADC2_SLP_SEL_R(crate::FieldReader<bool, bool>);
impl ADC2_SLP_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC2_SLP_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC2_SLP_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC2_SLP_SEL` writer - the sleep status selection signal of the pad"]
pub struct ADC2_SLP_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC2_SLP_SEL_W<'a> {
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
#[doc = "Field `ADC2_FUN_SEL` reader - the functional selection signal of the pad"]
pub struct ADC2_FUN_SEL_R(crate::FieldReader<u8, u8>);
impl ADC2_FUN_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ADC2_FUN_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC2_FUN_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC2_FUN_SEL` writer - the functional selection signal of the pad"]
pub struct ADC2_FUN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC2_FUN_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | ((value as u32 & 0x03) << 21);
        self.w
    }
}
#[doc = "Field `ADC1_FUN_IE` reader - the input enable of the pad"]
pub struct ADC1_FUN_IE_R(crate::FieldReader<bool, bool>);
impl ADC1_FUN_IE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC1_FUN_IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC1_FUN_IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC1_FUN_IE` writer - the input enable of the pad"]
pub struct ADC1_FUN_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC1_FUN_IE_W<'a> {
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
#[doc = "Field `ADC1_SLP_IE` reader - the input enable of the pad in sleep status"]
pub struct ADC1_SLP_IE_R(crate::FieldReader<bool, bool>);
impl ADC1_SLP_IE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC1_SLP_IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC1_SLP_IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC1_SLP_IE` writer - the input enable of the pad in sleep status"]
pub struct ADC1_SLP_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC1_SLP_IE_W<'a> {
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
#[doc = "Field `ADC1_SLP_SEL` reader - the sleep status selection signal of the pad"]
pub struct ADC1_SLP_SEL_R(crate::FieldReader<bool, bool>);
impl ADC1_SLP_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC1_SLP_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC1_SLP_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC1_SLP_SEL` writer - the sleep status selection signal of the pad"]
pub struct ADC1_SLP_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC1_SLP_SEL_W<'a> {
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
#[doc = "Field `ADC1_FUN_SEL` reader - the functional selection signal of the pad"]
pub struct ADC1_FUN_SEL_R(crate::FieldReader<u8, u8>);
impl ADC1_FUN_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ADC1_FUN_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC1_FUN_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC1_FUN_SEL` writer - the functional selection signal of the pad"]
pub struct ADC1_FUN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC1_FUN_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | ((value as u32 & 0x03) << 26);
        self.w
    }
}
#[doc = "Field `ADC2_MUX_SEL` reader - Ò1Ó select the digital function Ó0Óslection the rtc function"]
pub struct ADC2_MUX_SEL_R(crate::FieldReader<bool, bool>);
impl ADC2_MUX_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC2_MUX_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC2_MUX_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC2_MUX_SEL` writer - Ò1Ó select the digital function Ó0Óslection the rtc function"]
pub struct ADC2_MUX_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC2_MUX_SEL_W<'a> {
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
#[doc = "Field `ADC1_MUX_SEL` reader - Ò1Ó select the digital function Ó0Óslection the rtc function"]
pub struct ADC1_MUX_SEL_R(crate::FieldReader<bool, bool>);
impl ADC1_MUX_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC1_MUX_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC1_MUX_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC1_MUX_SEL` writer - Ò1Ó select the digital function Ó0Óslection the rtc function"]
pub struct ADC1_MUX_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC1_MUX_SEL_W<'a> {
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
#[doc = "Field `ADC2_HOLD` reader - hold the current value of the output when setting the hold to Ò1Ó"]
pub struct ADC2_HOLD_R(crate::FieldReader<bool, bool>);
impl ADC2_HOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC2_HOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC2_HOLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC2_HOLD` writer - hold the current value of the output when setting the hold to Ò1Ó"]
pub struct ADC2_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC2_HOLD_W<'a> {
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
#[doc = "Field `ADC1_HOLD` reader - hold the current value of the output when setting the hold to Ò1Ó"]
pub struct ADC1_HOLD_R(crate::FieldReader<bool, bool>);
impl ADC1_HOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC1_HOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC1_HOLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC1_HOLD` writer - hold the current value of the output when setting the hold to Ò1Ó"]
pub struct ADC1_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC1_HOLD_W<'a> {
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
    #[doc = "Bit 18 - the input enable of the pad"]
    #[inline(always)]
    pub fn adc2_fun_ie(&self) -> ADC2_FUN_IE_R {
        ADC2_FUN_IE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - the input enable of the pad in sleep status"]
    #[inline(always)]
    pub fn adc2_slp_ie(&self) -> ADC2_SLP_IE_R {
        ADC2_SLP_IE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - the sleep status selection signal of the pad"]
    #[inline(always)]
    pub fn adc2_slp_sel(&self) -> ADC2_SLP_SEL_R {
        ADC2_SLP_SEL_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 21:22 - the functional selection signal of the pad"]
    #[inline(always)]
    pub fn adc2_fun_sel(&self) -> ADC2_FUN_SEL_R {
        ADC2_FUN_SEL_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bit 23 - the input enable of the pad"]
    #[inline(always)]
    pub fn adc1_fun_ie(&self) -> ADC1_FUN_IE_R {
        ADC1_FUN_IE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - the input enable of the pad in sleep status"]
    #[inline(always)]
    pub fn adc1_slp_ie(&self) -> ADC1_SLP_IE_R {
        ADC1_SLP_IE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - the sleep status selection signal of the pad"]
    #[inline(always)]
    pub fn adc1_slp_sel(&self) -> ADC1_SLP_SEL_R {
        ADC1_SLP_SEL_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 26:27 - the functional selection signal of the pad"]
    #[inline(always)]
    pub fn adc1_fun_sel(&self) -> ADC1_FUN_SEL_R {
        ADC1_FUN_SEL_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bit 28 - Ò1Ó select the digital function Ó0Óslection the rtc function"]
    #[inline(always)]
    pub fn adc2_mux_sel(&self) -> ADC2_MUX_SEL_R {
        ADC2_MUX_SEL_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Ò1Ó select the digital function Ó0Óslection the rtc function"]
    #[inline(always)]
    pub fn adc1_mux_sel(&self) -> ADC1_MUX_SEL_R {
        ADC1_MUX_SEL_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - hold the current value of the output when setting the hold to Ò1Ó"]
    #[inline(always)]
    pub fn adc2_hold(&self) -> ADC2_HOLD_R {
        ADC2_HOLD_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - hold the current value of the output when setting the hold to Ò1Ó"]
    #[inline(always)]
    pub fn adc1_hold(&self) -> ADC1_HOLD_R {
        ADC1_HOLD_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 18 - the input enable of the pad"]
    #[inline(always)]
    pub fn adc2_fun_ie(&mut self) -> ADC2_FUN_IE_W {
        ADC2_FUN_IE_W { w: self }
    }
    #[doc = "Bit 19 - the input enable of the pad in sleep status"]
    #[inline(always)]
    pub fn adc2_slp_ie(&mut self) -> ADC2_SLP_IE_W {
        ADC2_SLP_IE_W { w: self }
    }
    #[doc = "Bit 20 - the sleep status selection signal of the pad"]
    #[inline(always)]
    pub fn adc2_slp_sel(&mut self) -> ADC2_SLP_SEL_W {
        ADC2_SLP_SEL_W { w: self }
    }
    #[doc = "Bits 21:22 - the functional selection signal of the pad"]
    #[inline(always)]
    pub fn adc2_fun_sel(&mut self) -> ADC2_FUN_SEL_W {
        ADC2_FUN_SEL_W { w: self }
    }
    #[doc = "Bit 23 - the input enable of the pad"]
    #[inline(always)]
    pub fn adc1_fun_ie(&mut self) -> ADC1_FUN_IE_W {
        ADC1_FUN_IE_W { w: self }
    }
    #[doc = "Bit 24 - the input enable of the pad in sleep status"]
    #[inline(always)]
    pub fn adc1_slp_ie(&mut self) -> ADC1_SLP_IE_W {
        ADC1_SLP_IE_W { w: self }
    }
    #[doc = "Bit 25 - the sleep status selection signal of the pad"]
    #[inline(always)]
    pub fn adc1_slp_sel(&mut self) -> ADC1_SLP_SEL_W {
        ADC1_SLP_SEL_W { w: self }
    }
    #[doc = "Bits 26:27 - the functional selection signal of the pad"]
    #[inline(always)]
    pub fn adc1_fun_sel(&mut self) -> ADC1_FUN_SEL_W {
        ADC1_FUN_SEL_W { w: self }
    }
    #[doc = "Bit 28 - Ò1Ó select the digital function Ó0Óslection the rtc function"]
    #[inline(always)]
    pub fn adc2_mux_sel(&mut self) -> ADC2_MUX_SEL_W {
        ADC2_MUX_SEL_W { w: self }
    }
    #[doc = "Bit 29 - Ò1Ó select the digital function Ó0Óslection the rtc function"]
    #[inline(always)]
    pub fn adc1_mux_sel(&mut self) -> ADC1_MUX_SEL_W {
        ADC1_MUX_SEL_W { w: self }
    }
    #[doc = "Bit 30 - hold the current value of the output when setting the hold to Ò1Ó"]
    #[inline(always)]
    pub fn adc2_hold(&mut self) -> ADC2_HOLD_W {
        ADC2_HOLD_W { w: self }
    }
    #[doc = "Bit 31 - hold the current value of the output when setting the hold to Ò1Ó"]
    #[inline(always)]
    pub fn adc1_hold(&mut self) -> ADC1_HOLD_W {
        ADC1_HOLD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_pad](index.html) module"]
pub struct ADC_PAD_SPEC;
impl crate::RegisterSpec for ADC_PAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_pad::R](R) reader structure"]
impl crate::Readable for ADC_PAD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_pad::W](W) writer structure"]
impl crate::Writable for ADC_PAD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC_PAD to value 0"]
impl crate::Resettable for ADC_PAD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
