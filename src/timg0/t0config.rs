#[doc = "Register `T0CONFIG` reader"]
pub struct R(crate::R<T0CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<T0CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<T0CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<T0CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `T0CONFIG` writer"]
pub struct W(crate::W<T0CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<T0CONFIG_SPEC>;
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
impl From<crate::W<T0CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<T0CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T0_ALARM_EN` reader - When set alarm is enabled"]
pub struct T0_ALARM_EN_R(crate::FieldReader<bool, bool>);
impl T0_ALARM_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        T0_ALARM_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T0_ALARM_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T0_ALARM_EN` writer - When set alarm is enabled"]
pub struct T0_ALARM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> T0_ALARM_EN_W<'a> {
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
#[doc = "Field `T0_LEVEL_INT_EN` reader - When set level type interrupt will be generated during alarm"]
pub struct T0_LEVEL_INT_EN_R(crate::FieldReader<bool, bool>);
impl T0_LEVEL_INT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        T0_LEVEL_INT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T0_LEVEL_INT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T0_LEVEL_INT_EN` writer - When set level type interrupt will be generated during alarm"]
pub struct T0_LEVEL_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> T0_LEVEL_INT_EN_W<'a> {
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
#[doc = "Field `T0_EDGE_INT_EN` reader - When set edge type interrupt will be generated during alarm"]
pub struct T0_EDGE_INT_EN_R(crate::FieldReader<bool, bool>);
impl T0_EDGE_INT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        T0_EDGE_INT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T0_EDGE_INT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T0_EDGE_INT_EN` writer - When set edge type interrupt will be generated during alarm"]
pub struct T0_EDGE_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> T0_EDGE_INT_EN_W<'a> {
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
#[doc = "Field `T0_DIVIDER` reader - Timer 0 clock (T0_clk) prescale value."]
pub struct T0_DIVIDER_R(crate::FieldReader<u16, u16>);
impl T0_DIVIDER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        T0_DIVIDER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T0_DIVIDER_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T0_DIVIDER` writer - Timer 0 clock (T0_clk) prescale value."]
pub struct T0_DIVIDER_W<'a> {
    w: &'a mut W,
}
impl<'a> T0_DIVIDER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 13)) | ((value as u32 & 0xffff) << 13);
        self.w
    }
}
#[doc = "Field `T0_AUTORELOAD` reader - When set timer 0 auto-reload at alarming is enabled"]
pub struct T0_AUTORELOAD_R(crate::FieldReader<bool, bool>);
impl T0_AUTORELOAD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        T0_AUTORELOAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T0_AUTORELOAD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T0_AUTORELOAD` writer - When set timer 0 auto-reload at alarming is enabled"]
pub struct T0_AUTORELOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> T0_AUTORELOAD_W<'a> {
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
#[doc = "Field `T0_INCREASE` reader - When set timer 0 time-base counter increment. When cleared timer 0 time-base counter decrement."]
pub struct T0_INCREASE_R(crate::FieldReader<bool, bool>);
impl T0_INCREASE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        T0_INCREASE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T0_INCREASE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T0_INCREASE` writer - When set timer 0 time-base counter increment. When cleared timer 0 time-base counter decrement."]
pub struct T0_INCREASE_W<'a> {
    w: &'a mut W,
}
impl<'a> T0_INCREASE_W<'a> {
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
#[doc = "Field `T0_EN` reader - When set timer 0 time-base counter is enabled"]
pub struct T0_EN_R(crate::FieldReader<bool, bool>);
impl T0_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        T0_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T0_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T0_EN` writer - When set timer 0 time-base counter is enabled"]
pub struct T0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> T0_EN_W<'a> {
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
    #[doc = "Bit 10 - When set alarm is enabled"]
    #[inline(always)]
    pub fn t0_alarm_en(&self) -> T0_ALARM_EN_R {
        T0_ALARM_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - When set level type interrupt will be generated during alarm"]
    #[inline(always)]
    pub fn t0_level_int_en(&self) -> T0_LEVEL_INT_EN_R {
        T0_LEVEL_INT_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - When set edge type interrupt will be generated during alarm"]
    #[inline(always)]
    pub fn t0_edge_int_en(&self) -> T0_EDGE_INT_EN_R {
        T0_EDGE_INT_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 13:28 - Timer 0 clock (T0_clk) prescale value."]
    #[inline(always)]
    pub fn t0_divider(&self) -> T0_DIVIDER_R {
        T0_DIVIDER_R::new(((self.bits >> 13) & 0xffff) as u16)
    }
    #[doc = "Bit 29 - When set timer 0 auto-reload at alarming is enabled"]
    #[inline(always)]
    pub fn t0_autoreload(&self) -> T0_AUTORELOAD_R {
        T0_AUTORELOAD_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - When set timer 0 time-base counter increment. When cleared timer 0 time-base counter decrement."]
    #[inline(always)]
    pub fn t0_increase(&self) -> T0_INCREASE_R {
        T0_INCREASE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - When set timer 0 time-base counter is enabled"]
    #[inline(always)]
    pub fn t0_en(&self) -> T0_EN_R {
        T0_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - When set alarm is enabled"]
    #[inline(always)]
    pub fn t0_alarm_en(&mut self) -> T0_ALARM_EN_W {
        T0_ALARM_EN_W { w: self }
    }
    #[doc = "Bit 11 - When set level type interrupt will be generated during alarm"]
    #[inline(always)]
    pub fn t0_level_int_en(&mut self) -> T0_LEVEL_INT_EN_W {
        T0_LEVEL_INT_EN_W { w: self }
    }
    #[doc = "Bit 12 - When set edge type interrupt will be generated during alarm"]
    #[inline(always)]
    pub fn t0_edge_int_en(&mut self) -> T0_EDGE_INT_EN_W {
        T0_EDGE_INT_EN_W { w: self }
    }
    #[doc = "Bits 13:28 - Timer 0 clock (T0_clk) prescale value."]
    #[inline(always)]
    pub fn t0_divider(&mut self) -> T0_DIVIDER_W {
        T0_DIVIDER_W { w: self }
    }
    #[doc = "Bit 29 - When set timer 0 auto-reload at alarming is enabled"]
    #[inline(always)]
    pub fn t0_autoreload(&mut self) -> T0_AUTORELOAD_W {
        T0_AUTORELOAD_W { w: self }
    }
    #[doc = "Bit 30 - When set timer 0 time-base counter increment. When cleared timer 0 time-base counter decrement."]
    #[inline(always)]
    pub fn t0_increase(&mut self) -> T0_INCREASE_W {
        T0_INCREASE_W { w: self }
    }
    #[doc = "Bit 31 - When set timer 0 time-base counter is enabled"]
    #[inline(always)]
    pub fn t0_en(&mut self) -> T0_EN_W {
        T0_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t0config](index.html) module"]
pub struct T0CONFIG_SPEC;
impl crate::RegisterSpec for T0CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [t0config::R](R) reader structure"]
impl crate::Readable for T0CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [t0config::W](W) writer structure"]
impl crate::Writable for T0CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets T0CONFIG to value 0x6000_2000"]
impl crate::Resettable for T0CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x6000_2000
    }
}
