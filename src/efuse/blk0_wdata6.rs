#[doc = "Register `BLK0_WDATA6` reader"]
pub struct R(crate::R<BLK0_WDATA6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLK0_WDATA6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLK0_WDATA6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLK0_WDATA6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLK0_WDATA6` writer"]
pub struct W(crate::W<BLK0_WDATA6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLK0_WDATA6_SPEC>;
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
impl From<crate::W<BLK0_WDATA6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLK0_WDATA6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CODING_SCHEME` reader - program for coding_scheme"]
pub struct CODING_SCHEME_R(crate::FieldReader<u8, u8>);
impl CODING_SCHEME_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CODING_SCHEME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CODING_SCHEME_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CODING_SCHEME` writer - program for coding_scheme"]
pub struct CODING_SCHEME_W<'a> {
    w: &'a mut W,
}
impl<'a> CODING_SCHEME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `CONSOLE_DEBUG_DISABLE` reader - program for console_debug_disable"]
pub struct CONSOLE_DEBUG_DISABLE_R(crate::FieldReader<bool, bool>);
impl CONSOLE_DEBUG_DISABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CONSOLE_DEBUG_DISABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CONSOLE_DEBUG_DISABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CONSOLE_DEBUG_DISABLE` writer - program for console_debug_disable"]
pub struct CONSOLE_DEBUG_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CONSOLE_DEBUG_DISABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `DISABLE_SDIO_HOST` reader - "]
pub struct DISABLE_SDIO_HOST_R(crate::FieldReader<bool, bool>);
impl DISABLE_SDIO_HOST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DISABLE_SDIO_HOST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DISABLE_SDIO_HOST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DISABLE_SDIO_HOST` writer - "]
pub struct DISABLE_SDIO_HOST_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLE_SDIO_HOST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `ABS_DONE_0` reader - program for abstract_done_0"]
pub struct ABS_DONE_0_R(crate::FieldReader<bool, bool>);
impl ABS_DONE_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ABS_DONE_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ABS_DONE_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABS_DONE_0` writer - program for abstract_done_0"]
pub struct ABS_DONE_0_W<'a> {
    w: &'a mut W,
}
impl<'a> ABS_DONE_0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `ABS_DONE_1` reader - program for abstract_done_1"]
pub struct ABS_DONE_1_R(crate::FieldReader<bool, bool>);
impl ABS_DONE_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ABS_DONE_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ABS_DONE_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABS_DONE_1` writer - program for abstract_done_1"]
pub struct ABS_DONE_1_W<'a> {
    w: &'a mut W,
}
impl<'a> ABS_DONE_1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `DISABLE_JTAG` reader - program for JTAG_disable"]
pub struct DISABLE_JTAG_R(crate::FieldReader<bool, bool>);
impl DISABLE_JTAG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DISABLE_JTAG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DISABLE_JTAG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DISABLE_JTAG` writer - program for JTAG_disable"]
pub struct DISABLE_JTAG_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLE_JTAG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `DISABLE_DL_ENCRYPT` reader - program for download_dis_encrypt"]
pub struct DISABLE_DL_ENCRYPT_R(crate::FieldReader<bool, bool>);
impl DISABLE_DL_ENCRYPT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DISABLE_DL_ENCRYPT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DISABLE_DL_ENCRYPT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DISABLE_DL_ENCRYPT` writer - program for download_dis_encrypt"]
pub struct DISABLE_DL_ENCRYPT_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLE_DL_ENCRYPT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `DISABLE_DL_DECRYPT` reader - program for download_dis_decrypt"]
pub struct DISABLE_DL_DECRYPT_R(crate::FieldReader<bool, bool>);
impl DISABLE_DL_DECRYPT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DISABLE_DL_DECRYPT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DISABLE_DL_DECRYPT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DISABLE_DL_DECRYPT` writer - program for download_dis_decrypt"]
pub struct DISABLE_DL_DECRYPT_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLE_DL_DECRYPT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `DISABLE_DL_CACHE` reader - program for download_dis_cache"]
pub struct DISABLE_DL_CACHE_R(crate::FieldReader<bool, bool>);
impl DISABLE_DL_CACHE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DISABLE_DL_CACHE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DISABLE_DL_CACHE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DISABLE_DL_CACHE` writer - program for download_dis_cache"]
pub struct DISABLE_DL_CACHE_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLE_DL_CACHE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `KEY_STATUS` reader - program for key_status"]
pub struct KEY_STATUS_R(crate::FieldReader<bool, bool>);
impl KEY_STATUS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        KEY_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY_STATUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY_STATUS` writer - program for key_status"]
pub struct KEY_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_STATUS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - program for coding_scheme"]
    #[inline(always)]
    pub fn coding_scheme(&self) -> CODING_SCHEME_R {
        CODING_SCHEME_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - program for console_debug_disable"]
    #[inline(always)]
    pub fn console_debug_disable(&self) -> CONSOLE_DEBUG_DISABLE_R {
        CONSOLE_DEBUG_DISABLE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn disable_sdio_host(&self) -> DISABLE_SDIO_HOST_R {
        DISABLE_SDIO_HOST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - program for abstract_done_0"]
    #[inline(always)]
    pub fn abs_done_0(&self) -> ABS_DONE_0_R {
        ABS_DONE_0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - program for abstract_done_1"]
    #[inline(always)]
    pub fn abs_done_1(&self) -> ABS_DONE_1_R {
        ABS_DONE_1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - program for JTAG_disable"]
    #[inline(always)]
    pub fn disable_jtag(&self) -> DISABLE_JTAG_R {
        DISABLE_JTAG_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - program for download_dis_encrypt"]
    #[inline(always)]
    pub fn disable_dl_encrypt(&self) -> DISABLE_DL_ENCRYPT_R {
        DISABLE_DL_ENCRYPT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - program for download_dis_decrypt"]
    #[inline(always)]
    pub fn disable_dl_decrypt(&self) -> DISABLE_DL_DECRYPT_R {
        DISABLE_DL_DECRYPT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - program for download_dis_cache"]
    #[inline(always)]
    pub fn disable_dl_cache(&self) -> DISABLE_DL_CACHE_R {
        DISABLE_DL_CACHE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - program for key_status"]
    #[inline(always)]
    pub fn key_status(&self) -> KEY_STATUS_R {
        KEY_STATUS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - program for coding_scheme"]
    #[inline(always)]
    pub fn coding_scheme(&mut self) -> CODING_SCHEME_W {
        CODING_SCHEME_W { w: self }
    }
    #[doc = "Bit 2 - program for console_debug_disable"]
    #[inline(always)]
    pub fn console_debug_disable(&mut self) -> CONSOLE_DEBUG_DISABLE_W {
        CONSOLE_DEBUG_DISABLE_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn disable_sdio_host(&mut self) -> DISABLE_SDIO_HOST_W {
        DISABLE_SDIO_HOST_W { w: self }
    }
    #[doc = "Bit 4 - program for abstract_done_0"]
    #[inline(always)]
    pub fn abs_done_0(&mut self) -> ABS_DONE_0_W {
        ABS_DONE_0_W { w: self }
    }
    #[doc = "Bit 5 - program for abstract_done_1"]
    #[inline(always)]
    pub fn abs_done_1(&mut self) -> ABS_DONE_1_W {
        ABS_DONE_1_W { w: self }
    }
    #[doc = "Bit 6 - program for JTAG_disable"]
    #[inline(always)]
    pub fn disable_jtag(&mut self) -> DISABLE_JTAG_W {
        DISABLE_JTAG_W { w: self }
    }
    #[doc = "Bit 7 - program for download_dis_encrypt"]
    #[inline(always)]
    pub fn disable_dl_encrypt(&mut self) -> DISABLE_DL_ENCRYPT_W {
        DISABLE_DL_ENCRYPT_W { w: self }
    }
    #[doc = "Bit 8 - program for download_dis_decrypt"]
    #[inline(always)]
    pub fn disable_dl_decrypt(&mut self) -> DISABLE_DL_DECRYPT_W {
        DISABLE_DL_DECRYPT_W { w: self }
    }
    #[doc = "Bit 9 - program for download_dis_cache"]
    #[inline(always)]
    pub fn disable_dl_cache(&mut self) -> DISABLE_DL_CACHE_W {
        DISABLE_DL_CACHE_W { w: self }
    }
    #[doc = "Bit 10 - program for key_status"]
    #[inline(always)]
    pub fn key_status(&mut self) -> KEY_STATUS_W {
        KEY_STATUS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk0_wdata6](index.html) module"]
pub struct BLK0_WDATA6_SPEC;
impl crate::RegisterSpec for BLK0_WDATA6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blk0_wdata6::R](R) reader structure"]
impl crate::Readable for BLK0_WDATA6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [blk0_wdata6::W](W) writer structure"]
impl crate::Writable for BLK0_WDATA6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BLK0_WDATA6 to value 0"]
impl crate::Resettable for BLK0_WDATA6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
