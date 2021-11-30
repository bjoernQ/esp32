#[doc = "Register `STATE0` reader"]
pub struct R(crate::R<STATE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATE0` writer"]
pub struct W(crate::W<STATE0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATE0_SPEC>;
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
impl From<crate::W<STATE0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATE0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOUCH_WAKEUP_FORCE_EN` reader - touch controller force wake up"]
pub struct TOUCH_WAKEUP_FORCE_EN_R(crate::FieldReader<bool, bool>);
impl TOUCH_WAKEUP_FORCE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOUCH_WAKEUP_FORCE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_WAKEUP_FORCE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_WAKEUP_FORCE_EN` writer - touch controller force wake up"]
pub struct TOUCH_WAKEUP_FORCE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_WAKEUP_FORCE_EN_W<'a> {
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
#[doc = "Field `ULP_CP_WAKEUP_FORCE_EN` reader - ULP-coprocessor force wake up"]
pub struct ULP_CP_WAKEUP_FORCE_EN_R(crate::FieldReader<bool, bool>);
impl ULP_CP_WAKEUP_FORCE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ULP_CP_WAKEUP_FORCE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ULP_CP_WAKEUP_FORCE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ULP_CP_WAKEUP_FORCE_EN` writer - ULP-coprocessor force wake up"]
pub struct ULP_CP_WAKEUP_FORCE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ULP_CP_WAKEUP_FORCE_EN_W<'a> {
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
#[doc = "Field `APB2RTC_BRIDGE_SEL` reader - 1: APB to RTC using bridge 0: APB to RTC using sync"]
pub struct APB2RTC_BRIDGE_SEL_R(crate::FieldReader<bool, bool>);
impl APB2RTC_BRIDGE_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APB2RTC_BRIDGE_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB2RTC_BRIDGE_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APB2RTC_BRIDGE_SEL` writer - 1: APB to RTC using bridge 0: APB to RTC using sync"]
pub struct APB2RTC_BRIDGE_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> APB2RTC_BRIDGE_SEL_W<'a> {
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
#[doc = "Field `TOUCH_SLP_TIMER_EN` reader - touch timer enable bit"]
pub struct TOUCH_SLP_TIMER_EN_R(crate::FieldReader<bool, bool>);
impl TOUCH_SLP_TIMER_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOUCH_SLP_TIMER_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_SLP_TIMER_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_SLP_TIMER_EN` writer - touch timer enable bit"]
pub struct TOUCH_SLP_TIMER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_SLP_TIMER_EN_W<'a> {
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
#[doc = "Field `ULP_CP_SLP_TIMER_EN` reader - ULP-coprocessor timer enable bit"]
pub struct ULP_CP_SLP_TIMER_EN_R(crate::FieldReader<bool, bool>);
impl ULP_CP_SLP_TIMER_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ULP_CP_SLP_TIMER_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ULP_CP_SLP_TIMER_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ULP_CP_SLP_TIMER_EN` writer - ULP-coprocessor timer enable bit"]
pub struct ULP_CP_SLP_TIMER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ULP_CP_SLP_TIMER_EN_W<'a> {
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
#[doc = "Field `SDIO_ACTIVE_IND` reader - SDIO active indication"]
pub struct SDIO_ACTIVE_IND_R(crate::FieldReader<bool, bool>);
impl SDIO_ACTIVE_IND_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDIO_ACTIVE_IND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIO_ACTIVE_IND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLP_WAKEUP` reader - sleep wakeup bit"]
pub struct SLP_WAKEUP_R(crate::FieldReader<bool, bool>);
impl SLP_WAKEUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLP_WAKEUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLP_WAKEUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLP_WAKEUP` writer - sleep wakeup bit"]
pub struct SLP_WAKEUP_W<'a> {
    w: &'a mut W,
}
impl<'a> SLP_WAKEUP_W<'a> {
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
#[doc = "Field `SLP_REJECT` reader - sleep reject bit"]
pub struct SLP_REJECT_R(crate::FieldReader<bool, bool>);
impl SLP_REJECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLP_REJECT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLP_REJECT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLP_REJECT` writer - sleep reject bit"]
pub struct SLP_REJECT_W<'a> {
    w: &'a mut W,
}
impl<'a> SLP_REJECT_W<'a> {
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
#[doc = "Field `SLEEP_EN` reader - sleep enable bit"]
pub struct SLEEP_EN_R(crate::FieldReader<bool, bool>);
impl SLEEP_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLEEP_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLEEP_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLEEP_EN` writer - sleep enable bit"]
pub struct SLEEP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEEP_EN_W<'a> {
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
    #[doc = "Bit 20 - touch controller force wake up"]
    #[inline(always)]
    pub fn touch_wakeup_force_en(&self) -> TOUCH_WAKEUP_FORCE_EN_R {
        TOUCH_WAKEUP_FORCE_EN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - ULP-coprocessor force wake up"]
    #[inline(always)]
    pub fn ulp_cp_wakeup_force_en(&self) -> ULP_CP_WAKEUP_FORCE_EN_R {
        ULP_CP_WAKEUP_FORCE_EN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - 1: APB to RTC using bridge 0: APB to RTC using sync"]
    #[inline(always)]
    pub fn apb2rtc_bridge_sel(&self) -> APB2RTC_BRIDGE_SEL_R {
        APB2RTC_BRIDGE_SEL_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - touch timer enable bit"]
    #[inline(always)]
    pub fn touch_slp_timer_en(&self) -> TOUCH_SLP_TIMER_EN_R {
        TOUCH_SLP_TIMER_EN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - ULP-coprocessor timer enable bit"]
    #[inline(always)]
    pub fn ulp_cp_slp_timer_en(&self) -> ULP_CP_SLP_TIMER_EN_R {
        ULP_CP_SLP_TIMER_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 28 - SDIO active indication"]
    #[inline(always)]
    pub fn sdio_active_ind(&self) -> SDIO_ACTIVE_IND_R {
        SDIO_ACTIVE_IND_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - sleep wakeup bit"]
    #[inline(always)]
    pub fn slp_wakeup(&self) -> SLP_WAKEUP_R {
        SLP_WAKEUP_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - sleep reject bit"]
    #[inline(always)]
    pub fn slp_reject(&self) -> SLP_REJECT_R {
        SLP_REJECT_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - sleep enable bit"]
    #[inline(always)]
    pub fn sleep_en(&self) -> SLEEP_EN_R {
        SLEEP_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 20 - touch controller force wake up"]
    #[inline(always)]
    pub fn touch_wakeup_force_en(&mut self) -> TOUCH_WAKEUP_FORCE_EN_W {
        TOUCH_WAKEUP_FORCE_EN_W { w: self }
    }
    #[doc = "Bit 21 - ULP-coprocessor force wake up"]
    #[inline(always)]
    pub fn ulp_cp_wakeup_force_en(&mut self) -> ULP_CP_WAKEUP_FORCE_EN_W {
        ULP_CP_WAKEUP_FORCE_EN_W { w: self }
    }
    #[doc = "Bit 22 - 1: APB to RTC using bridge 0: APB to RTC using sync"]
    #[inline(always)]
    pub fn apb2rtc_bridge_sel(&mut self) -> APB2RTC_BRIDGE_SEL_W {
        APB2RTC_BRIDGE_SEL_W { w: self }
    }
    #[doc = "Bit 23 - touch timer enable bit"]
    #[inline(always)]
    pub fn touch_slp_timer_en(&mut self) -> TOUCH_SLP_TIMER_EN_W {
        TOUCH_SLP_TIMER_EN_W { w: self }
    }
    #[doc = "Bit 24 - ULP-coprocessor timer enable bit"]
    #[inline(always)]
    pub fn ulp_cp_slp_timer_en(&mut self) -> ULP_CP_SLP_TIMER_EN_W {
        ULP_CP_SLP_TIMER_EN_W { w: self }
    }
    #[doc = "Bit 29 - sleep wakeup bit"]
    #[inline(always)]
    pub fn slp_wakeup(&mut self) -> SLP_WAKEUP_W {
        SLP_WAKEUP_W { w: self }
    }
    #[doc = "Bit 30 - sleep reject bit"]
    #[inline(always)]
    pub fn slp_reject(&mut self) -> SLP_REJECT_W {
        SLP_REJECT_W { w: self }
    }
    #[doc = "Bit 31 - sleep enable bit"]
    #[inline(always)]
    pub fn sleep_en(&mut self) -> SLEEP_EN_W {
        SLEEP_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [state0](index.html) module"]
pub struct STATE0_SPEC;
impl crate::RegisterSpec for STATE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [state0::R](R) reader structure"]
impl crate::Readable for STATE0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [state0::W](W) writer structure"]
impl crate::Writable for STATE0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STATE0 to value 0x0030_0000"]
impl crate::Resettable for STATE0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0030_0000
    }
}
