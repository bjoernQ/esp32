#[doc = "Register `SENSOR_PADS` reader"]
pub struct R(crate::R<SENSOR_PADS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SENSOR_PADS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SENSOR_PADS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SENSOR_PADS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SENSOR_PADS` writer"]
pub struct W(crate::W<SENSOR_PADS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SENSOR_PADS_SPEC>;
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
impl From<crate::W<SENSOR_PADS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SENSOR_PADS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SENSE4_FUN_IE` reader - the input enable of the pad"]
pub struct SENSE4_FUN_IE_R(crate::FieldReader<bool, bool>);
impl SENSE4_FUN_IE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SENSE4_FUN_IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SENSE4_FUN_IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SENSE4_FUN_IE` writer - the input enable of the pad"]
pub struct SENSE4_FUN_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSE4_FUN_IE_W<'a> {
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
#[doc = "Field `SENSE4_SLP_IE` reader - the input enable of the pad in sleep status"]
pub struct SENSE4_SLP_IE_R(crate::FieldReader<bool, bool>);
impl SENSE4_SLP_IE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SENSE4_SLP_IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SENSE4_SLP_IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SENSE4_SLP_IE` writer - the input enable of the pad in sleep status"]
pub struct SENSE4_SLP_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSE4_SLP_IE_W<'a> {
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
#[doc = "Field `SENSE4_SLP_SEL` reader - the sleep status selection signal of the pad"]
pub struct SENSE4_SLP_SEL_R(crate::FieldReader<bool, bool>);
impl SENSE4_SLP_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SENSE4_SLP_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SENSE4_SLP_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SENSE4_SLP_SEL` writer - the sleep status selection signal of the pad"]
pub struct SENSE4_SLP_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSE4_SLP_SEL_W<'a> {
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
#[doc = "Field `SENSE4_FUN_SEL` reader - the functional selection signal of the pad"]
pub struct SENSE4_FUN_SEL_R(crate::FieldReader<u8, u8>);
impl SENSE4_FUN_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SENSE4_FUN_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SENSE4_FUN_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SENSE4_FUN_SEL` writer - the functional selection signal of the pad"]
pub struct SENSE4_FUN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSE4_FUN_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 7)) | ((value as u32 & 0x03) << 7);
        self.w
    }
}
#[doc = "Field `SENSE3_FUN_IE` reader - the input enable of the pad"]
pub struct SENSE3_FUN_IE_R(crate::FieldReader<bool, bool>);
impl SENSE3_FUN_IE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SENSE3_FUN_IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SENSE3_FUN_IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SENSE3_FUN_IE` writer - the input enable of the pad"]
pub struct SENSE3_FUN_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSE3_FUN_IE_W<'a> {
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
#[doc = "Field `SENSE3_SLP_IE` reader - the input enable of the pad in sleep status"]
pub struct SENSE3_SLP_IE_R(crate::FieldReader<bool, bool>);
impl SENSE3_SLP_IE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SENSE3_SLP_IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SENSE3_SLP_IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SENSE3_SLP_IE` writer - the input enable of the pad in sleep status"]
pub struct SENSE3_SLP_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSE3_SLP_IE_W<'a> {
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
#[doc = "Field `SENSE3_SLP_SEL` reader - the sleep status selection signal of the pad"]
pub struct SENSE3_SLP_SEL_R(crate::FieldReader<bool, bool>);
impl SENSE3_SLP_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SENSE3_SLP_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SENSE3_SLP_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SENSE3_SLP_SEL` writer - the sleep status selection signal of the pad"]
pub struct SENSE3_SLP_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSE3_SLP_SEL_W<'a> {
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
#[doc = "Field `SENSE3_FUN_SEL` reader - the functional selection signal of the pad"]
pub struct SENSE3_FUN_SEL_R(crate::FieldReader<u8, u8>);
impl SENSE3_FUN_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SENSE3_FUN_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SENSE3_FUN_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SENSE3_FUN_SEL` writer - the functional selection signal of the pad"]
pub struct SENSE3_FUN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSE3_FUN_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `SENSE2_FUN_IE` reader - the input enable of the pad"]
pub struct SENSE2_FUN_IE_R(crate::FieldReader<bool, bool>);
impl SENSE2_FUN_IE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SENSE2_FUN_IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SENSE2_FUN_IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SENSE2_FUN_IE` writer - the input enable of the pad"]
pub struct SENSE2_FUN_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSE2_FUN_IE_W<'a> {
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
#[doc = "Field `SENSE2_SLP_IE` reader - the input enable of the pad in sleep status"]
pub struct SENSE2_SLP_IE_R(crate::FieldReader<bool, bool>);
impl SENSE2_SLP_IE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SENSE2_SLP_IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SENSE2_SLP_IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SENSE2_SLP_IE` writer - the input enable of the pad in sleep status"]
pub struct SENSE2_SLP_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSE2_SLP_IE_W<'a> {
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
#[doc = "Field `SENSE2_SLP_SEL` reader - the sleep status selection signal of the pad"]
pub struct SENSE2_SLP_SEL_R(crate::FieldReader<bool, bool>);
impl SENSE2_SLP_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SENSE2_SLP_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SENSE2_SLP_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SENSE2_SLP_SEL` writer - the sleep status selection signal of the pad"]
pub struct SENSE2_SLP_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSE2_SLP_SEL_W<'a> {
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
#[doc = "Field `SENSE2_FUN_SEL` reader - the functional selection signal of the pad"]
pub struct SENSE2_FUN_SEL_R(crate::FieldReader<u8, u8>);
impl SENSE2_FUN_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SENSE2_FUN_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SENSE2_FUN_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SENSE2_FUN_SEL` writer - the functional selection signal of the pad"]
pub struct SENSE2_FUN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSE2_FUN_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 17)) | ((value as u32 & 0x03) << 17);
        self.w
    }
}
#[doc = "Field `SENSE1_FUN_IE` reader - the input enable of the pad"]
pub struct SENSE1_FUN_IE_R(crate::FieldReader<bool, bool>);
impl SENSE1_FUN_IE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SENSE1_FUN_IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SENSE1_FUN_IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SENSE1_FUN_IE` writer - the input enable of the pad"]
pub struct SENSE1_FUN_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSE1_FUN_IE_W<'a> {
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
#[doc = "Field `SENSE1_SLP_IE` reader - the input enable of the pad in sleep status"]
pub struct SENSE1_SLP_IE_R(crate::FieldReader<bool, bool>);
impl SENSE1_SLP_IE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SENSE1_SLP_IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SENSE1_SLP_IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SENSE1_SLP_IE` writer - the input enable of the pad in sleep status"]
pub struct SENSE1_SLP_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSE1_SLP_IE_W<'a> {
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
#[doc = "Field `SENSE1_SLP_SEL` reader - the sleep status selection signal of the pad"]
pub struct SENSE1_SLP_SEL_R(crate::FieldReader<bool, bool>);
impl SENSE1_SLP_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SENSE1_SLP_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SENSE1_SLP_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SENSE1_SLP_SEL` writer - the sleep status selection signal of the pad"]
pub struct SENSE1_SLP_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSE1_SLP_SEL_W<'a> {
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
#[doc = "Field `SENSE1_FUN_SEL` reader - the functional selection signal of the pad"]
pub struct SENSE1_FUN_SEL_R(crate::FieldReader<u8, u8>);
impl SENSE1_FUN_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SENSE1_FUN_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SENSE1_FUN_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SENSE1_FUN_SEL` writer - the functional selection signal of the pad"]
pub struct SENSE1_FUN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSE1_FUN_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
#[doc = "Field `SENSE4_MUX_SEL` reader - Ò1Ó select the digital function Ó0Óslection the rtc function"]
pub struct SENSE4_MUX_SEL_R(crate::FieldReader<bool, bool>);
impl SENSE4_MUX_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SENSE4_MUX_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SENSE4_MUX_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SENSE4_MUX_SEL` writer - Ò1Ó select the digital function Ó0Óslection the rtc function"]
pub struct SENSE4_MUX_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSE4_MUX_SEL_W<'a> {
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
#[doc = "Field `SENSE3_MUX_SEL` reader - Ò1Ó select the digital function Ó0Óslection the rtc function"]
pub struct SENSE3_MUX_SEL_R(crate::FieldReader<bool, bool>);
impl SENSE3_MUX_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SENSE3_MUX_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SENSE3_MUX_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SENSE3_MUX_SEL` writer - Ò1Ó select the digital function Ó0Óslection the rtc function"]
pub struct SENSE3_MUX_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSE3_MUX_SEL_W<'a> {
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
#[doc = "Field `SENSE2_MUX_SEL` reader - Ò1Ó select the digital function Ó0Óslection the rtc function"]
pub struct SENSE2_MUX_SEL_R(crate::FieldReader<bool, bool>);
impl SENSE2_MUX_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SENSE2_MUX_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SENSE2_MUX_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SENSE2_MUX_SEL` writer - Ò1Ó select the digital function Ó0Óslection the rtc function"]
pub struct SENSE2_MUX_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSE2_MUX_SEL_W<'a> {
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
#[doc = "Field `SENSE1_MUX_SEL` reader - Ò1Ó select the digital function Ó0Óslection the rtc function"]
pub struct SENSE1_MUX_SEL_R(crate::FieldReader<bool, bool>);
impl SENSE1_MUX_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SENSE1_MUX_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SENSE1_MUX_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SENSE1_MUX_SEL` writer - Ò1Ó select the digital function Ó0Óslection the rtc function"]
pub struct SENSE1_MUX_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSE1_MUX_SEL_W<'a> {
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
#[doc = "Field `SENSE4_HOLD` reader - hold the current value of the output when setting the hold to Ò1Ó"]
pub struct SENSE4_HOLD_R(crate::FieldReader<bool, bool>);
impl SENSE4_HOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SENSE4_HOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SENSE4_HOLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SENSE4_HOLD` writer - hold the current value of the output when setting the hold to Ò1Ó"]
pub struct SENSE4_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSE4_HOLD_W<'a> {
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
#[doc = "Field `SENSE3_HOLD` reader - hold the current value of the output when setting the hold to Ò1Ó"]
pub struct SENSE3_HOLD_R(crate::FieldReader<bool, bool>);
impl SENSE3_HOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SENSE3_HOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SENSE3_HOLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SENSE3_HOLD` writer - hold the current value of the output when setting the hold to Ò1Ó"]
pub struct SENSE3_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSE3_HOLD_W<'a> {
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
#[doc = "Field `SENSE2_HOLD` reader - hold the current value of the output when setting the hold to Ò1Ó"]
pub struct SENSE2_HOLD_R(crate::FieldReader<bool, bool>);
impl SENSE2_HOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SENSE2_HOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SENSE2_HOLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SENSE2_HOLD` writer - hold the current value of the output when setting the hold to Ò1Ó"]
pub struct SENSE2_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSE2_HOLD_W<'a> {
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
#[doc = "Field `SENSE1_HOLD` reader - hold the current value of the output when setting the hold to Ò1Ó"]
pub struct SENSE1_HOLD_R(crate::FieldReader<bool, bool>);
impl SENSE1_HOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SENSE1_HOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SENSE1_HOLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SENSE1_HOLD` writer - hold the current value of the output when setting the hold to Ò1Ó"]
pub struct SENSE1_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSE1_HOLD_W<'a> {
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
    #[doc = "Bit 4 - the input enable of the pad"]
    #[inline(always)]
    pub fn sense4_fun_ie(&self) -> SENSE4_FUN_IE_R {
        SENSE4_FUN_IE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - the input enable of the pad in sleep status"]
    #[inline(always)]
    pub fn sense4_slp_ie(&self) -> SENSE4_SLP_IE_R {
        SENSE4_SLP_IE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - the sleep status selection signal of the pad"]
    #[inline(always)]
    pub fn sense4_slp_sel(&self) -> SENSE4_SLP_SEL_R {
        SENSE4_SLP_SEL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 7:8 - the functional selection signal of the pad"]
    #[inline(always)]
    pub fn sense4_fun_sel(&self) -> SENSE4_FUN_SEL_R {
        SENSE4_FUN_SEL_R::new(((self.bits >> 7) & 0x03) as u8)
    }
    #[doc = "Bit 9 - the input enable of the pad"]
    #[inline(always)]
    pub fn sense3_fun_ie(&self) -> SENSE3_FUN_IE_R {
        SENSE3_FUN_IE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - the input enable of the pad in sleep status"]
    #[inline(always)]
    pub fn sense3_slp_ie(&self) -> SENSE3_SLP_IE_R {
        SENSE3_SLP_IE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - the sleep status selection signal of the pad"]
    #[inline(always)]
    pub fn sense3_slp_sel(&self) -> SENSE3_SLP_SEL_R {
        SENSE3_SLP_SEL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - the functional selection signal of the pad"]
    #[inline(always)]
    pub fn sense3_fun_sel(&self) -> SENSE3_FUN_SEL_R {
        SENSE3_FUN_SEL_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 14 - the input enable of the pad"]
    #[inline(always)]
    pub fn sense2_fun_ie(&self) -> SENSE2_FUN_IE_R {
        SENSE2_FUN_IE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - the input enable of the pad in sleep status"]
    #[inline(always)]
    pub fn sense2_slp_ie(&self) -> SENSE2_SLP_IE_R {
        SENSE2_SLP_IE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - the sleep status selection signal of the pad"]
    #[inline(always)]
    pub fn sense2_slp_sel(&self) -> SENSE2_SLP_SEL_R {
        SENSE2_SLP_SEL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 17:18 - the functional selection signal of the pad"]
    #[inline(always)]
    pub fn sense2_fun_sel(&self) -> SENSE2_FUN_SEL_R {
        SENSE2_FUN_SEL_R::new(((self.bits >> 17) & 0x03) as u8)
    }
    #[doc = "Bit 19 - the input enable of the pad"]
    #[inline(always)]
    pub fn sense1_fun_ie(&self) -> SENSE1_FUN_IE_R {
        SENSE1_FUN_IE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - the input enable of the pad in sleep status"]
    #[inline(always)]
    pub fn sense1_slp_ie(&self) -> SENSE1_SLP_IE_R {
        SENSE1_SLP_IE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - the sleep status selection signal of the pad"]
    #[inline(always)]
    pub fn sense1_slp_sel(&self) -> SENSE1_SLP_SEL_R {
        SENSE1_SLP_SEL_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bits 22:23 - the functional selection signal of the pad"]
    #[inline(always)]
    pub fn sense1_fun_sel(&self) -> SENSE1_FUN_SEL_R {
        SENSE1_FUN_SEL_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bit 24 - Ò1Ó select the digital function Ó0Óslection the rtc function"]
    #[inline(always)]
    pub fn sense4_mux_sel(&self) -> SENSE4_MUX_SEL_R {
        SENSE4_MUX_SEL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Ò1Ó select the digital function Ó0Óslection the rtc function"]
    #[inline(always)]
    pub fn sense3_mux_sel(&self) -> SENSE3_MUX_SEL_R {
        SENSE3_MUX_SEL_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Ò1Ó select the digital function Ó0Óslection the rtc function"]
    #[inline(always)]
    pub fn sense2_mux_sel(&self) -> SENSE2_MUX_SEL_R {
        SENSE2_MUX_SEL_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Ò1Ó select the digital function Ó0Óslection the rtc function"]
    #[inline(always)]
    pub fn sense1_mux_sel(&self) -> SENSE1_MUX_SEL_R {
        SENSE1_MUX_SEL_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - hold the current value of the output when setting the hold to Ò1Ó"]
    #[inline(always)]
    pub fn sense4_hold(&self) -> SENSE4_HOLD_R {
        SENSE4_HOLD_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - hold the current value of the output when setting the hold to Ò1Ó"]
    #[inline(always)]
    pub fn sense3_hold(&self) -> SENSE3_HOLD_R {
        SENSE3_HOLD_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - hold the current value of the output when setting the hold to Ò1Ó"]
    #[inline(always)]
    pub fn sense2_hold(&self) -> SENSE2_HOLD_R {
        SENSE2_HOLD_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - hold the current value of the output when setting the hold to Ò1Ó"]
    #[inline(always)]
    pub fn sense1_hold(&self) -> SENSE1_HOLD_R {
        SENSE1_HOLD_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - the input enable of the pad"]
    #[inline(always)]
    pub fn sense4_fun_ie(&mut self) -> SENSE4_FUN_IE_W {
        SENSE4_FUN_IE_W { w: self }
    }
    #[doc = "Bit 5 - the input enable of the pad in sleep status"]
    #[inline(always)]
    pub fn sense4_slp_ie(&mut self) -> SENSE4_SLP_IE_W {
        SENSE4_SLP_IE_W { w: self }
    }
    #[doc = "Bit 6 - the sleep status selection signal of the pad"]
    #[inline(always)]
    pub fn sense4_slp_sel(&mut self) -> SENSE4_SLP_SEL_W {
        SENSE4_SLP_SEL_W { w: self }
    }
    #[doc = "Bits 7:8 - the functional selection signal of the pad"]
    #[inline(always)]
    pub fn sense4_fun_sel(&mut self) -> SENSE4_FUN_SEL_W {
        SENSE4_FUN_SEL_W { w: self }
    }
    #[doc = "Bit 9 - the input enable of the pad"]
    #[inline(always)]
    pub fn sense3_fun_ie(&mut self) -> SENSE3_FUN_IE_W {
        SENSE3_FUN_IE_W { w: self }
    }
    #[doc = "Bit 10 - the input enable of the pad in sleep status"]
    #[inline(always)]
    pub fn sense3_slp_ie(&mut self) -> SENSE3_SLP_IE_W {
        SENSE3_SLP_IE_W { w: self }
    }
    #[doc = "Bit 11 - the sleep status selection signal of the pad"]
    #[inline(always)]
    pub fn sense3_slp_sel(&mut self) -> SENSE3_SLP_SEL_W {
        SENSE3_SLP_SEL_W { w: self }
    }
    #[doc = "Bits 12:13 - the functional selection signal of the pad"]
    #[inline(always)]
    pub fn sense3_fun_sel(&mut self) -> SENSE3_FUN_SEL_W {
        SENSE3_FUN_SEL_W { w: self }
    }
    #[doc = "Bit 14 - the input enable of the pad"]
    #[inline(always)]
    pub fn sense2_fun_ie(&mut self) -> SENSE2_FUN_IE_W {
        SENSE2_FUN_IE_W { w: self }
    }
    #[doc = "Bit 15 - the input enable of the pad in sleep status"]
    #[inline(always)]
    pub fn sense2_slp_ie(&mut self) -> SENSE2_SLP_IE_W {
        SENSE2_SLP_IE_W { w: self }
    }
    #[doc = "Bit 16 - the sleep status selection signal of the pad"]
    #[inline(always)]
    pub fn sense2_slp_sel(&mut self) -> SENSE2_SLP_SEL_W {
        SENSE2_SLP_SEL_W { w: self }
    }
    #[doc = "Bits 17:18 - the functional selection signal of the pad"]
    #[inline(always)]
    pub fn sense2_fun_sel(&mut self) -> SENSE2_FUN_SEL_W {
        SENSE2_FUN_SEL_W { w: self }
    }
    #[doc = "Bit 19 - the input enable of the pad"]
    #[inline(always)]
    pub fn sense1_fun_ie(&mut self) -> SENSE1_FUN_IE_W {
        SENSE1_FUN_IE_W { w: self }
    }
    #[doc = "Bit 20 - the input enable of the pad in sleep status"]
    #[inline(always)]
    pub fn sense1_slp_ie(&mut self) -> SENSE1_SLP_IE_W {
        SENSE1_SLP_IE_W { w: self }
    }
    #[doc = "Bit 21 - the sleep status selection signal of the pad"]
    #[inline(always)]
    pub fn sense1_slp_sel(&mut self) -> SENSE1_SLP_SEL_W {
        SENSE1_SLP_SEL_W { w: self }
    }
    #[doc = "Bits 22:23 - the functional selection signal of the pad"]
    #[inline(always)]
    pub fn sense1_fun_sel(&mut self) -> SENSE1_FUN_SEL_W {
        SENSE1_FUN_SEL_W { w: self }
    }
    #[doc = "Bit 24 - Ò1Ó select the digital function Ó0Óslection the rtc function"]
    #[inline(always)]
    pub fn sense4_mux_sel(&mut self) -> SENSE4_MUX_SEL_W {
        SENSE4_MUX_SEL_W { w: self }
    }
    #[doc = "Bit 25 - Ò1Ó select the digital function Ó0Óslection the rtc function"]
    #[inline(always)]
    pub fn sense3_mux_sel(&mut self) -> SENSE3_MUX_SEL_W {
        SENSE3_MUX_SEL_W { w: self }
    }
    #[doc = "Bit 26 - Ò1Ó select the digital function Ó0Óslection the rtc function"]
    #[inline(always)]
    pub fn sense2_mux_sel(&mut self) -> SENSE2_MUX_SEL_W {
        SENSE2_MUX_SEL_W { w: self }
    }
    #[doc = "Bit 27 - Ò1Ó select the digital function Ó0Óslection the rtc function"]
    #[inline(always)]
    pub fn sense1_mux_sel(&mut self) -> SENSE1_MUX_SEL_W {
        SENSE1_MUX_SEL_W { w: self }
    }
    #[doc = "Bit 28 - hold the current value of the output when setting the hold to Ò1Ó"]
    #[inline(always)]
    pub fn sense4_hold(&mut self) -> SENSE4_HOLD_W {
        SENSE4_HOLD_W { w: self }
    }
    #[doc = "Bit 29 - hold the current value of the output when setting the hold to Ò1Ó"]
    #[inline(always)]
    pub fn sense3_hold(&mut self) -> SENSE3_HOLD_W {
        SENSE3_HOLD_W { w: self }
    }
    #[doc = "Bit 30 - hold the current value of the output when setting the hold to Ò1Ó"]
    #[inline(always)]
    pub fn sense2_hold(&mut self) -> SENSE2_HOLD_W {
        SENSE2_HOLD_W { w: self }
    }
    #[doc = "Bit 31 - hold the current value of the output when setting the hold to Ò1Ó"]
    #[inline(always)]
    pub fn sense1_hold(&mut self) -> SENSE1_HOLD_W {
        SENSE1_HOLD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensor_pads](index.html) module"]
pub struct SENSOR_PADS_SPEC;
impl crate::RegisterSpec for SENSOR_PADS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sensor_pads::R](R) reader structure"]
impl crate::Readable for SENSOR_PADS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sensor_pads::W](W) writer structure"]
impl crate::Writable for SENSOR_PADS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SENSOR_PADS to value 0"]
impl crate::Resettable for SENSOR_PADS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}