#[doc = "Register `HSCH2_CONF1` reader"]
pub struct R(crate::R<HSCH2_CONF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSCH2_CONF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSCH2_CONF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSCH2_CONF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSCH2_CONF1` writer"]
pub struct W(crate::W<HSCH2_CONF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSCH2_CONF1_SPEC>;
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
impl From<crate::W<HSCH2_CONF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSCH2_CONF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DUTY_SCALE_HSCH2` reader - This register controls the increase or decrease step scale for high speed channel2."]
pub struct DUTY_SCALE_HSCH2_R(crate::FieldReader<u16, u16>);
impl DUTY_SCALE_HSCH2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        DUTY_SCALE_HSCH2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_SCALE_HSCH2_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTY_SCALE_HSCH2` writer - This register controls the increase or decrease step scale for high speed channel2."]
pub struct DUTY_SCALE_HSCH2_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_SCALE_HSCH2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
#[doc = "Field `DUTY_CYCLE_HSCH2` reader - This register is used to increase or decrease the duty every reg_duty_cycle_hsch2 cycles for high speed channel2."]
pub struct DUTY_CYCLE_HSCH2_R(crate::FieldReader<u16, u16>);
impl DUTY_CYCLE_HSCH2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        DUTY_CYCLE_HSCH2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_CYCLE_HSCH2_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTY_CYCLE_HSCH2` writer - This register is used to increase or decrease the duty every reg_duty_cycle_hsch2 cycles for high speed channel2."]
pub struct DUTY_CYCLE_HSCH2_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_CYCLE_HSCH2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 10)) | ((value as u32 & 0x03ff) << 10);
        self.w
    }
}
#[doc = "Field `DUTY_NUM_HSCH2` reader - This register is used to control the num of increased or decreased times for high speed channel2."]
pub struct DUTY_NUM_HSCH2_R(crate::FieldReader<u16, u16>);
impl DUTY_NUM_HSCH2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        DUTY_NUM_HSCH2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_NUM_HSCH2_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTY_NUM_HSCH2` writer - This register is used to control the num of increased or decreased times for high speed channel2."]
pub struct DUTY_NUM_HSCH2_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_NUM_HSCH2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 20)) | ((value as u32 & 0x03ff) << 20);
        self.w
    }
}
#[doc = "Field `DUTY_INC_HSCH2` reader - This register is used to increase the duty of output signal or decrease the duty of output signal for high speed channel2."]
pub struct DUTY_INC_HSCH2_R(crate::FieldReader<bool, bool>);
impl DUTY_INC_HSCH2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DUTY_INC_HSCH2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_INC_HSCH2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTY_INC_HSCH2` writer - This register is used to increase the duty of output signal or decrease the duty of output signal for high speed channel2."]
pub struct DUTY_INC_HSCH2_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_INC_HSCH2_W<'a> {
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
#[doc = "Field `DUTY_START_HSCH2` reader - When reg_duty_num_hsch2 reg_duty_cycle_hsch2 and reg_duty_scale_hsch2 has been configured. these register won't take effect until set reg_duty_start_hsch2. this bit is automatically cleared by hardware."]
pub struct DUTY_START_HSCH2_R(crate::FieldReader<bool, bool>);
impl DUTY_START_HSCH2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DUTY_START_HSCH2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_START_HSCH2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTY_START_HSCH2` writer - When reg_duty_num_hsch2 reg_duty_cycle_hsch2 and reg_duty_scale_hsch2 has been configured. these register won't take effect until set reg_duty_start_hsch2. this bit is automatically cleared by hardware."]
pub struct DUTY_START_HSCH2_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_START_HSCH2_W<'a> {
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
    #[doc = "Bits 0:9 - This register controls the increase or decrease step scale for high speed channel2."]
    #[inline(always)]
    pub fn duty_scale_hsch2(&self) -> DUTY_SCALE_HSCH2_R {
        DUTY_SCALE_HSCH2_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - This register is used to increase or decrease the duty every reg_duty_cycle_hsch2 cycles for high speed channel2."]
    #[inline(always)]
    pub fn duty_cycle_hsch2(&self) -> DUTY_CYCLE_HSCH2_R {
        DUTY_CYCLE_HSCH2_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:29 - This register is used to control the num of increased or decreased times for high speed channel2."]
    #[inline(always)]
    pub fn duty_num_hsch2(&self) -> DUTY_NUM_HSCH2_R {
        DUTY_NUM_HSCH2_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
    #[doc = "Bit 30 - This register is used to increase the duty of output signal or decrease the duty of output signal for high speed channel2."]
    #[inline(always)]
    pub fn duty_inc_hsch2(&self) -> DUTY_INC_HSCH2_R {
        DUTY_INC_HSCH2_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - When reg_duty_num_hsch2 reg_duty_cycle_hsch2 and reg_duty_scale_hsch2 has been configured. these register won't take effect until set reg_duty_start_hsch2. this bit is automatically cleared by hardware."]
    #[inline(always)]
    pub fn duty_start_hsch2(&self) -> DUTY_START_HSCH2_R {
        DUTY_START_HSCH2_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - This register controls the increase or decrease step scale for high speed channel2."]
    #[inline(always)]
    pub fn duty_scale_hsch2(&mut self) -> DUTY_SCALE_HSCH2_W {
        DUTY_SCALE_HSCH2_W { w: self }
    }
    #[doc = "Bits 10:19 - This register is used to increase or decrease the duty every reg_duty_cycle_hsch2 cycles for high speed channel2."]
    #[inline(always)]
    pub fn duty_cycle_hsch2(&mut self) -> DUTY_CYCLE_HSCH2_W {
        DUTY_CYCLE_HSCH2_W { w: self }
    }
    #[doc = "Bits 20:29 - This register is used to control the num of increased or decreased times for high speed channel2."]
    #[inline(always)]
    pub fn duty_num_hsch2(&mut self) -> DUTY_NUM_HSCH2_W {
        DUTY_NUM_HSCH2_W { w: self }
    }
    #[doc = "Bit 30 - This register is used to increase the duty of output signal or decrease the duty of output signal for high speed channel2."]
    #[inline(always)]
    pub fn duty_inc_hsch2(&mut self) -> DUTY_INC_HSCH2_W {
        DUTY_INC_HSCH2_W { w: self }
    }
    #[doc = "Bit 31 - When reg_duty_num_hsch2 reg_duty_cycle_hsch2 and reg_duty_scale_hsch2 has been configured. these register won't take effect until set reg_duty_start_hsch2. this bit is automatically cleared by hardware."]
    #[inline(always)]
    pub fn duty_start_hsch2(&mut self) -> DUTY_START_HSCH2_W {
        DUTY_START_HSCH2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsch2_conf1](index.html) module"]
pub struct HSCH2_CONF1_SPEC;
impl crate::RegisterSpec for HSCH2_CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hsch2_conf1::R](R) reader structure"]
impl crate::Readable for HSCH2_CONF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hsch2_conf1::W](W) writer structure"]
impl crate::Writable for HSCH2_CONF1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HSCH2_CONF1 to value 0x4000_0000"]
impl crate::Resettable for HSCH2_CONF1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4000_0000
    }
}
