#[doc = "Register `cali_conf` reader"]
pub struct R(crate::R<CALI_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CALI_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CALI_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CALI_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `cali_conf` writer"]
pub struct W(crate::W<CALI_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CALI_CONF_SPEC>;
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
impl From<crate::W<CALI_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CALI_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CALI_RTC_MAX` reader - "]
pub struct CALI_RTC_MAX_R(crate::FieldReader<u16, u16>);
impl CALI_RTC_MAX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CALI_RTC_MAX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CALI_RTC_MAX_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CALI_RTC_MAX` writer - "]
pub struct CALI_RTC_MAX_W<'a> {
    w: &'a mut W,
}
impl<'a> CALI_RTC_MAX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
#[doc = "Field `CALI_START` reader - "]
pub struct CALI_START_R(crate::FieldReader<bool, bool>);
impl CALI_START_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CALI_START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CALI_START_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CALI_START` writer - "]
pub struct CALI_START_W<'a> {
    w: &'a mut W,
}
impl<'a> CALI_START_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn cali_rtc_max(&self) -> CALI_RTC_MAX_R {
        CALI_RTC_MAX_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn cali_start(&self) -> CALI_START_R {
        CALI_START_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn cali_rtc_max(&mut self) -> CALI_RTC_MAX_W {
        CALI_RTC_MAX_W { w: self }
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn cali_start(&mut self) -> CALI_START_W {
        CALI_START_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cali_conf](index.html) module"]
pub struct CALI_CONF_SPEC;
impl crate::RegisterSpec for CALI_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cali_conf::R](R) reader structure"]
impl crate::Readable for CALI_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cali_conf::W](W) writer structure"]
impl crate::Writable for CALI_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets cali_conf to value 0"]
impl crate::Resettable for CALI_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
