#[doc = "Register `LSCH3_CONF0` reader"]
pub struct R(crate::R<LSCH3_CONF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LSCH3_CONF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LSCH3_CONF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LSCH3_CONF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LSCH3_CONF0` writer"]
pub struct W(crate::W<LSCH3_CONF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LSCH3_CONF0_SPEC>;
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
impl From<crate::W<LSCH3_CONF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LSCH3_CONF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER_SEL_LSCH3` reader - There are four low speed timers the two bits are used to select one of them for low speed channel3. 2'b00: seletc lstimer0. 2'b01: select lstimer1. 2'b10: select lstimer2. 2'b11: select lstimer3."]
pub struct TIMER_SEL_LSCH3_R(crate::FieldReader<u8, u8>);
impl TIMER_SEL_LSCH3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TIMER_SEL_LSCH3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER_SEL_LSCH3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER_SEL_LSCH3` writer - There are four low speed timers the two bits are used to select one of them for low speed channel3. 2'b00: seletc lstimer0. 2'b01: select lstimer1. 2'b10: select lstimer2. 2'b11: select lstimer3."]
pub struct TIMER_SEL_LSCH3_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_SEL_LSCH3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `SIG_OUT_EN_LSCH3` reader - This is the output enable control bit for low speed channel3."]
pub struct SIG_OUT_EN_LSCH3_R(crate::FieldReader<bool, bool>);
impl SIG_OUT_EN_LSCH3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SIG_OUT_EN_LSCH3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIG_OUT_EN_LSCH3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIG_OUT_EN_LSCH3` writer - This is the output enable control bit for low speed channel3."]
pub struct SIG_OUT_EN_LSCH3_W<'a> {
    w: &'a mut W,
}
impl<'a> SIG_OUT_EN_LSCH3_W<'a> {
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
#[doc = "Field `IDLE_LV_LSCH3` reader - This bit is used to control the output value when low speed channel3 is off."]
pub struct IDLE_LV_LSCH3_R(crate::FieldReader<bool, bool>);
impl IDLE_LV_LSCH3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IDLE_LV_LSCH3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDLE_LV_LSCH3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDLE_LV_LSCH3` writer - This bit is used to control the output value when low speed channel3 is off."]
pub struct IDLE_LV_LSCH3_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLE_LV_LSCH3_W<'a> {
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
#[doc = "Field `PARA_UP_LSCH3` reader - This bit is used to update register LEDC_LSCH3_HPOINT and LEDC_LSCH3_DUTY for low speed channel3."]
pub struct PARA_UP_LSCH3_R(crate::FieldReader<bool, bool>);
impl PARA_UP_LSCH3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PARA_UP_LSCH3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PARA_UP_LSCH3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PARA_UP_LSCH3` writer - This bit is used to update register LEDC_LSCH3_HPOINT and LEDC_LSCH3_DUTY for low speed channel3."]
pub struct PARA_UP_LSCH3_W<'a> {
    w: &'a mut W,
}
impl<'a> PARA_UP_LSCH3_W<'a> {
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
impl R {
    #[doc = "Bits 0:1 - There are four low speed timers the two bits are used to select one of them for low speed channel3. 2'b00: seletc lstimer0. 2'b01: select lstimer1. 2'b10: select lstimer2. 2'b11: select lstimer3."]
    #[inline(always)]
    pub fn timer_sel_lsch3(&self) -> TIMER_SEL_LSCH3_R {
        TIMER_SEL_LSCH3_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - This is the output enable control bit for low speed channel3."]
    #[inline(always)]
    pub fn sig_out_en_lsch3(&self) -> SIG_OUT_EN_LSCH3_R {
        SIG_OUT_EN_LSCH3_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - This bit is used to control the output value when low speed channel3 is off."]
    #[inline(always)]
    pub fn idle_lv_lsch3(&self) -> IDLE_LV_LSCH3_R {
        IDLE_LV_LSCH3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - This bit is used to update register LEDC_LSCH3_HPOINT and LEDC_LSCH3_DUTY for low speed channel3."]
    #[inline(always)]
    pub fn para_up_lsch3(&self) -> PARA_UP_LSCH3_R {
        PARA_UP_LSCH3_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - There are four low speed timers the two bits are used to select one of them for low speed channel3. 2'b00: seletc lstimer0. 2'b01: select lstimer1. 2'b10: select lstimer2. 2'b11: select lstimer3."]
    #[inline(always)]
    pub fn timer_sel_lsch3(&mut self) -> TIMER_SEL_LSCH3_W {
        TIMER_SEL_LSCH3_W { w: self }
    }
    #[doc = "Bit 2 - This is the output enable control bit for low speed channel3."]
    #[inline(always)]
    pub fn sig_out_en_lsch3(&mut self) -> SIG_OUT_EN_LSCH3_W {
        SIG_OUT_EN_LSCH3_W { w: self }
    }
    #[doc = "Bit 3 - This bit is used to control the output value when low speed channel3 is off."]
    #[inline(always)]
    pub fn idle_lv_lsch3(&mut self) -> IDLE_LV_LSCH3_W {
        IDLE_LV_LSCH3_W { w: self }
    }
    #[doc = "Bit 4 - This bit is used to update register LEDC_LSCH3_HPOINT and LEDC_LSCH3_DUTY for low speed channel3."]
    #[inline(always)]
    pub fn para_up_lsch3(&mut self) -> PARA_UP_LSCH3_W {
        PARA_UP_LSCH3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lsch3_conf0](index.html) module"]
pub struct LSCH3_CONF0_SPEC;
impl crate::RegisterSpec for LSCH3_CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lsch3_conf0::R](R) reader structure"]
impl crate::Readable for LSCH3_CONF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lsch3_conf0::W](W) writer structure"]
impl crate::Writable for LSCH3_CONF0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LSCH3_CONF0 to value 0"]
impl crate::Resettable for LSCH3_CONF0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
