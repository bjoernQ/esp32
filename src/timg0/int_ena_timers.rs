#[doc = "Register `INT_ENA_TIMERS` reader"]
pub struct R(crate::R<INT_ENA_TIMERS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_ENA_TIMERS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_ENA_TIMERS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_ENA_TIMERS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_ENA_TIMERS` writer"]
pub struct W(crate::W<INT_ENA_TIMERS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_ENA_TIMERS_SPEC>;
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
impl From<crate::W<INT_ENA_TIMERS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_ENA_TIMERS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T0_INT_ENA` reader - interrupt when timer0 alarm"]
pub struct T0_INT_ENA_R(crate::FieldReader<bool, bool>);
impl T0_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        T0_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T0_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T0_INT_ENA` writer - interrupt when timer0 alarm"]
pub struct T0_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> T0_INT_ENA_W<'a> {
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
#[doc = "Field `T1_INT_ENA` reader - interrupt when timer1 alarm"]
pub struct T1_INT_ENA_R(crate::FieldReader<bool, bool>);
impl T1_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        T1_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T1_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T1_INT_ENA` writer - interrupt when timer1 alarm"]
pub struct T1_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> T1_INT_ENA_W<'a> {
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
#[doc = "Field `WDT_INT_ENA` reader - Interrupt when an interrupt stage timeout"]
pub struct WDT_INT_ENA_R(crate::FieldReader<bool, bool>);
impl WDT_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WDT_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDT_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDT_INT_ENA` writer - Interrupt when an interrupt stage timeout"]
pub struct WDT_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_INT_ENA_W<'a> {
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
#[doc = "Field `LACT_INT_ENA` reader - "]
pub struct LACT_INT_ENA_R(crate::FieldReader<bool, bool>);
impl LACT_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LACT_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LACT_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LACT_INT_ENA` writer - "]
pub struct LACT_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> LACT_INT_ENA_W<'a> {
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
impl R {
    #[doc = "Bit 0 - interrupt when timer0 alarm"]
    #[inline(always)]
    pub fn t0_int_ena(&self) -> T0_INT_ENA_R {
        T0_INT_ENA_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - interrupt when timer1 alarm"]
    #[inline(always)]
    pub fn t1_int_ena(&self) -> T1_INT_ENA_R {
        T1_INT_ENA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Interrupt when an interrupt stage timeout"]
    #[inline(always)]
    pub fn wdt_int_ena(&self) -> WDT_INT_ENA_R {
        WDT_INT_ENA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn lact_int_ena(&self) -> LACT_INT_ENA_R {
        LACT_INT_ENA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - interrupt when timer0 alarm"]
    #[inline(always)]
    pub fn t0_int_ena(&mut self) -> T0_INT_ENA_W {
        T0_INT_ENA_W { w: self }
    }
    #[doc = "Bit 1 - interrupt when timer1 alarm"]
    #[inline(always)]
    pub fn t1_int_ena(&mut self) -> T1_INT_ENA_W {
        T1_INT_ENA_W { w: self }
    }
    #[doc = "Bit 2 - Interrupt when an interrupt stage timeout"]
    #[inline(always)]
    pub fn wdt_int_ena(&mut self) -> WDT_INT_ENA_W {
        WDT_INT_ENA_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn lact_int_ena(&mut self) -> LACT_INT_ENA_W {
        LACT_INT_ENA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_ena_timers](index.html) module"]
pub struct INT_ENA_TIMERS_SPEC;
impl crate::RegisterSpec for INT_ENA_TIMERS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_ena_timers::R](R) reader structure"]
impl crate::Readable for INT_ENA_TIMERS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_ena_timers::W](W) writer structure"]
impl crate::Writable for INT_ENA_TIMERS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_ENA_TIMERS to value 0"]
impl crate::Resettable for INT_ENA_TIMERS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
