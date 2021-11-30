#[doc = "Register `T1LOADHI` reader"]
pub struct R(crate::R<T1LOADHI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<T1LOADHI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<T1LOADHI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<T1LOADHI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `T1LOADHI` writer"]
pub struct W(crate::W<T1LOADHI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<T1LOADHI_SPEC>;
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
impl From<crate::W<T1LOADHI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<T1LOADHI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T1_LOAD_HI` reader - higher 32 bits of the value that will load into timer 1 time-base counter"]
pub struct T1_LOAD_HI_R(crate::FieldReader<u32, u32>);
impl T1_LOAD_HI_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        T1_LOAD_HI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T1_LOAD_HI_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T1_LOAD_HI` writer - higher 32 bits of the value that will load into timer 1 time-base counter"]
pub struct T1_LOAD_HI_W<'a> {
    w: &'a mut W,
}
impl<'a> T1_LOAD_HI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - higher 32 bits of the value that will load into timer 1 time-base counter"]
    #[inline(always)]
    pub fn t1_load_hi(&self) -> T1_LOAD_HI_R {
        T1_LOAD_HI_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - higher 32 bits of the value that will load into timer 1 time-base counter"]
    #[inline(always)]
    pub fn t1_load_hi(&mut self) -> T1_LOAD_HI_W {
        T1_LOAD_HI_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t1loadhi](index.html) module"]
pub struct T1LOADHI_SPEC;
impl crate::RegisterSpec for T1LOADHI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [t1loadhi::R](R) reader structure"]
impl crate::Readable for T1LOADHI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [t1loadhi::W](W) writer structure"]
impl crate::Writable for T1LOADHI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets T1LOADHI to value 0"]
impl crate::Resettable for T1LOADHI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
