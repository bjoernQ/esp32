#[doc = "Register `IMMU_PAGE_MODE` reader"]
pub struct R(crate::R<IMMU_PAGE_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMMU_PAGE_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMMU_PAGE_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMMU_PAGE_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IMMU_PAGE_MODE` writer"]
pub struct W(crate::W<IMMU_PAGE_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IMMU_PAGE_MODE_SPEC>;
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
impl From<crate::W<IMMU_PAGE_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IMMU_PAGE_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTERNAL_SRAM_IMMU_ENA` reader - "]
pub struct INTERNAL_SRAM_IMMU_ENA_R(crate::FieldReader<bool, bool>);
impl INTERNAL_SRAM_IMMU_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTERNAL_SRAM_IMMU_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTERNAL_SRAM_IMMU_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTERNAL_SRAM_IMMU_ENA` writer - "]
pub struct INTERNAL_SRAM_IMMU_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> INTERNAL_SRAM_IMMU_ENA_W<'a> {
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
#[doc = "Field `IMMU_PAGE_MODE` reader - "]
pub struct IMMU_PAGE_MODE_R(crate::FieldReader<u8, u8>);
impl IMMU_PAGE_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IMMU_PAGE_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IMMU_PAGE_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IMMU_PAGE_MODE` writer - "]
pub struct IMMU_PAGE_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> IMMU_PAGE_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | ((value as u32 & 0x03) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn internal_sram_immu_ena(&self) -> INTERNAL_SRAM_IMMU_ENA_R {
        INTERNAL_SRAM_IMMU_ENA_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn immu_page_mode(&self) -> IMMU_PAGE_MODE_R {
        IMMU_PAGE_MODE_R::new(((self.bits >> 1) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn internal_sram_immu_ena(&mut self) -> INTERNAL_SRAM_IMMU_ENA_W {
        INTERNAL_SRAM_IMMU_ENA_W { w: self }
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn immu_page_mode(&mut self) -> IMMU_PAGE_MODE_W {
        IMMU_PAGE_MODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [immu_page_mode](index.html) module"]
pub struct IMMU_PAGE_MODE_SPEC;
impl crate::RegisterSpec for IMMU_PAGE_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [immu_page_mode::R](R) reader structure"]
impl crate::Readable for IMMU_PAGE_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [immu_page_mode::W](W) writer structure"]
impl crate::Writable for IMMU_PAGE_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IMMU_PAGE_MODE to value 0"]
impl crate::Resettable for IMMU_PAGE_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
