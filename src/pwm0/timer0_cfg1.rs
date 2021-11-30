#[doc = "Register `TIMER0_CFG1` reader"]
pub struct R(crate::R<TIMER0_CFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER0_CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER0_CFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER0_CFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER0_CFG1` writer"]
pub struct W(crate::W<TIMER0_CFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER0_CFG1_SPEC>;
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
impl From<crate::W<TIMER0_CFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER0_CFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER0_START` reader - "]
pub struct TIMER0_START_R(crate::FieldReader<u8, u8>);
impl TIMER0_START_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TIMER0_START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER0_START_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER0_START` writer - "]
pub struct TIMER0_START_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER0_START_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `TIMER0_MOD` reader - "]
pub struct TIMER0_MOD_R(crate::FieldReader<u8, u8>);
impl TIMER0_MOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TIMER0_MOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER0_MOD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER0_MOD` writer - "]
pub struct TIMER0_MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER0_MOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | ((value as u32 & 0x03) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn timer0_start(&self) -> TIMER0_START_R {
        TIMER0_START_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn timer0_mod(&self) -> TIMER0_MOD_R {
        TIMER0_MOD_R::new(((self.bits >> 3) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn timer0_start(&mut self) -> TIMER0_START_W {
        TIMER0_START_W { w: self }
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn timer0_mod(&mut self) -> TIMER0_MOD_W {
        TIMER0_MOD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer0_cfg1](index.html) module"]
pub struct TIMER0_CFG1_SPEC;
impl crate::RegisterSpec for TIMER0_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer0_cfg1::R](R) reader structure"]
impl crate::Readable for TIMER0_CFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer0_cfg1::W](W) writer structure"]
impl crate::Writable for TIMER0_CFG1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMER0_CFG1 to value 0"]
impl crate::Resettable for TIMER0_CFG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
