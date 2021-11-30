#[doc = "Register `SAR_TOUCH_CTRL1` reader"]
pub struct R(crate::R<SAR_TOUCH_CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_TOUCH_CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_TOUCH_CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_TOUCH_CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_TOUCH_CTRL1` writer"]
pub struct W(crate::W<SAR_TOUCH_CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_TOUCH_CTRL1_SPEC>;
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
impl From<crate::W<SAR_TOUCH_CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_TOUCH_CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOUCH_MEAS_DELAY` reader - the meas length (in 8MHz)"]
pub struct TOUCH_MEAS_DELAY_R(crate::FieldReader<u16, u16>);
impl TOUCH_MEAS_DELAY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TOUCH_MEAS_DELAY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_MEAS_DELAY_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_MEAS_DELAY` writer - the meas length (in 8MHz)"]
pub struct TOUCH_MEAS_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_MEAS_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `TOUCH_XPD_WAIT` reader - the waiting cycles (in 8MHz) between TOUCH_START and TOUCH_XPD"]
pub struct TOUCH_XPD_WAIT_R(crate::FieldReader<u8, u8>);
impl TOUCH_XPD_WAIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TOUCH_XPD_WAIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_XPD_WAIT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_XPD_WAIT` writer - the waiting cycles (in 8MHz) between TOUCH_START and TOUCH_XPD"]
pub struct TOUCH_XPD_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_XPD_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `TOUCH_OUT_SEL` reader - 1: when the counter is greater then the threshold the touch pad is considered as \"touched\" 0: when the counter is less than the threshold the touch pad is considered as \"touched\""]
pub struct TOUCH_OUT_SEL_R(crate::FieldReader<bool, bool>);
impl TOUCH_OUT_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOUCH_OUT_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_OUT_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_OUT_SEL` writer - 1: when the counter is greater then the threshold the touch pad is considered as \"touched\" 0: when the counter is less than the threshold the touch pad is considered as \"touched\""]
pub struct TOUCH_OUT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_OUT_SEL_W<'a> {
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
#[doc = "Field `TOUCH_OUT_1EN` reader - 1: wakeup interrupt is generated if SET1 is \"touched\" 0: wakeup interrupt is generated only if SET1 & SET2 is both \"touched\""]
pub struct TOUCH_OUT_1EN_R(crate::FieldReader<bool, bool>);
impl TOUCH_OUT_1EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOUCH_OUT_1EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_OUT_1EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_OUT_1EN` writer - 1: wakeup interrupt is generated if SET1 is \"touched\" 0: wakeup interrupt is generated only if SET1 & SET2 is both \"touched\""]
pub struct TOUCH_OUT_1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_OUT_1EN_W<'a> {
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
#[doc = "Field `XPD_HALL_FORCE` reader - 1: XPD HALL is controlled by SW. 0: XPD HALL is controlled by FSM in ULP-coprocessor"]
pub struct XPD_HALL_FORCE_R(crate::FieldReader<bool, bool>);
impl XPD_HALL_FORCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XPD_HALL_FORCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XPD_HALL_FORCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XPD_HALL_FORCE` writer - 1: XPD HALL is controlled by SW. 0: XPD HALL is controlled by FSM in ULP-coprocessor"]
pub struct XPD_HALL_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> XPD_HALL_FORCE_W<'a> {
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
#[doc = "Field `HALL_PHASE_FORCE` reader - 1: HALL PHASE is controlled by SW 0: HALL PHASE is controlled by FSM in ULP-coprocessor"]
pub struct HALL_PHASE_FORCE_R(crate::FieldReader<bool, bool>);
impl HALL_PHASE_FORCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HALL_PHASE_FORCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HALL_PHASE_FORCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HALL_PHASE_FORCE` writer - 1: HALL PHASE is controlled by SW 0: HALL PHASE is controlled by FSM in ULP-coprocessor"]
pub struct HALL_PHASE_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> HALL_PHASE_FORCE_W<'a> {
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
impl R {
    #[doc = "Bits 0:15 - the meas length (in 8MHz)"]
    #[inline(always)]
    pub fn touch_meas_delay(&self) -> TOUCH_MEAS_DELAY_R {
        TOUCH_MEAS_DELAY_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - the waiting cycles (in 8MHz) between TOUCH_START and TOUCH_XPD"]
    #[inline(always)]
    pub fn touch_xpd_wait(&self) -> TOUCH_XPD_WAIT_R {
        TOUCH_XPD_WAIT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - 1: when the counter is greater then the threshold the touch pad is considered as \"touched\" 0: when the counter is less than the threshold the touch pad is considered as \"touched\""]
    #[inline(always)]
    pub fn touch_out_sel(&self) -> TOUCH_OUT_SEL_R {
        TOUCH_OUT_SEL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - 1: wakeup interrupt is generated if SET1 is \"touched\" 0: wakeup interrupt is generated only if SET1 & SET2 is both \"touched\""]
    #[inline(always)]
    pub fn touch_out_1en(&self) -> TOUCH_OUT_1EN_R {
        TOUCH_OUT_1EN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - 1: XPD HALL is controlled by SW. 0: XPD HALL is controlled by FSM in ULP-coprocessor"]
    #[inline(always)]
    pub fn xpd_hall_force(&self) -> XPD_HALL_FORCE_R {
        XPD_HALL_FORCE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - 1: HALL PHASE is controlled by SW 0: HALL PHASE is controlled by FSM in ULP-coprocessor"]
    #[inline(always)]
    pub fn hall_phase_force(&self) -> HALL_PHASE_FORCE_R {
        HALL_PHASE_FORCE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - the meas length (in 8MHz)"]
    #[inline(always)]
    pub fn touch_meas_delay(&mut self) -> TOUCH_MEAS_DELAY_W {
        TOUCH_MEAS_DELAY_W { w: self }
    }
    #[doc = "Bits 16:23 - the waiting cycles (in 8MHz) between TOUCH_START and TOUCH_XPD"]
    #[inline(always)]
    pub fn touch_xpd_wait(&mut self) -> TOUCH_XPD_WAIT_W {
        TOUCH_XPD_WAIT_W { w: self }
    }
    #[doc = "Bit 24 - 1: when the counter is greater then the threshold the touch pad is considered as \"touched\" 0: when the counter is less than the threshold the touch pad is considered as \"touched\""]
    #[inline(always)]
    pub fn touch_out_sel(&mut self) -> TOUCH_OUT_SEL_W {
        TOUCH_OUT_SEL_W { w: self }
    }
    #[doc = "Bit 25 - 1: wakeup interrupt is generated if SET1 is \"touched\" 0: wakeup interrupt is generated only if SET1 & SET2 is both \"touched\""]
    #[inline(always)]
    pub fn touch_out_1en(&mut self) -> TOUCH_OUT_1EN_W {
        TOUCH_OUT_1EN_W { w: self }
    }
    #[doc = "Bit 26 - 1: XPD HALL is controlled by SW. 0: XPD HALL is controlled by FSM in ULP-coprocessor"]
    #[inline(always)]
    pub fn xpd_hall_force(&mut self) -> XPD_HALL_FORCE_W {
        XPD_HALL_FORCE_W { w: self }
    }
    #[doc = "Bit 27 - 1: HALL PHASE is controlled by SW 0: HALL PHASE is controlled by FSM in ULP-coprocessor"]
    #[inline(always)]
    pub fn hall_phase_force(&mut self) -> HALL_PHASE_FORCE_W {
        HALL_PHASE_FORCE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_touch_ctrl1](index.html) module"]
pub struct SAR_TOUCH_CTRL1_SPEC;
impl crate::RegisterSpec for SAR_TOUCH_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_touch_ctrl1::R](R) reader structure"]
impl crate::Readable for SAR_TOUCH_CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_touch_ctrl1::W](W) writer structure"]
impl crate::Writable for SAR_TOUCH_CTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAR_TOUCH_CTRL1 to value 0x0204_1000"]
impl crate::Resettable for SAR_TOUCH_CTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0204_1000
    }
}
