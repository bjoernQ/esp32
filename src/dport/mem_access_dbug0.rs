#[doc = "Register `MEM_ACCESS_DBUG0` reader"]
pub struct R(crate::R<MEM_ACCESS_DBUG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEM_ACCESS_DBUG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEM_ACCESS_DBUG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEM_ACCESS_DBUG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PRO_ROM_MPU_AD` reader - "]
pub struct PRO_ROM_MPU_AD_R(crate::FieldReader<bool, bool>);
impl PRO_ROM_MPU_AD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_ROM_MPU_AD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_ROM_MPU_AD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_ROM_IA` reader - "]
pub struct PRO_ROM_IA_R(crate::FieldReader<bool, bool>);
impl PRO_ROM_IA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_ROM_IA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_ROM_IA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APP_ROM_MPU_AD` reader - "]
pub struct APP_ROM_MPU_AD_R(crate::FieldReader<bool, bool>);
impl APP_ROM_MPU_AD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APP_ROM_MPU_AD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APP_ROM_MPU_AD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APP_ROM_IA` reader - "]
pub struct APP_ROM_IA_R(crate::FieldReader<bool, bool>);
impl APP_ROM_IA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APP_ROM_IA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APP_ROM_IA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SHARE_ROM_MPU_AD` reader - "]
pub struct SHARE_ROM_MPU_AD_R(crate::FieldReader<u8, u8>);
impl SHARE_ROM_MPU_AD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SHARE_ROM_MPU_AD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SHARE_ROM_MPU_AD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SHARE_ROM_IA` reader - "]
pub struct SHARE_ROM_IA_R(crate::FieldReader<u8, u8>);
impl SHARE_ROM_IA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SHARE_ROM_IA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SHARE_ROM_IA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTERNAL_SRAM_MMU_AD` reader - "]
pub struct INTERNAL_SRAM_MMU_AD_R(crate::FieldReader<u8, u8>);
impl INTERNAL_SRAM_MMU_AD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        INTERNAL_SRAM_MMU_AD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTERNAL_SRAM_MMU_AD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTERNAL_SRAM_IA` reader - "]
pub struct INTERNAL_SRAM_IA_R(crate::FieldReader<u16, u16>);
impl INTERNAL_SRAM_IA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        INTERNAL_SRAM_IA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTERNAL_SRAM_IA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTERNAL_SRAM_MMU_MULTI_HIT` reader - "]
pub struct INTERNAL_SRAM_MMU_MULTI_HIT_R(crate::FieldReader<u8, u8>);
impl INTERNAL_SRAM_MMU_MULTI_HIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        INTERNAL_SRAM_MMU_MULTI_HIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTERNAL_SRAM_MMU_MULTI_HIT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pro_rom_mpu_ad(&self) -> PRO_ROM_MPU_AD_R {
        PRO_ROM_MPU_AD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pro_rom_ia(&self) -> PRO_ROM_IA_R {
        PRO_ROM_IA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn app_rom_mpu_ad(&self) -> APP_ROM_MPU_AD_R {
        APP_ROM_MPU_AD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn app_rom_ia(&self) -> APP_ROM_IA_R {
        APP_ROM_IA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn share_rom_mpu_ad(&self) -> SHARE_ROM_MPU_AD_R {
        SHARE_ROM_MPU_AD_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:9"]
    #[inline(always)]
    pub fn share_rom_ia(&self) -> SHARE_ROM_IA_R {
        SHARE_ROM_IA_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 10:13"]
    #[inline(always)]
    pub fn internal_sram_mmu_ad(&self) -> INTERNAL_SRAM_MMU_AD_R {
        INTERNAL_SRAM_MMU_AD_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bits 14:25"]
    #[inline(always)]
    pub fn internal_sram_ia(&self) -> INTERNAL_SRAM_IA_R {
        INTERNAL_SRAM_IA_R::new(((self.bits >> 14) & 0x0fff) as u16)
    }
    #[doc = "Bits 26:29"]
    #[inline(always)]
    pub fn internal_sram_mmu_multi_hit(&self) -> INTERNAL_SRAM_MMU_MULTI_HIT_R {
        INTERNAL_SRAM_MMU_MULTI_HIT_R::new(((self.bits >> 26) & 0x0f) as u8)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_access_dbug0](index.html) module"]
pub struct MEM_ACCESS_DBUG0_SPEC;
impl crate::RegisterSpec for MEM_ACCESS_DBUG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mem_access_dbug0::R](R) reader structure"]
impl crate::Readable for MEM_ACCESS_DBUG0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MEM_ACCESS_DBUG0 to value 0"]
impl crate::Resettable for MEM_ACCESS_DBUG0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
