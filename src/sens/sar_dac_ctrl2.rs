#[doc = "Register `SAR_DAC_CTRL2` reader"]
pub struct R(crate::R<SAR_DAC_CTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_DAC_CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_DAC_CTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_DAC_CTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_DAC_CTRL2` writer"]
pub struct W(crate::W<SAR_DAC_CTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_DAC_CTRL2_SPEC>;
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
impl From<crate::W<SAR_DAC_CTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_DAC_CTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAC_DC1` reader - DC offset for DAC1 CW generator"]
pub struct DAC_DC1_R(crate::FieldReader<u8, u8>);
impl DAC_DC1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DAC_DC1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAC_DC1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAC_DC1` writer - DC offset for DAC1 CW generator"]
pub struct DAC_DC1_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_DC1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `DAC_DC2` reader - DC offset for DAC2 CW generator"]
pub struct DAC_DC2_R(crate::FieldReader<u8, u8>);
impl DAC_DC2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DAC_DC2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAC_DC2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAC_DC2` writer - DC offset for DAC2 CW generator"]
pub struct DAC_DC2_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_DC2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `DAC_SCALE1` reader - 00: no scale 01: scale to 1/2 10: scale to 1/4 scale to 1/8"]
pub struct DAC_SCALE1_R(crate::FieldReader<u8, u8>);
impl DAC_SCALE1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DAC_SCALE1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAC_SCALE1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAC_SCALE1` writer - 00: no scale 01: scale to 1/2 10: scale to 1/4 scale to 1/8"]
pub struct DAC_SCALE1_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_SCALE1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `DAC_SCALE2` reader - 00: no scale 01: scale to 1/2 10: scale to 1/4 scale to 1/8"]
pub struct DAC_SCALE2_R(crate::FieldReader<u8, u8>);
impl DAC_SCALE2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DAC_SCALE2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAC_SCALE2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAC_SCALE2` writer - 00: no scale 01: scale to 1/2 10: scale to 1/4 scale to 1/8"]
pub struct DAC_SCALE2_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_SCALE2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
#[doc = "Field `DAC_INV1` reader - 00: do not invert any bits 01: invert all bits 10: invert MSB 11: invert all bits except MSB"]
pub struct DAC_INV1_R(crate::FieldReader<u8, u8>);
impl DAC_INV1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DAC_INV1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAC_INV1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAC_INV1` writer - 00: do not invert any bits 01: invert all bits 10: invert MSB 11: invert all bits except MSB"]
pub struct DAC_INV1_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_INV1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Field `DAC_INV2` reader - 00: do not invert any bits 01: invert all bits 10: invert MSB 11: invert all bits except MSB"]
pub struct DAC_INV2_R(crate::FieldReader<u8, u8>);
impl DAC_INV2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DAC_INV2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAC_INV2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAC_INV2` writer - 00: do not invert any bits 01: invert all bits 10: invert MSB 11: invert all bits except MSB"]
pub struct DAC_INV2_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_INV2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
#[doc = "Field `DAC_CW_EN1` reader - 1: to select CW generator as source to PDAC1_DAC\\[7:0\\]
0: to select register reg_pdac1_dac\\[7:0\\]
as source to PDAC1_DAC\\[7:0\\]"]
pub struct DAC_CW_EN1_R(crate::FieldReader<bool, bool>);
impl DAC_CW_EN1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DAC_CW_EN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAC_CW_EN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAC_CW_EN1` writer - 1: to select CW generator as source to PDAC1_DAC\\[7:0\\]
0: to select register reg_pdac1_dac\\[7:0\\]
as source to PDAC1_DAC\\[7:0\\]"]
pub struct DAC_CW_EN1_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_CW_EN1_W<'a> {
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
#[doc = "Field `DAC_CW_EN2` reader - 1: to select CW generator as source to PDAC2_DAC\\[7:0\\]
0: to select register reg_pdac2_dac\\[7:0\\]
as source to PDAC2_DAC\\[7:0\\]"]
pub struct DAC_CW_EN2_R(crate::FieldReader<bool, bool>);
impl DAC_CW_EN2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DAC_CW_EN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAC_CW_EN2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAC_CW_EN2` writer - 1: to select CW generator as source to PDAC2_DAC\\[7:0\\]
0: to select register reg_pdac2_dac\\[7:0\\]
as source to PDAC2_DAC\\[7:0\\]"]
pub struct DAC_CW_EN2_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_CW_EN2_W<'a> {
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
    #[doc = "Bits 0:7 - DC offset for DAC1 CW generator"]
    #[inline(always)]
    pub fn dac_dc1(&self) -> DAC_DC1_R {
        DAC_DC1_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DC offset for DAC2 CW generator"]
    #[inline(always)]
    pub fn dac_dc2(&self) -> DAC_DC2_R {
        DAC_DC2_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - 00: no scale 01: scale to 1/2 10: scale to 1/4 scale to 1/8"]
    #[inline(always)]
    pub fn dac_scale1(&self) -> DAC_SCALE1_R {
        DAC_SCALE1_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - 00: no scale 01: scale to 1/2 10: scale to 1/4 scale to 1/8"]
    #[inline(always)]
    pub fn dac_scale2(&self) -> DAC_SCALE2_R {
        DAC_SCALE2_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - 00: do not invert any bits 01: invert all bits 10: invert MSB 11: invert all bits except MSB"]
    #[inline(always)]
    pub fn dac_inv1(&self) -> DAC_INV1_R {
        DAC_INV1_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - 00: do not invert any bits 01: invert all bits 10: invert MSB 11: invert all bits except MSB"]
    #[inline(always)]
    pub fn dac_inv2(&self) -> DAC_INV2_R {
        DAC_INV2_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bit 24 - 1: to select CW generator as source to PDAC1_DAC\\[7:0\\]
0: to select register reg_pdac1_dac\\[7:0\\]
as source to PDAC1_DAC\\[7:0\\]"]
    #[inline(always)]
    pub fn dac_cw_en1(&self) -> DAC_CW_EN1_R {
        DAC_CW_EN1_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - 1: to select CW generator as source to PDAC2_DAC\\[7:0\\]
0: to select register reg_pdac2_dac\\[7:0\\]
as source to PDAC2_DAC\\[7:0\\]"]
    #[inline(always)]
    pub fn dac_cw_en2(&self) -> DAC_CW_EN2_R {
        DAC_CW_EN2_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - DC offset for DAC1 CW generator"]
    #[inline(always)]
    pub fn dac_dc1(&mut self) -> DAC_DC1_W {
        DAC_DC1_W { w: self }
    }
    #[doc = "Bits 8:15 - DC offset for DAC2 CW generator"]
    #[inline(always)]
    pub fn dac_dc2(&mut self) -> DAC_DC2_W {
        DAC_DC2_W { w: self }
    }
    #[doc = "Bits 16:17 - 00: no scale 01: scale to 1/2 10: scale to 1/4 scale to 1/8"]
    #[inline(always)]
    pub fn dac_scale1(&mut self) -> DAC_SCALE1_W {
        DAC_SCALE1_W { w: self }
    }
    #[doc = "Bits 18:19 - 00: no scale 01: scale to 1/2 10: scale to 1/4 scale to 1/8"]
    #[inline(always)]
    pub fn dac_scale2(&mut self) -> DAC_SCALE2_W {
        DAC_SCALE2_W { w: self }
    }
    #[doc = "Bits 20:21 - 00: do not invert any bits 01: invert all bits 10: invert MSB 11: invert all bits except MSB"]
    #[inline(always)]
    pub fn dac_inv1(&mut self) -> DAC_INV1_W {
        DAC_INV1_W { w: self }
    }
    #[doc = "Bits 22:23 - 00: do not invert any bits 01: invert all bits 10: invert MSB 11: invert all bits except MSB"]
    #[inline(always)]
    pub fn dac_inv2(&mut self) -> DAC_INV2_W {
        DAC_INV2_W { w: self }
    }
    #[doc = "Bit 24 - 1: to select CW generator as source to PDAC1_DAC\\[7:0\\]
0: to select register reg_pdac1_dac\\[7:0\\]
as source to PDAC1_DAC\\[7:0\\]"]
    #[inline(always)]
    pub fn dac_cw_en1(&mut self) -> DAC_CW_EN1_W {
        DAC_CW_EN1_W { w: self }
    }
    #[doc = "Bit 25 - 1: to select CW generator as source to PDAC2_DAC\\[7:0\\]
0: to select register reg_pdac2_dac\\[7:0\\]
as source to PDAC2_DAC\\[7:0\\]"]
    #[inline(always)]
    pub fn dac_cw_en2(&mut self) -> DAC_CW_EN2_W {
        DAC_CW_EN2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_dac_ctrl2](index.html) module"]
pub struct SAR_DAC_CTRL2_SPEC;
impl crate::RegisterSpec for SAR_DAC_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_dac_ctrl2::R](R) reader structure"]
impl crate::Readable for SAR_DAC_CTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_dac_ctrl2::W](W) writer structure"]
impl crate::Writable for SAR_DAC_CTRL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAR_DAC_CTRL2 to value 0x0300_0000"]
impl crate::Resettable for SAR_DAC_CTRL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0300_0000
    }
}
