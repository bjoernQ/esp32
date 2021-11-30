#[doc = "Register `MMU_IA_INT_EN` reader"]
pub struct R(crate::R<MMU_IA_INT_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMU_IA_INT_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMU_IA_INT_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMU_IA_INT_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MMU_IA_INT_EN` writer"]
pub struct W(crate::W<MMU_IA_INT_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMU_IA_INT_EN_SPEC>;
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
impl From<crate::W<MMU_IA_INT_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMU_IA_INT_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MMU_IA_INT_EN` reader - "]
pub struct MMU_IA_INT_EN_R(crate::FieldReader<u32, u32>);
impl MMU_IA_INT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        MMU_IA_INT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MMU_IA_INT_EN_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MMU_IA_INT_EN` writer - "]
pub struct MMU_IA_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> MMU_IA_INT_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | (value as u32 & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn mmu_ia_int_en(&self) -> MMU_IA_INT_EN_R {
        MMU_IA_INT_EN_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn mmu_ia_int_en(&mut self) -> MMU_IA_INT_EN_W {
        MMU_IA_INT_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmu_ia_int_en](index.html) module"]
pub struct MMU_IA_INT_EN_SPEC;
impl crate::RegisterSpec for MMU_IA_INT_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmu_ia_int_en::R](R) reader structure"]
impl crate::Readable for MMU_IA_INT_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mmu_ia_int_en::W](W) writer structure"]
impl crate::Writable for MMU_IA_INT_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MMU_IA_INT_EN to value 0"]
impl crate::Resettable for MMU_IA_INT_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
