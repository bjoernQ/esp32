#[doc = "Register `BLK3_WDATA3` reader"]
pub struct R(crate::R<BLK3_WDATA3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLK3_WDATA3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLK3_WDATA3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLK3_WDATA3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLK3_WDATA3` writer"]
pub struct W(crate::W<BLK3_WDATA3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLK3_WDATA3_SPEC>;
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
impl From<crate::W<BLK3_WDATA3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLK3_WDATA3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLK3_DIN3` reader - program for BLOCK3"]
pub struct BLK3_DIN3_R(crate::FieldReader<u32, u32>);
impl BLK3_DIN3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        BLK3_DIN3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLK3_DIN3_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLK3_DIN3` writer - program for BLOCK3"]
pub struct BLK3_DIN3_W<'a> {
    w: &'a mut W,
}
impl<'a> BLK3_DIN3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
#[doc = "Field `ADC1_TP_LOW` reader - ADC1 Two Point calibration low point. Only valid if EFUSE_RD_BLK3_PART_RESERVE"]
pub struct ADC1_TP_LOW_R(crate::FieldReader<u8, u8>);
impl ADC1_TP_LOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ADC1_TP_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC1_TP_LOW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC1_TP_LOW` writer - ADC1 Two Point calibration low point. Only valid if EFUSE_RD_BLK3_PART_RESERVE"]
pub struct ADC1_TP_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC1_TP_LOW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
#[doc = "Field `ADC1_TP_HIGH` reader - ADC1 Two Point calibration high point. Only valid if EFUSE_RD_BLK3_PART_RESERVE"]
pub struct ADC1_TP_HIGH_R(crate::FieldReader<u16, u16>);
impl ADC1_TP_HIGH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        ADC1_TP_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC1_TP_HIGH_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC1_TP_HIGH` writer - ADC1 Two Point calibration high point. Only valid if EFUSE_RD_BLK3_PART_RESERVE"]
pub struct ADC1_TP_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC1_TP_HIGH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 7)) | ((value as u32 & 0x01ff) << 7);
        self.w
    }
}
#[doc = "Field `ADC2_TP_LOW` reader - ADC2 Two Point calibration low point. Only valid if EFUSE_RD_BLK3_PART_RESERVE"]
pub struct ADC2_TP_LOW_R(crate::FieldReader<u8, u8>);
impl ADC2_TP_LOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ADC2_TP_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC2_TP_LOW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC2_TP_LOW` writer - ADC2 Two Point calibration low point. Only valid if EFUSE_RD_BLK3_PART_RESERVE"]
pub struct ADC2_TP_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC2_TP_LOW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | ((value as u32 & 0x7f) << 16);
        self.w
    }
}
#[doc = "Field `ADC2_TP_HIGH` reader - ADC2 Two Point calibration high point. Only valid if EFUSE_RD_BLK3_PART_RESERVE"]
pub struct ADC2_TP_HIGH_R(crate::FieldReader<u16, u16>);
impl ADC2_TP_HIGH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        ADC2_TP_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC2_TP_HIGH_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC2_TP_HIGH` writer - ADC2 Two Point calibration high point. Only valid if EFUSE_RD_BLK3_PART_RESERVE"]
pub struct ADC2_TP_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC2_TP_HIGH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 23)) | ((value as u32 & 0x01ff) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - program for BLOCK3"]
    #[inline(always)]
    pub fn blk3_din3(&self) -> BLK3_DIN3_R {
        BLK3_DIN3_R::new(self.bits as u32)
    }
    #[doc = "Bits 0:6 - ADC1 Two Point calibration low point. Only valid if EFUSE_RD_BLK3_PART_RESERVE"]
    #[inline(always)]
    pub fn adc1_tp_low(&self) -> ADC1_TP_LOW_R {
        ADC1_TP_LOW_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:15 - ADC1 Two Point calibration high point. Only valid if EFUSE_RD_BLK3_PART_RESERVE"]
    #[inline(always)]
    pub fn adc1_tp_high(&self) -> ADC1_TP_HIGH_R {
        ADC1_TP_HIGH_R::new(((self.bits >> 7) & 0x01ff) as u16)
    }
    #[doc = "Bits 16:22 - ADC2 Two Point calibration low point. Only valid if EFUSE_RD_BLK3_PART_RESERVE"]
    #[inline(always)]
    pub fn adc2_tp_low(&self) -> ADC2_TP_LOW_R {
        ADC2_TP_LOW_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 23:31 - ADC2 Two Point calibration high point. Only valid if EFUSE_RD_BLK3_PART_RESERVE"]
    #[inline(always)]
    pub fn adc2_tp_high(&self) -> ADC2_TP_HIGH_R {
        ADC2_TP_HIGH_R::new(((self.bits >> 23) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:31 - program for BLOCK3"]
    #[inline(always)]
    pub fn blk3_din3(&mut self) -> BLK3_DIN3_W {
        BLK3_DIN3_W { w: self }
    }
    #[doc = "Bits 0:6 - ADC1 Two Point calibration low point. Only valid if EFUSE_RD_BLK3_PART_RESERVE"]
    #[inline(always)]
    pub fn adc1_tp_low(&mut self) -> ADC1_TP_LOW_W {
        ADC1_TP_LOW_W { w: self }
    }
    #[doc = "Bits 7:15 - ADC1 Two Point calibration high point. Only valid if EFUSE_RD_BLK3_PART_RESERVE"]
    #[inline(always)]
    pub fn adc1_tp_high(&mut self) -> ADC1_TP_HIGH_W {
        ADC1_TP_HIGH_W { w: self }
    }
    #[doc = "Bits 16:22 - ADC2 Two Point calibration low point. Only valid if EFUSE_RD_BLK3_PART_RESERVE"]
    #[inline(always)]
    pub fn adc2_tp_low(&mut self) -> ADC2_TP_LOW_W {
        ADC2_TP_LOW_W { w: self }
    }
    #[doc = "Bits 23:31 - ADC2 Two Point calibration high point. Only valid if EFUSE_RD_BLK3_PART_RESERVE"]
    #[inline(always)]
    pub fn adc2_tp_high(&mut self) -> ADC2_TP_HIGH_W {
        ADC2_TP_HIGH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk3_wdata3](index.html) module"]
pub struct BLK3_WDATA3_SPEC;
impl crate::RegisterSpec for BLK3_WDATA3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blk3_wdata3::R](R) reader structure"]
impl crate::Readable for BLK3_WDATA3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [blk3_wdata3::W](W) writer structure"]
impl crate::Writable for BLK3_WDATA3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BLK3_WDATA3 to value 0"]
impl crate::Resettable for BLK3_WDATA3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
