#[doc = "Register `HOLD_FORCE` reader"]
pub struct R(crate::R<HOLD_FORCE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOLD_FORCE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOLD_FORCE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOLD_FORCE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HOLD_FORCE` writer"]
pub struct W(crate::W<HOLD_FORCE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HOLD_FORCE_SPEC>;
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
impl From<crate::W<HOLD_FORCE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HOLD_FORCE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC1_HOLD_FORCE` reader - "]
pub struct ADC1_HOLD_FORCE_R(crate::FieldReader<bool, bool>);
impl ADC1_HOLD_FORCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC1_HOLD_FORCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC1_HOLD_FORCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC1_HOLD_FORCE` writer - "]
pub struct ADC1_HOLD_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC1_HOLD_FORCE_W<'a> {
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
#[doc = "Field `ADC2_HOLD_FORCE` reader - "]
pub struct ADC2_HOLD_FORCE_R(crate::FieldReader<bool, bool>);
impl ADC2_HOLD_FORCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC2_HOLD_FORCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC2_HOLD_FORCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC2_HOLD_FORCE` writer - "]
pub struct ADC2_HOLD_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC2_HOLD_FORCE_W<'a> {
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
#[doc = "Field `PDAC1_HOLD_FORCE` reader - "]
pub struct PDAC1_HOLD_FORCE_R(crate::FieldReader<bool, bool>);
impl PDAC1_HOLD_FORCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDAC1_HOLD_FORCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDAC1_HOLD_FORCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDAC1_HOLD_FORCE` writer - "]
pub struct PDAC1_HOLD_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> PDAC1_HOLD_FORCE_W<'a> {
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
#[doc = "Field `PDAC2_HOLD_FORCE` reader - "]
pub struct PDAC2_HOLD_FORCE_R(crate::FieldReader<bool, bool>);
impl PDAC2_HOLD_FORCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDAC2_HOLD_FORCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDAC2_HOLD_FORCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDAC2_HOLD_FORCE` writer - "]
pub struct PDAC2_HOLD_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> PDAC2_HOLD_FORCE_W<'a> {
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
#[doc = "Field `SENSE1_HOLD_FORCE` reader - "]
pub struct SENSE1_HOLD_FORCE_R(crate::FieldReader<bool, bool>);
impl SENSE1_HOLD_FORCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SENSE1_HOLD_FORCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SENSE1_HOLD_FORCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SENSE1_HOLD_FORCE` writer - "]
pub struct SENSE1_HOLD_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSE1_HOLD_FORCE_W<'a> {
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
#[doc = "Field `SENSE2_HOLD_FORCE` reader - "]
pub struct SENSE2_HOLD_FORCE_R(crate::FieldReader<bool, bool>);
impl SENSE2_HOLD_FORCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SENSE2_HOLD_FORCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SENSE2_HOLD_FORCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SENSE2_HOLD_FORCE` writer - "]
pub struct SENSE2_HOLD_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSE2_HOLD_FORCE_W<'a> {
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
#[doc = "Field `SENSE3_HOLD_FORCE` reader - "]
pub struct SENSE3_HOLD_FORCE_R(crate::FieldReader<bool, bool>);
impl SENSE3_HOLD_FORCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SENSE3_HOLD_FORCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SENSE3_HOLD_FORCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SENSE3_HOLD_FORCE` writer - "]
pub struct SENSE3_HOLD_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSE3_HOLD_FORCE_W<'a> {
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
#[doc = "Field `SENSE4_HOLD_FORCE` reader - "]
pub struct SENSE4_HOLD_FORCE_R(crate::FieldReader<bool, bool>);
impl SENSE4_HOLD_FORCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SENSE4_HOLD_FORCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SENSE4_HOLD_FORCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SENSE4_HOLD_FORCE` writer - "]
pub struct SENSE4_HOLD_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSE4_HOLD_FORCE_W<'a> {
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
#[doc = "Field `TOUCH_PAD0_HOLD_FORCE` reader - "]
pub struct TOUCH_PAD0_HOLD_FORCE_R(crate::FieldReader<bool, bool>);
impl TOUCH_PAD0_HOLD_FORCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOUCH_PAD0_HOLD_FORCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_PAD0_HOLD_FORCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_PAD0_HOLD_FORCE` writer - "]
pub struct TOUCH_PAD0_HOLD_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_PAD0_HOLD_FORCE_W<'a> {
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
#[doc = "Field `TOUCH_PAD1_HOLD_FORCE` reader - "]
pub struct TOUCH_PAD1_HOLD_FORCE_R(crate::FieldReader<bool, bool>);
impl TOUCH_PAD1_HOLD_FORCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOUCH_PAD1_HOLD_FORCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_PAD1_HOLD_FORCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_PAD1_HOLD_FORCE` writer - "]
pub struct TOUCH_PAD1_HOLD_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_PAD1_HOLD_FORCE_W<'a> {
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
#[doc = "Field `TOUCH_PAD2_HOLD_FORCE` reader - "]
pub struct TOUCH_PAD2_HOLD_FORCE_R(crate::FieldReader<bool, bool>);
impl TOUCH_PAD2_HOLD_FORCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOUCH_PAD2_HOLD_FORCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_PAD2_HOLD_FORCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_PAD2_HOLD_FORCE` writer - "]
pub struct TOUCH_PAD2_HOLD_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_PAD2_HOLD_FORCE_W<'a> {
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
#[doc = "Field `TOUCH_PAD3_HOLD_FORCE` reader - "]
pub struct TOUCH_PAD3_HOLD_FORCE_R(crate::FieldReader<bool, bool>);
impl TOUCH_PAD3_HOLD_FORCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOUCH_PAD3_HOLD_FORCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_PAD3_HOLD_FORCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_PAD3_HOLD_FORCE` writer - "]
pub struct TOUCH_PAD3_HOLD_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_PAD3_HOLD_FORCE_W<'a> {
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
#[doc = "Field `TOUCH_PAD4_HOLD_FORCE` reader - "]
pub struct TOUCH_PAD4_HOLD_FORCE_R(crate::FieldReader<bool, bool>);
impl TOUCH_PAD4_HOLD_FORCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOUCH_PAD4_HOLD_FORCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_PAD4_HOLD_FORCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_PAD4_HOLD_FORCE` writer - "]
pub struct TOUCH_PAD4_HOLD_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_PAD4_HOLD_FORCE_W<'a> {
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
#[doc = "Field `TOUCH_PAD5_HOLD_FORCE` reader - "]
pub struct TOUCH_PAD5_HOLD_FORCE_R(crate::FieldReader<bool, bool>);
impl TOUCH_PAD5_HOLD_FORCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOUCH_PAD5_HOLD_FORCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_PAD5_HOLD_FORCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_PAD5_HOLD_FORCE` writer - "]
pub struct TOUCH_PAD5_HOLD_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_PAD5_HOLD_FORCE_W<'a> {
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
#[doc = "Field `TOUCH_PAD6_HOLD_FORCE` reader - "]
pub struct TOUCH_PAD6_HOLD_FORCE_R(crate::FieldReader<bool, bool>);
impl TOUCH_PAD6_HOLD_FORCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOUCH_PAD6_HOLD_FORCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_PAD6_HOLD_FORCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_PAD6_HOLD_FORCE` writer - "]
pub struct TOUCH_PAD6_HOLD_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_PAD6_HOLD_FORCE_W<'a> {
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
#[doc = "Field `TOUCH_PAD7_HOLD_FORCE` reader - "]
pub struct TOUCH_PAD7_HOLD_FORCE_R(crate::FieldReader<bool, bool>);
impl TOUCH_PAD7_HOLD_FORCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOUCH_PAD7_HOLD_FORCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_PAD7_HOLD_FORCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_PAD7_HOLD_FORCE` writer - "]
pub struct TOUCH_PAD7_HOLD_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_PAD7_HOLD_FORCE_W<'a> {
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
#[doc = "Field `X32P_HOLD_FORCE` reader - "]
pub struct X32P_HOLD_FORCE_R(crate::FieldReader<bool, bool>);
impl X32P_HOLD_FORCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        X32P_HOLD_FORCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for X32P_HOLD_FORCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `X32P_HOLD_FORCE` writer - "]
pub struct X32P_HOLD_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> X32P_HOLD_FORCE_W<'a> {
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
#[doc = "Field `X32N_HOLD_FORCE` reader - "]
pub struct X32N_HOLD_FORCE_R(crate::FieldReader<bool, bool>);
impl X32N_HOLD_FORCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        X32N_HOLD_FORCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for X32N_HOLD_FORCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `X32N_HOLD_FORCE` writer - "]
pub struct X32N_HOLD_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> X32N_HOLD_FORCE_W<'a> {
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
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn adc1_hold_force(&self) -> ADC1_HOLD_FORCE_R {
        ADC1_HOLD_FORCE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn adc2_hold_force(&self) -> ADC2_HOLD_FORCE_R {
        ADC2_HOLD_FORCE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pdac1_hold_force(&self) -> PDAC1_HOLD_FORCE_R {
        PDAC1_HOLD_FORCE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pdac2_hold_force(&self) -> PDAC2_HOLD_FORCE_R {
        PDAC2_HOLD_FORCE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sense1_hold_force(&self) -> SENSE1_HOLD_FORCE_R {
        SENSE1_HOLD_FORCE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn sense2_hold_force(&self) -> SENSE2_HOLD_FORCE_R {
        SENSE2_HOLD_FORCE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn sense3_hold_force(&self) -> SENSE3_HOLD_FORCE_R {
        SENSE3_HOLD_FORCE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn sense4_hold_force(&self) -> SENSE4_HOLD_FORCE_R {
        SENSE4_HOLD_FORCE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn touch_pad0_hold_force(&self) -> TOUCH_PAD0_HOLD_FORCE_R {
        TOUCH_PAD0_HOLD_FORCE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn touch_pad1_hold_force(&self) -> TOUCH_PAD1_HOLD_FORCE_R {
        TOUCH_PAD1_HOLD_FORCE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn touch_pad2_hold_force(&self) -> TOUCH_PAD2_HOLD_FORCE_R {
        TOUCH_PAD2_HOLD_FORCE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn touch_pad3_hold_force(&self) -> TOUCH_PAD3_HOLD_FORCE_R {
        TOUCH_PAD3_HOLD_FORCE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn touch_pad4_hold_force(&self) -> TOUCH_PAD4_HOLD_FORCE_R {
        TOUCH_PAD4_HOLD_FORCE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn touch_pad5_hold_force(&self) -> TOUCH_PAD5_HOLD_FORCE_R {
        TOUCH_PAD5_HOLD_FORCE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn touch_pad6_hold_force(&self) -> TOUCH_PAD6_HOLD_FORCE_R {
        TOUCH_PAD6_HOLD_FORCE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn touch_pad7_hold_force(&self) -> TOUCH_PAD7_HOLD_FORCE_R {
        TOUCH_PAD7_HOLD_FORCE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn x32p_hold_force(&self) -> X32P_HOLD_FORCE_R {
        X32P_HOLD_FORCE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn x32n_hold_force(&self) -> X32N_HOLD_FORCE_R {
        X32N_HOLD_FORCE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn adc1_hold_force(&mut self) -> ADC1_HOLD_FORCE_W {
        ADC1_HOLD_FORCE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn adc2_hold_force(&mut self) -> ADC2_HOLD_FORCE_W {
        ADC2_HOLD_FORCE_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pdac1_hold_force(&mut self) -> PDAC1_HOLD_FORCE_W {
        PDAC1_HOLD_FORCE_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pdac2_hold_force(&mut self) -> PDAC2_HOLD_FORCE_W {
        PDAC2_HOLD_FORCE_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sense1_hold_force(&mut self) -> SENSE1_HOLD_FORCE_W {
        SENSE1_HOLD_FORCE_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn sense2_hold_force(&mut self) -> SENSE2_HOLD_FORCE_W {
        SENSE2_HOLD_FORCE_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn sense3_hold_force(&mut self) -> SENSE3_HOLD_FORCE_W {
        SENSE3_HOLD_FORCE_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn sense4_hold_force(&mut self) -> SENSE4_HOLD_FORCE_W {
        SENSE4_HOLD_FORCE_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn touch_pad0_hold_force(&mut self) -> TOUCH_PAD0_HOLD_FORCE_W {
        TOUCH_PAD0_HOLD_FORCE_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn touch_pad1_hold_force(&mut self) -> TOUCH_PAD1_HOLD_FORCE_W {
        TOUCH_PAD1_HOLD_FORCE_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn touch_pad2_hold_force(&mut self) -> TOUCH_PAD2_HOLD_FORCE_W {
        TOUCH_PAD2_HOLD_FORCE_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn touch_pad3_hold_force(&mut self) -> TOUCH_PAD3_HOLD_FORCE_W {
        TOUCH_PAD3_HOLD_FORCE_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn touch_pad4_hold_force(&mut self) -> TOUCH_PAD4_HOLD_FORCE_W {
        TOUCH_PAD4_HOLD_FORCE_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn touch_pad5_hold_force(&mut self) -> TOUCH_PAD5_HOLD_FORCE_W {
        TOUCH_PAD5_HOLD_FORCE_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn touch_pad6_hold_force(&mut self) -> TOUCH_PAD6_HOLD_FORCE_W {
        TOUCH_PAD6_HOLD_FORCE_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn touch_pad7_hold_force(&mut self) -> TOUCH_PAD7_HOLD_FORCE_W {
        TOUCH_PAD7_HOLD_FORCE_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn x32p_hold_force(&mut self) -> X32P_HOLD_FORCE_W {
        X32P_HOLD_FORCE_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn x32n_hold_force(&mut self) -> X32N_HOLD_FORCE_W {
        X32N_HOLD_FORCE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hold_force](index.html) module"]
pub struct HOLD_FORCE_SPEC;
impl crate::RegisterSpec for HOLD_FORCE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hold_force::R](R) reader structure"]
impl crate::Readable for HOLD_FORCE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hold_force::W](W) writer structure"]
impl crate::Writable for HOLD_FORCE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HOLD_FORCE to value 0"]
impl crate::Resettable for HOLD_FORCE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
