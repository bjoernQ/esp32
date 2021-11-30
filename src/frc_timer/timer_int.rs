#[doc = "Register `TIMER_INT` reader"]
pub struct R(crate::R<TIMER_INT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER_INT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER_INT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER_INT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER_INT` writer"]
pub struct W(crate::W<TIMER_INT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER_INT_SPEC>;
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
impl From<crate::W<TIMER_INT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER_INT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLR` reader - "]
pub struct CLR_R(crate::FieldReader<bool, bool>);
impl CLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLR` writer - "]
pub struct CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_W<'a> {
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
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn clr(&self) -> CLR_R {
        CLR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn clr(&mut self) -> CLR_W {
        CLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer_int](index.html) module"]
pub struct TIMER_INT_SPEC;
impl crate::RegisterSpec for TIMER_INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer_int::R](R) reader structure"]
impl crate::Readable for TIMER_INT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer_int::W](W) writer structure"]
impl crate::Writable for TIMER_INT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMER_INT to value 0"]
impl crate::Resettable for TIMER_INT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
