#[doc = "Register `BLK0_RDATA6` reader"]
pub struct R(crate::R<BLK0_RDATA6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLK0_RDATA6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLK0_RDATA6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLK0_RDATA6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RD_CODING_SCHEME` reader - read for coding_scheme"]
pub struct RD_CODING_SCHEME_R(crate::FieldReader<u8, u8>);
impl RD_CODING_SCHEME_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RD_CODING_SCHEME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_CODING_SCHEME_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RD_CONSOLE_DEBUG_DISABLE` reader - read for console_debug_disable"]
pub struct RD_CONSOLE_DEBUG_DISABLE_R(crate::FieldReader<bool, bool>);
impl RD_CONSOLE_DEBUG_DISABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RD_CONSOLE_DEBUG_DISABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_CONSOLE_DEBUG_DISABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RD_DISABLE_SDIO_HOST` reader - "]
pub struct RD_DISABLE_SDIO_HOST_R(crate::FieldReader<bool, bool>);
impl RD_DISABLE_SDIO_HOST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RD_DISABLE_SDIO_HOST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_DISABLE_SDIO_HOST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RD_ABS_DONE_0` reader - read for abstract_done_0"]
pub struct RD_ABS_DONE_0_R(crate::FieldReader<bool, bool>);
impl RD_ABS_DONE_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RD_ABS_DONE_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_ABS_DONE_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RD_ABS_DONE_1` reader - read for abstract_done_1"]
pub struct RD_ABS_DONE_1_R(crate::FieldReader<bool, bool>);
impl RD_ABS_DONE_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RD_ABS_DONE_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_ABS_DONE_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RD_DISABLE_JTAG` reader - read for JTAG_disable"]
pub struct RD_DISABLE_JTAG_R(crate::FieldReader<bool, bool>);
impl RD_DISABLE_JTAG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RD_DISABLE_JTAG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_DISABLE_JTAG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RD_DISABLE_DL_ENCRYPT` reader - read for download_dis_encrypt"]
pub struct RD_DISABLE_DL_ENCRYPT_R(crate::FieldReader<bool, bool>);
impl RD_DISABLE_DL_ENCRYPT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RD_DISABLE_DL_ENCRYPT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_DISABLE_DL_ENCRYPT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RD_DISABLE_DL_DECRYPT` reader - read for download_dis_decrypt"]
pub struct RD_DISABLE_DL_DECRYPT_R(crate::FieldReader<bool, bool>);
impl RD_DISABLE_DL_DECRYPT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RD_DISABLE_DL_DECRYPT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_DISABLE_DL_DECRYPT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RD_DISABLE_DL_CACHE` reader - read for download_dis_cache"]
pub struct RD_DISABLE_DL_CACHE_R(crate::FieldReader<bool, bool>);
impl RD_DISABLE_DL_CACHE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RD_DISABLE_DL_CACHE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_DISABLE_DL_CACHE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RD_KEY_STATUS` reader - read for key_status"]
pub struct RD_KEY_STATUS_R(crate::FieldReader<bool, bool>);
impl RD_KEY_STATUS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RD_KEY_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_KEY_STATUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:1 - read for coding_scheme"]
    #[inline(always)]
    pub fn rd_coding_scheme(&self) -> RD_CODING_SCHEME_R {
        RD_CODING_SCHEME_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - read for console_debug_disable"]
    #[inline(always)]
    pub fn rd_console_debug_disable(&self) -> RD_CONSOLE_DEBUG_DISABLE_R {
        RD_CONSOLE_DEBUG_DISABLE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rd_disable_sdio_host(&self) -> RD_DISABLE_SDIO_HOST_R {
        RD_DISABLE_SDIO_HOST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - read for abstract_done_0"]
    #[inline(always)]
    pub fn rd_abs_done_0(&self) -> RD_ABS_DONE_0_R {
        RD_ABS_DONE_0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - read for abstract_done_1"]
    #[inline(always)]
    pub fn rd_abs_done_1(&self) -> RD_ABS_DONE_1_R {
        RD_ABS_DONE_1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - read for JTAG_disable"]
    #[inline(always)]
    pub fn rd_disable_jtag(&self) -> RD_DISABLE_JTAG_R {
        RD_DISABLE_JTAG_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - read for download_dis_encrypt"]
    #[inline(always)]
    pub fn rd_disable_dl_encrypt(&self) -> RD_DISABLE_DL_ENCRYPT_R {
        RD_DISABLE_DL_ENCRYPT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - read for download_dis_decrypt"]
    #[inline(always)]
    pub fn rd_disable_dl_decrypt(&self) -> RD_DISABLE_DL_DECRYPT_R {
        RD_DISABLE_DL_DECRYPT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - read for download_dis_cache"]
    #[inline(always)]
    pub fn rd_disable_dl_cache(&self) -> RD_DISABLE_DL_CACHE_R {
        RD_DISABLE_DL_CACHE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - read for key_status"]
    #[inline(always)]
    pub fn rd_key_status(&self) -> RD_KEY_STATUS_R {
        RD_KEY_STATUS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk0_rdata6](index.html) module"]
pub struct BLK0_RDATA6_SPEC;
impl crate::RegisterSpec for BLK0_RDATA6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blk0_rdata6::R](R) reader structure"]
impl crate::Readable for BLK0_RDATA6_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BLK0_RDATA6 to value 0"]
impl crate::Resettable for BLK0_RDATA6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
