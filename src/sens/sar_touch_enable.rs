#[doc = "Register `SAR_TOUCH_ENABLE` reader"]
pub struct R(crate::R<SAR_TOUCH_ENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_TOUCH_ENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_TOUCH_ENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_TOUCH_ENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_TOUCH_ENABLE` writer"]
pub struct W(crate::W<SAR_TOUCH_ENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_TOUCH_ENABLE_SPEC>;
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
impl From<crate::W<SAR_TOUCH_ENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_TOUCH_ENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOUCH_PAD_WORKEN` reader - Bitmap defining the working set during the measurement."]
pub struct TOUCH_PAD_WORKEN_R(crate::FieldReader<u16, u16>);
impl TOUCH_PAD_WORKEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TOUCH_PAD_WORKEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_PAD_WORKEN_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_PAD_WORKEN` writer - Bitmap defining the working set during the measurement."]
pub struct TOUCH_PAD_WORKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_PAD_WORKEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
#[doc = "Field `TOUCH_PAD_OUTEN2` reader - Bitmap defining SET2 for generating wakeup interrupt. SET2 is \"touched\" only if at least one of touch pad in SET2 is \"touched\"."]
pub struct TOUCH_PAD_OUTEN2_R(crate::FieldReader<u16, u16>);
impl TOUCH_PAD_OUTEN2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TOUCH_PAD_OUTEN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_PAD_OUTEN2_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_PAD_OUTEN2` writer - Bitmap defining SET2 for generating wakeup interrupt. SET2 is \"touched\" only if at least one of touch pad in SET2 is \"touched\"."]
pub struct TOUCH_PAD_OUTEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_PAD_OUTEN2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 10)) | ((value as u32 & 0x03ff) << 10);
        self.w
    }
}
#[doc = "Field `TOUCH_PAD_OUTEN1` reader - Bitmap defining SET1 for generating wakeup interrupt. SET1 is \"touched\" only if at least one of touch pad in SET1 is \"touched\"."]
pub struct TOUCH_PAD_OUTEN1_R(crate::FieldReader<u16, u16>);
impl TOUCH_PAD_OUTEN1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TOUCH_PAD_OUTEN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_PAD_OUTEN1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_PAD_OUTEN1` writer - Bitmap defining SET1 for generating wakeup interrupt. SET1 is \"touched\" only if at least one of touch pad in SET1 is \"touched\"."]
pub struct TOUCH_PAD_OUTEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_PAD_OUTEN1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 20)) | ((value as u32 & 0x03ff) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Bitmap defining the working set during the measurement."]
    #[inline(always)]
    pub fn touch_pad_worken(&self) -> TOUCH_PAD_WORKEN_R {
        TOUCH_PAD_WORKEN_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - Bitmap defining SET2 for generating wakeup interrupt. SET2 is \"touched\" only if at least one of touch pad in SET2 is \"touched\"."]
    #[inline(always)]
    pub fn touch_pad_outen2(&self) -> TOUCH_PAD_OUTEN2_R {
        TOUCH_PAD_OUTEN2_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:29 - Bitmap defining SET1 for generating wakeup interrupt. SET1 is \"touched\" only if at least one of touch pad in SET1 is \"touched\"."]
    #[inline(always)]
    pub fn touch_pad_outen1(&self) -> TOUCH_PAD_OUTEN1_R {
        TOUCH_PAD_OUTEN1_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Bitmap defining the working set during the measurement."]
    #[inline(always)]
    pub fn touch_pad_worken(&mut self) -> TOUCH_PAD_WORKEN_W {
        TOUCH_PAD_WORKEN_W { w: self }
    }
    #[doc = "Bits 10:19 - Bitmap defining SET2 for generating wakeup interrupt. SET2 is \"touched\" only if at least one of touch pad in SET2 is \"touched\"."]
    #[inline(always)]
    pub fn touch_pad_outen2(&mut self) -> TOUCH_PAD_OUTEN2_W {
        TOUCH_PAD_OUTEN2_W { w: self }
    }
    #[doc = "Bits 20:29 - Bitmap defining SET1 for generating wakeup interrupt. SET1 is \"touched\" only if at least one of touch pad in SET1 is \"touched\"."]
    #[inline(always)]
    pub fn touch_pad_outen1(&mut self) -> TOUCH_PAD_OUTEN1_W {
        TOUCH_PAD_OUTEN1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_touch_enable](index.html) module"]
pub struct SAR_TOUCH_ENABLE_SPEC;
impl crate::RegisterSpec for SAR_TOUCH_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_touch_enable::R](R) reader structure"]
impl crate::Readable for SAR_TOUCH_ENABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_touch_enable::W](W) writer structure"]
impl crate::Writable for SAR_TOUCH_ENABLE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAR_TOUCH_ENABLE to value 0x3fff_ffff"]
impl crate::Resettable for SAR_TOUCH_ENABLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3fff_ffff
    }
}
