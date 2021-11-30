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
#[doc = "Field `HSTIMER0_OVF_INT_ENA` reader - The interrupt enable bit for high speed channel0 counter overflow interrupt."]
pub struct HSTIMER0_OVF_INT_ENA_R(crate::FieldReader<bool, bool>);
impl HSTIMER0_OVF_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HSTIMER0_OVF_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSTIMER0_OVF_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSTIMER0_OVF_INT_ENA` writer - The interrupt enable bit for high speed channel0 counter overflow interrupt."]
pub struct HSTIMER0_OVF_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTIMER0_OVF_INT_ENA_W<'a> {
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
#[doc = "Field `HSTIMER1_OVF_INT_ENA` reader - The interrupt enable bit for high speed channel1 counter overflow interrupt."]
pub struct HSTIMER1_OVF_INT_ENA_R(crate::FieldReader<bool, bool>);
impl HSTIMER1_OVF_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HSTIMER1_OVF_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSTIMER1_OVF_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSTIMER1_OVF_INT_ENA` writer - The interrupt enable bit for high speed channel1 counter overflow interrupt."]
pub struct HSTIMER1_OVF_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTIMER1_OVF_INT_ENA_W<'a> {
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
#[doc = "Field `HSTIMER2_OVF_INT_ENA` reader - The interrupt enable bit for high speed channel2 counter overflow interrupt."]
pub struct HSTIMER2_OVF_INT_ENA_R(crate::FieldReader<bool, bool>);
impl HSTIMER2_OVF_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HSTIMER2_OVF_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSTIMER2_OVF_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSTIMER2_OVF_INT_ENA` writer - The interrupt enable bit for high speed channel2 counter overflow interrupt."]
pub struct HSTIMER2_OVF_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTIMER2_OVF_INT_ENA_W<'a> {
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
#[doc = "Field `HSTIMER3_OVF_INT_ENA` reader - The interrupt enable bit for high speed channel3 counter overflow interrupt."]
pub struct HSTIMER3_OVF_INT_ENA_R(crate::FieldReader<bool, bool>);
impl HSTIMER3_OVF_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HSTIMER3_OVF_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSTIMER3_OVF_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSTIMER3_OVF_INT_ENA` writer - The interrupt enable bit for high speed channel3 counter overflow interrupt."]
pub struct HSTIMER3_OVF_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTIMER3_OVF_INT_ENA_W<'a> {
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
#[doc = "Field `LSTIMER0_OVF_INT_ENA` reader - The interrupt enable bit for low speed channel0 counter overflow interrupt."]
pub struct LSTIMER0_OVF_INT_ENA_R(crate::FieldReader<bool, bool>);
impl LSTIMER0_OVF_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LSTIMER0_OVF_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSTIMER0_OVF_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSTIMER0_OVF_INT_ENA` writer - The interrupt enable bit for low speed channel0 counter overflow interrupt."]
pub struct LSTIMER0_OVF_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> LSTIMER0_OVF_INT_ENA_W<'a> {
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
#[doc = "Field `LSTIMER1_OVF_INT_ENA` reader - The interrupt enable bit for low speed channel1 counter overflow interrupt."]
pub struct LSTIMER1_OVF_INT_ENA_R(crate::FieldReader<bool, bool>);
impl LSTIMER1_OVF_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LSTIMER1_OVF_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSTIMER1_OVF_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSTIMER1_OVF_INT_ENA` writer - The interrupt enable bit for low speed channel1 counter overflow interrupt."]
pub struct LSTIMER1_OVF_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> LSTIMER1_OVF_INT_ENA_W<'a> {
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
#[doc = "Field `LSTIMER2_OVF_INT_ENA` reader - The interrupt enable bit for low speed channel2 counter overflow interrupt."]
pub struct LSTIMER2_OVF_INT_ENA_R(crate::FieldReader<bool, bool>);
impl LSTIMER2_OVF_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LSTIMER2_OVF_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSTIMER2_OVF_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSTIMER2_OVF_INT_ENA` writer - The interrupt enable bit for low speed channel2 counter overflow interrupt."]
pub struct LSTIMER2_OVF_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> LSTIMER2_OVF_INT_ENA_W<'a> {
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
#[doc = "Field `LSTIMER3_OVF_INT_ENA` reader - The interrupt enable bit for low speed channel3 counter overflow interrupt."]
pub struct LSTIMER3_OVF_INT_ENA_R(crate::FieldReader<bool, bool>);
impl LSTIMER3_OVF_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LSTIMER3_OVF_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSTIMER3_OVF_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSTIMER3_OVF_INT_ENA` writer - The interrupt enable bit for low speed channel3 counter overflow interrupt."]
pub struct LSTIMER3_OVF_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> LSTIMER3_OVF_INT_ENA_W<'a> {
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
#[doc = "Field `DUTY_CHNG_END_HSCH0_INT_ENA` reader - The interrupt enable bit for high speed channel 0 duty change done interrupt."]
pub struct DUTY_CHNG_END_HSCH0_INT_ENA_R(crate::FieldReader<bool, bool>);
impl DUTY_CHNG_END_HSCH0_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DUTY_CHNG_END_HSCH0_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_CHNG_END_HSCH0_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTY_CHNG_END_HSCH0_INT_ENA` writer - The interrupt enable bit for high speed channel 0 duty change done interrupt."]
pub struct DUTY_CHNG_END_HSCH0_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_CHNG_END_HSCH0_INT_ENA_W<'a> {
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
#[doc = "Field `DUTY_CHNG_END_HSCH1_INT_ENA` reader - The interrupt enable bit for high speed channel 1 duty change done interrupt."]
pub struct DUTY_CHNG_END_HSCH1_INT_ENA_R(crate::FieldReader<bool, bool>);
impl DUTY_CHNG_END_HSCH1_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DUTY_CHNG_END_HSCH1_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_CHNG_END_HSCH1_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTY_CHNG_END_HSCH1_INT_ENA` writer - The interrupt enable bit for high speed channel 1 duty change done interrupt."]
pub struct DUTY_CHNG_END_HSCH1_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_CHNG_END_HSCH1_INT_ENA_W<'a> {
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
#[doc = "Field `DUTY_CHNG_END_HSCH2_INT_ENA` reader - The interrupt enable bit for high speed channel 2 duty change done interrupt."]
pub struct DUTY_CHNG_END_HSCH2_INT_ENA_R(crate::FieldReader<bool, bool>);
impl DUTY_CHNG_END_HSCH2_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DUTY_CHNG_END_HSCH2_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_CHNG_END_HSCH2_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTY_CHNG_END_HSCH2_INT_ENA` writer - The interrupt enable bit for high speed channel 2 duty change done interrupt."]
pub struct DUTY_CHNG_END_HSCH2_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_CHNG_END_HSCH2_INT_ENA_W<'a> {
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
#[doc = "Field `DUTY_CHNG_END_HSCH3_INT_ENA` reader - The interrupt enable bit for high speed channel 3 duty change done interrupt."]
pub struct DUTY_CHNG_END_HSCH3_INT_ENA_R(crate::FieldReader<bool, bool>);
impl DUTY_CHNG_END_HSCH3_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DUTY_CHNG_END_HSCH3_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_CHNG_END_HSCH3_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTY_CHNG_END_HSCH3_INT_ENA` writer - The interrupt enable bit for high speed channel 3 duty change done interrupt."]
pub struct DUTY_CHNG_END_HSCH3_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_CHNG_END_HSCH3_INT_ENA_W<'a> {
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
#[doc = "Field `DUTY_CHNG_END_HSCH4_INT_ENA` reader - The interrupt enable bit for high speed channel 4 duty change done interrupt."]
pub struct DUTY_CHNG_END_HSCH4_INT_ENA_R(crate::FieldReader<bool, bool>);
impl DUTY_CHNG_END_HSCH4_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DUTY_CHNG_END_HSCH4_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_CHNG_END_HSCH4_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTY_CHNG_END_HSCH4_INT_ENA` writer - The interrupt enable bit for high speed channel 4 duty change done interrupt."]
pub struct DUTY_CHNG_END_HSCH4_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_CHNG_END_HSCH4_INT_ENA_W<'a> {
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
#[doc = "Field `DUTY_CHNG_END_HSCH5_INT_ENA` reader - The interrupt enable bit for high speed channel 5 duty change done interrupt."]
pub struct DUTY_CHNG_END_HSCH5_INT_ENA_R(crate::FieldReader<bool, bool>);
impl DUTY_CHNG_END_HSCH5_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DUTY_CHNG_END_HSCH5_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_CHNG_END_HSCH5_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTY_CHNG_END_HSCH5_INT_ENA` writer - The interrupt enable bit for high speed channel 5 duty change done interrupt."]
pub struct DUTY_CHNG_END_HSCH5_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_CHNG_END_HSCH5_INT_ENA_W<'a> {
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
#[doc = "Field `DUTY_CHNG_END_HSCH6_INT_ENA` reader - The interrupt enable bit for high speed channel 6 duty change done interrupt."]
pub struct DUTY_CHNG_END_HSCH6_INT_ENA_R(crate::FieldReader<bool, bool>);
impl DUTY_CHNG_END_HSCH6_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DUTY_CHNG_END_HSCH6_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_CHNG_END_HSCH6_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTY_CHNG_END_HSCH6_INT_ENA` writer - The interrupt enable bit for high speed channel 6 duty change done interrupt."]
pub struct DUTY_CHNG_END_HSCH6_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_CHNG_END_HSCH6_INT_ENA_W<'a> {
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
#[doc = "Field `DUTY_CHNG_END_HSCH7_INT_ENA` reader - The interrupt enable bit for high speed channel 7 duty change done interrupt."]
pub struct DUTY_CHNG_END_HSCH7_INT_ENA_R(crate::FieldReader<bool, bool>);
impl DUTY_CHNG_END_HSCH7_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DUTY_CHNG_END_HSCH7_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_CHNG_END_HSCH7_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTY_CHNG_END_HSCH7_INT_ENA` writer - The interrupt enable bit for high speed channel 7 duty change done interrupt."]
pub struct DUTY_CHNG_END_HSCH7_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_CHNG_END_HSCH7_INT_ENA_W<'a> {
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
#[doc = "Field `DUTY_CHNG_END_LSCH0_INT_ENA` reader - The interrupt enable bit for low speed channel 0 duty change done interrupt."]
pub struct DUTY_CHNG_END_LSCH0_INT_ENA_R(crate::FieldReader<bool, bool>);
impl DUTY_CHNG_END_LSCH0_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DUTY_CHNG_END_LSCH0_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_CHNG_END_LSCH0_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTY_CHNG_END_LSCH0_INT_ENA` writer - The interrupt enable bit for low speed channel 0 duty change done interrupt."]
pub struct DUTY_CHNG_END_LSCH0_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_CHNG_END_LSCH0_INT_ENA_W<'a> {
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
#[doc = "Field `DUTY_CHNG_END_LSCH1_INT_ENA` reader - The interrupt enable bit for low speed channel 1 duty change done interrupt."]
pub struct DUTY_CHNG_END_LSCH1_INT_ENA_R(crate::FieldReader<bool, bool>);
impl DUTY_CHNG_END_LSCH1_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DUTY_CHNG_END_LSCH1_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_CHNG_END_LSCH1_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTY_CHNG_END_LSCH1_INT_ENA` writer - The interrupt enable bit for low speed channel 1 duty change done interrupt."]
pub struct DUTY_CHNG_END_LSCH1_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_CHNG_END_LSCH1_INT_ENA_W<'a> {
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
#[doc = "Field `DUTY_CHNG_END_LSCH2_INT_ENA` reader - The interrupt enable bit for low speed channel 2 duty change done interrupt."]
pub struct DUTY_CHNG_END_LSCH2_INT_ENA_R(crate::FieldReader<bool, bool>);
impl DUTY_CHNG_END_LSCH2_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DUTY_CHNG_END_LSCH2_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_CHNG_END_LSCH2_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTY_CHNG_END_LSCH2_INT_ENA` writer - The interrupt enable bit for low speed channel 2 duty change done interrupt."]
pub struct DUTY_CHNG_END_LSCH2_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_CHNG_END_LSCH2_INT_ENA_W<'a> {
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
#[doc = "Field `DUTY_CHNG_END_LSCH3_INT_ENA` reader - The interrupt enable bit for low speed channel 3 duty change done interrupt."]
pub struct DUTY_CHNG_END_LSCH3_INT_ENA_R(crate::FieldReader<bool, bool>);
impl DUTY_CHNG_END_LSCH3_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DUTY_CHNG_END_LSCH3_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_CHNG_END_LSCH3_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTY_CHNG_END_LSCH3_INT_ENA` writer - The interrupt enable bit for low speed channel 3 duty change done interrupt."]
pub struct DUTY_CHNG_END_LSCH3_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_CHNG_END_LSCH3_INT_ENA_W<'a> {
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
#[doc = "Field `DUTY_CHNG_END_LSCH4_INT_ENA` reader - The interrupt enable bit for low speed channel 4 duty change done interrupt."]
pub struct DUTY_CHNG_END_LSCH4_INT_ENA_R(crate::FieldReader<bool, bool>);
impl DUTY_CHNG_END_LSCH4_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DUTY_CHNG_END_LSCH4_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_CHNG_END_LSCH4_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTY_CHNG_END_LSCH4_INT_ENA` writer - The interrupt enable bit for low speed channel 4 duty change done interrupt."]
pub struct DUTY_CHNG_END_LSCH4_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_CHNG_END_LSCH4_INT_ENA_W<'a> {
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
#[doc = "Field `DUTY_CHNG_END_LSCH5_INT_ENA` reader - The interrupt enable bit for low speed channel 5 duty change done interrupt."]
pub struct DUTY_CHNG_END_LSCH5_INT_ENA_R(crate::FieldReader<bool, bool>);
impl DUTY_CHNG_END_LSCH5_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DUTY_CHNG_END_LSCH5_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_CHNG_END_LSCH5_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTY_CHNG_END_LSCH5_INT_ENA` writer - The interrupt enable bit for low speed channel 5 duty change done interrupt."]
pub struct DUTY_CHNG_END_LSCH5_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_CHNG_END_LSCH5_INT_ENA_W<'a> {
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
#[doc = "Field `DUTY_CHNG_END_LSCH6_INT_ENA` reader - The interrupt enable bit for low speed channel 6 duty change done interrupt."]
pub struct DUTY_CHNG_END_LSCH6_INT_ENA_R(crate::FieldReader<bool, bool>);
impl DUTY_CHNG_END_LSCH6_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DUTY_CHNG_END_LSCH6_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_CHNG_END_LSCH6_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTY_CHNG_END_LSCH6_INT_ENA` writer - The interrupt enable bit for low speed channel 6 duty change done interrupt."]
pub struct DUTY_CHNG_END_LSCH6_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_CHNG_END_LSCH6_INT_ENA_W<'a> {
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
#[doc = "Field `DUTY_CHNG_END_LSCH7_INT_ENA` reader - The interrupt enable bit for low speed channel 7 duty change done interrupt."]
pub struct DUTY_CHNG_END_LSCH7_INT_ENA_R(crate::FieldReader<bool, bool>);
impl DUTY_CHNG_END_LSCH7_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DUTY_CHNG_END_LSCH7_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_CHNG_END_LSCH7_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTY_CHNG_END_LSCH7_INT_ENA` writer - The interrupt enable bit for low speed channel 7 duty change done interrupt."]
pub struct DUTY_CHNG_END_LSCH7_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_CHNG_END_LSCH7_INT_ENA_W<'a> {
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
impl R {
    #[doc = "Bit 0 - The interrupt enable bit for high speed channel0 counter overflow interrupt."]
    #[inline(always)]
    pub fn hstimer0_ovf_int_ena(&self) -> HSTIMER0_OVF_INT_ENA_R {
        HSTIMER0_OVF_INT_ENA_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for high speed channel1 counter overflow interrupt."]
    #[inline(always)]
    pub fn hstimer1_ovf_int_ena(&self) -> HSTIMER1_OVF_INT_ENA_R {
        HSTIMER1_OVF_INT_ENA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - The interrupt enable bit for high speed channel2 counter overflow interrupt."]
    #[inline(always)]
    pub fn hstimer2_ovf_int_ena(&self) -> HSTIMER2_OVF_INT_ENA_R {
        HSTIMER2_OVF_INT_ENA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - The interrupt enable bit for high speed channel3 counter overflow interrupt."]
    #[inline(always)]
    pub fn hstimer3_ovf_int_ena(&self) -> HSTIMER3_OVF_INT_ENA_R {
        HSTIMER3_OVF_INT_ENA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - The interrupt enable bit for low speed channel0 counter overflow interrupt."]
    #[inline(always)]
    pub fn lstimer0_ovf_int_ena(&self) -> LSTIMER0_OVF_INT_ENA_R {
        LSTIMER0_OVF_INT_ENA_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - The interrupt enable bit for low speed channel1 counter overflow interrupt."]
    #[inline(always)]
    pub fn lstimer1_ovf_int_ena(&self) -> LSTIMER1_OVF_INT_ENA_R {
        LSTIMER1_OVF_INT_ENA_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - The interrupt enable bit for low speed channel2 counter overflow interrupt."]
    #[inline(always)]
    pub fn lstimer2_ovf_int_ena(&self) -> LSTIMER2_OVF_INT_ENA_R {
        LSTIMER2_OVF_INT_ENA_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - The interrupt enable bit for low speed channel3 counter overflow interrupt."]
    #[inline(always)]
    pub fn lstimer3_ovf_int_ena(&self) -> LSTIMER3_OVF_INT_ENA_R {
        LSTIMER3_OVF_INT_ENA_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - The interrupt enable bit for high speed channel 0 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_hsch0_int_ena(&self) -> DUTY_CHNG_END_HSCH0_INT_ENA_R {
        DUTY_CHNG_END_HSCH0_INT_ENA_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - The interrupt enable bit for high speed channel 1 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_hsch1_int_ena(&self) -> DUTY_CHNG_END_HSCH1_INT_ENA_R {
        DUTY_CHNG_END_HSCH1_INT_ENA_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - The interrupt enable bit for high speed channel 2 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_hsch2_int_ena(&self) -> DUTY_CHNG_END_HSCH2_INT_ENA_R {
        DUTY_CHNG_END_HSCH2_INT_ENA_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - The interrupt enable bit for high speed channel 3 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_hsch3_int_ena(&self) -> DUTY_CHNG_END_HSCH3_INT_ENA_R {
        DUTY_CHNG_END_HSCH3_INT_ENA_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - The interrupt enable bit for high speed channel 4 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_hsch4_int_ena(&self) -> DUTY_CHNG_END_HSCH4_INT_ENA_R {
        DUTY_CHNG_END_HSCH4_INT_ENA_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - The interrupt enable bit for high speed channel 5 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_hsch5_int_ena(&self) -> DUTY_CHNG_END_HSCH5_INT_ENA_R {
        DUTY_CHNG_END_HSCH5_INT_ENA_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - The interrupt enable bit for high speed channel 6 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_hsch6_int_ena(&self) -> DUTY_CHNG_END_HSCH6_INT_ENA_R {
        DUTY_CHNG_END_HSCH6_INT_ENA_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - The interrupt enable bit for high speed channel 7 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_hsch7_int_ena(&self) -> DUTY_CHNG_END_HSCH7_INT_ENA_R {
        DUTY_CHNG_END_HSCH7_INT_ENA_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - The interrupt enable bit for low speed channel 0 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_lsch0_int_ena(&self) -> DUTY_CHNG_END_LSCH0_INT_ENA_R {
        DUTY_CHNG_END_LSCH0_INT_ENA_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - The interrupt enable bit for low speed channel 1 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_lsch1_int_ena(&self) -> DUTY_CHNG_END_LSCH1_INT_ENA_R {
        DUTY_CHNG_END_LSCH1_INT_ENA_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - The interrupt enable bit for low speed channel 2 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_lsch2_int_ena(&self) -> DUTY_CHNG_END_LSCH2_INT_ENA_R {
        DUTY_CHNG_END_LSCH2_INT_ENA_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - The interrupt enable bit for low speed channel 3 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_lsch3_int_ena(&self) -> DUTY_CHNG_END_LSCH3_INT_ENA_R {
        DUTY_CHNG_END_LSCH3_INT_ENA_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - The interrupt enable bit for low speed channel 4 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_lsch4_int_ena(&self) -> DUTY_CHNG_END_LSCH4_INT_ENA_R {
        DUTY_CHNG_END_LSCH4_INT_ENA_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - The interrupt enable bit for low speed channel 5 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_lsch5_int_ena(&self) -> DUTY_CHNG_END_LSCH5_INT_ENA_R {
        DUTY_CHNG_END_LSCH5_INT_ENA_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - The interrupt enable bit for low speed channel 6 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_lsch6_int_ena(&self) -> DUTY_CHNG_END_LSCH6_INT_ENA_R {
        DUTY_CHNG_END_LSCH6_INT_ENA_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - The interrupt enable bit for low speed channel 7 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_lsch7_int_ena(&self) -> DUTY_CHNG_END_LSCH7_INT_ENA_R {
        DUTY_CHNG_END_LSCH7_INT_ENA_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The interrupt enable bit for high speed channel0 counter overflow interrupt."]
    #[inline(always)]
    pub fn hstimer0_ovf_int_ena(&mut self) -> HSTIMER0_OVF_INT_ENA_W {
        HSTIMER0_OVF_INT_ENA_W { w: self }
    }
    #[doc = "Bit 1 - The interrupt enable bit for high speed channel1 counter overflow interrupt."]
    #[inline(always)]
    pub fn hstimer1_ovf_int_ena(&mut self) -> HSTIMER1_OVF_INT_ENA_W {
        HSTIMER1_OVF_INT_ENA_W { w: self }
    }
    #[doc = "Bit 2 - The interrupt enable bit for high speed channel2 counter overflow interrupt."]
    #[inline(always)]
    pub fn hstimer2_ovf_int_ena(&mut self) -> HSTIMER2_OVF_INT_ENA_W {
        HSTIMER2_OVF_INT_ENA_W { w: self }
    }
    #[doc = "Bit 3 - The interrupt enable bit for high speed channel3 counter overflow interrupt."]
    #[inline(always)]
    pub fn hstimer3_ovf_int_ena(&mut self) -> HSTIMER3_OVF_INT_ENA_W {
        HSTIMER3_OVF_INT_ENA_W { w: self }
    }
    #[doc = "Bit 4 - The interrupt enable bit for low speed channel0 counter overflow interrupt."]
    #[inline(always)]
    pub fn lstimer0_ovf_int_ena(&mut self) -> LSTIMER0_OVF_INT_ENA_W {
        LSTIMER0_OVF_INT_ENA_W { w: self }
    }
    #[doc = "Bit 5 - The interrupt enable bit for low speed channel1 counter overflow interrupt."]
    #[inline(always)]
    pub fn lstimer1_ovf_int_ena(&mut self) -> LSTIMER1_OVF_INT_ENA_W {
        LSTIMER1_OVF_INT_ENA_W { w: self }
    }
    #[doc = "Bit 6 - The interrupt enable bit for low speed channel2 counter overflow interrupt."]
    #[inline(always)]
    pub fn lstimer2_ovf_int_ena(&mut self) -> LSTIMER2_OVF_INT_ENA_W {
        LSTIMER2_OVF_INT_ENA_W { w: self }
    }
    #[doc = "Bit 7 - The interrupt enable bit for low speed channel3 counter overflow interrupt."]
    #[inline(always)]
    pub fn lstimer3_ovf_int_ena(&mut self) -> LSTIMER3_OVF_INT_ENA_W {
        LSTIMER3_OVF_INT_ENA_W { w: self }
    }
    #[doc = "Bit 8 - The interrupt enable bit for high speed channel 0 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_hsch0_int_ena(&mut self) -> DUTY_CHNG_END_HSCH0_INT_ENA_W {
        DUTY_CHNG_END_HSCH0_INT_ENA_W { w: self }
    }
    #[doc = "Bit 9 - The interrupt enable bit for high speed channel 1 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_hsch1_int_ena(&mut self) -> DUTY_CHNG_END_HSCH1_INT_ENA_W {
        DUTY_CHNG_END_HSCH1_INT_ENA_W { w: self }
    }
    #[doc = "Bit 10 - The interrupt enable bit for high speed channel 2 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_hsch2_int_ena(&mut self) -> DUTY_CHNG_END_HSCH2_INT_ENA_W {
        DUTY_CHNG_END_HSCH2_INT_ENA_W { w: self }
    }
    #[doc = "Bit 11 - The interrupt enable bit for high speed channel 3 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_hsch3_int_ena(&mut self) -> DUTY_CHNG_END_HSCH3_INT_ENA_W {
        DUTY_CHNG_END_HSCH3_INT_ENA_W { w: self }
    }
    #[doc = "Bit 12 - The interrupt enable bit for high speed channel 4 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_hsch4_int_ena(&mut self) -> DUTY_CHNG_END_HSCH4_INT_ENA_W {
        DUTY_CHNG_END_HSCH4_INT_ENA_W { w: self }
    }
    #[doc = "Bit 13 - The interrupt enable bit for high speed channel 5 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_hsch5_int_ena(&mut self) -> DUTY_CHNG_END_HSCH5_INT_ENA_W {
        DUTY_CHNG_END_HSCH5_INT_ENA_W { w: self }
    }
    #[doc = "Bit 14 - The interrupt enable bit for high speed channel 6 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_hsch6_int_ena(&mut self) -> DUTY_CHNG_END_HSCH6_INT_ENA_W {
        DUTY_CHNG_END_HSCH6_INT_ENA_W { w: self }
    }
    #[doc = "Bit 15 - The interrupt enable bit for high speed channel 7 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_hsch7_int_ena(&mut self) -> DUTY_CHNG_END_HSCH7_INT_ENA_W {
        DUTY_CHNG_END_HSCH7_INT_ENA_W { w: self }
    }
    #[doc = "Bit 16 - The interrupt enable bit for low speed channel 0 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_lsch0_int_ena(&mut self) -> DUTY_CHNG_END_LSCH0_INT_ENA_W {
        DUTY_CHNG_END_LSCH0_INT_ENA_W { w: self }
    }
    #[doc = "Bit 17 - The interrupt enable bit for low speed channel 1 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_lsch1_int_ena(&mut self) -> DUTY_CHNG_END_LSCH1_INT_ENA_W {
        DUTY_CHNG_END_LSCH1_INT_ENA_W { w: self }
    }
    #[doc = "Bit 18 - The interrupt enable bit for low speed channel 2 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_lsch2_int_ena(&mut self) -> DUTY_CHNG_END_LSCH2_INT_ENA_W {
        DUTY_CHNG_END_LSCH2_INT_ENA_W { w: self }
    }
    #[doc = "Bit 19 - The interrupt enable bit for low speed channel 3 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_lsch3_int_ena(&mut self) -> DUTY_CHNG_END_LSCH3_INT_ENA_W {
        DUTY_CHNG_END_LSCH3_INT_ENA_W { w: self }
    }
    #[doc = "Bit 20 - The interrupt enable bit for low speed channel 4 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_lsch4_int_ena(&mut self) -> DUTY_CHNG_END_LSCH4_INT_ENA_W {
        DUTY_CHNG_END_LSCH4_INT_ENA_W { w: self }
    }
    #[doc = "Bit 21 - The interrupt enable bit for low speed channel 5 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_lsch5_int_ena(&mut self) -> DUTY_CHNG_END_LSCH5_INT_ENA_W {
        DUTY_CHNG_END_LSCH5_INT_ENA_W { w: self }
    }
    #[doc = "Bit 22 - The interrupt enable bit for low speed channel 6 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_lsch6_int_ena(&mut self) -> DUTY_CHNG_END_LSCH6_INT_ENA_W {
        DUTY_CHNG_END_LSCH6_INT_ENA_W { w: self }
    }
    #[doc = "Bit 23 - The interrupt enable bit for low speed channel 7 duty change done interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_lsch7_int_ena(&mut self) -> DUTY_CHNG_END_LSCH7_INT_ENA_W {
        DUTY_CHNG_END_LSCH7_INT_ENA_W { w: self }
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
