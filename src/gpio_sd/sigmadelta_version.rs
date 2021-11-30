#[doc = "Register `SIGMADELTA_VERSION` reader"]
pub struct R(crate::R<SIGMADELTA_VERSION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIGMADELTA_VERSION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIGMADELTA_VERSION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIGMADELTA_VERSION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SIGMADELTA_VERSION` writer"]
pub struct W(crate::W<SIGMADELTA_VERSION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIGMADELTA_VERSION_SPEC>;
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
impl From<crate::W<SIGMADELTA_VERSION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIGMADELTA_VERSION_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO_SD_DATE` reader - "]
pub struct GPIO_SD_DATE_R(crate::FieldReader<u32, u32>);
impl GPIO_SD_DATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        GPIO_SD_DATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_SD_DATE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_SD_DATE` writer - "]
pub struct GPIO_SD_DATE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_SD_DATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff_ffff) | (value as u32 & 0x0fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:27"]
    #[inline(always)]
    pub fn gpio_sd_date(&self) -> GPIO_SD_DATE_R {
        GPIO_SD_DATE_R::new((self.bits & 0x0fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:27"]
    #[inline(always)]
    pub fn gpio_sd_date(&mut self) -> GPIO_SD_DATE_W {
        GPIO_SD_DATE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sigmadelta_version](index.html) module"]
pub struct SIGMADELTA_VERSION_SPEC;
impl crate::RegisterSpec for SIGMADELTA_VERSION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sigmadelta_version::R](R) reader structure"]
impl crate::Readable for SIGMADELTA_VERSION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sigmadelta_version::W](W) writer structure"]
impl crate::Writable for SIGMADELTA_VERSION_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SIGMADELTA_VERSION to value 0x0150_6190"]
impl crate::Resettable for SIGMADELTA_VERSION_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0150_6190
    }
}
