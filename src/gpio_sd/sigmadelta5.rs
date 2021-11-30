#[doc = "Register `SIGMADELTA5` reader"]
pub struct R(crate::R<SIGMADELTA5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIGMADELTA5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIGMADELTA5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIGMADELTA5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SIGMADELTA5` writer"]
pub struct W(crate::W<SIGMADELTA5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIGMADELTA5_SPEC>;
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
impl From<crate::W<SIGMADELTA5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIGMADELTA5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO_SD5_IN` reader - "]
pub struct GPIO_SD5_IN_R(crate::FieldReader<u8, u8>);
impl GPIO_SD5_IN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GPIO_SD5_IN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_SD5_IN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_SD5_IN` writer - "]
pub struct GPIO_SD5_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_SD5_IN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `GPIO_SD5_PRESCALE` reader - "]
pub struct GPIO_SD5_PRESCALE_R(crate::FieldReader<u8, u8>);
impl GPIO_SD5_PRESCALE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GPIO_SD5_PRESCALE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_SD5_PRESCALE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_SD5_PRESCALE` writer - "]
pub struct GPIO_SD5_PRESCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_SD5_PRESCALE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn gpio_sd5_in(&self) -> GPIO_SD5_IN_R {
        GPIO_SD5_IN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn gpio_sd5_prescale(&self) -> GPIO_SD5_PRESCALE_R {
        GPIO_SD5_PRESCALE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn gpio_sd5_in(&mut self) -> GPIO_SD5_IN_W {
        GPIO_SD5_IN_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn gpio_sd5_prescale(&mut self) -> GPIO_SD5_PRESCALE_W {
        GPIO_SD5_PRESCALE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sigmadelta5](index.html) module"]
pub struct SIGMADELTA5_SPEC;
impl crate::RegisterSpec for SIGMADELTA5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sigmadelta5::R](R) reader structure"]
impl crate::Readable for SIGMADELTA5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sigmadelta5::W](W) writer structure"]
impl crate::Writable for SIGMADELTA5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SIGMADELTA5 to value 0xff00"]
impl crate::Resettable for SIGMADELTA5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xff00
    }
}
