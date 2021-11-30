#[doc = "Register `ACCESS_CHECK` reader"]
pub struct R(crate::R<ACCESS_CHECK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACCESS_CHECK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACCESS_CHECK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACCESS_CHECK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PRO` reader - "]
pub struct PRO_R(crate::FieldReader<bool, bool>);
impl PRO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APP` reader - "]
pub struct APP_R(crate::FieldReader<bool, bool>);
impl APP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pro(&self) -> PRO_R {
        PRO_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn app(&self) -> APP_R {
        APP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [access_check](index.html) module"]
pub struct ACCESS_CHECK_SPEC;
impl crate::RegisterSpec for ACCESS_CHECK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [access_check::R](R) reader structure"]
impl crate::Readable for ACCESS_CHECK_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ACCESS_CHECK to value 0"]
impl crate::Resettable for ACCESS_CHECK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
