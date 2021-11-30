#[doc = "Register `AUTOBAUD` reader"]
pub struct R(crate::R<AUTOBAUD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUTOBAUD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUTOBAUD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUTOBAUD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AUTOBAUD` writer"]
pub struct W(crate::W<AUTOBAUD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AUTOBAUD_SPEC>;
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
impl From<crate::W<AUTOBAUD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AUTOBAUD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - This is the enable bit for detecting baudrate."]
pub struct EN_R(crate::FieldReader<bool, bool>);
impl EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN` writer - This is the enable bit for detecting baudrate."]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
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
#[doc = "Field `GLITCH_FILT` reader - when input pulse width is lower then this value igore this pulse.this register is used in autobaud detect process."]
pub struct GLITCH_FILT_R(crate::FieldReader<u8, u8>);
impl GLITCH_FILT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GLITCH_FILT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GLITCH_FILT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GLITCH_FILT` writer - when input pulse width is lower then this value igore this pulse.this register is used in autobaud detect process."]
pub struct GLITCH_FILT_W<'a> {
    w: &'a mut W,
}
impl<'a> GLITCH_FILT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - This is the enable bit for detecting baudrate."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - when input pulse width is lower then this value igore this pulse.this register is used in autobaud detect process."]
    #[inline(always)]
    pub fn glitch_filt(&self) -> GLITCH_FILT_R {
        GLITCH_FILT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - This is the enable bit for detecting baudrate."]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Bits 8:15 - when input pulse width is lower then this value igore this pulse.this register is used in autobaud detect process."]
    #[inline(always)]
    pub fn glitch_filt(&mut self) -> GLITCH_FILT_W {
        GLITCH_FILT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [autobaud](index.html) module"]
pub struct AUTOBAUD_SPEC;
impl crate::RegisterSpec for AUTOBAUD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [autobaud::R](R) reader structure"]
impl crate::Readable for AUTOBAUD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [autobaud::W](W) writer structure"]
impl crate::Writable for AUTOBAUD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AUTOBAUD to value 0x1000"]
impl crate::Resettable for AUTOBAUD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1000
    }
}
