#[doc = "Register `LACTCONFIG` reader"]
pub struct R(crate::R<LACTCONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LACTCONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LACTCONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LACTCONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LACTCONFIG` writer"]
pub struct W(crate::W<LACTCONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LACTCONFIG_SPEC>;
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
impl From<crate::W<LACTCONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LACTCONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LACT_RTC_ONLY` reader - "]
pub struct LACT_RTC_ONLY_R(crate::FieldReader<bool, bool>);
impl LACT_RTC_ONLY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LACT_RTC_ONLY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LACT_RTC_ONLY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LACT_RTC_ONLY` writer - "]
pub struct LACT_RTC_ONLY_W<'a> {
    w: &'a mut W,
}
impl<'a> LACT_RTC_ONLY_W<'a> {
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
#[doc = "Field `LACT_CPST_EN` reader - "]
pub struct LACT_CPST_EN_R(crate::FieldReader<bool, bool>);
impl LACT_CPST_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LACT_CPST_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LACT_CPST_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LACT_CPST_EN` writer - "]
pub struct LACT_CPST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LACT_CPST_EN_W<'a> {
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
#[doc = "Field `LACT_LAC_EN` reader - "]
pub struct LACT_LAC_EN_R(crate::FieldReader<bool, bool>);
impl LACT_LAC_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LACT_LAC_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LACT_LAC_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LACT_LAC_EN` writer - "]
pub struct LACT_LAC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LACT_LAC_EN_W<'a> {
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
#[doc = "Field `LACT_ALARM_EN` reader - "]
pub struct LACT_ALARM_EN_R(crate::FieldReader<bool, bool>);
impl LACT_ALARM_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LACT_ALARM_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LACT_ALARM_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LACT_ALARM_EN` writer - "]
pub struct LACT_ALARM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LACT_ALARM_EN_W<'a> {
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
#[doc = "Field `LACT_LEVEL_INT_EN` reader - "]
pub struct LACT_LEVEL_INT_EN_R(crate::FieldReader<bool, bool>);
impl LACT_LEVEL_INT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LACT_LEVEL_INT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LACT_LEVEL_INT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LACT_LEVEL_INT_EN` writer - "]
pub struct LACT_LEVEL_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LACT_LEVEL_INT_EN_W<'a> {
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
#[doc = "Field `LACT_EDGE_INT_EN` reader - "]
pub struct LACT_EDGE_INT_EN_R(crate::FieldReader<bool, bool>);
impl LACT_EDGE_INT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LACT_EDGE_INT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LACT_EDGE_INT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LACT_EDGE_INT_EN` writer - "]
pub struct LACT_EDGE_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LACT_EDGE_INT_EN_W<'a> {
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
#[doc = "Field `LACT_DIVIDER` reader - "]
pub struct LACT_DIVIDER_R(crate::FieldReader<u16, u16>);
impl LACT_DIVIDER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        LACT_DIVIDER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LACT_DIVIDER_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LACT_DIVIDER` writer - "]
pub struct LACT_DIVIDER_W<'a> {
    w: &'a mut W,
}
impl<'a> LACT_DIVIDER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 13)) | ((value as u32 & 0xffff) << 13);
        self.w
    }
}
#[doc = "Field `LACT_AUTORELOAD` reader - "]
pub struct LACT_AUTORELOAD_R(crate::FieldReader<bool, bool>);
impl LACT_AUTORELOAD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LACT_AUTORELOAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LACT_AUTORELOAD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LACT_AUTORELOAD` writer - "]
pub struct LACT_AUTORELOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> LACT_AUTORELOAD_W<'a> {
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
#[doc = "Field `LACT_INCREASE` reader - "]
pub struct LACT_INCREASE_R(crate::FieldReader<bool, bool>);
impl LACT_INCREASE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LACT_INCREASE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LACT_INCREASE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LACT_INCREASE` writer - "]
pub struct LACT_INCREASE_W<'a> {
    w: &'a mut W,
}
impl<'a> LACT_INCREASE_W<'a> {
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
#[doc = "Field `LACT_EN` reader - "]
pub struct LACT_EN_R(crate::FieldReader<bool, bool>);
impl LACT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LACT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LACT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LACT_EN` writer - "]
pub struct LACT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LACT_EN_W<'a> {
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
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn lact_rtc_only(&self) -> LACT_RTC_ONLY_R {
        LACT_RTC_ONLY_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn lact_cpst_en(&self) -> LACT_CPST_EN_R {
        LACT_CPST_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn lact_lac_en(&self) -> LACT_LAC_EN_R {
        LACT_LAC_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn lact_alarm_en(&self) -> LACT_ALARM_EN_R {
        LACT_ALARM_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn lact_level_int_en(&self) -> LACT_LEVEL_INT_EN_R {
        LACT_LEVEL_INT_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn lact_edge_int_en(&self) -> LACT_EDGE_INT_EN_R {
        LACT_EDGE_INT_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 13:28"]
    #[inline(always)]
    pub fn lact_divider(&self) -> LACT_DIVIDER_R {
        LACT_DIVIDER_R::new(((self.bits >> 13) & 0xffff) as u16)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn lact_autoreload(&self) -> LACT_AUTORELOAD_R {
        LACT_AUTORELOAD_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn lact_increase(&self) -> LACT_INCREASE_R {
        LACT_INCREASE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn lact_en(&self) -> LACT_EN_R {
        LACT_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn lact_rtc_only(&mut self) -> LACT_RTC_ONLY_W {
        LACT_RTC_ONLY_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn lact_cpst_en(&mut self) -> LACT_CPST_EN_W {
        LACT_CPST_EN_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn lact_lac_en(&mut self) -> LACT_LAC_EN_W {
        LACT_LAC_EN_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn lact_alarm_en(&mut self) -> LACT_ALARM_EN_W {
        LACT_ALARM_EN_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn lact_level_int_en(&mut self) -> LACT_LEVEL_INT_EN_W {
        LACT_LEVEL_INT_EN_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn lact_edge_int_en(&mut self) -> LACT_EDGE_INT_EN_W {
        LACT_EDGE_INT_EN_W { w: self }
    }
    #[doc = "Bits 13:28"]
    #[inline(always)]
    pub fn lact_divider(&mut self) -> LACT_DIVIDER_W {
        LACT_DIVIDER_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn lact_autoreload(&mut self) -> LACT_AUTORELOAD_W {
        LACT_AUTORELOAD_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn lact_increase(&mut self) -> LACT_INCREASE_W {
        LACT_INCREASE_W { w: self }
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn lact_en(&mut self) -> LACT_EN_W {
        LACT_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lactconfig](index.html) module"]
pub struct LACTCONFIG_SPEC;
impl crate::RegisterSpec for LACTCONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lactconfig::R](R) reader structure"]
impl crate::Readable for LACTCONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lactconfig::W](W) writer structure"]
impl crate::Writable for LACTCONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LACTCONFIG to value 0x6000_2300"]
impl crate::Resettable for LACTCONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x6000_2300
    }
}
