#[doc = "Register `PRO_CACHE_CTRL` reader"]
pub struct R(crate::R<PRO_CACHE_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_CACHE_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_CACHE_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_CACHE_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRO_CACHE_CTRL` writer"]
pub struct W(crate::W<PRO_CACHE_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRO_CACHE_CTRL_SPEC>;
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
impl From<crate::W<PRO_CACHE_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRO_CACHE_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRO_CACHE_MODE` reader - "]
pub struct PRO_CACHE_MODE_R(crate::FieldReader<bool, bool>);
impl PRO_CACHE_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_CACHE_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_CACHE_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_CACHE_MODE` writer - "]
pub struct PRO_CACHE_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_CACHE_MODE_W<'a> {
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
#[doc = "Field `PRO_CACHE_ENABLE` reader - "]
pub struct PRO_CACHE_ENABLE_R(crate::FieldReader<bool, bool>);
impl PRO_CACHE_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_CACHE_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_CACHE_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_CACHE_ENABLE` writer - "]
pub struct PRO_CACHE_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_CACHE_ENABLE_W<'a> {
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
#[doc = "Field `PRO_CACHE_FLUSH_ENA` reader - "]
pub struct PRO_CACHE_FLUSH_ENA_R(crate::FieldReader<bool, bool>);
impl PRO_CACHE_FLUSH_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_CACHE_FLUSH_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_CACHE_FLUSH_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_CACHE_FLUSH_ENA` writer - "]
pub struct PRO_CACHE_FLUSH_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_CACHE_FLUSH_ENA_W<'a> {
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
#[doc = "Field `PRO_CACHE_FLUSH_DONE` reader - "]
pub struct PRO_CACHE_FLUSH_DONE_R(crate::FieldReader<bool, bool>);
impl PRO_CACHE_FLUSH_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_CACHE_FLUSH_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_CACHE_FLUSH_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_CACHE_LOCK_0_EN` reader - "]
pub struct PRO_CACHE_LOCK_0_EN_R(crate::FieldReader<bool, bool>);
impl PRO_CACHE_LOCK_0_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_CACHE_LOCK_0_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_CACHE_LOCK_0_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_CACHE_LOCK_0_EN` writer - "]
pub struct PRO_CACHE_LOCK_0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_CACHE_LOCK_0_EN_W<'a> {
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
#[doc = "Field `PRO_CACHE_LOCK_1_EN` reader - "]
pub struct PRO_CACHE_LOCK_1_EN_R(crate::FieldReader<bool, bool>);
impl PRO_CACHE_LOCK_1_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_CACHE_LOCK_1_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_CACHE_LOCK_1_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_CACHE_LOCK_1_EN` writer - "]
pub struct PRO_CACHE_LOCK_1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_CACHE_LOCK_1_EN_W<'a> {
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
#[doc = "Field `PRO_CACHE_LOCK_2_EN` reader - "]
pub struct PRO_CACHE_LOCK_2_EN_R(crate::FieldReader<bool, bool>);
impl PRO_CACHE_LOCK_2_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_CACHE_LOCK_2_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_CACHE_LOCK_2_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_CACHE_LOCK_2_EN` writer - "]
pub struct PRO_CACHE_LOCK_2_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_CACHE_LOCK_2_EN_W<'a> {
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
#[doc = "Field `PRO_CACHE_LOCK_3_EN` reader - "]
pub struct PRO_CACHE_LOCK_3_EN_R(crate::FieldReader<bool, bool>);
impl PRO_CACHE_LOCK_3_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_CACHE_LOCK_3_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_CACHE_LOCK_3_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_CACHE_LOCK_3_EN` writer - "]
pub struct PRO_CACHE_LOCK_3_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_CACHE_LOCK_3_EN_W<'a> {
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
#[doc = "Field `PRO_SINGLE_IRAM_ENA` reader - "]
pub struct PRO_SINGLE_IRAM_ENA_R(crate::FieldReader<bool, bool>);
impl PRO_SINGLE_IRAM_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_SINGLE_IRAM_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_SINGLE_IRAM_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_SINGLE_IRAM_ENA` writer - "]
pub struct PRO_SINGLE_IRAM_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_SINGLE_IRAM_ENA_W<'a> {
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
#[doc = "Field `PRO_DRAM_SPLIT` reader - "]
pub struct PRO_DRAM_SPLIT_R(crate::FieldReader<bool, bool>);
impl PRO_DRAM_SPLIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_DRAM_SPLIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_DRAM_SPLIT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_DRAM_SPLIT` writer - "]
pub struct PRO_DRAM_SPLIT_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_DRAM_SPLIT_W<'a> {
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
#[doc = "Field `PRO_AHB_SPI_REQ` reader - "]
pub struct PRO_AHB_SPI_REQ_R(crate::FieldReader<bool, bool>);
impl PRO_AHB_SPI_REQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_AHB_SPI_REQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_AHB_SPI_REQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_SLAVE_REQ` reader - "]
pub struct PRO_SLAVE_REQ_R(crate::FieldReader<bool, bool>);
impl PRO_SLAVE_REQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_SLAVE_REQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_SLAVE_REQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AHB_SPI_REQ` reader - "]
pub struct AHB_SPI_REQ_R(crate::FieldReader<bool, bool>);
impl AHB_SPI_REQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AHB_SPI_REQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AHB_SPI_REQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLAVE_REQ` reader - "]
pub struct SLAVE_REQ_R(crate::FieldReader<bool, bool>);
impl SLAVE_REQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLAVE_REQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLAVE_REQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_DRAM_HL` reader - "]
pub struct PRO_DRAM_HL_R(crate::FieldReader<bool, bool>);
impl PRO_DRAM_HL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_DRAM_HL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_DRAM_HL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_DRAM_HL` writer - "]
pub struct PRO_DRAM_HL_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_DRAM_HL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pro_cache_mode(&self) -> PRO_CACHE_MODE_R {
        PRO_CACHE_MODE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pro_cache_enable(&self) -> PRO_CACHE_ENABLE_R {
        PRO_CACHE_ENABLE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pro_cache_flush_ena(&self) -> PRO_CACHE_FLUSH_ENA_R {
        PRO_CACHE_FLUSH_ENA_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pro_cache_flush_done(&self) -> PRO_CACHE_FLUSH_DONE_R {
        PRO_CACHE_FLUSH_DONE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn pro_cache_lock_0_en(&self) -> PRO_CACHE_LOCK_0_EN_R {
        PRO_CACHE_LOCK_0_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pro_cache_lock_1_en(&self) -> PRO_CACHE_LOCK_1_EN_R {
        PRO_CACHE_LOCK_1_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pro_cache_lock_2_en(&self) -> PRO_CACHE_LOCK_2_EN_R {
        PRO_CACHE_LOCK_2_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pro_cache_lock_3_en(&self) -> PRO_CACHE_LOCK_3_EN_R {
        PRO_CACHE_LOCK_3_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pro_single_iram_ena(&self) -> PRO_SINGLE_IRAM_ENA_R {
        PRO_SINGLE_IRAM_ENA_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pro_dram_split(&self) -> PRO_DRAM_SPLIT_R {
        PRO_DRAM_SPLIT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn pro_ahb_spi_req(&self) -> PRO_AHB_SPI_REQ_R {
        PRO_AHB_SPI_REQ_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn pro_slave_req(&self) -> PRO_SLAVE_REQ_R {
        PRO_SLAVE_REQ_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ahb_spi_req(&self) -> AHB_SPI_REQ_R {
        AHB_SPI_REQ_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn slave_req(&self) -> SLAVE_REQ_R {
        SLAVE_REQ_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pro_dram_hl(&self) -> PRO_DRAM_HL_R {
        PRO_DRAM_HL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pro_cache_mode(&mut self) -> PRO_CACHE_MODE_W {
        PRO_CACHE_MODE_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pro_cache_enable(&mut self) -> PRO_CACHE_ENABLE_W {
        PRO_CACHE_ENABLE_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pro_cache_flush_ena(&mut self) -> PRO_CACHE_FLUSH_ENA_W {
        PRO_CACHE_FLUSH_ENA_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn pro_cache_lock_0_en(&mut self) -> PRO_CACHE_LOCK_0_EN_W {
        PRO_CACHE_LOCK_0_EN_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pro_cache_lock_1_en(&mut self) -> PRO_CACHE_LOCK_1_EN_W {
        PRO_CACHE_LOCK_1_EN_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pro_cache_lock_2_en(&mut self) -> PRO_CACHE_LOCK_2_EN_W {
        PRO_CACHE_LOCK_2_EN_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pro_cache_lock_3_en(&mut self) -> PRO_CACHE_LOCK_3_EN_W {
        PRO_CACHE_LOCK_3_EN_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pro_single_iram_ena(&mut self) -> PRO_SINGLE_IRAM_ENA_W {
        PRO_SINGLE_IRAM_ENA_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pro_dram_split(&mut self) -> PRO_DRAM_SPLIT_W {
        PRO_DRAM_SPLIT_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pro_dram_hl(&mut self) -> PRO_DRAM_HL_W {
        PRO_DRAM_HL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_cache_ctrl](index.html) module"]
pub struct PRO_CACHE_CTRL_SPEC;
impl crate::RegisterSpec for PRO_CACHE_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_cache_ctrl::R](R) reader structure"]
impl crate::Readable for PRO_CACHE_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pro_cache_ctrl::W](W) writer structure"]
impl crate::Writable for PRO_CACHE_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRO_CACHE_CTRL to value 0x10"]
impl crate::Resettable for PRO_CACHE_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
