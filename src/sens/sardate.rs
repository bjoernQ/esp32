#[doc = "Register `SARDATE` reader"]
pub struct R(crate::R<SARDATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SARDATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SARDATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SARDATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SARDATE` writer"]
pub struct W(crate::W<SARDATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SARDATE_SPEC>;
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
impl From<crate::W<SARDATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SARDATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAR_DATE` reader - "]
pub struct SAR_DATE_R(crate::FieldReader<u32, u32>);
impl SAR_DATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        SAR_DATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR_DATE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR_DATE` writer - "]
pub struct SAR_DATE_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR_DATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff_ffff) | (value as u32 & 0x0fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:27"]
    #[inline(always)]
    pub fn sar_date(&self) -> SAR_DATE_R {
        SAR_DATE_R::new((self.bits & 0x0fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:27"]
    #[inline(always)]
    pub fn sar_date(&mut self) -> SAR_DATE_W {
        SAR_DATE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sardate](index.html) module"]
pub struct SARDATE_SPEC;
impl crate::RegisterSpec for SARDATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sardate::R](R) reader structure"]
impl crate::Readable for SARDATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sardate::W](W) writer structure"]
impl crate::Writable for SARDATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SARDATE to value 0x0160_5180"]
impl crate::Resettable for SARDATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0160_5180
    }
}
