#[doc = "Register `TIMER1_CFG1` reader"]
pub struct R(crate::R<TIMER1_CFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER1_CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER1_CFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER1_CFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER1_CFG1` writer"]
pub struct W(crate::W<TIMER1_CFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER1_CFG1_SPEC>;
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
impl From<crate::W<TIMER1_CFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER1_CFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER1_START` reader - "]
pub struct TIMER1_START_R(crate::FieldReader<u8, u8>);
impl TIMER1_START_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TIMER1_START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER1_START_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER1_START` writer - "]
pub struct TIMER1_START_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER1_START_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `TIMER1_MOD` reader - "]
pub struct TIMER1_MOD_R(crate::FieldReader<u8, u8>);
impl TIMER1_MOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TIMER1_MOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER1_MOD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER1_MOD` writer - "]
pub struct TIMER1_MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER1_MOD_W<'a> {
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
    pub fn timer1_start(&self) -> TIMER1_START_R {
        TIMER1_START_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn timer1_mod(&self) -> TIMER1_MOD_R {
        TIMER1_MOD_R::new(((self.bits >> 3) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn timer1_start(&mut self) -> TIMER1_START_W {
        TIMER1_START_W { w: self }
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn timer1_mod(&mut self) -> TIMER1_MOD_W {
        TIMER1_MOD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer1_cfg1](index.html) module"]
pub struct TIMER1_CFG1_SPEC;
impl crate::RegisterSpec for TIMER1_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer1_cfg1::R](R) reader structure"]
impl crate::Readable for TIMER1_CFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer1_cfg1::W](W) writer structure"]
impl crate::Writable for TIMER1_CFG1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMER1_CFG1 to value 0"]
impl crate::Resettable for TIMER1_CFG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
