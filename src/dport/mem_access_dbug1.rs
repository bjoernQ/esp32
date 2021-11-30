#[doc = "Register `MEM_ACCESS_DBUG1` reader"]
pub struct R(crate::R<MEM_ACCESS_DBUG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEM_ACCESS_DBUG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEM_ACCESS_DBUG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEM_ACCESS_DBUG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INTERNAL_SRAM_MMU_MISS` reader - "]
pub struct INTERNAL_SRAM_MMU_MISS_R(crate::FieldReader<u8, u8>);
impl INTERNAL_SRAM_MMU_MISS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        INTERNAL_SRAM_MMU_MISS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTERNAL_SRAM_MMU_MISS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARB_IA` reader - "]
pub struct ARB_IA_R(crate::FieldReader<u8, u8>);
impl ARB_IA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ARB_IA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ARB_IA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIDGEN_IA` reader - "]
pub struct PIDGEN_IA_R(crate::FieldReader<u8, u8>);
impl PIDGEN_IA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PIDGEN_IA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIDGEN_IA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AHB_ACCESS_DENY` reader - "]
pub struct AHB_ACCESS_DENY_R(crate::FieldReader<bool, bool>);
impl AHB_ACCESS_DENY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AHB_ACCESS_DENY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AHB_ACCESS_DENY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AHBLITE_ACCESS_DENY` reader - "]
pub struct AHBLITE_ACCESS_DENY_R(crate::FieldReader<bool, bool>);
impl AHBLITE_ACCESS_DENY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AHBLITE_ACCESS_DENY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AHBLITE_ACCESS_DENY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AHBLITE_IA` reader - "]
pub struct AHBLITE_IA_R(crate::FieldReader<bool, bool>);
impl AHBLITE_IA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AHBLITE_IA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AHBLITE_IA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn internal_sram_mmu_miss(&self) -> INTERNAL_SRAM_MMU_MISS_R {
        INTERNAL_SRAM_MMU_MISS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn arb_ia(&self) -> ARB_IA_R {
        ARB_IA_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn pidgen_ia(&self) -> PIDGEN_IA_R {
        PIDGEN_IA_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ahb_access_deny(&self) -> AHB_ACCESS_DENY_R {
        AHB_ACCESS_DENY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ahblite_access_deny(&self) -> AHBLITE_ACCESS_DENY_R {
        AHBLITE_ACCESS_DENY_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ahblite_ia(&self) -> AHBLITE_IA_R {
        AHBLITE_IA_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_access_dbug1](index.html) module"]
pub struct MEM_ACCESS_DBUG1_SPEC;
impl crate::RegisterSpec for MEM_ACCESS_DBUG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mem_access_dbug1::R](R) reader structure"]
impl crate::Readable for MEM_ACCESS_DBUG1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MEM_ACCESS_DBUG1 to value 0"]
impl crate::Resettable for MEM_ACCESS_DBUG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
