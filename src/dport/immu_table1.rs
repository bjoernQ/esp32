#[doc = "Register `IMMU_TABLE1` reader"]
pub struct R(crate::R<IMMU_TABLE1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMMU_TABLE1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMMU_TABLE1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMMU_TABLE1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IMMU_TABLE1` writer"]
pub struct W(crate::W<IMMU_TABLE1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IMMU_TABLE1_SPEC>;
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
impl From<crate::W<IMMU_TABLE1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IMMU_TABLE1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IMMU_TABLE1` reader - "]
pub struct IMMU_TABLE1_R(crate::FieldReader<u8, u8>);
impl IMMU_TABLE1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IMMU_TABLE1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IMMU_TABLE1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IMMU_TABLE1` writer - "]
pub struct IMMU_TABLE1_W<'a> {
    w: &'a mut W,
}
impl<'a> IMMU_TABLE1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn immu_table1(&self) -> IMMU_TABLE1_R {
        IMMU_TABLE1_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn immu_table1(&mut self) -> IMMU_TABLE1_W {
        IMMU_TABLE1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [immu_table1](index.html) module"]
pub struct IMMU_TABLE1_SPEC;
impl crate::RegisterSpec for IMMU_TABLE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [immu_table1::R](R) reader structure"]
impl crate::Readable for IMMU_TABLE1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [immu_table1::W](W) writer structure"]
impl crate::Writable for IMMU_TABLE1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IMMU_TABLE1 to value 0x01"]
impl crate::Resettable for IMMU_TABLE1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
