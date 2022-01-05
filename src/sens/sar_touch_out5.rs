#[doc = "Register `SAR_TOUCH_OUT5` reader"]
pub struct R(crate::R<SAR_TOUCH_OUT5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_TOUCH_OUT5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_TOUCH_OUT5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_TOUCH_OUT5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TOUCH_MEAS_OUT9` reader - the counter for touch pad 9"]
pub struct TOUCH_MEAS_OUT9_R(crate::FieldReader<u16, u16>);
impl TOUCH_MEAS_OUT9_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TOUCH_MEAS_OUT9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_MEAS_OUT9_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_MEAS_OUT8` reader - the counter for touch pad 8"]
pub struct TOUCH_MEAS_OUT8_R(crate::FieldReader<u16, u16>);
impl TOUCH_MEAS_OUT8_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TOUCH_MEAS_OUT8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_MEAS_OUT8_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - the counter for touch pad 9"]
    #[inline(always)]
    pub fn touch_meas_out9(&self) -> TOUCH_MEAS_OUT9_R {
        TOUCH_MEAS_OUT9_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - the counter for touch pad 8"]
    #[inline(always)]
    pub fn touch_meas_out8(&self) -> TOUCH_MEAS_OUT8_R {
        TOUCH_MEAS_OUT8_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_touch_out5](index.html) module"]
pub struct SAR_TOUCH_OUT5_SPEC;
impl crate::RegisterSpec for SAR_TOUCH_OUT5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_touch_out5::R](R) reader structure"]
impl crate::Readable for SAR_TOUCH_OUT5_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SAR_TOUCH_OUT5 to value 0"]
impl crate::Resettable for SAR_TOUCH_OUT5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}