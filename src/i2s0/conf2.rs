#[doc = "Register `CONF2` reader"]
pub struct R(crate::R<CONF2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONF2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONF2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONF2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONF2` writer"]
pub struct W(crate::W<CONF2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONF2_SPEC>;
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
impl From<crate::W<CONF2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONF2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAMERA_EN` reader - "]
pub struct CAMERA_EN_R(crate::FieldReader<bool, bool>);
impl CAMERA_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CAMERA_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAMERA_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAMERA_EN` writer - "]
pub struct CAMERA_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CAMERA_EN_W<'a> {
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
#[doc = "Field `LCD_TX_WRX2_EN` reader - "]
pub struct LCD_TX_WRX2_EN_R(crate::FieldReader<bool, bool>);
impl LCD_TX_WRX2_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LCD_TX_WRX2_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCD_TX_WRX2_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCD_TX_WRX2_EN` writer - "]
pub struct LCD_TX_WRX2_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LCD_TX_WRX2_EN_W<'a> {
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
#[doc = "Field `LCD_TX_SDX2_EN` reader - "]
pub struct LCD_TX_SDX2_EN_R(crate::FieldReader<bool, bool>);
impl LCD_TX_SDX2_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LCD_TX_SDX2_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCD_TX_SDX2_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCD_TX_SDX2_EN` writer - "]
pub struct LCD_TX_SDX2_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LCD_TX_SDX2_EN_W<'a> {
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
#[doc = "Field `DATA_ENABLE_TEST_EN` reader - "]
pub struct DATA_ENABLE_TEST_EN_R(crate::FieldReader<bool, bool>);
impl DATA_ENABLE_TEST_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DATA_ENABLE_TEST_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_ENABLE_TEST_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA_ENABLE_TEST_EN` writer - "]
pub struct DATA_ENABLE_TEST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_ENABLE_TEST_EN_W<'a> {
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
#[doc = "Field `DATA_ENABLE` reader - "]
pub struct DATA_ENABLE_R(crate::FieldReader<bool, bool>);
impl DATA_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DATA_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA_ENABLE` writer - "]
pub struct DATA_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_ENABLE_W<'a> {
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
#[doc = "Field `LCD_EN` reader - "]
pub struct LCD_EN_R(crate::FieldReader<bool, bool>);
impl LCD_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LCD_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCD_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCD_EN` writer - "]
pub struct LCD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LCD_EN_W<'a> {
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
#[doc = "Field `EXT_ADC_START_EN` reader - "]
pub struct EXT_ADC_START_EN_R(crate::FieldReader<bool, bool>);
impl EXT_ADC_START_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXT_ADC_START_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXT_ADC_START_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXT_ADC_START_EN` writer - "]
pub struct EXT_ADC_START_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXT_ADC_START_EN_W<'a> {
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
#[doc = "Field `INTER_VALID_EN` reader - "]
pub struct INTER_VALID_EN_R(crate::FieldReader<bool, bool>);
impl INTER_VALID_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTER_VALID_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTER_VALID_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTER_VALID_EN` writer - "]
pub struct INTER_VALID_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> INTER_VALID_EN_W<'a> {
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
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn camera_en(&self) -> CAMERA_EN_R {
        CAMERA_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn lcd_tx_wrx2_en(&self) -> LCD_TX_WRX2_EN_R {
        LCD_TX_WRX2_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn lcd_tx_sdx2_en(&self) -> LCD_TX_SDX2_EN_R {
        LCD_TX_SDX2_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn data_enable_test_en(&self) -> DATA_ENABLE_TEST_EN_R {
        DATA_ENABLE_TEST_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn data_enable(&self) -> DATA_ENABLE_R {
        DATA_ENABLE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn lcd_en(&self) -> LCD_EN_R {
        LCD_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ext_adc_start_en(&self) -> EXT_ADC_START_EN_R {
        EXT_ADC_START_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn inter_valid_en(&self) -> INTER_VALID_EN_R {
        INTER_VALID_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn camera_en(&mut self) -> CAMERA_EN_W {
        CAMERA_EN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn lcd_tx_wrx2_en(&mut self) -> LCD_TX_WRX2_EN_W {
        LCD_TX_WRX2_EN_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn lcd_tx_sdx2_en(&mut self) -> LCD_TX_SDX2_EN_W {
        LCD_TX_SDX2_EN_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn data_enable_test_en(&mut self) -> DATA_ENABLE_TEST_EN_W {
        DATA_ENABLE_TEST_EN_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn data_enable(&mut self) -> DATA_ENABLE_W {
        DATA_ENABLE_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn lcd_en(&mut self) -> LCD_EN_W {
        LCD_EN_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ext_adc_start_en(&mut self) -> EXT_ADC_START_EN_W {
        EXT_ADC_START_EN_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn inter_valid_en(&mut self) -> INTER_VALID_EN_W {
        INTER_VALID_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conf2](index.html) module"]
pub struct CONF2_SPEC;
impl crate::RegisterSpec for CONF2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conf2::R](R) reader structure"]
impl crate::Readable for CONF2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conf2::W](W) writer structure"]
impl crate::Writable for CONF2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONF2 to value 0"]
impl crate::Resettable for CONF2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
