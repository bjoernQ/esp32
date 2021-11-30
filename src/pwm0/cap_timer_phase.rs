#[doc = "Register `CAP_TIMER_PHASE` reader"]
pub struct R(crate::R<CAP_TIMER_PHASE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAP_TIMER_PHASE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAP_TIMER_PHASE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAP_TIMER_PHASE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAP_TIMER_PHASE` writer"]
pub struct W(crate::W<CAP_TIMER_PHASE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAP_TIMER_PHASE_SPEC>;
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
impl From<crate::W<CAP_TIMER_PHASE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAP_TIMER_PHASE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAP_TIMER_PHASE` reader - "]
pub struct CAP_TIMER_PHASE_R(crate::FieldReader<u32, u32>);
impl CAP_TIMER_PHASE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CAP_TIMER_PHASE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAP_TIMER_PHASE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAP_TIMER_PHASE` writer - "]
pub struct CAP_TIMER_PHASE_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP_TIMER_PHASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cap_timer_phase(&self) -> CAP_TIMER_PHASE_R {
        CAP_TIMER_PHASE_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cap_timer_phase(&mut self) -> CAP_TIMER_PHASE_W {
        CAP_TIMER_PHASE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cap_timer_phase](index.html) module"]
pub struct CAP_TIMER_PHASE_SPEC;
impl crate::RegisterSpec for CAP_TIMER_PHASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cap_timer_phase::R](R) reader structure"]
impl crate::Readable for CAP_TIMER_PHASE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cap_timer_phase::W](W) writer structure"]
impl crate::Writable for CAP_TIMER_PHASE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAP_TIMER_PHASE to value 0"]
impl crate::Resettable for CAP_TIMER_PHASE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
