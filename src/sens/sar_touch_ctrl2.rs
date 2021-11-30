#[doc = "Register `SAR_TOUCH_CTRL2` reader"]
pub struct R(crate::R<SAR_TOUCH_CTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_TOUCH_CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_TOUCH_CTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_TOUCH_CTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_TOUCH_CTRL2` writer"]
pub struct W(crate::W<SAR_TOUCH_CTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_TOUCH_CTRL2_SPEC>;
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
impl From<crate::W<SAR_TOUCH_CTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_TOUCH_CTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOUCH_MEAS_EN` reader - 10-bit register to indicate which pads are \"touched\""]
pub struct TOUCH_MEAS_EN_R(crate::FieldReader<u16, u16>);
impl TOUCH_MEAS_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TOUCH_MEAS_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_MEAS_EN_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_MEAS_DONE` reader - fsm set 1 to indicate touch touch meas is done"]
pub struct TOUCH_MEAS_DONE_R(crate::FieldReader<bool, bool>);
impl TOUCH_MEAS_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOUCH_MEAS_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_MEAS_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_START_FSM_EN` reader - 1: TOUCH_START & TOUCH_XPD is controlled by touch fsm 0: TOUCH_START & TOUCH_XPD is controlled by registers"]
pub struct TOUCH_START_FSM_EN_R(crate::FieldReader<bool, bool>);
impl TOUCH_START_FSM_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOUCH_START_FSM_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_START_FSM_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_START_FSM_EN` writer - 1: TOUCH_START & TOUCH_XPD is controlled by touch fsm 0: TOUCH_START & TOUCH_XPD is controlled by registers"]
pub struct TOUCH_START_FSM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_START_FSM_EN_W<'a> {
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
#[doc = "Field `TOUCH_START_EN` reader - 1: start touch fsm valid when reg_touch_start_force is set"]
pub struct TOUCH_START_EN_R(crate::FieldReader<bool, bool>);
impl TOUCH_START_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOUCH_START_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_START_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_START_EN` writer - 1: start touch fsm valid when reg_touch_start_force is set"]
pub struct TOUCH_START_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_START_EN_W<'a> {
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
#[doc = "Field `TOUCH_START_FORCE` reader - 1: to start touch fsm by SW 0: to start touch fsm by timer"]
pub struct TOUCH_START_FORCE_R(crate::FieldReader<bool, bool>);
impl TOUCH_START_FORCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOUCH_START_FORCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_START_FORCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_START_FORCE` writer - 1: to start touch fsm by SW 0: to start touch fsm by timer"]
pub struct TOUCH_START_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_START_FORCE_W<'a> {
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
#[doc = "Field `TOUCH_SLEEP_CYCLES` reader - sleep cycles for timer"]
pub struct TOUCH_SLEEP_CYCLES_R(crate::FieldReader<u16, u16>);
impl TOUCH_SLEEP_CYCLES_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TOUCH_SLEEP_CYCLES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_SLEEP_CYCLES_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_SLEEP_CYCLES` writer - sleep cycles for timer"]
pub struct TOUCH_SLEEP_CYCLES_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_SLEEP_CYCLES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 14)) | ((value as u32 & 0xffff) << 14);
        self.w
    }
}
#[doc = "Field `TOUCH_MEAS_EN_CLR` writer - to clear reg_touch_meas_en"]
pub struct TOUCH_MEAS_EN_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_MEAS_EN_CLR_W<'a> {
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
impl R {
    #[doc = "Bits 0:9 - 10-bit register to indicate which pads are \"touched\""]
    #[inline(always)]
    pub fn touch_meas_en(&self) -> TOUCH_MEAS_EN_R {
        TOUCH_MEAS_EN_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - fsm set 1 to indicate touch touch meas is done"]
    #[inline(always)]
    pub fn touch_meas_done(&self) -> TOUCH_MEAS_DONE_R {
        TOUCH_MEAS_DONE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 1: TOUCH_START & TOUCH_XPD is controlled by touch fsm 0: TOUCH_START & TOUCH_XPD is controlled by registers"]
    #[inline(always)]
    pub fn touch_start_fsm_en(&self) -> TOUCH_START_FSM_EN_R {
        TOUCH_START_FSM_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 1: start touch fsm valid when reg_touch_start_force is set"]
    #[inline(always)]
    pub fn touch_start_en(&self) -> TOUCH_START_EN_R {
        TOUCH_START_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - 1: to start touch fsm by SW 0: to start touch fsm by timer"]
    #[inline(always)]
    pub fn touch_start_force(&self) -> TOUCH_START_FORCE_R {
        TOUCH_START_FORCE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 14:29 - sleep cycles for timer"]
    #[inline(always)]
    pub fn touch_sleep_cycles(&self) -> TOUCH_SLEEP_CYCLES_R {
        TOUCH_SLEEP_CYCLES_R::new(((self.bits >> 14) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 11 - 1: TOUCH_START & TOUCH_XPD is controlled by touch fsm 0: TOUCH_START & TOUCH_XPD is controlled by registers"]
    #[inline(always)]
    pub fn touch_start_fsm_en(&mut self) -> TOUCH_START_FSM_EN_W {
        TOUCH_START_FSM_EN_W { w: self }
    }
    #[doc = "Bit 12 - 1: start touch fsm valid when reg_touch_start_force is set"]
    #[inline(always)]
    pub fn touch_start_en(&mut self) -> TOUCH_START_EN_W {
        TOUCH_START_EN_W { w: self }
    }
    #[doc = "Bit 13 - 1: to start touch fsm by SW 0: to start touch fsm by timer"]
    #[inline(always)]
    pub fn touch_start_force(&mut self) -> TOUCH_START_FORCE_W {
        TOUCH_START_FORCE_W { w: self }
    }
    #[doc = "Bits 14:29 - sleep cycles for timer"]
    #[inline(always)]
    pub fn touch_sleep_cycles(&mut self) -> TOUCH_SLEEP_CYCLES_W {
        TOUCH_SLEEP_CYCLES_W { w: self }
    }
    #[doc = "Bit 30 - to clear reg_touch_meas_en"]
    #[inline(always)]
    pub fn touch_meas_en_clr(&mut self) -> TOUCH_MEAS_EN_CLR_W {
        TOUCH_MEAS_EN_CLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_touch_ctrl2](index.html) module"]
pub struct SAR_TOUCH_CTRL2_SPEC;
impl crate::RegisterSpec for SAR_TOUCH_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_touch_ctrl2::R](R) reader structure"]
impl crate::Readable for SAR_TOUCH_CTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_touch_ctrl2::W](W) writer structure"]
impl crate::Writable for SAR_TOUCH_CTRL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAR_TOUCH_CTRL2 to value 0x0040_0800"]
impl crate::Resettable for SAR_TOUCH_CTRL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0040_0800
    }
}
