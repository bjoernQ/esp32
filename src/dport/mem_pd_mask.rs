#[doc = "Register `MEM_PD_MASK` reader"]
pub struct R(crate::R<MEM_PD_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEM_PD_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEM_PD_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEM_PD_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MEM_PD_MASK` writer"]
pub struct W(crate::W<MEM_PD_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MEM_PD_MASK_SPEC>;
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
impl From<crate::W<MEM_PD_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MEM_PD_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LSLP_MEM_PD_MASK` reader - "]
pub struct LSLP_MEM_PD_MASK_R(crate::FieldReader<bool, bool>);
impl LSLP_MEM_PD_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LSLP_MEM_PD_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSLP_MEM_PD_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSLP_MEM_PD_MASK` writer - "]
pub struct LSLP_MEM_PD_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> LSLP_MEM_PD_MASK_W<'a> {
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
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn lslp_mem_pd_mask(&self) -> LSLP_MEM_PD_MASK_R {
        LSLP_MEM_PD_MASK_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn lslp_mem_pd_mask(&mut self) -> LSLP_MEM_PD_MASK_W {
        LSLP_MEM_PD_MASK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_pd_mask](index.html) module"]
pub struct MEM_PD_MASK_SPEC;
impl crate::RegisterSpec for MEM_PD_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mem_pd_mask::R](R) reader structure"]
impl crate::Readable for MEM_PD_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mem_pd_mask::W](W) writer structure"]
impl crate::Writable for MEM_PD_MASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MEM_PD_MASK to value 0x01"]
impl crate::Resettable for MEM_PD_MASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
