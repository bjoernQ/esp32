#[doc = "Register `CARRIER0_CFG` reader"]
pub struct R(crate::R<CARRIER0_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CARRIER0_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CARRIER0_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CARRIER0_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CARRIER0_CFG` writer"]
pub struct W(crate::W<CARRIER0_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CARRIER0_CFG_SPEC>;
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
impl From<crate::W<CARRIER0_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CARRIER0_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CARRIER0_EN` reader - "]
pub struct CARRIER0_EN_R(crate::FieldReader<bool, bool>);
impl CARRIER0_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CARRIER0_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CARRIER0_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CARRIER0_EN` writer - "]
pub struct CARRIER0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CARRIER0_EN_W<'a> {
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
#[doc = "Field `CARRIER0_PRESCALE` reader - "]
pub struct CARRIER0_PRESCALE_R(crate::FieldReader<u8, u8>);
impl CARRIER0_PRESCALE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CARRIER0_PRESCALE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CARRIER0_PRESCALE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CARRIER0_PRESCALE` writer - "]
pub struct CARRIER0_PRESCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> CARRIER0_PRESCALE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 1)) | ((value as u32 & 0x0f) << 1);
        self.w
    }
}
#[doc = "Field `CARRIER0_DUTY` reader - "]
pub struct CARRIER0_DUTY_R(crate::FieldReader<u8, u8>);
impl CARRIER0_DUTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CARRIER0_DUTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CARRIER0_DUTY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CARRIER0_DUTY` writer - "]
pub struct CARRIER0_DUTY_W<'a> {
    w: &'a mut W,
}
impl<'a> CARRIER0_DUTY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | ((value as u32 & 0x07) << 5);
        self.w
    }
}
#[doc = "Field `CARRIER0_OSHTWTH` reader - "]
pub struct CARRIER0_OSHTWTH_R(crate::FieldReader<u8, u8>);
impl CARRIER0_OSHTWTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CARRIER0_OSHTWTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CARRIER0_OSHTWTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CARRIER0_OSHTWTH` writer - "]
pub struct CARRIER0_OSHTWTH_W<'a> {
    w: &'a mut W,
}
impl<'a> CARRIER0_OSHTWTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `CARRIER0_OUT_INVERT` reader - "]
pub struct CARRIER0_OUT_INVERT_R(crate::FieldReader<bool, bool>);
impl CARRIER0_OUT_INVERT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CARRIER0_OUT_INVERT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CARRIER0_OUT_INVERT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CARRIER0_OUT_INVERT` writer - "]
pub struct CARRIER0_OUT_INVERT_W<'a> {
    w: &'a mut W,
}
impl<'a> CARRIER0_OUT_INVERT_W<'a> {
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
#[doc = "Field `CARRIER0_IN_INVERT` reader - "]
pub struct CARRIER0_IN_INVERT_R(crate::FieldReader<bool, bool>);
impl CARRIER0_IN_INVERT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CARRIER0_IN_INVERT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CARRIER0_IN_INVERT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CARRIER0_IN_INVERT` writer - "]
pub struct CARRIER0_IN_INVERT_W<'a> {
    w: &'a mut W,
}
impl<'a> CARRIER0_IN_INVERT_W<'a> {
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
    pub fn carrier0_en(&self) -> CARRIER0_EN_R {
        CARRIER0_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:4"]
    #[inline(always)]
    pub fn carrier0_prescale(&self) -> CARRIER0_PRESCALE_R {
        CARRIER0_PRESCALE_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bits 5:7"]
    #[inline(always)]
    pub fn carrier0_duty(&self) -> CARRIER0_DUTY_R {
        CARRIER0_DUTY_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn carrier0_oshtwth(&self) -> CARRIER0_OSHTWTH_R {
        CARRIER0_OSHTWTH_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn carrier0_out_invert(&self) -> CARRIER0_OUT_INVERT_R {
        CARRIER0_OUT_INVERT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn carrier0_in_invert(&self) -> CARRIER0_IN_INVERT_R {
        CARRIER0_IN_INVERT_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn carrier0_en(&mut self) -> CARRIER0_EN_W {
        CARRIER0_EN_W { w: self }
    }
    #[doc = "Bits 1:4"]
    #[inline(always)]
    pub fn carrier0_prescale(&mut self) -> CARRIER0_PRESCALE_W {
        CARRIER0_PRESCALE_W { w: self }
    }
    #[doc = "Bits 5:7"]
    #[inline(always)]
    pub fn carrier0_duty(&mut self) -> CARRIER0_DUTY_W {
        CARRIER0_DUTY_W { w: self }
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn carrier0_oshtwth(&mut self) -> CARRIER0_OSHTWTH_W {
        CARRIER0_OSHTWTH_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn carrier0_out_invert(&mut self) -> CARRIER0_OUT_INVERT_W {
        CARRIER0_OUT_INVERT_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn carrier0_in_invert(&mut self) -> CARRIER0_IN_INVERT_W {
        CARRIER0_IN_INVERT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [carrier0_cfg](index.html) module"]
pub struct CARRIER0_CFG_SPEC;
impl crate::RegisterSpec for CARRIER0_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [carrier0_cfg::R](R) reader structure"]
impl crate::Readable for CARRIER0_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [carrier0_cfg::W](W) writer structure"]
impl crate::Writable for CARRIER0_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CARRIER0_CFG to value 0"]
impl crate::Resettable for CARRIER0_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
