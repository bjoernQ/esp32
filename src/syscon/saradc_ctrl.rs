#[doc = "Register `SARADC_CTRL` reader"]
pub struct R(crate::R<SARADC_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SARADC_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SARADC_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SARADC_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SARADC_CTRL` writer"]
pub struct W(crate::W<SARADC_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SARADC_CTRL_SPEC>;
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
impl From<crate::W<SARADC_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SARADC_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SARADC_START_FORCE` reader - "]
pub struct SARADC_START_FORCE_R(crate::FieldReader<bool, bool>);
impl SARADC_START_FORCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SARADC_START_FORCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SARADC_START_FORCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SARADC_START_FORCE` writer - "]
pub struct SARADC_START_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> SARADC_START_FORCE_W<'a> {
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
#[doc = "Field `SARADC_START` reader - "]
pub struct SARADC_START_R(crate::FieldReader<bool, bool>);
impl SARADC_START_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SARADC_START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SARADC_START_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SARADC_START` writer - "]
pub struct SARADC_START_W<'a> {
    w: &'a mut W,
}
impl<'a> SARADC_START_W<'a> {
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
#[doc = "Field `SARADC_SAR2_MUX` reader - 1: SAR ADC2 is controlled by DIG ADC2 CTRL 0: SAR ADC2 is controlled by PWDET CTRL"]
pub struct SARADC_SAR2_MUX_R(crate::FieldReader<bool, bool>);
impl SARADC_SAR2_MUX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SARADC_SAR2_MUX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SARADC_SAR2_MUX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SARADC_SAR2_MUX` writer - 1: SAR ADC2 is controlled by DIG ADC2 CTRL 0: SAR ADC2 is controlled by PWDET CTRL"]
pub struct SARADC_SAR2_MUX_W<'a> {
    w: &'a mut W,
}
impl<'a> SARADC_SAR2_MUX_W<'a> {
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
#[doc = "Field `SARADC_WORK_MODE` reader - 0: single mode 1: double mode 2: alternate mode"]
pub struct SARADC_WORK_MODE_R(crate::FieldReader<u8, u8>);
impl SARADC_WORK_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SARADC_WORK_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SARADC_WORK_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SARADC_WORK_MODE` writer - 0: single mode 1: double mode 2: alternate mode"]
pub struct SARADC_WORK_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SARADC_WORK_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | ((value as u32 & 0x03) << 3);
        self.w
    }
}
#[doc = "Field `SARADC_SAR_SEL` reader - 0: SAR1 1: SAR2 only work for single SAR mode"]
pub struct SARADC_SAR_SEL_R(crate::FieldReader<bool, bool>);
impl SARADC_SAR_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SARADC_SAR_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SARADC_SAR_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SARADC_SAR_SEL` writer - 0: SAR1 1: SAR2 only work for single SAR mode"]
pub struct SARADC_SAR_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SARADC_SAR_SEL_W<'a> {
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
#[doc = "Field `SARADC_SAR_CLK_GATED` reader - "]
pub struct SARADC_SAR_CLK_GATED_R(crate::FieldReader<bool, bool>);
impl SARADC_SAR_CLK_GATED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SARADC_SAR_CLK_GATED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SARADC_SAR_CLK_GATED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SARADC_SAR_CLK_GATED` writer - "]
pub struct SARADC_SAR_CLK_GATED_W<'a> {
    w: &'a mut W,
}
impl<'a> SARADC_SAR_CLK_GATED_W<'a> {
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
#[doc = "Field `SARADC_SAR_CLK_DIV` reader - SAR clock divider"]
pub struct SARADC_SAR_CLK_DIV_R(crate::FieldReader<u8, u8>);
impl SARADC_SAR_CLK_DIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SARADC_SAR_CLK_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SARADC_SAR_CLK_DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SARADC_SAR_CLK_DIV` writer - SAR clock divider"]
pub struct SARADC_SAR_CLK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> SARADC_SAR_CLK_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 7)) | ((value as u32 & 0xff) << 7);
        self.w
    }
}
#[doc = "Field `SARADC_SAR1_PATT_LEN` reader - 0 ~ 15 means length 1 ~ 16"]
pub struct SARADC_SAR1_PATT_LEN_R(crate::FieldReader<u8, u8>);
impl SARADC_SAR1_PATT_LEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SARADC_SAR1_PATT_LEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SARADC_SAR1_PATT_LEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SARADC_SAR1_PATT_LEN` writer - 0 ~ 15 means length 1 ~ 16"]
pub struct SARADC_SAR1_PATT_LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SARADC_SAR1_PATT_LEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 15)) | ((value as u32 & 0x0f) << 15);
        self.w
    }
}
#[doc = "Field `SARADC_SAR2_PATT_LEN` reader - 0 ~ 15 means length 1 ~ 16"]
pub struct SARADC_SAR2_PATT_LEN_R(crate::FieldReader<u8, u8>);
impl SARADC_SAR2_PATT_LEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SARADC_SAR2_PATT_LEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SARADC_SAR2_PATT_LEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SARADC_SAR2_PATT_LEN` writer - 0 ~ 15 means length 1 ~ 16"]
pub struct SARADC_SAR2_PATT_LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SARADC_SAR2_PATT_LEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 19)) | ((value as u32 & 0x0f) << 19);
        self.w
    }
}
#[doc = "Field `SARADC_SAR1_PATT_P_CLEAR` reader - clear the pointer of pattern table for DIG ADC1 CTRL"]
pub struct SARADC_SAR1_PATT_P_CLEAR_R(crate::FieldReader<bool, bool>);
impl SARADC_SAR1_PATT_P_CLEAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SARADC_SAR1_PATT_P_CLEAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SARADC_SAR1_PATT_P_CLEAR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SARADC_SAR1_PATT_P_CLEAR` writer - clear the pointer of pattern table for DIG ADC1 CTRL"]
pub struct SARADC_SAR1_PATT_P_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> SARADC_SAR1_PATT_P_CLEAR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `SARADC_SAR2_PATT_P_CLEAR` reader - clear the pointer of pattern table for DIG ADC2 CTRL"]
pub struct SARADC_SAR2_PATT_P_CLEAR_R(crate::FieldReader<bool, bool>);
impl SARADC_SAR2_PATT_P_CLEAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SARADC_SAR2_PATT_P_CLEAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SARADC_SAR2_PATT_P_CLEAR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SARADC_SAR2_PATT_P_CLEAR` writer - clear the pointer of pattern table for DIG ADC2 CTRL"]
pub struct SARADC_SAR2_PATT_P_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> SARADC_SAR2_PATT_P_CLEAR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `SARADC_DATA_SAR_SEL` reader - 1: sar_sel will be coded by the MSB of the 16-bit output data in this case the resolution should not be larger than 11 bits."]
pub struct SARADC_DATA_SAR_SEL_R(crate::FieldReader<bool, bool>);
impl SARADC_DATA_SAR_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SARADC_DATA_SAR_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SARADC_DATA_SAR_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SARADC_DATA_SAR_SEL` writer - 1: sar_sel will be coded by the MSB of the 16-bit output data in this case the resolution should not be larger than 11 bits."]
pub struct SARADC_DATA_SAR_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SARADC_DATA_SAR_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `SARADC_DATA_TO_I2S` reader - 1: I2S input data is from SAR ADC (for DMA) 0: I2S input data is from GPIO matrix"]
pub struct SARADC_DATA_TO_I2S_R(crate::FieldReader<bool, bool>);
impl SARADC_DATA_TO_I2S_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SARADC_DATA_TO_I2S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SARADC_DATA_TO_I2S_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SARADC_DATA_TO_I2S` writer - 1: I2S input data is from SAR ADC (for DMA) 0: I2S input data is from GPIO matrix"]
pub struct SARADC_DATA_TO_I2S_W<'a> {
    w: &'a mut W,
}
impl<'a> SARADC_DATA_TO_I2S_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn saradc_start_force(&self) -> SARADC_START_FORCE_R {
        SARADC_START_FORCE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn saradc_start(&self) -> SARADC_START_R {
        SARADC_START_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 1: SAR ADC2 is controlled by DIG ADC2 CTRL 0: SAR ADC2 is controlled by PWDET CTRL"]
    #[inline(always)]
    pub fn saradc_sar2_mux(&self) -> SARADC_SAR2_MUX_R {
        SARADC_SAR2_MUX_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - 0: single mode 1: double mode 2: alternate mode"]
    #[inline(always)]
    pub fn saradc_work_mode(&self) -> SARADC_WORK_MODE_R {
        SARADC_WORK_MODE_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 5 - 0: SAR1 1: SAR2 only work for single SAR mode"]
    #[inline(always)]
    pub fn saradc_sar_sel(&self) -> SARADC_SAR_SEL_R {
        SARADC_SAR_SEL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn saradc_sar_clk_gated(&self) -> SARADC_SAR_CLK_GATED_R {
        SARADC_SAR_CLK_GATED_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 7:14 - SAR clock divider"]
    #[inline(always)]
    pub fn saradc_sar_clk_div(&self) -> SARADC_SAR_CLK_DIV_R {
        SARADC_SAR_CLK_DIV_R::new(((self.bits >> 7) & 0xff) as u8)
    }
    #[doc = "Bits 15:18 - 0 ~ 15 means length 1 ~ 16"]
    #[inline(always)]
    pub fn saradc_sar1_patt_len(&self) -> SARADC_SAR1_PATT_LEN_R {
        SARADC_SAR1_PATT_LEN_R::new(((self.bits >> 15) & 0x0f) as u8)
    }
    #[doc = "Bits 19:22 - 0 ~ 15 means length 1 ~ 16"]
    #[inline(always)]
    pub fn saradc_sar2_patt_len(&self) -> SARADC_SAR2_PATT_LEN_R {
        SARADC_SAR2_PATT_LEN_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bit 23 - clear the pointer of pattern table for DIG ADC1 CTRL"]
    #[inline(always)]
    pub fn saradc_sar1_patt_p_clear(&self) -> SARADC_SAR1_PATT_P_CLEAR_R {
        SARADC_SAR1_PATT_P_CLEAR_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - clear the pointer of pattern table for DIG ADC2 CTRL"]
    #[inline(always)]
    pub fn saradc_sar2_patt_p_clear(&self) -> SARADC_SAR2_PATT_P_CLEAR_R {
        SARADC_SAR2_PATT_P_CLEAR_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - 1: sar_sel will be coded by the MSB of the 16-bit output data in this case the resolution should not be larger than 11 bits."]
    #[inline(always)]
    pub fn saradc_data_sar_sel(&self) -> SARADC_DATA_SAR_SEL_R {
        SARADC_DATA_SAR_SEL_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - 1: I2S input data is from SAR ADC (for DMA) 0: I2S input data is from GPIO matrix"]
    #[inline(always)]
    pub fn saradc_data_to_i2s(&self) -> SARADC_DATA_TO_I2S_R {
        SARADC_DATA_TO_I2S_R::new(((self.bits >> 26) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn saradc_start_force(&mut self) -> SARADC_START_FORCE_W {
        SARADC_START_FORCE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn saradc_start(&mut self) -> SARADC_START_W {
        SARADC_START_W { w: self }
    }
    #[doc = "Bit 2 - 1: SAR ADC2 is controlled by DIG ADC2 CTRL 0: SAR ADC2 is controlled by PWDET CTRL"]
    #[inline(always)]
    pub fn saradc_sar2_mux(&mut self) -> SARADC_SAR2_MUX_W {
        SARADC_SAR2_MUX_W { w: self }
    }
    #[doc = "Bits 3:4 - 0: single mode 1: double mode 2: alternate mode"]
    #[inline(always)]
    pub fn saradc_work_mode(&mut self) -> SARADC_WORK_MODE_W {
        SARADC_WORK_MODE_W { w: self }
    }
    #[doc = "Bit 5 - 0: SAR1 1: SAR2 only work for single SAR mode"]
    #[inline(always)]
    pub fn saradc_sar_sel(&mut self) -> SARADC_SAR_SEL_W {
        SARADC_SAR_SEL_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn saradc_sar_clk_gated(&mut self) -> SARADC_SAR_CLK_GATED_W {
        SARADC_SAR_CLK_GATED_W { w: self }
    }
    #[doc = "Bits 7:14 - SAR clock divider"]
    #[inline(always)]
    pub fn saradc_sar_clk_div(&mut self) -> SARADC_SAR_CLK_DIV_W {
        SARADC_SAR_CLK_DIV_W { w: self }
    }
    #[doc = "Bits 15:18 - 0 ~ 15 means length 1 ~ 16"]
    #[inline(always)]
    pub fn saradc_sar1_patt_len(&mut self) -> SARADC_SAR1_PATT_LEN_W {
        SARADC_SAR1_PATT_LEN_W { w: self }
    }
    #[doc = "Bits 19:22 - 0 ~ 15 means length 1 ~ 16"]
    #[inline(always)]
    pub fn saradc_sar2_patt_len(&mut self) -> SARADC_SAR2_PATT_LEN_W {
        SARADC_SAR2_PATT_LEN_W { w: self }
    }
    #[doc = "Bit 23 - clear the pointer of pattern table for DIG ADC1 CTRL"]
    #[inline(always)]
    pub fn saradc_sar1_patt_p_clear(&mut self) -> SARADC_SAR1_PATT_P_CLEAR_W {
        SARADC_SAR1_PATT_P_CLEAR_W { w: self }
    }
    #[doc = "Bit 24 - clear the pointer of pattern table for DIG ADC2 CTRL"]
    #[inline(always)]
    pub fn saradc_sar2_patt_p_clear(&mut self) -> SARADC_SAR2_PATT_P_CLEAR_W {
        SARADC_SAR2_PATT_P_CLEAR_W { w: self }
    }
    #[doc = "Bit 25 - 1: sar_sel will be coded by the MSB of the 16-bit output data in this case the resolution should not be larger than 11 bits."]
    #[inline(always)]
    pub fn saradc_data_sar_sel(&mut self) -> SARADC_DATA_SAR_SEL_W {
        SARADC_DATA_SAR_SEL_W { w: self }
    }
    #[doc = "Bit 26 - 1: I2S input data is from SAR ADC (for DMA) 0: I2S input data is from GPIO matrix"]
    #[inline(always)]
    pub fn saradc_data_to_i2s(&mut self) -> SARADC_DATA_TO_I2S_W {
        SARADC_DATA_TO_I2S_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [saradc_ctrl](index.html) module"]
pub struct SARADC_CTRL_SPEC;
impl crate::RegisterSpec for SARADC_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [saradc_ctrl::R](R) reader structure"]
impl crate::Readable for SARADC_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [saradc_ctrl::W](W) writer structure"]
impl crate::Writable for SARADC_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SARADC_CTRL to value 0x007f_8240"]
impl crate::Resettable for SARADC_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x007f_8240
    }
}
