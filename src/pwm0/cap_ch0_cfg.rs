#[doc = "Register `CAP_CH0_CFG` reader"]
pub struct R(crate::R<CAP_CH0_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAP_CH0_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAP_CH0_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAP_CH0_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAP_CH0_CFG` writer"]
pub struct W(crate::W<CAP_CH0_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAP_CH0_CFG_SPEC>;
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
impl From<crate::W<CAP_CH0_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAP_CH0_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAP0_EN` reader - "]
pub struct CAP0_EN_R(crate::FieldReader<bool, bool>);
impl CAP0_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CAP0_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAP0_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAP0_EN` writer - "]
pub struct CAP0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP0_EN_W<'a> {
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
#[doc = "Field `CAP0_MODE` reader - "]
pub struct CAP0_MODE_R(crate::FieldReader<u8, u8>);
impl CAP0_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CAP0_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAP0_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAP0_MODE` writer - "]
pub struct CAP0_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP0_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | ((value as u32 & 0x03) << 1);
        self.w
    }
}
#[doc = "Field `CAP0_PRESCALE` reader - "]
pub struct CAP0_PRESCALE_R(crate::FieldReader<u8, u8>);
impl CAP0_PRESCALE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CAP0_PRESCALE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAP0_PRESCALE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAP0_PRESCALE` writer - "]
pub struct CAP0_PRESCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP0_PRESCALE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 3)) | ((value as u32 & 0xff) << 3);
        self.w
    }
}
#[doc = "Field `CAP0_IN_INVERT` reader - "]
pub struct CAP0_IN_INVERT_R(crate::FieldReader<bool, bool>);
impl CAP0_IN_INVERT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CAP0_IN_INVERT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAP0_IN_INVERT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAP0_IN_INVERT` writer - "]
pub struct CAP0_IN_INVERT_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP0_IN_INVERT_W<'a> {
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
#[doc = "Field `CAP0_SW` writer - "]
pub struct CAP0_SW_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP0_SW_W<'a> {
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
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cap0_en(&self) -> CAP0_EN_R {
        CAP0_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn cap0_mode(&self) -> CAP0_MODE_R {
        CAP0_MODE_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bits 3:10"]
    #[inline(always)]
    pub fn cap0_prescale(&self) -> CAP0_PRESCALE_R {
        CAP0_PRESCALE_R::new(((self.bits >> 3) & 0xff) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cap0_in_invert(&self) -> CAP0_IN_INVERT_R {
        CAP0_IN_INVERT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cap0_en(&mut self) -> CAP0_EN_W {
        CAP0_EN_W { w: self }
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn cap0_mode(&mut self) -> CAP0_MODE_W {
        CAP0_MODE_W { w: self }
    }
    #[doc = "Bits 3:10"]
    #[inline(always)]
    pub fn cap0_prescale(&mut self) -> CAP0_PRESCALE_W {
        CAP0_PRESCALE_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cap0_in_invert(&mut self) -> CAP0_IN_INVERT_W {
        CAP0_IN_INVERT_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn cap0_sw(&mut self) -> CAP0_SW_W {
        CAP0_SW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cap_ch0_cfg](index.html) module"]
pub struct CAP_CH0_CFG_SPEC;
impl crate::RegisterSpec for CAP_CH0_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cap_ch0_cfg::R](R) reader structure"]
impl crate::Readable for CAP_CH0_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cap_ch0_cfg::W](W) writer structure"]
impl crate::Writable for CAP_CH0_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAP_CH0_CFG to value 0"]
impl crate::Resettable for CAP_CH0_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
