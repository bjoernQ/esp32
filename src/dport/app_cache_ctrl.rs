#[doc = "Register `APP_CACHE_CTRL` reader"]
pub struct R(crate::R<APP_CACHE_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APP_CACHE_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APP_CACHE_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APP_CACHE_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APP_CACHE_CTRL` writer"]
pub struct W(crate::W<APP_CACHE_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APP_CACHE_CTRL_SPEC>;
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
impl From<crate::W<APP_CACHE_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APP_CACHE_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `APP_CACHE_MODE` reader - "]
pub struct APP_CACHE_MODE_R(crate::FieldReader<bool, bool>);
impl APP_CACHE_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APP_CACHE_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APP_CACHE_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APP_CACHE_MODE` writer - "]
pub struct APP_CACHE_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_CACHE_MODE_W<'a> {
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
#[doc = "Field `APP_CACHE_ENABLE` reader - "]
pub struct APP_CACHE_ENABLE_R(crate::FieldReader<bool, bool>);
impl APP_CACHE_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APP_CACHE_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APP_CACHE_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APP_CACHE_ENABLE` writer - "]
pub struct APP_CACHE_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_CACHE_ENABLE_W<'a> {
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
#[doc = "Field `APP_CACHE_FLUSH_ENA` reader - "]
pub struct APP_CACHE_FLUSH_ENA_R(crate::FieldReader<bool, bool>);
impl APP_CACHE_FLUSH_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APP_CACHE_FLUSH_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APP_CACHE_FLUSH_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APP_CACHE_FLUSH_ENA` writer - "]
pub struct APP_CACHE_FLUSH_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_CACHE_FLUSH_ENA_W<'a> {
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
#[doc = "Field `APP_CACHE_FLUSH_DONE` reader - "]
pub struct APP_CACHE_FLUSH_DONE_R(crate::FieldReader<bool, bool>);
impl APP_CACHE_FLUSH_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APP_CACHE_FLUSH_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APP_CACHE_FLUSH_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APP_CACHE_LOCK_0_EN` reader - "]
pub struct APP_CACHE_LOCK_0_EN_R(crate::FieldReader<bool, bool>);
impl APP_CACHE_LOCK_0_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APP_CACHE_LOCK_0_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APP_CACHE_LOCK_0_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APP_CACHE_LOCK_0_EN` writer - "]
pub struct APP_CACHE_LOCK_0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_CACHE_LOCK_0_EN_W<'a> {
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
#[doc = "Field `APP_CACHE_LOCK_1_EN` reader - "]
pub struct APP_CACHE_LOCK_1_EN_R(crate::FieldReader<bool, bool>);
impl APP_CACHE_LOCK_1_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APP_CACHE_LOCK_1_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APP_CACHE_LOCK_1_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APP_CACHE_LOCK_1_EN` writer - "]
pub struct APP_CACHE_LOCK_1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_CACHE_LOCK_1_EN_W<'a> {
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
#[doc = "Field `APP_CACHE_LOCK_2_EN` reader - "]
pub struct APP_CACHE_LOCK_2_EN_R(crate::FieldReader<bool, bool>);
impl APP_CACHE_LOCK_2_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APP_CACHE_LOCK_2_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APP_CACHE_LOCK_2_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APP_CACHE_LOCK_2_EN` writer - "]
pub struct APP_CACHE_LOCK_2_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_CACHE_LOCK_2_EN_W<'a> {
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
#[doc = "Field `APP_CACHE_LOCK_3_EN` reader - "]
pub struct APP_CACHE_LOCK_3_EN_R(crate::FieldReader<bool, bool>);
impl APP_CACHE_LOCK_3_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APP_CACHE_LOCK_3_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APP_CACHE_LOCK_3_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APP_CACHE_LOCK_3_EN` writer - "]
pub struct APP_CACHE_LOCK_3_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_CACHE_LOCK_3_EN_W<'a> {
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
#[doc = "Field `APP_SINGLE_IRAM_ENA` reader - "]
pub struct APP_SINGLE_IRAM_ENA_R(crate::FieldReader<bool, bool>);
impl APP_SINGLE_IRAM_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APP_SINGLE_IRAM_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APP_SINGLE_IRAM_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APP_SINGLE_IRAM_ENA` writer - "]
pub struct APP_SINGLE_IRAM_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_SINGLE_IRAM_ENA_W<'a> {
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
#[doc = "Field `APP_DRAM_SPLIT` reader - "]
pub struct APP_DRAM_SPLIT_R(crate::FieldReader<bool, bool>);
impl APP_DRAM_SPLIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APP_DRAM_SPLIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APP_DRAM_SPLIT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APP_DRAM_SPLIT` writer - "]
pub struct APP_DRAM_SPLIT_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_DRAM_SPLIT_W<'a> {
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
#[doc = "Field `APP_AHB_SPI_REQ` reader - "]
pub struct APP_AHB_SPI_REQ_R(crate::FieldReader<bool, bool>);
impl APP_AHB_SPI_REQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APP_AHB_SPI_REQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APP_AHB_SPI_REQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APP_SLAVE_REQ` reader - "]
pub struct APP_SLAVE_REQ_R(crate::FieldReader<bool, bool>);
impl APP_SLAVE_REQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APP_SLAVE_REQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APP_SLAVE_REQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APP_DRAM_HL` reader - "]
pub struct APP_DRAM_HL_R(crate::FieldReader<bool, bool>);
impl APP_DRAM_HL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APP_DRAM_HL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APP_DRAM_HL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APP_DRAM_HL` writer - "]
pub struct APP_DRAM_HL_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_DRAM_HL_W<'a> {
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
impl R {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn app_cache_mode(&self) -> APP_CACHE_MODE_R {
        APP_CACHE_MODE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn app_cache_enable(&self) -> APP_CACHE_ENABLE_R {
        APP_CACHE_ENABLE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn app_cache_flush_ena(&self) -> APP_CACHE_FLUSH_ENA_R {
        APP_CACHE_FLUSH_ENA_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn app_cache_flush_done(&self) -> APP_CACHE_FLUSH_DONE_R {
        APP_CACHE_FLUSH_DONE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn app_cache_lock_0_en(&self) -> APP_CACHE_LOCK_0_EN_R {
        APP_CACHE_LOCK_0_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn app_cache_lock_1_en(&self) -> APP_CACHE_LOCK_1_EN_R {
        APP_CACHE_LOCK_1_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn app_cache_lock_2_en(&self) -> APP_CACHE_LOCK_2_EN_R {
        APP_CACHE_LOCK_2_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn app_cache_lock_3_en(&self) -> APP_CACHE_LOCK_3_EN_R {
        APP_CACHE_LOCK_3_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn app_single_iram_ena(&self) -> APP_SINGLE_IRAM_ENA_R {
        APP_SINGLE_IRAM_ENA_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn app_dram_split(&self) -> APP_DRAM_SPLIT_R {
        APP_DRAM_SPLIT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn app_ahb_spi_req(&self) -> APP_AHB_SPI_REQ_R {
        APP_AHB_SPI_REQ_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn app_slave_req(&self) -> APP_SLAVE_REQ_R {
        APP_SLAVE_REQ_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn app_dram_hl(&self) -> APP_DRAM_HL_R {
        APP_DRAM_HL_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn app_cache_mode(&mut self) -> APP_CACHE_MODE_W {
        APP_CACHE_MODE_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn app_cache_enable(&mut self) -> APP_CACHE_ENABLE_W {
        APP_CACHE_ENABLE_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn app_cache_flush_ena(&mut self) -> APP_CACHE_FLUSH_ENA_W {
        APP_CACHE_FLUSH_ENA_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn app_cache_lock_0_en(&mut self) -> APP_CACHE_LOCK_0_EN_W {
        APP_CACHE_LOCK_0_EN_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn app_cache_lock_1_en(&mut self) -> APP_CACHE_LOCK_1_EN_W {
        APP_CACHE_LOCK_1_EN_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn app_cache_lock_2_en(&mut self) -> APP_CACHE_LOCK_2_EN_W {
        APP_CACHE_LOCK_2_EN_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn app_cache_lock_3_en(&mut self) -> APP_CACHE_LOCK_3_EN_W {
        APP_CACHE_LOCK_3_EN_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn app_single_iram_ena(&mut self) -> APP_SINGLE_IRAM_ENA_W {
        APP_SINGLE_IRAM_ENA_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn app_dram_split(&mut self) -> APP_DRAM_SPLIT_W {
        APP_DRAM_SPLIT_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn app_dram_hl(&mut self) -> APP_DRAM_HL_W {
        APP_DRAM_HL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [app_cache_ctrl](index.html) module"]
pub struct APP_CACHE_CTRL_SPEC;
impl crate::RegisterSpec for APP_CACHE_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [app_cache_ctrl::R](R) reader structure"]
impl crate::Readable for APP_CACHE_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [app_cache_ctrl::W](W) writer structure"]
impl crate::Writable for APP_CACHE_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APP_CACHE_CTRL to value 0x10"]
impl crate::Resettable for APP_CACHE_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
