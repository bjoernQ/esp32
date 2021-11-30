#[doc = "Register `GEN2_CFG0` reader"]
pub struct R(crate::R<GEN2_CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GEN2_CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GEN2_CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GEN2_CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GEN2_CFG0` writer"]
pub struct W(crate::W<GEN2_CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GEN2_CFG0_SPEC>;
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
impl From<crate::W<GEN2_CFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GEN2_CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GEN2_CFG_UPMETHOD` reader - "]
pub struct GEN2_CFG_UPMETHOD_R(crate::FieldReader<u8, u8>);
impl GEN2_CFG_UPMETHOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GEN2_CFG_UPMETHOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GEN2_CFG_UPMETHOD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GEN2_CFG_UPMETHOD` writer - "]
pub struct GEN2_CFG_UPMETHOD_W<'a> {
    w: &'a mut W,
}
impl<'a> GEN2_CFG_UPMETHOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `GEN2_T0_SEL` reader - "]
pub struct GEN2_T0_SEL_R(crate::FieldReader<u8, u8>);
impl GEN2_T0_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GEN2_T0_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GEN2_T0_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GEN2_T0_SEL` writer - "]
pub struct GEN2_T0_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GEN2_T0_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `GEN2_T1_SEL` reader - "]
pub struct GEN2_T1_SEL_R(crate::FieldReader<u8, u8>);
impl GEN2_T1_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GEN2_T1_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GEN2_T1_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GEN2_T1_SEL` writer - "]
pub struct GEN2_T1_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GEN2_T1_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 7)) | ((value as u32 & 0x07) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn gen2_cfg_upmethod(&self) -> GEN2_CFG_UPMETHOD_R {
        GEN2_CFG_UPMETHOD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn gen2_t0_sel(&self) -> GEN2_T0_SEL_R {
        GEN2_T0_SEL_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 7:9"]
    #[inline(always)]
    pub fn gen2_t1_sel(&self) -> GEN2_T1_SEL_R {
        GEN2_T1_SEL_R::new(((self.bits >> 7) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn gen2_cfg_upmethod(&mut self) -> GEN2_CFG_UPMETHOD_W {
        GEN2_CFG_UPMETHOD_W { w: self }
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn gen2_t0_sel(&mut self) -> GEN2_T0_SEL_W {
        GEN2_T0_SEL_W { w: self }
    }
    #[doc = "Bits 7:9"]
    #[inline(always)]
    pub fn gen2_t1_sel(&mut self) -> GEN2_T1_SEL_W {
        GEN2_T1_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gen2_cfg0](index.html) module"]
pub struct GEN2_CFG0_SPEC;
impl crate::RegisterSpec for GEN2_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gen2_cfg0::R](R) reader structure"]
impl crate::Readable for GEN2_CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gen2_cfg0::W](W) writer structure"]
impl crate::Writable for GEN2_CFG0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GEN2_CFG0 to value 0"]
impl crate::Resettable for GEN2_CFG0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
