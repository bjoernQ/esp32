#[doc = "Register `TIMER_CTRL` reader"]
pub struct R(crate::R<TIMER_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER_CTRL` writer"]
pub struct W(crate::W<TIMER_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER_CTRL_SPEC>;
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
impl From<crate::W<TIMER_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER_PRESCALER` reader - "]
pub struct TIMER_PRESCALER_R(crate::FieldReader<u8, u8>);
impl TIMER_PRESCALER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TIMER_PRESCALER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER_PRESCALER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER_PRESCALER` writer - "]
pub struct TIMER_PRESCALER_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_PRESCALER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 1)) | ((value as u32 & 0xff) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bits 1:8"]
    #[inline(always)]
    pub fn timer_prescaler(&self) -> TIMER_PRESCALER_R {
        TIMER_PRESCALER_R::new(((self.bits >> 1) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 1:8"]
    #[inline(always)]
    pub fn timer_prescaler(&mut self) -> TIMER_PRESCALER_W {
        TIMER_PRESCALER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer_ctrl](index.html) module"]
pub struct TIMER_CTRL_SPEC;
impl crate::RegisterSpec for TIMER_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer_ctrl::R](R) reader structure"]
impl crate::Readable for TIMER_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer_ctrl::W](W) writer structure"]
impl crate::Writable for TIMER_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMER_CTRL to value 0"]
impl crate::Resettable for TIMER_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
