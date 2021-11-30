#[doc = "Register `PRO_DCACHE_DBUG3` reader"]
pub struct R(crate::R<PRO_DCACHE_DBUG3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_DCACHE_DBUG3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_DCACHE_DBUG3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_DCACHE_DBUG3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRO_DCACHE_DBUG3` writer"]
pub struct W(crate::W<PRO_DCACHE_DBUG3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRO_DCACHE_DBUG3_SPEC>;
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
impl From<crate::W<PRO_DCACHE_DBUG3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRO_DCACHE_DBUG3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRO_MMU_RDATA` reader - "]
pub struct PRO_MMU_RDATA_R(crate::FieldReader<u16, u16>);
impl PRO_MMU_RDATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        PRO_MMU_RDATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_MMU_RDATA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_CPU_DISABLED_CACHE_IA` reader - "]
pub struct PRO_CPU_DISABLED_CACHE_IA_R(crate::FieldReader<u8, u8>);
impl PRO_CPU_DISABLED_CACHE_IA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PRO_CPU_DISABLED_CACHE_IA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_CPU_DISABLED_CACHE_IA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_CPU_DISABLED_CACHE_IA_OPPOSITE` reader - "]
pub struct PRO_CPU_DISABLED_CACHE_IA_OPPOSITE_R(crate::FieldReader<bool, bool>);
impl PRO_CPU_DISABLED_CACHE_IA_OPPOSITE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_CPU_DISABLED_CACHE_IA_OPPOSITE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_CPU_DISABLED_CACHE_IA_OPPOSITE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_CPU_DISABLED_CACHE_IA_OPPOSITE` writer - "]
pub struct PRO_CPU_DISABLED_CACHE_IA_OPPOSITE_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_CPU_DISABLED_CACHE_IA_OPPOSITE_W<'a> {
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
#[doc = "Field `PRO_CPU_DISABLED_CACHE_IA_DRAM1` reader - "]
pub struct PRO_CPU_DISABLED_CACHE_IA_DRAM1_R(crate::FieldReader<bool, bool>);
impl PRO_CPU_DISABLED_CACHE_IA_DRAM1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_CPU_DISABLED_CACHE_IA_DRAM1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_CPU_DISABLED_CACHE_IA_DRAM1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_CPU_DISABLED_CACHE_IA_DRAM1` writer - "]
pub struct PRO_CPU_DISABLED_CACHE_IA_DRAM1_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_CPU_DISABLED_CACHE_IA_DRAM1_W<'a> {
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
#[doc = "Field `PRO_CPU_DISABLED_CACHE_IA_IROM0` reader - "]
pub struct PRO_CPU_DISABLED_CACHE_IA_IROM0_R(crate::FieldReader<bool, bool>);
impl PRO_CPU_DISABLED_CACHE_IA_IROM0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_CPU_DISABLED_CACHE_IA_IROM0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_CPU_DISABLED_CACHE_IA_IROM0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_CPU_DISABLED_CACHE_IA_IROM0` writer - "]
pub struct PRO_CPU_DISABLED_CACHE_IA_IROM0_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_CPU_DISABLED_CACHE_IA_IROM0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `PRO_CPU_DISABLED_CACHE_IA_IRAM1` reader - "]
pub struct PRO_CPU_DISABLED_CACHE_IA_IRAM1_R(crate::FieldReader<bool, bool>);
impl PRO_CPU_DISABLED_CACHE_IA_IRAM1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_CPU_DISABLED_CACHE_IA_IRAM1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_CPU_DISABLED_CACHE_IA_IRAM1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_CPU_DISABLED_CACHE_IA_IRAM1` writer - "]
pub struct PRO_CPU_DISABLED_CACHE_IA_IRAM1_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_CPU_DISABLED_CACHE_IA_IRAM1_W<'a> {
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
#[doc = "Field `PRO_CPU_DISABLED_CACHE_IA_IRAM0` reader - "]
pub struct PRO_CPU_DISABLED_CACHE_IA_IRAM0_R(crate::FieldReader<bool, bool>);
impl PRO_CPU_DISABLED_CACHE_IA_IRAM0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_CPU_DISABLED_CACHE_IA_IRAM0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_CPU_DISABLED_CACHE_IA_IRAM0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_CPU_DISABLED_CACHE_IA_IRAM0` writer - "]
pub struct PRO_CPU_DISABLED_CACHE_IA_IRAM0_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_CPU_DISABLED_CACHE_IA_IRAM0_W<'a> {
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
#[doc = "Field `PRO_CPU_DISABLED_CACHE_IA_DROM0` reader - "]
pub struct PRO_CPU_DISABLED_CACHE_IA_DROM0_R(crate::FieldReader<bool, bool>);
impl PRO_CPU_DISABLED_CACHE_IA_DROM0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_CPU_DISABLED_CACHE_IA_DROM0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_CPU_DISABLED_CACHE_IA_DROM0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_CPU_DISABLED_CACHE_IA_DROM0` writer - "]
pub struct PRO_CPU_DISABLED_CACHE_IA_DROM0_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_CPU_DISABLED_CACHE_IA_DROM0_W<'a> {
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
#[doc = "Field `PRO_CACHE_IRAM0_PID_ERROR` reader - "]
pub struct PRO_CACHE_IRAM0_PID_ERROR_R(crate::FieldReader<bool, bool>);
impl PRO_CACHE_IRAM0_PID_ERROR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_CACHE_IRAM0_PID_ERROR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_CACHE_IRAM0_PID_ERROR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn pro_mmu_rdata(&self) -> PRO_MMU_RDATA_R {
        PRO_MMU_RDATA_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:14"]
    #[inline(always)]
    pub fn pro_cpu_disabled_cache_ia(&self) -> PRO_CPU_DISABLED_CACHE_IA_R {
        PRO_CPU_DISABLED_CACHE_IA_R::new(((self.bits >> 9) & 0x3f) as u8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pro_cpu_disabled_cache_ia_opposite(&self) -> PRO_CPU_DISABLED_CACHE_IA_OPPOSITE_R {
        PRO_CPU_DISABLED_CACHE_IA_OPPOSITE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pro_cpu_disabled_cache_ia_dram1(&self) -> PRO_CPU_DISABLED_CACHE_IA_DRAM1_R {
        PRO_CPU_DISABLED_CACHE_IA_DRAM1_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pro_cpu_disabled_cache_ia_irom0(&self) -> PRO_CPU_DISABLED_CACHE_IA_IROM0_R {
        PRO_CPU_DISABLED_CACHE_IA_IROM0_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn pro_cpu_disabled_cache_ia_iram1(&self) -> PRO_CPU_DISABLED_CACHE_IA_IRAM1_R {
        PRO_CPU_DISABLED_CACHE_IA_IRAM1_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn pro_cpu_disabled_cache_ia_iram0(&self) -> PRO_CPU_DISABLED_CACHE_IA_IRAM0_R {
        PRO_CPU_DISABLED_CACHE_IA_IRAM0_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn pro_cpu_disabled_cache_ia_drom0(&self) -> PRO_CPU_DISABLED_CACHE_IA_DROM0_R {
        PRO_CPU_DISABLED_CACHE_IA_DROM0_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn pro_cache_iram0_pid_error(&self) -> PRO_CACHE_IRAM0_PID_ERROR_R {
        PRO_CACHE_IRAM0_PID_ERROR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pro_cpu_disabled_cache_ia_opposite(&mut self) -> PRO_CPU_DISABLED_CACHE_IA_OPPOSITE_W {
        PRO_CPU_DISABLED_CACHE_IA_OPPOSITE_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pro_cpu_disabled_cache_ia_dram1(&mut self) -> PRO_CPU_DISABLED_CACHE_IA_DRAM1_W {
        PRO_CPU_DISABLED_CACHE_IA_DRAM1_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pro_cpu_disabled_cache_ia_irom0(&mut self) -> PRO_CPU_DISABLED_CACHE_IA_IROM0_W {
        PRO_CPU_DISABLED_CACHE_IA_IROM0_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn pro_cpu_disabled_cache_ia_iram1(&mut self) -> PRO_CPU_DISABLED_CACHE_IA_IRAM1_W {
        PRO_CPU_DISABLED_CACHE_IA_IRAM1_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn pro_cpu_disabled_cache_ia_iram0(&mut self) -> PRO_CPU_DISABLED_CACHE_IA_IRAM0_W {
        PRO_CPU_DISABLED_CACHE_IA_IRAM0_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn pro_cpu_disabled_cache_ia_drom0(&mut self) -> PRO_CPU_DISABLED_CACHE_IA_DROM0_W {
        PRO_CPU_DISABLED_CACHE_IA_DROM0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_dcache_dbug3](index.html) module"]
pub struct PRO_DCACHE_DBUG3_SPEC;
impl crate::RegisterSpec for PRO_DCACHE_DBUG3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_dcache_dbug3::R](R) reader structure"]
impl crate::Readable for PRO_DCACHE_DBUG3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pro_dcache_dbug3::W](W) writer structure"]
impl crate::Writable for PRO_DCACHE_DBUG3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRO_DCACHE_DBUG3 to value 0"]
impl crate::Resettable for PRO_DCACHE_DBUG3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
