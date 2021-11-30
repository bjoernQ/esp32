#[doc = "Register `FH1_CFG1` reader"]
pub struct R(crate::R<FH1_CFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FH1_CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FH1_CFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FH1_CFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FH1_CFG1` writer"]
pub struct W(crate::W<FH1_CFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FH1_CFG1_SPEC>;
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
impl From<crate::W<FH1_CFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FH1_CFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FH1_CLR_OST` reader - "]
pub struct FH1_CLR_OST_R(crate::FieldReader<bool, bool>);
impl FH1_CLR_OST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FH1_CLR_OST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FH1_CLR_OST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FH1_CLR_OST` writer - "]
pub struct FH1_CLR_OST_W<'a> {
    w: &'a mut W,
}
impl<'a> FH1_CLR_OST_W<'a> {
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
#[doc = "Field `FH1_CBCPULSE` reader - "]
pub struct FH1_CBCPULSE_R(crate::FieldReader<u8, u8>);
impl FH1_CBCPULSE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FH1_CBCPULSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FH1_CBCPULSE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FH1_CBCPULSE` writer - "]
pub struct FH1_CBCPULSE_W<'a> {
    w: &'a mut W,
}
impl<'a> FH1_CBCPULSE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | ((value as u32 & 0x03) << 1);
        self.w
    }
}
#[doc = "Field `FH1_FORCE_CBC` reader - "]
pub struct FH1_FORCE_CBC_R(crate::FieldReader<bool, bool>);
impl FH1_FORCE_CBC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FH1_FORCE_CBC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FH1_FORCE_CBC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FH1_FORCE_CBC` writer - "]
pub struct FH1_FORCE_CBC_W<'a> {
    w: &'a mut W,
}
impl<'a> FH1_FORCE_CBC_W<'a> {
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
#[doc = "Field `FH1_FORCE_OST` reader - "]
pub struct FH1_FORCE_OST_R(crate::FieldReader<bool, bool>);
impl FH1_FORCE_OST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FH1_FORCE_OST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FH1_FORCE_OST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FH1_FORCE_OST` writer - "]
pub struct FH1_FORCE_OST_W<'a> {
    w: &'a mut W,
}
impl<'a> FH1_FORCE_OST_W<'a> {
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
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn fh1_clr_ost(&self) -> FH1_CLR_OST_R {
        FH1_CLR_OST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn fh1_cbcpulse(&self) -> FH1_CBCPULSE_R {
        FH1_CBCPULSE_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn fh1_force_cbc(&self) -> FH1_FORCE_CBC_R {
        FH1_FORCE_CBC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn fh1_force_ost(&self) -> FH1_FORCE_OST_R {
        FH1_FORCE_OST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn fh1_clr_ost(&mut self) -> FH1_CLR_OST_W {
        FH1_CLR_OST_W { w: self }
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn fh1_cbcpulse(&mut self) -> FH1_CBCPULSE_W {
        FH1_CBCPULSE_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn fh1_force_cbc(&mut self) -> FH1_FORCE_CBC_W {
        FH1_FORCE_CBC_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn fh1_force_ost(&mut self) -> FH1_FORCE_OST_W {
        FH1_FORCE_OST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fh1_cfg1](index.html) module"]
pub struct FH1_CFG1_SPEC;
impl crate::RegisterSpec for FH1_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fh1_cfg1::R](R) reader structure"]
impl crate::Readable for FH1_CFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fh1_cfg1::W](W) writer structure"]
impl crate::Writable for FH1_CFG1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FH1_CFG1 to value 0"]
impl crate::Resettable for FH1_CFG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
