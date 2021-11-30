#[doc = "Register `SAR_TOUCH_OUT1` reader"]
pub struct R(crate::R<SAR_TOUCH_OUT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_TOUCH_OUT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_TOUCH_OUT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_TOUCH_OUT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TOUCH_MEAS_OUT1` reader - the counter for touch pad 1"]
pub struct TOUCH_MEAS_OUT1_R(crate::FieldReader<u16, u16>);
impl TOUCH_MEAS_OUT1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TOUCH_MEAS_OUT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_MEAS_OUT1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_MEAS_OUT0` reader - the counter for touch pad 0"]
pub struct TOUCH_MEAS_OUT0_R(crate::FieldReader<u16, u16>);
impl TOUCH_MEAS_OUT0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TOUCH_MEAS_OUT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_MEAS_OUT0_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - the counter for touch pad 1"]
    #[inline(always)]
    pub fn touch_meas_out1(&self) -> TOUCH_MEAS_OUT1_R {
        TOUCH_MEAS_OUT1_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - the counter for touch pad 0"]
    #[inline(always)]
    pub fn touch_meas_out0(&self) -> TOUCH_MEAS_OUT0_R {
        TOUCH_MEAS_OUT0_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_touch_out1](index.html) module"]
pub struct SAR_TOUCH_OUT1_SPEC;
impl crate::RegisterSpec for SAR_TOUCH_OUT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_touch_out1::R](R) reader structure"]
impl crate::Readable for SAR_TOUCH_OUT1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SAR_TOUCH_OUT1 to value 0"]
impl crate::Resettable for SAR_TOUCH_OUT1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
