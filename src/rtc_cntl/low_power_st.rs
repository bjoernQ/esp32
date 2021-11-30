#[doc = "Register `LOW_POWER_ST` reader"]
pub struct R(crate::R<LOW_POWER_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOW_POWER_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOW_POWER_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOW_POWER_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LOW_POWER_DIAG0` reader - "]
pub struct LOW_POWER_DIAG0_R(crate::FieldReader<u32, u32>);
impl LOW_POWER_DIAG0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        LOW_POWER_DIAG0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOW_POWER_DIAG0_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDY_FOR_WAKEUP` reader - 1 if RTC controller is ready to execute WAKE instruction, 0 otherwise"]
pub struct RDY_FOR_WAKEUP_R(crate::FieldReader<bool, bool>);
impl RDY_FOR_WAKEUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RDY_FOR_WAKEUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDY_FOR_WAKEUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn low_power_diag0(&self) -> LOW_POWER_DIAG0_R {
        LOW_POWER_DIAG0_R::new(self.bits as u32)
    }
    #[doc = "Bit 19 - 1 if RTC controller is ready to execute WAKE instruction, 0 otherwise"]
    #[inline(always)]
    pub fn rdy_for_wakeup(&self) -> RDY_FOR_WAKEUP_R {
        RDY_FOR_WAKEUP_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [low_power_st](index.html) module"]
pub struct LOW_POWER_ST_SPEC;
impl crate::RegisterSpec for LOW_POWER_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [low_power_st::R](R) reader structure"]
impl crate::Readable for LOW_POWER_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LOW_POWER_ST to value 0"]
impl crate::Resettable for LOW_POWER_ST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
