#[doc = "Register `APP_CACHE_CTRL1` reader"]
pub struct R(crate::R<APP_CACHE_CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APP_CACHE_CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APP_CACHE_CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APP_CACHE_CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APP_CACHE_CTRL1` writer"]
pub struct W(crate::W<APP_CACHE_CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APP_CACHE_CTRL1_SPEC>;
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
impl From<crate::W<APP_CACHE_CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APP_CACHE_CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `APP_CACHE_MASK_IRAM0` reader - "]
pub struct APP_CACHE_MASK_IRAM0_R(crate::FieldReader<bool, bool>);
impl APP_CACHE_MASK_IRAM0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APP_CACHE_MASK_IRAM0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APP_CACHE_MASK_IRAM0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APP_CACHE_MASK_IRAM0` writer - "]
pub struct APP_CACHE_MASK_IRAM0_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_CACHE_MASK_IRAM0_W<'a> {
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
#[doc = "Field `APP_CACHE_MASK_IRAM1` reader - "]
pub struct APP_CACHE_MASK_IRAM1_R(crate::FieldReader<bool, bool>);
impl APP_CACHE_MASK_IRAM1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APP_CACHE_MASK_IRAM1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APP_CACHE_MASK_IRAM1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APP_CACHE_MASK_IRAM1` writer - "]
pub struct APP_CACHE_MASK_IRAM1_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_CACHE_MASK_IRAM1_W<'a> {
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
#[doc = "Field `APP_CACHE_MASK_IROM0` reader - "]
pub struct APP_CACHE_MASK_IROM0_R(crate::FieldReader<bool, bool>);
impl APP_CACHE_MASK_IROM0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APP_CACHE_MASK_IROM0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APP_CACHE_MASK_IROM0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APP_CACHE_MASK_IROM0` writer - "]
pub struct APP_CACHE_MASK_IROM0_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_CACHE_MASK_IROM0_W<'a> {
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
#[doc = "Field `APP_CACHE_MASK_DRAM1` reader - "]
pub struct APP_CACHE_MASK_DRAM1_R(crate::FieldReader<bool, bool>);
impl APP_CACHE_MASK_DRAM1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APP_CACHE_MASK_DRAM1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APP_CACHE_MASK_DRAM1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APP_CACHE_MASK_DRAM1` writer - "]
pub struct APP_CACHE_MASK_DRAM1_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_CACHE_MASK_DRAM1_W<'a> {
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
#[doc = "Field `APP_CACHE_MASK_DROM0` reader - "]
pub struct APP_CACHE_MASK_DROM0_R(crate::FieldReader<bool, bool>);
impl APP_CACHE_MASK_DROM0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APP_CACHE_MASK_DROM0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APP_CACHE_MASK_DROM0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APP_CACHE_MASK_DROM0` writer - "]
pub struct APP_CACHE_MASK_DROM0_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_CACHE_MASK_DROM0_W<'a> {
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
#[doc = "Field `APP_CACHE_MASK_OPSDRAM` reader - "]
pub struct APP_CACHE_MASK_OPSDRAM_R(crate::FieldReader<bool, bool>);
impl APP_CACHE_MASK_OPSDRAM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APP_CACHE_MASK_OPSDRAM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APP_CACHE_MASK_OPSDRAM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APP_CACHE_MASK_OPSDRAM` writer - "]
pub struct APP_CACHE_MASK_OPSDRAM_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_CACHE_MASK_OPSDRAM_W<'a> {
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
#[doc = "Field `APP_CMMU_SRAM_PAGE_MODE` reader - "]
pub struct APP_CMMU_SRAM_PAGE_MODE_R(crate::FieldReader<u8, u8>);
impl APP_CMMU_SRAM_PAGE_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        APP_CMMU_SRAM_PAGE_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APP_CMMU_SRAM_PAGE_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APP_CMMU_SRAM_PAGE_MODE` writer - "]
pub struct APP_CMMU_SRAM_PAGE_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_CMMU_SRAM_PAGE_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | ((value as u32 & 0x07) << 6);
        self.w
    }
}
#[doc = "Field `APP_CMMU_FLASH_PAGE_MODE` reader - "]
pub struct APP_CMMU_FLASH_PAGE_MODE_R(crate::FieldReader<u8, u8>);
impl APP_CMMU_FLASH_PAGE_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        APP_CMMU_FLASH_PAGE_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APP_CMMU_FLASH_PAGE_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APP_CMMU_FLASH_PAGE_MODE` writer - "]
pub struct APP_CMMU_FLASH_PAGE_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_CMMU_FLASH_PAGE_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | ((value as u32 & 0x03) << 9);
        self.w
    }
}
#[doc = "Field `APP_CMMU_FORCE_ON` reader - "]
pub struct APP_CMMU_FORCE_ON_R(crate::FieldReader<bool, bool>);
impl APP_CMMU_FORCE_ON_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APP_CMMU_FORCE_ON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APP_CMMU_FORCE_ON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APP_CMMU_FORCE_ON` writer - "]
pub struct APP_CMMU_FORCE_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_CMMU_FORCE_ON_W<'a> {
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
#[doc = "Field `APP_CMMU_PD` reader - "]
pub struct APP_CMMU_PD_R(crate::FieldReader<bool, bool>);
impl APP_CMMU_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APP_CMMU_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APP_CMMU_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APP_CMMU_PD` writer - "]
pub struct APP_CMMU_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_CMMU_PD_W<'a> {
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
#[doc = "Field `APP_CACHE_MMU_IA_CLR` reader - "]
pub struct APP_CACHE_MMU_IA_CLR_R(crate::FieldReader<bool, bool>);
impl APP_CACHE_MMU_IA_CLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APP_CACHE_MMU_IA_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APP_CACHE_MMU_IA_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APP_CACHE_MMU_IA_CLR` writer - "]
pub struct APP_CACHE_MMU_IA_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_CACHE_MMU_IA_CLR_W<'a> {
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
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn app_cache_mask_iram0(&self) -> APP_CACHE_MASK_IRAM0_R {
        APP_CACHE_MASK_IRAM0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn app_cache_mask_iram1(&self) -> APP_CACHE_MASK_IRAM1_R {
        APP_CACHE_MASK_IRAM1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn app_cache_mask_irom0(&self) -> APP_CACHE_MASK_IROM0_R {
        APP_CACHE_MASK_IROM0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn app_cache_mask_dram1(&self) -> APP_CACHE_MASK_DRAM1_R {
        APP_CACHE_MASK_DRAM1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn app_cache_mask_drom0(&self) -> APP_CACHE_MASK_DROM0_R {
        APP_CACHE_MASK_DROM0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn app_cache_mask_opsdram(&self) -> APP_CACHE_MASK_OPSDRAM_R {
        APP_CACHE_MASK_OPSDRAM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:8"]
    #[inline(always)]
    pub fn app_cmmu_sram_page_mode(&self) -> APP_CMMU_SRAM_PAGE_MODE_R {
        APP_CMMU_SRAM_PAGE_MODE_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn app_cmmu_flash_page_mode(&self) -> APP_CMMU_FLASH_PAGE_MODE_R {
        APP_CMMU_FLASH_PAGE_MODE_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn app_cmmu_force_on(&self) -> APP_CMMU_FORCE_ON_R {
        APP_CMMU_FORCE_ON_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn app_cmmu_pd(&self) -> APP_CMMU_PD_R {
        APP_CMMU_PD_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn app_cache_mmu_ia_clr(&self) -> APP_CACHE_MMU_IA_CLR_R {
        APP_CACHE_MMU_IA_CLR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn app_cache_mask_iram0(&mut self) -> APP_CACHE_MASK_IRAM0_W {
        APP_CACHE_MASK_IRAM0_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn app_cache_mask_iram1(&mut self) -> APP_CACHE_MASK_IRAM1_W {
        APP_CACHE_MASK_IRAM1_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn app_cache_mask_irom0(&mut self) -> APP_CACHE_MASK_IROM0_W {
        APP_CACHE_MASK_IROM0_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn app_cache_mask_dram1(&mut self) -> APP_CACHE_MASK_DRAM1_W {
        APP_CACHE_MASK_DRAM1_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn app_cache_mask_drom0(&mut self) -> APP_CACHE_MASK_DROM0_W {
        APP_CACHE_MASK_DROM0_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn app_cache_mask_opsdram(&mut self) -> APP_CACHE_MASK_OPSDRAM_W {
        APP_CACHE_MASK_OPSDRAM_W { w: self }
    }
    #[doc = "Bits 6:8"]
    #[inline(always)]
    pub fn app_cmmu_sram_page_mode(&mut self) -> APP_CMMU_SRAM_PAGE_MODE_W {
        APP_CMMU_SRAM_PAGE_MODE_W { w: self }
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn app_cmmu_flash_page_mode(&mut self) -> APP_CMMU_FLASH_PAGE_MODE_W {
        APP_CMMU_FLASH_PAGE_MODE_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn app_cmmu_force_on(&mut self) -> APP_CMMU_FORCE_ON_W {
        APP_CMMU_FORCE_ON_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn app_cmmu_pd(&mut self) -> APP_CMMU_PD_W {
        APP_CMMU_PD_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn app_cache_mmu_ia_clr(&mut self) -> APP_CACHE_MMU_IA_CLR_W {
        APP_CACHE_MMU_IA_CLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [app_cache_ctrl1](index.html) module"]
pub struct APP_CACHE_CTRL1_SPEC;
impl crate::RegisterSpec for APP_CACHE_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [app_cache_ctrl1::R](R) reader structure"]
impl crate::Readable for APP_CACHE_CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [app_cache_ctrl1::W](W) writer structure"]
impl crate::Writable for APP_CACHE_CTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APP_CACHE_CTRL1 to value 0x08ff"]
impl crate::Resettable for APP_CACHE_CTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x08ff
    }
}
