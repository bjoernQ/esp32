#[doc = "Register `SAR_READ_CTRL2` reader"]
pub struct R(crate::R<SAR_READ_CTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_READ_CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_READ_CTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_READ_CTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_READ_CTRL2` writer"]
pub struct W(crate::W<SAR_READ_CTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_READ_CTRL2_SPEC>;
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
impl From<crate::W<SAR_READ_CTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_READ_CTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAR2_CLK_DIV` reader - clock divider"]
pub struct SAR2_CLK_DIV_R(crate::FieldReader<u8, u8>);
impl SAR2_CLK_DIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SAR2_CLK_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR2_CLK_DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR2_CLK_DIV` writer - clock divider"]
pub struct SAR2_CLK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR2_CLK_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `SAR2_SAMPLE_CYCLE` reader - sample cycles for SAR ADC2"]
pub struct SAR2_SAMPLE_CYCLE_R(crate::FieldReader<u8, u8>);
impl SAR2_SAMPLE_CYCLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SAR2_SAMPLE_CYCLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR2_SAMPLE_CYCLE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR2_SAMPLE_CYCLE` writer - sample cycles for SAR ADC2"]
pub struct SAR2_SAMPLE_CYCLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR2_SAMPLE_CYCLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `SAR2_SAMPLE_BIT` reader - 00: for 9-bit width 01: for 10-bit width 10: for 11-bit width 11: for 12-bit width"]
pub struct SAR2_SAMPLE_BIT_R(crate::FieldReader<u8, u8>);
impl SAR2_SAMPLE_BIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SAR2_SAMPLE_BIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR2_SAMPLE_BIT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR2_SAMPLE_BIT` writer - 00: for 9-bit width 01: for 10-bit width 10: for 11-bit width 11: for 12-bit width"]
pub struct SAR2_SAMPLE_BIT_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR2_SAMPLE_BIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `SAR2_CLK_GATED` reader - "]
pub struct SAR2_CLK_GATED_R(crate::FieldReader<bool, bool>);
impl SAR2_CLK_GATED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SAR2_CLK_GATED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR2_CLK_GATED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR2_CLK_GATED` writer - "]
pub struct SAR2_CLK_GATED_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR2_CLK_GATED_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `SAR2_SAMPLE_NUM` reader - "]
pub struct SAR2_SAMPLE_NUM_R(crate::FieldReader<u8, u8>);
impl SAR2_SAMPLE_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SAR2_SAMPLE_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR2_SAMPLE_NUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR2_SAMPLE_NUM` writer - "]
pub struct SAR2_SAMPLE_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR2_SAMPLE_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 19)) | ((value as u32 & 0xff) << 19);
        self.w
    }
}
#[doc = "Field `SAR2_PWDET_FORCE` reader - "]
pub struct SAR2_PWDET_FORCE_R(crate::FieldReader<bool, bool>);
impl SAR2_PWDET_FORCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SAR2_PWDET_FORCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR2_PWDET_FORCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR2_PWDET_FORCE` writer - "]
pub struct SAR2_PWDET_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR2_PWDET_FORCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `SAR2_DIG_FORCE` reader - 1: SAR ADC2 controlled by DIG ADC2 CTRL or PWDET CTRL 0: SAR ADC2 controlled by RTC ADC2 CTRL"]
pub struct SAR2_DIG_FORCE_R(crate::FieldReader<bool, bool>);
impl SAR2_DIG_FORCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SAR2_DIG_FORCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR2_DIG_FORCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR2_DIG_FORCE` writer - 1: SAR ADC2 controlled by DIG ADC2 CTRL or PWDET CTRL 0: SAR ADC2 controlled by RTC ADC2 CTRL"]
pub struct SAR2_DIG_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR2_DIG_FORCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `SAR2_DATA_INV` reader - Invert SAR ADC2 data"]
pub struct SAR2_DATA_INV_R(crate::FieldReader<bool, bool>);
impl SAR2_DATA_INV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SAR2_DATA_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR2_DATA_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR2_DATA_INV` writer - Invert SAR ADC2 data"]
pub struct SAR2_DATA_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR2_DATA_INV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - clock divider"]
    #[inline(always)]
    pub fn sar2_clk_div(&self) -> SAR2_CLK_DIV_R {
        SAR2_CLK_DIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - sample cycles for SAR ADC2"]
    #[inline(always)]
    pub fn sar2_sample_cycle(&self) -> SAR2_SAMPLE_CYCLE_R {
        SAR2_SAMPLE_CYCLE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - 00: for 9-bit width 01: for 10-bit width 10: for 11-bit width 11: for 12-bit width"]
    #[inline(always)]
    pub fn sar2_sample_bit(&self) -> SAR2_SAMPLE_BIT_R {
        SAR2_SAMPLE_BIT_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn sar2_clk_gated(&self) -> SAR2_CLK_GATED_R {
        SAR2_CLK_GATED_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 19:26"]
    #[inline(always)]
    pub fn sar2_sample_num(&self) -> SAR2_SAMPLE_NUM_R {
        SAR2_SAMPLE_NUM_R::new(((self.bits >> 19) & 0xff) as u8)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn sar2_pwdet_force(&self) -> SAR2_PWDET_FORCE_R {
        SAR2_PWDET_FORCE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - 1: SAR ADC2 controlled by DIG ADC2 CTRL or PWDET CTRL 0: SAR ADC2 controlled by RTC ADC2 CTRL"]
    #[inline(always)]
    pub fn sar2_dig_force(&self) -> SAR2_DIG_FORCE_R {
        SAR2_DIG_FORCE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Invert SAR ADC2 data"]
    #[inline(always)]
    pub fn sar2_data_inv(&self) -> SAR2_DATA_INV_R {
        SAR2_DATA_INV_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - clock divider"]
    #[inline(always)]
    pub fn sar2_clk_div(&mut self) -> SAR2_CLK_DIV_W {
        SAR2_CLK_DIV_W { w: self }
    }
    #[doc = "Bits 8:15 - sample cycles for SAR ADC2"]
    #[inline(always)]
    pub fn sar2_sample_cycle(&mut self) -> SAR2_SAMPLE_CYCLE_W {
        SAR2_SAMPLE_CYCLE_W { w: self }
    }
    #[doc = "Bits 16:17 - 00: for 9-bit width 01: for 10-bit width 10: for 11-bit width 11: for 12-bit width"]
    #[inline(always)]
    pub fn sar2_sample_bit(&mut self) -> SAR2_SAMPLE_BIT_W {
        SAR2_SAMPLE_BIT_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn sar2_clk_gated(&mut self) -> SAR2_CLK_GATED_W {
        SAR2_CLK_GATED_W { w: self }
    }
    #[doc = "Bits 19:26"]
    #[inline(always)]
    pub fn sar2_sample_num(&mut self) -> SAR2_SAMPLE_NUM_W {
        SAR2_SAMPLE_NUM_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn sar2_pwdet_force(&mut self) -> SAR2_PWDET_FORCE_W {
        SAR2_PWDET_FORCE_W { w: self }
    }
    #[doc = "Bit 28 - 1: SAR ADC2 controlled by DIG ADC2 CTRL or PWDET CTRL 0: SAR ADC2 controlled by RTC ADC2 CTRL"]
    #[inline(always)]
    pub fn sar2_dig_force(&mut self) -> SAR2_DIG_FORCE_W {
        SAR2_DIG_FORCE_W { w: self }
    }
    #[doc = "Bit 29 - Invert SAR ADC2 data"]
    #[inline(always)]
    pub fn sar2_data_inv(&mut self) -> SAR2_DATA_INV_W {
        SAR2_DATA_INV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_read_ctrl2](index.html) module"]
pub struct SAR_READ_CTRL2_SPEC;
impl crate::RegisterSpec for SAR_READ_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_read_ctrl2::R](R) reader structure"]
impl crate::Readable for SAR_READ_CTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_read_ctrl2::W](W) writer structure"]
impl crate::Writable for SAR_READ_CTRL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAR_READ_CTRL2 to value 0x0007_0902"]
impl crate::Resettable for SAR_READ_CTRL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0007_0902
    }
}
