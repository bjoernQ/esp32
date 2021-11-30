#[doc = "Register `SDIO_ST` reader"]
pub struct R(crate::R<SDIO_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDIO_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDIO_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDIO_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CMD_ST` reader - "]
pub struct CMD_ST_R(crate::FieldReader<u8, u8>);
impl CMD_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CMD_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMD_ST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FUNC_ST` reader - "]
pub struct FUNC_ST_R(crate::FieldReader<u8, u8>);
impl FUNC_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FUNC_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FUNC_ST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIO_WAKEUP` reader - "]
pub struct SDIO_WAKEUP_R(crate::FieldReader<bool, bool>);
impl SDIO_WAKEUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDIO_WAKEUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIO_WAKEUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUS_ST` reader - "]
pub struct BUS_ST_R(crate::FieldReader<u8, u8>);
impl BUS_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BUS_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUS_ST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FUNC1_ACC_STATE` reader - "]
pub struct FUNC1_ACC_STATE_R(crate::FieldReader<u8, u8>);
impl FUNC1_ACC_STATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FUNC1_ACC_STATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FUNC1_ACC_STATE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FUNC2_ACC_STATE` reader - "]
pub struct FUNC2_ACC_STATE_R(crate::FieldReader<u8, u8>);
impl FUNC2_ACC_STATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FUNC2_ACC_STATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FUNC2_ACC_STATE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn cmd_st(&self) -> CMD_ST_R {
        CMD_ST_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn func_st(&self) -> FUNC_ST_R {
        FUNC_ST_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn sdio_wakeup(&self) -> SDIO_WAKEUP_R {
        SDIO_WAKEUP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn bus_st(&self) -> BUS_ST_R {
        BUS_ST_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn func1_acc_state(&self) -> FUNC1_ACC_STATE_R {
        FUNC1_ACC_STATE_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28"]
    #[inline(always)]
    pub fn func2_acc_state(&self) -> FUNC2_ACC_STATE_R {
        FUNC2_ACC_STATE_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdio_st](index.html) module"]
pub struct SDIO_ST_SPEC;
impl crate::RegisterSpec for SDIO_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdio_st::R](R) reader structure"]
impl crate::Readable for SDIO_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SDIO_ST to value 0"]
impl crate::Resettable for SDIO_ST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
