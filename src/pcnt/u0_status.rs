#[doc = "Register `U0_STATUS` reader"]
pub struct R(crate::R<U0_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<U0_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<U0_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<U0_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `U0_STATUS` writer"]
pub struct W(crate::W<U0_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<U0_STATUS_SPEC>;
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
impl From<crate::W<U0_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<U0_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_STATUS_U0` reader - "]
pub struct CORE_STATUS_U0_R(crate::FieldReader<u32, u32>);
impl CORE_STATUS_U0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CORE_STATUS_U0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_STATUS_U0_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STATUS_CNT_MODE` reader - "]
pub struct STATUS_CNT_MODE_R(crate::FieldReader<u8, u8>);
impl STATUS_CNT_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        STATUS_CNT_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STATUS_CNT_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STATUS_CNT_MODE` writer - "]
pub struct STATUS_CNT_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> STATUS_CNT_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `STATUS_THRES1` reader - "]
pub struct STATUS_THRES1_R(crate::FieldReader<bool, bool>);
impl STATUS_THRES1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STATUS_THRES1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STATUS_THRES1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STATUS_THRES1` writer - "]
pub struct STATUS_THRES1_W<'a> {
    w: &'a mut W,
}
impl<'a> STATUS_THRES1_W<'a> {
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
#[doc = "Field `STATUS_THRES0` reader - "]
pub struct STATUS_THRES0_R(crate::FieldReader<bool, bool>);
impl STATUS_THRES0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STATUS_THRES0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STATUS_THRES0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STATUS_THRES0` writer - "]
pub struct STATUS_THRES0_W<'a> {
    w: &'a mut W,
}
impl<'a> STATUS_THRES0_W<'a> {
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
#[doc = "Field `STATUS_L_LIM` reader - "]
pub struct STATUS_L_LIM_R(crate::FieldReader<bool, bool>);
impl STATUS_L_LIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STATUS_L_LIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STATUS_L_LIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STATUS_L_LIM` writer - "]
pub struct STATUS_L_LIM_W<'a> {
    w: &'a mut W,
}
impl<'a> STATUS_L_LIM_W<'a> {
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
#[doc = "Field `STATUS_H_LIM` reader - "]
pub struct STATUS_H_LIM_R(crate::FieldReader<bool, bool>);
impl STATUS_H_LIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STATUS_H_LIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STATUS_H_LIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STATUS_H_LIM` writer - "]
pub struct STATUS_H_LIM_W<'a> {
    w: &'a mut W,
}
impl<'a> STATUS_H_LIM_W<'a> {
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
#[doc = "Field `STATUS_ZERO` reader - "]
pub struct STATUS_ZERO_R(crate::FieldReader<bool, bool>);
impl STATUS_ZERO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STATUS_ZERO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STATUS_ZERO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STATUS_ZERO` writer - "]
pub struct STATUS_ZERO_W<'a> {
    w: &'a mut W,
}
impl<'a> STATUS_ZERO_W<'a> {
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
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn core_status_u0(&self) -> CORE_STATUS_U0_R {
        CORE_STATUS_U0_R::new(self.bits as u32)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn status_cnt_mode(&self) -> STATUS_CNT_MODE_R {
        STATUS_CNT_MODE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn status_thres1(&self) -> STATUS_THRES1_R {
        STATUS_THRES1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn status_thres0(&self) -> STATUS_THRES0_R {
        STATUS_THRES0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn status_l_lim(&self) -> STATUS_L_LIM_R {
        STATUS_L_LIM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn status_h_lim(&self) -> STATUS_H_LIM_R {
        STATUS_H_LIM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn status_zero(&self) -> STATUS_ZERO_R {
        STATUS_ZERO_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn status_cnt_mode(&mut self) -> STATUS_CNT_MODE_W {
        STATUS_CNT_MODE_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn status_thres1(&mut self) -> STATUS_THRES1_W {
        STATUS_THRES1_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn status_thres0(&mut self) -> STATUS_THRES0_W {
        STATUS_THRES0_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn status_l_lim(&mut self) -> STATUS_L_LIM_W {
        STATUS_L_LIM_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn status_h_lim(&mut self) -> STATUS_H_LIM_W {
        STATUS_H_LIM_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn status_zero(&mut self) -> STATUS_ZERO_W {
        STATUS_ZERO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u0_status](index.html) module"]
pub struct U0_STATUS_SPEC;
impl crate::RegisterSpec for U0_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [u0_status::R](R) reader structure"]
impl crate::Readable for U0_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [u0_status::W](W) writer structure"]
impl crate::Writable for U0_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets U0_STATUS to value 0"]
impl crate::Resettable for U0_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
