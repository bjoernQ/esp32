#[doc = "Register `PRO_CPU_RECORD_PDEBUGDATA` reader"]
pub struct R(crate::R<PRO_CPU_RECORD_PDEBUGDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_CPU_RECORD_PDEBUGDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_CPU_RECORD_PDEBUGDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_CPU_RECORD_PDEBUGDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRO_CPU_RECORD_PDEBUGDATA` writer"]
pub struct W(crate::W<PRO_CPU_RECORD_PDEBUGDATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRO_CPU_RECORD_PDEBUGDATA_SPEC>;
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
impl From<crate::W<PRO_CPU_RECORD_PDEBUGDATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRO_CPU_RECORD_PDEBUGDATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RECORD_PRO_PDEBUGDATA` reader - "]
pub struct RECORD_PRO_PDEBUGDATA_R(crate::FieldReader<u32, u32>);
impl RECORD_PRO_PDEBUGDATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        RECORD_PRO_PDEBUGDATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RECORD_PRO_PDEBUGDATA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RECORD_PDEBUGDATA_EXCVEC` reader - "]
pub struct RECORD_PDEBUGDATA_EXCVEC_R(crate::FieldReader<u8, u8>);
impl RECORD_PDEBUGDATA_EXCVEC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RECORD_PDEBUGDATA_EXCVEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RECORD_PDEBUGDATA_EXCVEC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RECORD_PDEBUGDATA_EXCVEC` writer - "]
pub struct RECORD_PDEBUGDATA_EXCVEC_W<'a> {
    w: &'a mut W,
}
impl<'a> RECORD_PDEBUGDATA_EXCVEC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Field `RECORD_PDEBUGDATA_INSNTYPE_SR` reader - "]
pub struct RECORD_PDEBUGDATA_INSNTYPE_SR_R(crate::FieldReader<u8, u8>);
impl RECORD_PDEBUGDATA_INSNTYPE_SR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RECORD_PDEBUGDATA_INSNTYPE_SR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RECORD_PDEBUGDATA_INSNTYPE_SR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RECORD_PDEBUGDATA_INSNTYPE_SR` writer - "]
pub struct RECORD_PDEBUGDATA_INSNTYPE_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> RECORD_PDEBUGDATA_INSNTYPE_SR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `RECORD_PDEBUGDATA_INSNTYPE_ER` reader - "]
pub struct RECORD_PDEBUGDATA_INSNTYPE_ER_R(crate::FieldReader<u16, u16>);
impl RECORD_PDEBUGDATA_INSNTYPE_ER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        RECORD_PDEBUGDATA_INSNTYPE_ER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RECORD_PDEBUGDATA_INSNTYPE_ER_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RECORD_PDEBUGDATA_INSNTYPE_ER` writer - "]
pub struct RECORD_PDEBUGDATA_INSNTYPE_ER_W<'a> {
    w: &'a mut W,
}
impl<'a> RECORD_PDEBUGDATA_INSNTYPE_ER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 2)) | ((value as u32 & 0x0fff) << 2);
        self.w
    }
}
#[doc = "Field `RECORD_PDEBUGDATA_EXCCAUSE` reader - "]
pub struct RECORD_PDEBUGDATA_EXCCAUSE_R(crate::FieldReader<u8, u8>);
impl RECORD_PDEBUGDATA_EXCCAUSE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RECORD_PDEBUGDATA_EXCCAUSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RECORD_PDEBUGDATA_EXCCAUSE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RECORD_PDEBUGDATA_EXCCAUSE` writer - "]
pub struct RECORD_PDEBUGDATA_EXCCAUSE_W<'a> {
    w: &'a mut W,
}
impl<'a> RECORD_PDEBUGDATA_EXCCAUSE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn record_pro_pdebugdata(&self) -> RECORD_PRO_PDEBUGDATA_R {
        RECORD_PRO_PDEBUGDATA_R::new(self.bits as u32)
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn record_pdebugdata_excvec(&self) -> RECORD_PDEBUGDATA_EXCVEC_R {
        RECORD_PDEBUGDATA_EXCVEC_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn record_pdebugdata_insntype_sr(&self) -> RECORD_PDEBUGDATA_INSNTYPE_SR_R {
        RECORD_PDEBUGDATA_INSNTYPE_SR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 2:13"]
    #[inline(always)]
    pub fn record_pdebugdata_insntype_er(&self) -> RECORD_PDEBUGDATA_INSNTYPE_ER_R {
        RECORD_PDEBUGDATA_INSNTYPE_ER_R::new(((self.bits >> 2) & 0x0fff) as u16)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn record_pdebugdata_exccause(&self) -> RECORD_PDEBUGDATA_EXCCAUSE_R {
        RECORD_PDEBUGDATA_EXCCAUSE_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn record_pdebugdata_excvec(&mut self) -> RECORD_PDEBUGDATA_EXCVEC_W {
        RECORD_PDEBUGDATA_EXCVEC_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn record_pdebugdata_insntype_sr(&mut self) -> RECORD_PDEBUGDATA_INSNTYPE_SR_W {
        RECORD_PDEBUGDATA_INSNTYPE_SR_W { w: self }
    }
    #[doc = "Bits 2:13"]
    #[inline(always)]
    pub fn record_pdebugdata_insntype_er(&mut self) -> RECORD_PDEBUGDATA_INSNTYPE_ER_W {
        RECORD_PDEBUGDATA_INSNTYPE_ER_W { w: self }
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn record_pdebugdata_exccause(&mut self) -> RECORD_PDEBUGDATA_EXCCAUSE_W {
        RECORD_PDEBUGDATA_EXCCAUSE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_cpu_record_pdebugdata](index.html) module"]
pub struct PRO_CPU_RECORD_PDEBUGDATA_SPEC;
impl crate::RegisterSpec for PRO_CPU_RECORD_PDEBUGDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_cpu_record_pdebugdata::R](R) reader structure"]
impl crate::Readable for PRO_CPU_RECORD_PDEBUGDATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pro_cpu_record_pdebugdata::W](W) writer structure"]
impl crate::Writable for PRO_CPU_RECORD_PDEBUGDATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRO_CPU_RECORD_PDEBUGDATA to value 0"]
impl crate::Resettable for PRO_CPU_RECORD_PDEBUGDATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
