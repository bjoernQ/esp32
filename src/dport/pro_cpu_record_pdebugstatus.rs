#[doc = "Register `PRO_CPU_RECORD_PDEBUGSTATUS` reader"]
pub struct R(crate::R<PRO_CPU_RECORD_PDEBUGSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_CPU_RECORD_PDEBUGSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_CPU_RECORD_PDEBUGSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_CPU_RECORD_PDEBUGSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRO_CPU_RECORD_PDEBUGSTATUS` writer"]
pub struct W(crate::W<PRO_CPU_RECORD_PDEBUGSTATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRO_CPU_RECORD_PDEBUGSTATUS_SPEC>;
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
impl From<crate::W<PRO_CPU_RECORD_PDEBUGSTATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRO_CPU_RECORD_PDEBUGSTATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RECORD_PRO_PDEBUGSTATUS` reader - "]
pub struct RECORD_PRO_PDEBUGSTATUS_R(crate::FieldReader<u8, u8>);
impl RECORD_PRO_PDEBUGSTATUS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RECORD_PRO_PDEBUGSTATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RECORD_PRO_PDEBUGSTATUS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RECORD_PDEBUGSTATUS_BBCAUSE` reader - "]
pub struct RECORD_PDEBUGSTATUS_BBCAUSE_R(crate::FieldReader<u8, u8>);
impl RECORD_PDEBUGSTATUS_BBCAUSE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RECORD_PDEBUGSTATUS_BBCAUSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RECORD_PDEBUGSTATUS_BBCAUSE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RECORD_PDEBUGSTATUS_BBCAUSE` writer - "]
pub struct RECORD_PDEBUGSTATUS_BBCAUSE_W<'a> {
    w: &'a mut W,
}
impl<'a> RECORD_PDEBUGSTATUS_BBCAUSE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Field `RECORD_PDEBUGSTATUS_INSNTYPE` reader - "]
pub struct RECORD_PDEBUGSTATUS_INSNTYPE_R(crate::FieldReader<u8, u8>);
impl RECORD_PDEBUGSTATUS_INSNTYPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RECORD_PDEBUGSTATUS_INSNTYPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RECORD_PDEBUGSTATUS_INSNTYPE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RECORD_PDEBUGSTATUS_INSNTYPE` writer - "]
pub struct RECORD_PDEBUGSTATUS_INSNTYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> RECORD_PDEBUGSTATUS_INSNTYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn record_pro_pdebugstatus(&self) -> RECORD_PRO_PDEBUGSTATUS_R {
        RECORD_PRO_PDEBUGSTATUS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn record_pdebugstatus_bbcause(&self) -> RECORD_PDEBUGSTATUS_BBCAUSE_R {
        RECORD_PDEBUGSTATUS_BBCAUSE_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn record_pdebugstatus_insntype(&self) -> RECORD_PDEBUGSTATUS_INSNTYPE_R {
        RECORD_PDEBUGSTATUS_INSNTYPE_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn record_pdebugstatus_bbcause(&mut self) -> RECORD_PDEBUGSTATUS_BBCAUSE_W {
        RECORD_PDEBUGSTATUS_BBCAUSE_W { w: self }
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn record_pdebugstatus_insntype(&mut self) -> RECORD_PDEBUGSTATUS_INSNTYPE_W {
        RECORD_PDEBUGSTATUS_INSNTYPE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_cpu_record_pdebugstatus](index.html) module"]
pub struct PRO_CPU_RECORD_PDEBUGSTATUS_SPEC;
impl crate::RegisterSpec for PRO_CPU_RECORD_PDEBUGSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_cpu_record_pdebugstatus::R](R) reader structure"]
impl crate::Readable for PRO_CPU_RECORD_PDEBUGSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pro_cpu_record_pdebugstatus::W](W) writer structure"]
impl crate::Writable for PRO_CPU_RECORD_PDEBUGSTATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRO_CPU_RECORD_PDEBUGSTATUS to value 0"]
impl crate::Resettable for PRO_CPU_RECORD_PDEBUGSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
