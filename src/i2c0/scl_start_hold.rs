#[doc = "Register `SCL_START_HOLD` reader"]
pub struct R(crate::R<SCL_START_HOLD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCL_START_HOLD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCL_START_HOLD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCL_START_HOLD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCL_START_HOLD` writer"]
pub struct W(crate::W<SCL_START_HOLD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCL_START_HOLD_SPEC>;
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
impl From<crate::W<SCL_START_HOLD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCL_START_HOLD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIME` reader - This register is used to configure the clock num between the negedge of SDA and negedge of SCL for start mark."]
pub struct TIME_R(crate::FieldReader<u16, u16>);
impl TIME_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TIME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIME_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIME` writer - This register is used to configure the clock num between the negedge of SDA and negedge of SCL for start mark."]
pub struct TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - This register is used to configure the clock num between the negedge of SDA and negedge of SCL for start mark."]
    #[inline(always)]
    pub fn time(&self) -> TIME_R {
        TIME_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - This register is used to configure the clock num between the negedge of SDA and negedge of SCL for start mark."]
    #[inline(always)]
    pub fn time(&mut self) -> TIME_W {
        TIME_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scl_start_hold](index.html) module"]
pub struct SCL_START_HOLD_SPEC;
impl crate::RegisterSpec for SCL_START_HOLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scl_start_hold::R](R) reader structure"]
impl crate::Readable for SCL_START_HOLD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scl_start_hold::W](W) writer structure"]
impl crate::Writable for SCL_START_HOLD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCL_START_HOLD to value 0x08"]
impl crate::Resettable for SCL_START_HOLD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x08
    }
}
