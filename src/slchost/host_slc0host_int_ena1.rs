#[doc = "Register `HOST_SLC0HOST_INT_ENA1` reader"]
pub struct R(crate::R<HOST_SLC0HOST_INT_ENA1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOST_SLC0HOST_INT_ENA1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOST_SLC0HOST_INT_ENA1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOST_SLC0HOST_INT_ENA1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HOST_SLC0HOST_INT_ENA1` writer"]
pub struct W(crate::W<HOST_SLC0HOST_INT_ENA1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HOST_SLC0HOST_INT_ENA1_SPEC>;
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
impl From<crate::W<HOST_SLC0HOST_INT_ENA1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HOST_SLC0HOST_INT_ENA1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HOST_SLC0_TOHOST_BIT0_INT_ENA1` reader - "]
pub struct HOST_SLC0_TOHOST_BIT0_INT_ENA1_R(crate::FieldReader<bool, bool>);
impl HOST_SLC0_TOHOST_BIT0_INT_ENA1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HOST_SLC0_TOHOST_BIT0_INT_ENA1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOST_SLC0_TOHOST_BIT0_INT_ENA1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOST_SLC0_TOHOST_BIT0_INT_ENA1` writer - "]
pub struct HOST_SLC0_TOHOST_BIT0_INT_ENA1_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC0_TOHOST_BIT0_INT_ENA1_W<'a> {
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
#[doc = "Field `HOST_SLC0_TOHOST_BIT1_INT_ENA1` reader - "]
pub struct HOST_SLC0_TOHOST_BIT1_INT_ENA1_R(crate::FieldReader<bool, bool>);
impl HOST_SLC0_TOHOST_BIT1_INT_ENA1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HOST_SLC0_TOHOST_BIT1_INT_ENA1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOST_SLC0_TOHOST_BIT1_INT_ENA1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOST_SLC0_TOHOST_BIT1_INT_ENA1` writer - "]
pub struct HOST_SLC0_TOHOST_BIT1_INT_ENA1_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC0_TOHOST_BIT1_INT_ENA1_W<'a> {
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
#[doc = "Field `HOST_SLC0_TOHOST_BIT2_INT_ENA1` reader - "]
pub struct HOST_SLC0_TOHOST_BIT2_INT_ENA1_R(crate::FieldReader<bool, bool>);
impl HOST_SLC0_TOHOST_BIT2_INT_ENA1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HOST_SLC0_TOHOST_BIT2_INT_ENA1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOST_SLC0_TOHOST_BIT2_INT_ENA1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOST_SLC0_TOHOST_BIT2_INT_ENA1` writer - "]
pub struct HOST_SLC0_TOHOST_BIT2_INT_ENA1_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC0_TOHOST_BIT2_INT_ENA1_W<'a> {
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
#[doc = "Field `HOST_SLC0_TOHOST_BIT3_INT_ENA1` reader - "]
pub struct HOST_SLC0_TOHOST_BIT3_INT_ENA1_R(crate::FieldReader<bool, bool>);
impl HOST_SLC0_TOHOST_BIT3_INT_ENA1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HOST_SLC0_TOHOST_BIT3_INT_ENA1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOST_SLC0_TOHOST_BIT3_INT_ENA1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOST_SLC0_TOHOST_BIT3_INT_ENA1` writer - "]
pub struct HOST_SLC0_TOHOST_BIT3_INT_ENA1_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC0_TOHOST_BIT3_INT_ENA1_W<'a> {
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
#[doc = "Field `HOST_SLC0_TOHOST_BIT4_INT_ENA1` reader - "]
pub struct HOST_SLC0_TOHOST_BIT4_INT_ENA1_R(crate::FieldReader<bool, bool>);
impl HOST_SLC0_TOHOST_BIT4_INT_ENA1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HOST_SLC0_TOHOST_BIT4_INT_ENA1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOST_SLC0_TOHOST_BIT4_INT_ENA1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOST_SLC0_TOHOST_BIT4_INT_ENA1` writer - "]
pub struct HOST_SLC0_TOHOST_BIT4_INT_ENA1_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC0_TOHOST_BIT4_INT_ENA1_W<'a> {
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
#[doc = "Field `HOST_SLC0_TOHOST_BIT5_INT_ENA1` reader - "]
pub struct HOST_SLC0_TOHOST_BIT5_INT_ENA1_R(crate::FieldReader<bool, bool>);
impl HOST_SLC0_TOHOST_BIT5_INT_ENA1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HOST_SLC0_TOHOST_BIT5_INT_ENA1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOST_SLC0_TOHOST_BIT5_INT_ENA1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOST_SLC0_TOHOST_BIT5_INT_ENA1` writer - "]
pub struct HOST_SLC0_TOHOST_BIT5_INT_ENA1_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC0_TOHOST_BIT5_INT_ENA1_W<'a> {
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
#[doc = "Field `HOST_SLC0_TOHOST_BIT6_INT_ENA1` reader - "]
pub struct HOST_SLC0_TOHOST_BIT6_INT_ENA1_R(crate::FieldReader<bool, bool>);
impl HOST_SLC0_TOHOST_BIT6_INT_ENA1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HOST_SLC0_TOHOST_BIT6_INT_ENA1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOST_SLC0_TOHOST_BIT6_INT_ENA1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOST_SLC0_TOHOST_BIT6_INT_ENA1` writer - "]
pub struct HOST_SLC0_TOHOST_BIT6_INT_ENA1_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC0_TOHOST_BIT6_INT_ENA1_W<'a> {
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
#[doc = "Field `HOST_SLC0_TOHOST_BIT7_INT_ENA1` reader - "]
pub struct HOST_SLC0_TOHOST_BIT7_INT_ENA1_R(crate::FieldReader<bool, bool>);
impl HOST_SLC0_TOHOST_BIT7_INT_ENA1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HOST_SLC0_TOHOST_BIT7_INT_ENA1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOST_SLC0_TOHOST_BIT7_INT_ENA1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOST_SLC0_TOHOST_BIT7_INT_ENA1` writer - "]
pub struct HOST_SLC0_TOHOST_BIT7_INT_ENA1_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC0_TOHOST_BIT7_INT_ENA1_W<'a> {
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
#[doc = "Field `HOST_SLC0_TOKEN0_1TO0_INT_ENA1` reader - "]
pub struct HOST_SLC0_TOKEN0_1TO0_INT_ENA1_R(crate::FieldReader<bool, bool>);
impl HOST_SLC0_TOKEN0_1TO0_INT_ENA1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HOST_SLC0_TOKEN0_1TO0_INT_ENA1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOST_SLC0_TOKEN0_1TO0_INT_ENA1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOST_SLC0_TOKEN0_1TO0_INT_ENA1` writer - "]
pub struct HOST_SLC0_TOKEN0_1TO0_INT_ENA1_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC0_TOKEN0_1TO0_INT_ENA1_W<'a> {
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
#[doc = "Field `HOST_SLC0_TOKEN1_1TO0_INT_ENA1` reader - "]
pub struct HOST_SLC0_TOKEN1_1TO0_INT_ENA1_R(crate::FieldReader<bool, bool>);
impl HOST_SLC0_TOKEN1_1TO0_INT_ENA1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HOST_SLC0_TOKEN1_1TO0_INT_ENA1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOST_SLC0_TOKEN1_1TO0_INT_ENA1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOST_SLC0_TOKEN1_1TO0_INT_ENA1` writer - "]
pub struct HOST_SLC0_TOKEN1_1TO0_INT_ENA1_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC0_TOKEN1_1TO0_INT_ENA1_W<'a> {
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
#[doc = "Field `HOST_SLC0_TOKEN0_0TO1_INT_ENA1` reader - "]
pub struct HOST_SLC0_TOKEN0_0TO1_INT_ENA1_R(crate::FieldReader<bool, bool>);
impl HOST_SLC0_TOKEN0_0TO1_INT_ENA1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HOST_SLC0_TOKEN0_0TO1_INT_ENA1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOST_SLC0_TOKEN0_0TO1_INT_ENA1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOST_SLC0_TOKEN0_0TO1_INT_ENA1` writer - "]
pub struct HOST_SLC0_TOKEN0_0TO1_INT_ENA1_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC0_TOKEN0_0TO1_INT_ENA1_W<'a> {
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
#[doc = "Field `HOST_SLC0_TOKEN1_0TO1_INT_ENA1` reader - "]
pub struct HOST_SLC0_TOKEN1_0TO1_INT_ENA1_R(crate::FieldReader<bool, bool>);
impl HOST_SLC0_TOKEN1_0TO1_INT_ENA1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HOST_SLC0_TOKEN1_0TO1_INT_ENA1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOST_SLC0_TOKEN1_0TO1_INT_ENA1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOST_SLC0_TOKEN1_0TO1_INT_ENA1` writer - "]
pub struct HOST_SLC0_TOKEN1_0TO1_INT_ENA1_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC0_TOKEN1_0TO1_INT_ENA1_W<'a> {
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
#[doc = "Field `HOST_SLC0HOST_RX_SOF_INT_ENA1` reader - "]
pub struct HOST_SLC0HOST_RX_SOF_INT_ENA1_R(crate::FieldReader<bool, bool>);
impl HOST_SLC0HOST_RX_SOF_INT_ENA1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HOST_SLC0HOST_RX_SOF_INT_ENA1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOST_SLC0HOST_RX_SOF_INT_ENA1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOST_SLC0HOST_RX_SOF_INT_ENA1` writer - "]
pub struct HOST_SLC0HOST_RX_SOF_INT_ENA1_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC0HOST_RX_SOF_INT_ENA1_W<'a> {
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
#[doc = "Field `HOST_SLC0HOST_RX_EOF_INT_ENA1` reader - "]
pub struct HOST_SLC0HOST_RX_EOF_INT_ENA1_R(crate::FieldReader<bool, bool>);
impl HOST_SLC0HOST_RX_EOF_INT_ENA1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HOST_SLC0HOST_RX_EOF_INT_ENA1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOST_SLC0HOST_RX_EOF_INT_ENA1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOST_SLC0HOST_RX_EOF_INT_ENA1` writer - "]
pub struct HOST_SLC0HOST_RX_EOF_INT_ENA1_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC0HOST_RX_EOF_INT_ENA1_W<'a> {
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
#[doc = "Field `HOST_SLC0HOST_RX_START_INT_ENA1` reader - "]
pub struct HOST_SLC0HOST_RX_START_INT_ENA1_R(crate::FieldReader<bool, bool>);
impl HOST_SLC0HOST_RX_START_INT_ENA1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HOST_SLC0HOST_RX_START_INT_ENA1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOST_SLC0HOST_RX_START_INT_ENA1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOST_SLC0HOST_RX_START_INT_ENA1` writer - "]
pub struct HOST_SLC0HOST_RX_START_INT_ENA1_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC0HOST_RX_START_INT_ENA1_W<'a> {
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
#[doc = "Field `HOST_SLC0HOST_TX_START_INT_ENA1` reader - "]
pub struct HOST_SLC0HOST_TX_START_INT_ENA1_R(crate::FieldReader<bool, bool>);
impl HOST_SLC0HOST_TX_START_INT_ENA1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HOST_SLC0HOST_TX_START_INT_ENA1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOST_SLC0HOST_TX_START_INT_ENA1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOST_SLC0HOST_TX_START_INT_ENA1` writer - "]
pub struct HOST_SLC0HOST_TX_START_INT_ENA1_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC0HOST_TX_START_INT_ENA1_W<'a> {
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
#[doc = "Field `HOST_SLC0_RX_UDF_INT_ENA1` reader - "]
pub struct HOST_SLC0_RX_UDF_INT_ENA1_R(crate::FieldReader<bool, bool>);
impl HOST_SLC0_RX_UDF_INT_ENA1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HOST_SLC0_RX_UDF_INT_ENA1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOST_SLC0_RX_UDF_INT_ENA1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOST_SLC0_RX_UDF_INT_ENA1` writer - "]
pub struct HOST_SLC0_RX_UDF_INT_ENA1_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC0_RX_UDF_INT_ENA1_W<'a> {
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
#[doc = "Field `HOST_SLC0_TX_OVF_INT_ENA1` reader - "]
pub struct HOST_SLC0_TX_OVF_INT_ENA1_R(crate::FieldReader<bool, bool>);
impl HOST_SLC0_TX_OVF_INT_ENA1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HOST_SLC0_TX_OVF_INT_ENA1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOST_SLC0_TX_OVF_INT_ENA1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOST_SLC0_TX_OVF_INT_ENA1` writer - "]
pub struct HOST_SLC0_TX_OVF_INT_ENA1_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC0_TX_OVF_INT_ENA1_W<'a> {
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
#[doc = "Field `HOST_SLC0_RX_PF_VALID_INT_ENA1` reader - "]
pub struct HOST_SLC0_RX_PF_VALID_INT_ENA1_R(crate::FieldReader<bool, bool>);
impl HOST_SLC0_RX_PF_VALID_INT_ENA1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HOST_SLC0_RX_PF_VALID_INT_ENA1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOST_SLC0_RX_PF_VALID_INT_ENA1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOST_SLC0_RX_PF_VALID_INT_ENA1` writer - "]
pub struct HOST_SLC0_RX_PF_VALID_INT_ENA1_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC0_RX_PF_VALID_INT_ENA1_W<'a> {
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
#[doc = "Field `HOST_SLC0_EXT_BIT0_INT_ENA1` reader - "]
pub struct HOST_SLC0_EXT_BIT0_INT_ENA1_R(crate::FieldReader<bool, bool>);
impl HOST_SLC0_EXT_BIT0_INT_ENA1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HOST_SLC0_EXT_BIT0_INT_ENA1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOST_SLC0_EXT_BIT0_INT_ENA1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOST_SLC0_EXT_BIT0_INT_ENA1` writer - "]
pub struct HOST_SLC0_EXT_BIT0_INT_ENA1_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC0_EXT_BIT0_INT_ENA1_W<'a> {
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
#[doc = "Field `HOST_SLC0_EXT_BIT1_INT_ENA1` reader - "]
pub struct HOST_SLC0_EXT_BIT1_INT_ENA1_R(crate::FieldReader<bool, bool>);
impl HOST_SLC0_EXT_BIT1_INT_ENA1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HOST_SLC0_EXT_BIT1_INT_ENA1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOST_SLC0_EXT_BIT1_INT_ENA1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOST_SLC0_EXT_BIT1_INT_ENA1` writer - "]
pub struct HOST_SLC0_EXT_BIT1_INT_ENA1_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC0_EXT_BIT1_INT_ENA1_W<'a> {
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
#[doc = "Field `HOST_SLC0_EXT_BIT2_INT_ENA1` reader - "]
pub struct HOST_SLC0_EXT_BIT2_INT_ENA1_R(crate::FieldReader<bool, bool>);
impl HOST_SLC0_EXT_BIT2_INT_ENA1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HOST_SLC0_EXT_BIT2_INT_ENA1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOST_SLC0_EXT_BIT2_INT_ENA1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOST_SLC0_EXT_BIT2_INT_ENA1` writer - "]
pub struct HOST_SLC0_EXT_BIT2_INT_ENA1_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC0_EXT_BIT2_INT_ENA1_W<'a> {
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
#[doc = "Field `HOST_SLC0_EXT_BIT3_INT_ENA1` reader - "]
pub struct HOST_SLC0_EXT_BIT3_INT_ENA1_R(crate::FieldReader<bool, bool>);
impl HOST_SLC0_EXT_BIT3_INT_ENA1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HOST_SLC0_EXT_BIT3_INT_ENA1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOST_SLC0_EXT_BIT3_INT_ENA1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOST_SLC0_EXT_BIT3_INT_ENA1` writer - "]
pub struct HOST_SLC0_EXT_BIT3_INT_ENA1_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC0_EXT_BIT3_INT_ENA1_W<'a> {
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
#[doc = "Field `HOST_SLC0_RX_NEW_PACKET_INT_ENA1` reader - "]
pub struct HOST_SLC0_RX_NEW_PACKET_INT_ENA1_R(crate::FieldReader<bool, bool>);
impl HOST_SLC0_RX_NEW_PACKET_INT_ENA1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HOST_SLC0_RX_NEW_PACKET_INT_ENA1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOST_SLC0_RX_NEW_PACKET_INT_ENA1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOST_SLC0_RX_NEW_PACKET_INT_ENA1` writer - "]
pub struct HOST_SLC0_RX_NEW_PACKET_INT_ENA1_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC0_RX_NEW_PACKET_INT_ENA1_W<'a> {
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
#[doc = "Field `HOST_SLC0_HOST_RD_RETRY_INT_ENA1` reader - "]
pub struct HOST_SLC0_HOST_RD_RETRY_INT_ENA1_R(crate::FieldReader<bool, bool>);
impl HOST_SLC0_HOST_RD_RETRY_INT_ENA1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HOST_SLC0_HOST_RD_RETRY_INT_ENA1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOST_SLC0_HOST_RD_RETRY_INT_ENA1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOST_SLC0_HOST_RD_RETRY_INT_ENA1` writer - "]
pub struct HOST_SLC0_HOST_RD_RETRY_INT_ENA1_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC0_HOST_RD_RETRY_INT_ENA1_W<'a> {
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
#[doc = "Field `HOST_GPIO_SDIO_INT_ENA1` reader - "]
pub struct HOST_GPIO_SDIO_INT_ENA1_R(crate::FieldReader<bool, bool>);
impl HOST_GPIO_SDIO_INT_ENA1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HOST_GPIO_SDIO_INT_ENA1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOST_GPIO_SDIO_INT_ENA1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOST_GPIO_SDIO_INT_ENA1` writer - "]
pub struct HOST_GPIO_SDIO_INT_ENA1_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_GPIO_SDIO_INT_ENA1_W<'a> {
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
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn host_slc0_tohost_bit0_int_ena1(&self) -> HOST_SLC0_TOHOST_BIT0_INT_ENA1_R {
        HOST_SLC0_TOHOST_BIT0_INT_ENA1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn host_slc0_tohost_bit1_int_ena1(&self) -> HOST_SLC0_TOHOST_BIT1_INT_ENA1_R {
        HOST_SLC0_TOHOST_BIT1_INT_ENA1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn host_slc0_tohost_bit2_int_ena1(&self) -> HOST_SLC0_TOHOST_BIT2_INT_ENA1_R {
        HOST_SLC0_TOHOST_BIT2_INT_ENA1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn host_slc0_tohost_bit3_int_ena1(&self) -> HOST_SLC0_TOHOST_BIT3_INT_ENA1_R {
        HOST_SLC0_TOHOST_BIT3_INT_ENA1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn host_slc0_tohost_bit4_int_ena1(&self) -> HOST_SLC0_TOHOST_BIT4_INT_ENA1_R {
        HOST_SLC0_TOHOST_BIT4_INT_ENA1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn host_slc0_tohost_bit5_int_ena1(&self) -> HOST_SLC0_TOHOST_BIT5_INT_ENA1_R {
        HOST_SLC0_TOHOST_BIT5_INT_ENA1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn host_slc0_tohost_bit6_int_ena1(&self) -> HOST_SLC0_TOHOST_BIT6_INT_ENA1_R {
        HOST_SLC0_TOHOST_BIT6_INT_ENA1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn host_slc0_tohost_bit7_int_ena1(&self) -> HOST_SLC0_TOHOST_BIT7_INT_ENA1_R {
        HOST_SLC0_TOHOST_BIT7_INT_ENA1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn host_slc0_token0_1to0_int_ena1(&self) -> HOST_SLC0_TOKEN0_1TO0_INT_ENA1_R {
        HOST_SLC0_TOKEN0_1TO0_INT_ENA1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn host_slc0_token1_1to0_int_ena1(&self) -> HOST_SLC0_TOKEN1_1TO0_INT_ENA1_R {
        HOST_SLC0_TOKEN1_1TO0_INT_ENA1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn host_slc0_token0_0to1_int_ena1(&self) -> HOST_SLC0_TOKEN0_0TO1_INT_ENA1_R {
        HOST_SLC0_TOKEN0_0TO1_INT_ENA1_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn host_slc0_token1_0to1_int_ena1(&self) -> HOST_SLC0_TOKEN1_0TO1_INT_ENA1_R {
        HOST_SLC0_TOKEN1_0TO1_INT_ENA1_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn host_slc0host_rx_sof_int_ena1(&self) -> HOST_SLC0HOST_RX_SOF_INT_ENA1_R {
        HOST_SLC0HOST_RX_SOF_INT_ENA1_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn host_slc0host_rx_eof_int_ena1(&self) -> HOST_SLC0HOST_RX_EOF_INT_ENA1_R {
        HOST_SLC0HOST_RX_EOF_INT_ENA1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn host_slc0host_rx_start_int_ena1(&self) -> HOST_SLC0HOST_RX_START_INT_ENA1_R {
        HOST_SLC0HOST_RX_START_INT_ENA1_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn host_slc0host_tx_start_int_ena1(&self) -> HOST_SLC0HOST_TX_START_INT_ENA1_R {
        HOST_SLC0HOST_TX_START_INT_ENA1_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn host_slc0_rx_udf_int_ena1(&self) -> HOST_SLC0_RX_UDF_INT_ENA1_R {
        HOST_SLC0_RX_UDF_INT_ENA1_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn host_slc0_tx_ovf_int_ena1(&self) -> HOST_SLC0_TX_OVF_INT_ENA1_R {
        HOST_SLC0_TX_OVF_INT_ENA1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn host_slc0_rx_pf_valid_int_ena1(&self) -> HOST_SLC0_RX_PF_VALID_INT_ENA1_R {
        HOST_SLC0_RX_PF_VALID_INT_ENA1_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn host_slc0_ext_bit0_int_ena1(&self) -> HOST_SLC0_EXT_BIT0_INT_ENA1_R {
        HOST_SLC0_EXT_BIT0_INT_ENA1_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn host_slc0_ext_bit1_int_ena1(&self) -> HOST_SLC0_EXT_BIT1_INT_ENA1_R {
        HOST_SLC0_EXT_BIT1_INT_ENA1_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn host_slc0_ext_bit2_int_ena1(&self) -> HOST_SLC0_EXT_BIT2_INT_ENA1_R {
        HOST_SLC0_EXT_BIT2_INT_ENA1_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn host_slc0_ext_bit3_int_ena1(&self) -> HOST_SLC0_EXT_BIT3_INT_ENA1_R {
        HOST_SLC0_EXT_BIT3_INT_ENA1_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn host_slc0_rx_new_packet_int_ena1(&self) -> HOST_SLC0_RX_NEW_PACKET_INT_ENA1_R {
        HOST_SLC0_RX_NEW_PACKET_INT_ENA1_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn host_slc0_host_rd_retry_int_ena1(&self) -> HOST_SLC0_HOST_RD_RETRY_INT_ENA1_R {
        HOST_SLC0_HOST_RD_RETRY_INT_ENA1_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn host_gpio_sdio_int_ena1(&self) -> HOST_GPIO_SDIO_INT_ENA1_R {
        HOST_GPIO_SDIO_INT_ENA1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn host_slc0_tohost_bit0_int_ena1(&mut self) -> HOST_SLC0_TOHOST_BIT0_INT_ENA1_W {
        HOST_SLC0_TOHOST_BIT0_INT_ENA1_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn host_slc0_tohost_bit1_int_ena1(&mut self) -> HOST_SLC0_TOHOST_BIT1_INT_ENA1_W {
        HOST_SLC0_TOHOST_BIT1_INT_ENA1_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn host_slc0_tohost_bit2_int_ena1(&mut self) -> HOST_SLC0_TOHOST_BIT2_INT_ENA1_W {
        HOST_SLC0_TOHOST_BIT2_INT_ENA1_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn host_slc0_tohost_bit3_int_ena1(&mut self) -> HOST_SLC0_TOHOST_BIT3_INT_ENA1_W {
        HOST_SLC0_TOHOST_BIT3_INT_ENA1_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn host_slc0_tohost_bit4_int_ena1(&mut self) -> HOST_SLC0_TOHOST_BIT4_INT_ENA1_W {
        HOST_SLC0_TOHOST_BIT4_INT_ENA1_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn host_slc0_tohost_bit5_int_ena1(&mut self) -> HOST_SLC0_TOHOST_BIT5_INT_ENA1_W {
        HOST_SLC0_TOHOST_BIT5_INT_ENA1_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn host_slc0_tohost_bit6_int_ena1(&mut self) -> HOST_SLC0_TOHOST_BIT6_INT_ENA1_W {
        HOST_SLC0_TOHOST_BIT6_INT_ENA1_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn host_slc0_tohost_bit7_int_ena1(&mut self) -> HOST_SLC0_TOHOST_BIT7_INT_ENA1_W {
        HOST_SLC0_TOHOST_BIT7_INT_ENA1_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn host_slc0_token0_1to0_int_ena1(&mut self) -> HOST_SLC0_TOKEN0_1TO0_INT_ENA1_W {
        HOST_SLC0_TOKEN0_1TO0_INT_ENA1_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn host_slc0_token1_1to0_int_ena1(&mut self) -> HOST_SLC0_TOKEN1_1TO0_INT_ENA1_W {
        HOST_SLC0_TOKEN1_1TO0_INT_ENA1_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn host_slc0_token0_0to1_int_ena1(&mut self) -> HOST_SLC0_TOKEN0_0TO1_INT_ENA1_W {
        HOST_SLC0_TOKEN0_0TO1_INT_ENA1_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn host_slc0_token1_0to1_int_ena1(&mut self) -> HOST_SLC0_TOKEN1_0TO1_INT_ENA1_W {
        HOST_SLC0_TOKEN1_0TO1_INT_ENA1_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn host_slc0host_rx_sof_int_ena1(&mut self) -> HOST_SLC0HOST_RX_SOF_INT_ENA1_W {
        HOST_SLC0HOST_RX_SOF_INT_ENA1_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn host_slc0host_rx_eof_int_ena1(&mut self) -> HOST_SLC0HOST_RX_EOF_INT_ENA1_W {
        HOST_SLC0HOST_RX_EOF_INT_ENA1_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn host_slc0host_rx_start_int_ena1(&mut self) -> HOST_SLC0HOST_RX_START_INT_ENA1_W {
        HOST_SLC0HOST_RX_START_INT_ENA1_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn host_slc0host_tx_start_int_ena1(&mut self) -> HOST_SLC0HOST_TX_START_INT_ENA1_W {
        HOST_SLC0HOST_TX_START_INT_ENA1_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn host_slc0_rx_udf_int_ena1(&mut self) -> HOST_SLC0_RX_UDF_INT_ENA1_W {
        HOST_SLC0_RX_UDF_INT_ENA1_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn host_slc0_tx_ovf_int_ena1(&mut self) -> HOST_SLC0_TX_OVF_INT_ENA1_W {
        HOST_SLC0_TX_OVF_INT_ENA1_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn host_slc0_rx_pf_valid_int_ena1(&mut self) -> HOST_SLC0_RX_PF_VALID_INT_ENA1_W {
        HOST_SLC0_RX_PF_VALID_INT_ENA1_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn host_slc0_ext_bit0_int_ena1(&mut self) -> HOST_SLC0_EXT_BIT0_INT_ENA1_W {
        HOST_SLC0_EXT_BIT0_INT_ENA1_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn host_slc0_ext_bit1_int_ena1(&mut self) -> HOST_SLC0_EXT_BIT1_INT_ENA1_W {
        HOST_SLC0_EXT_BIT1_INT_ENA1_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn host_slc0_ext_bit2_int_ena1(&mut self) -> HOST_SLC0_EXT_BIT2_INT_ENA1_W {
        HOST_SLC0_EXT_BIT2_INT_ENA1_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn host_slc0_ext_bit3_int_ena1(&mut self) -> HOST_SLC0_EXT_BIT3_INT_ENA1_W {
        HOST_SLC0_EXT_BIT3_INT_ENA1_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn host_slc0_rx_new_packet_int_ena1(&mut self) -> HOST_SLC0_RX_NEW_PACKET_INT_ENA1_W {
        HOST_SLC0_RX_NEW_PACKET_INT_ENA1_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn host_slc0_host_rd_retry_int_ena1(&mut self) -> HOST_SLC0_HOST_RD_RETRY_INT_ENA1_W {
        HOST_SLC0_HOST_RD_RETRY_INT_ENA1_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn host_gpio_sdio_int_ena1(&mut self) -> HOST_GPIO_SDIO_INT_ENA1_W {
        HOST_GPIO_SDIO_INT_ENA1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_slc0host_int_ena1](index.html) module"]
pub struct HOST_SLC0HOST_INT_ENA1_SPEC;
impl crate::RegisterSpec for HOST_SLC0HOST_INT_ENA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [host_slc0host_int_ena1::R](R) reader structure"]
impl crate::Readable for HOST_SLC0HOST_INT_ENA1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [host_slc0host_int_ena1::W](W) writer structure"]
impl crate::Writable for HOST_SLC0HOST_INT_ENA1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HOST_SLC0HOST_INT_ENA1 to value 0"]
impl crate::Resettable for HOST_SLC0HOST_INT_ENA1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
