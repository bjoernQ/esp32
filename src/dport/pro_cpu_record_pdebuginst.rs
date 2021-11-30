#[doc = "Register `PRO_CPU_RECORD_PDEBUGINST` reader"]
pub struct R(crate::R<PRO_CPU_RECORD_PDEBUGINST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_CPU_RECORD_PDEBUGINST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_CPU_RECORD_PDEBUGINST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_CPU_RECORD_PDEBUGINST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRO_CPU_RECORD_PDEBUGINST` writer"]
pub struct W(crate::W<PRO_CPU_RECORD_PDEBUGINST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRO_CPU_RECORD_PDEBUGINST_SPEC>;
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
impl From<crate::W<PRO_CPU_RECORD_PDEBUGINST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRO_CPU_RECORD_PDEBUGINST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RECORD_PRO_PDEBUGINST` reader - "]
pub struct RECORD_PRO_PDEBUGINST_R(crate::FieldReader<u32, u32>);
impl RECORD_PRO_PDEBUGINST_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        RECORD_PRO_PDEBUGINST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RECORD_PRO_PDEBUGINST_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RECORD_PDEBUGINST_SZ` reader - "]
pub struct RECORD_PDEBUGINST_SZ_R(crate::FieldReader<u8, u8>);
impl RECORD_PDEBUGINST_SZ_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RECORD_PDEBUGINST_SZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RECORD_PDEBUGINST_SZ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RECORD_PDEBUGINST_SZ` writer - "]
pub struct RECORD_PDEBUGINST_SZ_W<'a> {
    w: &'a mut W,
}
impl<'a> RECORD_PDEBUGINST_SZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `RECORD_PDEBUGINST_ISRC` reader - "]
pub struct RECORD_PDEBUGINST_ISRC_R(crate::FieldReader<u8, u8>);
impl RECORD_PDEBUGINST_ISRC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RECORD_PDEBUGINST_ISRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RECORD_PDEBUGINST_ISRC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RECORD_PDEBUGINST_ISRC` writer - "]
pub struct RECORD_PDEBUGINST_ISRC_W<'a> {
    w: &'a mut W,
}
impl<'a> RECORD_PDEBUGINST_ISRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
#[doc = "Field `RECORD_PDEBUGINST_CINTL` reader - "]
pub struct RECORD_PDEBUGINST_CINTL_R(crate::FieldReader<u8, u8>);
impl RECORD_PDEBUGINST_CINTL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RECORD_PDEBUGINST_CINTL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RECORD_PDEBUGINST_CINTL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RECORD_PDEBUGINST_CINTL` writer - "]
pub struct RECORD_PDEBUGINST_CINTL_W<'a> {
    w: &'a mut W,
}
impl<'a> RECORD_PDEBUGINST_CINTL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn record_pro_pdebuginst(&self) -> RECORD_PRO_PDEBUGINST_R {
        RECORD_PRO_PDEBUGINST_R::new(self.bits as u32)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn record_pdebuginst_sz(&self) -> RECORD_PDEBUGINST_SZ_R {
        RECORD_PDEBUGINST_SZ_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn record_pdebuginst_isrc(&self) -> RECORD_PDEBUGINST_ISRC_R {
        RECORD_PDEBUGINST_ISRC_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn record_pdebuginst_cintl(&self) -> RECORD_PDEBUGINST_CINTL_R {
        RECORD_PDEBUGINST_CINTL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn record_pdebuginst_sz(&mut self) -> RECORD_PDEBUGINST_SZ_W {
        RECORD_PDEBUGINST_SZ_W { w: self }
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn record_pdebuginst_isrc(&mut self) -> RECORD_PDEBUGINST_ISRC_W {
        RECORD_PDEBUGINST_ISRC_W { w: self }
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn record_pdebuginst_cintl(&mut self) -> RECORD_PDEBUGINST_CINTL_W {
        RECORD_PDEBUGINST_CINTL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_cpu_record_pdebuginst](index.html) module"]
pub struct PRO_CPU_RECORD_PDEBUGINST_SPEC;
impl crate::RegisterSpec for PRO_CPU_RECORD_PDEBUGINST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_cpu_record_pdebuginst::R](R) reader structure"]
impl crate::Readable for PRO_CPU_RECORD_PDEBUGINST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pro_cpu_record_pdebuginst::W](W) writer structure"]
impl crate::Writable for PRO_CPU_RECORD_PDEBUGINST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRO_CPU_RECORD_PDEBUGINST to value 0"]
impl crate::Resettable for PRO_CPU_RECORD_PDEBUGINST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
