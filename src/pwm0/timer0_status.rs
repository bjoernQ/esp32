#[doc = "Register `TIMER0_STATUS` reader"]
pub struct R(crate::R<TIMER0_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER0_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER0_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER0_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TIMER0_VALUE` reader - "]
pub struct TIMER0_VALUE_R(crate::FieldReader<u16, u16>);
impl TIMER0_VALUE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TIMER0_VALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER0_VALUE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER0_DIRECTION` reader - "]
pub struct TIMER0_DIRECTION_R(crate::FieldReader<bool, bool>);
impl TIMER0_DIRECTION_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER0_DIRECTION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER0_DIRECTION_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn timer0_value(&self) -> TIMER0_VALUE_R {
        TIMER0_VALUE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn timer0_direction(&self) -> TIMER0_DIRECTION_R {
        TIMER0_DIRECTION_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer0_status](index.html) module"]
pub struct TIMER0_STATUS_SPEC;
impl crate::RegisterSpec for TIMER0_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer0_status::R](R) reader structure"]
impl crate::Readable for TIMER0_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TIMER0_STATUS to value 0"]
impl crate::Resettable for TIMER0_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}