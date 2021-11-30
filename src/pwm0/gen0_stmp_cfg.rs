#[doc = "Register `GEN0_STMP_CFG` reader"]
pub struct R(crate::R<GEN0_STMP_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GEN0_STMP_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GEN0_STMP_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GEN0_STMP_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GEN0_STMP_CFG` writer"]
pub struct W(crate::W<GEN0_STMP_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GEN0_STMP_CFG_SPEC>;
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
impl From<crate::W<GEN0_STMP_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GEN0_STMP_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GEN0_A_UPMETHOD` reader - "]
pub struct GEN0_A_UPMETHOD_R(crate::FieldReader<u8, u8>);
impl GEN0_A_UPMETHOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GEN0_A_UPMETHOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GEN0_A_UPMETHOD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GEN0_A_UPMETHOD` writer - "]
pub struct GEN0_A_UPMETHOD_W<'a> {
    w: &'a mut W,
}
impl<'a> GEN0_A_UPMETHOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `GEN0_B_UPMETHOD` reader - "]
pub struct GEN0_B_UPMETHOD_R(crate::FieldReader<u8, u8>);
impl GEN0_B_UPMETHOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GEN0_B_UPMETHOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GEN0_B_UPMETHOD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GEN0_B_UPMETHOD` writer - "]
pub struct GEN0_B_UPMETHOD_W<'a> {
    w: &'a mut W,
}
impl<'a> GEN0_B_UPMETHOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `GEN0_A_SHDW_FULL` reader - "]
pub struct GEN0_A_SHDW_FULL_R(crate::FieldReader<bool, bool>);
impl GEN0_A_SHDW_FULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GEN0_A_SHDW_FULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GEN0_A_SHDW_FULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GEN0_A_SHDW_FULL` writer - "]
pub struct GEN0_A_SHDW_FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> GEN0_A_SHDW_FULL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `GEN0_B_SHDW_FULL` reader - "]
pub struct GEN0_B_SHDW_FULL_R(crate::FieldReader<bool, bool>);
impl GEN0_B_SHDW_FULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GEN0_B_SHDW_FULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GEN0_B_SHDW_FULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GEN0_B_SHDW_FULL` writer - "]
pub struct GEN0_B_SHDW_FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> GEN0_B_SHDW_FULL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn gen0_a_upmethod(&self) -> GEN0_A_UPMETHOD_R {
        GEN0_A_UPMETHOD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn gen0_b_upmethod(&self) -> GEN0_B_UPMETHOD_R {
        GEN0_B_UPMETHOD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gen0_a_shdw_full(&self) -> GEN0_A_SHDW_FULL_R {
        GEN0_A_SHDW_FULL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn gen0_b_shdw_full(&self) -> GEN0_B_SHDW_FULL_R {
        GEN0_B_SHDW_FULL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn gen0_a_upmethod(&mut self) -> GEN0_A_UPMETHOD_W {
        GEN0_A_UPMETHOD_W { w: self }
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn gen0_b_upmethod(&mut self) -> GEN0_B_UPMETHOD_W {
        GEN0_B_UPMETHOD_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gen0_a_shdw_full(&mut self) -> GEN0_A_SHDW_FULL_W {
        GEN0_A_SHDW_FULL_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn gen0_b_shdw_full(&mut self) -> GEN0_B_SHDW_FULL_W {
        GEN0_B_SHDW_FULL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gen0_stmp_cfg](index.html) module"]
pub struct GEN0_STMP_CFG_SPEC;
impl crate::RegisterSpec for GEN0_STMP_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gen0_stmp_cfg::R](R) reader structure"]
impl crate::Readable for GEN0_STMP_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gen0_stmp_cfg::W](W) writer structure"]
impl crate::Writable for GEN0_STMP_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GEN0_STMP_CFG to value 0"]
impl crate::Resettable for GEN0_STMP_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
