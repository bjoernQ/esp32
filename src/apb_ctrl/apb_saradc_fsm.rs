#[doc = "Register `APB_SARADC_FSM` reader"]
pub struct R(crate::R<APB_SARADC_FSM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB_SARADC_FSM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB_SARADC_FSM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB_SARADC_FSM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB_SARADC_FSM` writer"]
pub struct W(crate::W<APB_SARADC_FSM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB_SARADC_FSM_SPEC>;
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
impl From<crate::W<APB_SARADC_FSM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB_SARADC_FSM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SARADC_RSTB_WAIT` reader - "]
pub struct SARADC_RSTB_WAIT_R(crate::FieldReader<u8, u8>);
impl SARADC_RSTB_WAIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SARADC_RSTB_WAIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SARADC_RSTB_WAIT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SARADC_RSTB_WAIT` writer - "]
pub struct SARADC_RSTB_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> SARADC_RSTB_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `SARADC_STANDBY_WAIT` reader - "]
pub struct SARADC_STANDBY_WAIT_R(crate::FieldReader<u8, u8>);
impl SARADC_STANDBY_WAIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SARADC_STANDBY_WAIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SARADC_STANDBY_WAIT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SARADC_STANDBY_WAIT` writer - "]
pub struct SARADC_STANDBY_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> SARADC_STANDBY_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `SARADC_START_WAIT` reader - "]
pub struct SARADC_START_WAIT_R(crate::FieldReader<u8, u8>);
impl SARADC_START_WAIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SARADC_START_WAIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SARADC_START_WAIT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SARADC_START_WAIT` writer - "]
pub struct SARADC_START_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> SARADC_START_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `SARADC_SAMPLE_CYCLE` reader - sample cycles"]
pub struct SARADC_SAMPLE_CYCLE_R(crate::FieldReader<u8, u8>);
impl SARADC_SAMPLE_CYCLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SARADC_SAMPLE_CYCLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SARADC_SAMPLE_CYCLE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SARADC_SAMPLE_CYCLE` writer - sample cycles"]
pub struct SARADC_SAMPLE_CYCLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SARADC_SAMPLE_CYCLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn saradc_rstb_wait(&self) -> SARADC_RSTB_WAIT_R {
        SARADC_RSTB_WAIT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn saradc_standby_wait(&self) -> SARADC_STANDBY_WAIT_R {
        SARADC_STANDBY_WAIT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn saradc_start_wait(&self) -> SARADC_START_WAIT_R {
        SARADC_START_WAIT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - sample cycles"]
    #[inline(always)]
    pub fn saradc_sample_cycle(&self) -> SARADC_SAMPLE_CYCLE_R {
        SARADC_SAMPLE_CYCLE_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn saradc_rstb_wait(&mut self) -> SARADC_RSTB_WAIT_W {
        SARADC_RSTB_WAIT_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn saradc_standby_wait(&mut self) -> SARADC_STANDBY_WAIT_W {
        SARADC_STANDBY_WAIT_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn saradc_start_wait(&mut self) -> SARADC_START_WAIT_W {
        SARADC_START_WAIT_W { w: self }
    }
    #[doc = "Bits 24:31 - sample cycles"]
    #[inline(always)]
    pub fn saradc_sample_cycle(&mut self) -> SARADC_SAMPLE_CYCLE_W {
        SARADC_SAMPLE_CYCLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_saradc_fsm](index.html) module"]
pub struct APB_SARADC_FSM_SPEC;
impl crate::RegisterSpec for APB_SARADC_FSM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb_saradc_fsm::R](R) reader structure"]
impl crate::Readable for APB_SARADC_FSM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb_saradc_fsm::W](W) writer structure"]
impl crate::Writable for APB_SARADC_FSM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APB_SARADC_FSM to value 0x0208_ff08"]
impl crate::Resettable for APB_SARADC_FSM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0208_ff08
    }
}
