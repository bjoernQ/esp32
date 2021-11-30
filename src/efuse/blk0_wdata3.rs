#[doc = "Register `BLK0_WDATA3` reader"]
pub struct R(crate::R<BLK0_WDATA3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLK0_WDATA3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLK0_WDATA3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLK0_WDATA3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLK0_WDATA3` writer"]
pub struct W(crate::W<BLK0_WDATA3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLK0_WDATA3_SPEC>;
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
impl From<crate::W<BLK0_WDATA3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLK0_WDATA3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHIP_VER_DIS_APP_CPU` reader - "]
pub struct CHIP_VER_DIS_APP_CPU_R(crate::FieldReader<bool, bool>);
impl CHIP_VER_DIS_APP_CPU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHIP_VER_DIS_APP_CPU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHIP_VER_DIS_APP_CPU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHIP_VER_DIS_APP_CPU` writer - "]
pub struct CHIP_VER_DIS_APP_CPU_W<'a> {
    w: &'a mut W,
}
impl<'a> CHIP_VER_DIS_APP_CPU_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `CHIP_VER_DIS_BT` reader - "]
pub struct CHIP_VER_DIS_BT_R(crate::FieldReader<bool, bool>);
impl CHIP_VER_DIS_BT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHIP_VER_DIS_BT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHIP_VER_DIS_BT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHIP_VER_DIS_BT` writer - "]
pub struct CHIP_VER_DIS_BT_W<'a> {
    w: &'a mut W,
}
impl<'a> CHIP_VER_DIS_BT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `CHIP_VER_PKG_4BIT` reader - most significant bit of chip package"]
pub struct CHIP_VER_PKG_4BIT_R(crate::FieldReader<bool, bool>);
impl CHIP_VER_PKG_4BIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHIP_VER_PKG_4BIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHIP_VER_PKG_4BIT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHIP_VER_DIS_CACHE` reader - "]
pub struct CHIP_VER_DIS_CACHE_R(crate::FieldReader<bool, bool>);
impl CHIP_VER_DIS_CACHE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHIP_VER_DIS_CACHE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHIP_VER_DIS_CACHE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHIP_VER_DIS_CACHE` writer - "]
pub struct CHIP_VER_DIS_CACHE_W<'a> {
    w: &'a mut W,
}
impl<'a> CHIP_VER_DIS_CACHE_W<'a> {
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
#[doc = "Field `SPI_PAD_CONFIG_HD` reader - program for SPI_pad_config_hd"]
pub struct SPI_PAD_CONFIG_HD_R(crate::FieldReader<u8, u8>);
impl SPI_PAD_CONFIG_HD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SPI_PAD_CONFIG_HD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_PAD_CONFIG_HD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_PAD_CONFIG_HD` writer - program for SPI_pad_config_hd"]
pub struct SPI_PAD_CONFIG_HD_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_PAD_CONFIG_HD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 4)) | ((value as u32 & 0x1f) << 4);
        self.w
    }
}
#[doc = "Field `CHIP_VER_PKG` reader - least significant bits of chip package"]
pub struct CHIP_VER_PKG_R(crate::FieldReader<u8, u8>);
impl CHIP_VER_PKG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CHIP_VER_PKG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHIP_VER_PKG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHIP_VER_PKG` writer - least significant bits of chip package"]
pub struct CHIP_VER_PKG_W<'a> {
    w: &'a mut W,
}
impl<'a> CHIP_VER_PKG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | ((value as u32 & 0x07) << 9);
        self.w
    }
}
#[doc = "Field `CHIP_CPU_FREQ_LOW` reader - If set alongside EFUSE_CHIP_CPU_FREQ_RATED, the ESP32's max CPU frequency is rated for 160MHz. 240MHz otherwise"]
pub struct CHIP_CPU_FREQ_LOW_R(crate::FieldReader<bool, bool>);
impl CHIP_CPU_FREQ_LOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHIP_CPU_FREQ_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHIP_CPU_FREQ_LOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHIP_CPU_FREQ_LOW` writer - If set alongside EFUSE_CHIP_CPU_FREQ_RATED, the ESP32's max CPU frequency is rated for 160MHz. 240MHz otherwise"]
pub struct CHIP_CPU_FREQ_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> CHIP_CPU_FREQ_LOW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `CHIP_CPU_FREQ_RATED` reader - If set, the ESP32's maximum CPU frequency has been rated"]
pub struct CHIP_CPU_FREQ_RATED_R(crate::FieldReader<bool, bool>);
impl CHIP_CPU_FREQ_RATED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHIP_CPU_FREQ_RATED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHIP_CPU_FREQ_RATED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHIP_CPU_FREQ_RATED` writer - If set, the ESP32's maximum CPU frequency has been rated"]
pub struct CHIP_CPU_FREQ_RATED_W<'a> {
    w: &'a mut W,
}
impl<'a> CHIP_CPU_FREQ_RATED_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `BLK3_PART_RESERVE` reader - If set, this bit indicates that BLOCK3\\[143:96\\]
is reserved for internal use"]
pub struct BLK3_PART_RESERVE_R(crate::FieldReader<bool, bool>);
impl BLK3_PART_RESERVE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BLK3_PART_RESERVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLK3_PART_RESERVE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLK3_PART_RESERVE` writer - If set, this bit indicates that BLOCK3\\[143:96\\]
is reserved for internal use"]
pub struct BLK3_PART_RESERVE_W<'a> {
    w: &'a mut W,
}
impl<'a> BLK3_PART_RESERVE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `CHIP_VER_REV1` reader - "]
pub struct CHIP_VER_REV1_R(crate::FieldReader<bool, bool>);
impl CHIP_VER_REV1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHIP_VER_REV1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHIP_VER_REV1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHIP_VER_REV1` writer - "]
pub struct CHIP_VER_REV1_W<'a> {
    w: &'a mut W,
}
impl<'a> CHIP_VER_REV1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn chip_ver_dis_app_cpu(&self) -> CHIP_VER_DIS_APP_CPU_R {
        CHIP_VER_DIS_APP_CPU_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn chip_ver_dis_bt(&self) -> CHIP_VER_DIS_BT_R {
        CHIP_VER_DIS_BT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - most significant bit of chip package"]
    #[inline(always)]
    pub fn chip_ver_pkg_4bit(&self) -> CHIP_VER_PKG_4BIT_R {
        CHIP_VER_PKG_4BIT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn chip_ver_dis_cache(&self) -> CHIP_VER_DIS_CACHE_R {
        CHIP_VER_DIS_CACHE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:8 - program for SPI_pad_config_hd"]
    #[inline(always)]
    pub fn spi_pad_config_hd(&self) -> SPI_PAD_CONFIG_HD_R {
        SPI_PAD_CONFIG_HD_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bits 9:11 - least significant bits of chip package"]
    #[inline(always)]
    pub fn chip_ver_pkg(&self) -> CHIP_VER_PKG_R {
        CHIP_VER_PKG_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bit 12 - If set alongside EFUSE_CHIP_CPU_FREQ_RATED, the ESP32's max CPU frequency is rated for 160MHz. 240MHz otherwise"]
    #[inline(always)]
    pub fn chip_cpu_freq_low(&self) -> CHIP_CPU_FREQ_LOW_R {
        CHIP_CPU_FREQ_LOW_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - If set, the ESP32's maximum CPU frequency has been rated"]
    #[inline(always)]
    pub fn chip_cpu_freq_rated(&self) -> CHIP_CPU_FREQ_RATED_R {
        CHIP_CPU_FREQ_RATED_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - If set, this bit indicates that BLOCK3\\[143:96\\]
is reserved for internal use"]
    #[inline(always)]
    pub fn blk3_part_reserve(&self) -> BLK3_PART_RESERVE_R {
        BLK3_PART_RESERVE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn chip_ver_rev1(&self) -> CHIP_VER_REV1_R {
        CHIP_VER_REV1_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn chip_ver_dis_app_cpu(&mut self) -> CHIP_VER_DIS_APP_CPU_W {
        CHIP_VER_DIS_APP_CPU_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn chip_ver_dis_bt(&mut self) -> CHIP_VER_DIS_BT_W {
        CHIP_VER_DIS_BT_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn chip_ver_dis_cache(&mut self) -> CHIP_VER_DIS_CACHE_W {
        CHIP_VER_DIS_CACHE_W { w: self }
    }
    #[doc = "Bits 4:8 - program for SPI_pad_config_hd"]
    #[inline(always)]
    pub fn spi_pad_config_hd(&mut self) -> SPI_PAD_CONFIG_HD_W {
        SPI_PAD_CONFIG_HD_W { w: self }
    }
    #[doc = "Bits 9:11 - least significant bits of chip package"]
    #[inline(always)]
    pub fn chip_ver_pkg(&mut self) -> CHIP_VER_PKG_W {
        CHIP_VER_PKG_W { w: self }
    }
    #[doc = "Bit 12 - If set alongside EFUSE_CHIP_CPU_FREQ_RATED, the ESP32's max CPU frequency is rated for 160MHz. 240MHz otherwise"]
    #[inline(always)]
    pub fn chip_cpu_freq_low(&mut self) -> CHIP_CPU_FREQ_LOW_W {
        CHIP_CPU_FREQ_LOW_W { w: self }
    }
    #[doc = "Bit 13 - If set, the ESP32's maximum CPU frequency has been rated"]
    #[inline(always)]
    pub fn chip_cpu_freq_rated(&mut self) -> CHIP_CPU_FREQ_RATED_W {
        CHIP_CPU_FREQ_RATED_W { w: self }
    }
    #[doc = "Bit 14 - If set, this bit indicates that BLOCK3\\[143:96\\]
is reserved for internal use"]
    #[inline(always)]
    pub fn blk3_part_reserve(&mut self) -> BLK3_PART_RESERVE_W {
        BLK3_PART_RESERVE_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn chip_ver_rev1(&mut self) -> CHIP_VER_REV1_W {
        CHIP_VER_REV1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk0_wdata3](index.html) module"]
pub struct BLK0_WDATA3_SPEC;
impl crate::RegisterSpec for BLK0_WDATA3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blk0_wdata3::R](R) reader structure"]
impl crate::Readable for BLK0_WDATA3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [blk0_wdata3::W](W) writer structure"]
impl crate::Writable for BLK0_WDATA3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BLK0_WDATA3 to value 0"]
impl crate::Resettable for BLK0_WDATA3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
