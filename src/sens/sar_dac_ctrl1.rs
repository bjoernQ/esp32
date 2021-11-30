#[doc = "Register `SAR_DAC_CTRL1` reader"]
pub struct R(crate::R<SAR_DAC_CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_DAC_CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_DAC_CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_DAC_CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_DAC_CTRL1` writer"]
pub struct W(crate::W<SAR_DAC_CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_DAC_CTRL1_SPEC>;
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
impl From<crate::W<SAR_DAC_CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_DAC_CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SW_FSTEP` reader - frequency step for CW generator can be used to adjust the frequency"]
pub struct SW_FSTEP_R(crate::FieldReader<u16, u16>);
impl SW_FSTEP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        SW_FSTEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_FSTEP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_FSTEP` writer - frequency step for CW generator can be used to adjust the frequency"]
pub struct SW_FSTEP_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_FSTEP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `SW_TONE_EN` reader - 1: enable CW generator 0: disable CW generator"]
pub struct SW_TONE_EN_R(crate::FieldReader<bool, bool>);
impl SW_TONE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SW_TONE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_TONE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_TONE_EN` writer - 1: enable CW generator 0: disable CW generator"]
pub struct SW_TONE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_TONE_EN_W<'a> {
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
#[doc = "Field `DEBUG_BIT_SEL` reader - "]
pub struct DEBUG_BIT_SEL_R(crate::FieldReader<u8, u8>);
impl DEBUG_BIT_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DEBUG_BIT_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEBUG_BIT_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEBUG_BIT_SEL` writer - "]
pub struct DEBUG_BIT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBUG_BIT_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 17)) | ((value as u32 & 0x1f) << 17);
        self.w
    }
}
#[doc = "Field `DAC_DIG_FORCE` reader - 1: DAC1 & DAC2 use DMA 0: DAC1 & DAC2 do not use DMA"]
pub struct DAC_DIG_FORCE_R(crate::FieldReader<bool, bool>);
impl DAC_DIG_FORCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DAC_DIG_FORCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAC_DIG_FORCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAC_DIG_FORCE` writer - 1: DAC1 & DAC2 use DMA 0: DAC1 & DAC2 do not use DMA"]
pub struct DAC_DIG_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_DIG_FORCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `DAC_CLK_FORCE_LOW` reader - 1: force PDAC_CLK to low"]
pub struct DAC_CLK_FORCE_LOW_R(crate::FieldReader<bool, bool>);
impl DAC_CLK_FORCE_LOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DAC_CLK_FORCE_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAC_CLK_FORCE_LOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAC_CLK_FORCE_LOW` writer - 1: force PDAC_CLK to low"]
pub struct DAC_CLK_FORCE_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_CLK_FORCE_LOW_W<'a> {
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
#[doc = "Field `DAC_CLK_FORCE_HIGH` reader - 1: force PDAC_CLK to high"]
pub struct DAC_CLK_FORCE_HIGH_R(crate::FieldReader<bool, bool>);
impl DAC_CLK_FORCE_HIGH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DAC_CLK_FORCE_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAC_CLK_FORCE_HIGH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAC_CLK_FORCE_HIGH` writer - 1: force PDAC_CLK to high"]
pub struct DAC_CLK_FORCE_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_CLK_FORCE_HIGH_W<'a> {
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
#[doc = "Field `DAC_CLK_INV` reader - 1: invert PDAC_CLK"]
pub struct DAC_CLK_INV_R(crate::FieldReader<bool, bool>);
impl DAC_CLK_INV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DAC_CLK_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAC_CLK_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAC_CLK_INV` writer - 1: invert PDAC_CLK"]
pub struct DAC_CLK_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_CLK_INV_W<'a> {
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
impl R {
    #[doc = "Bits 0:15 - frequency step for CW generator can be used to adjust the frequency"]
    #[inline(always)]
    pub fn sw_fstep(&self) -> SW_FSTEP_R {
        SW_FSTEP_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - 1: enable CW generator 0: disable CW generator"]
    #[inline(always)]
    pub fn sw_tone_en(&self) -> SW_TONE_EN_R {
        SW_TONE_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    pub fn debug_bit_sel(&self) -> DEBUG_BIT_SEL_R {
        DEBUG_BIT_SEL_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bit 22 - 1: DAC1 & DAC2 use DMA 0: DAC1 & DAC2 do not use DMA"]
    #[inline(always)]
    pub fn dac_dig_force(&self) -> DAC_DIG_FORCE_R {
        DAC_DIG_FORCE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - 1: force PDAC_CLK to low"]
    #[inline(always)]
    pub fn dac_clk_force_low(&self) -> DAC_CLK_FORCE_LOW_R {
        DAC_CLK_FORCE_LOW_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - 1: force PDAC_CLK to high"]
    #[inline(always)]
    pub fn dac_clk_force_high(&self) -> DAC_CLK_FORCE_HIGH_R {
        DAC_CLK_FORCE_HIGH_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - 1: invert PDAC_CLK"]
    #[inline(always)]
    pub fn dac_clk_inv(&self) -> DAC_CLK_INV_R {
        DAC_CLK_INV_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - frequency step for CW generator can be used to adjust the frequency"]
    #[inline(always)]
    pub fn sw_fstep(&mut self) -> SW_FSTEP_W {
        SW_FSTEP_W { w: self }
    }
    #[doc = "Bit 16 - 1: enable CW generator 0: disable CW generator"]
    #[inline(always)]
    pub fn sw_tone_en(&mut self) -> SW_TONE_EN_W {
        SW_TONE_EN_W { w: self }
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    pub fn debug_bit_sel(&mut self) -> DEBUG_BIT_SEL_W {
        DEBUG_BIT_SEL_W { w: self }
    }
    #[doc = "Bit 22 - 1: DAC1 & DAC2 use DMA 0: DAC1 & DAC2 do not use DMA"]
    #[inline(always)]
    pub fn dac_dig_force(&mut self) -> DAC_DIG_FORCE_W {
        DAC_DIG_FORCE_W { w: self }
    }
    #[doc = "Bit 23 - 1: force PDAC_CLK to low"]
    #[inline(always)]
    pub fn dac_clk_force_low(&mut self) -> DAC_CLK_FORCE_LOW_W {
        DAC_CLK_FORCE_LOW_W { w: self }
    }
    #[doc = "Bit 24 - 1: force PDAC_CLK to high"]
    #[inline(always)]
    pub fn dac_clk_force_high(&mut self) -> DAC_CLK_FORCE_HIGH_W {
        DAC_CLK_FORCE_HIGH_W { w: self }
    }
    #[doc = "Bit 25 - 1: invert PDAC_CLK"]
    #[inline(always)]
    pub fn dac_clk_inv(&mut self) -> DAC_CLK_INV_W {
        DAC_CLK_INV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_dac_ctrl1](index.html) module"]
pub struct SAR_DAC_CTRL1_SPEC;
impl crate::RegisterSpec for SAR_DAC_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_dac_ctrl1::R](R) reader structure"]
impl crate::Readable for SAR_DAC_CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_dac_ctrl1::W](W) writer structure"]
impl crate::Writable for SAR_DAC_CTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAR_DAC_CTRL1 to value 0"]
impl crate::Resettable for SAR_DAC_CTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
