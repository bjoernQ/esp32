#[doc = "Register `LSCH4_HPOINT` reader"]
pub struct R(crate::R<LSCH4_HPOINT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LSCH4_HPOINT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LSCH4_HPOINT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LSCH4_HPOINT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LSCH4_HPOINT` writer"]
pub struct W(crate::W<LSCH4_HPOINT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LSCH4_HPOINT_SPEC>;
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
impl From<crate::W<LSCH4_HPOINT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LSCH4_HPOINT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HPOINT_LSCH4` reader - The output value changes to high when lstimerx(x=\\[0 3\\]) selected by low speed channel4 has reached reg_hpoint_lsch4\\[19:0\\]"]
pub struct HPOINT_LSCH4_R(crate::FieldReader<u32, u32>);
impl HPOINT_LSCH4_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        HPOINT_LSCH4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HPOINT_LSCH4_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HPOINT_LSCH4` writer - The output value changes to high when lstimerx(x=\\[0 3\\]) selected by low speed channel4 has reached reg_hpoint_lsch4\\[19:0\\]"]
pub struct HPOINT_LSCH4_W<'a> {
    w: &'a mut W,
}
impl<'a> HPOINT_LSCH4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | (value as u32 & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:19 - The output value changes to high when lstimerx(x=\\[0 3\\]) selected by low speed channel4 has reached reg_hpoint_lsch4\\[19:0\\]"]
    #[inline(always)]
    pub fn hpoint_lsch4(&self) -> HPOINT_LSCH4_R {
        HPOINT_LSCH4_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19 - The output value changes to high when lstimerx(x=\\[0 3\\]) selected by low speed channel4 has reached reg_hpoint_lsch4\\[19:0\\]"]
    #[inline(always)]
    pub fn hpoint_lsch4(&mut self) -> HPOINT_LSCH4_W {
        HPOINT_LSCH4_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lsch4_hpoint](index.html) module"]
pub struct LSCH4_HPOINT_SPEC;
impl crate::RegisterSpec for LSCH4_HPOINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lsch4_hpoint::R](R) reader structure"]
impl crate::Readable for LSCH4_HPOINT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lsch4_hpoint::W](W) writer structure"]
impl crate::Writable for LSCH4_HPOINT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LSCH4_HPOINT to value 0"]
impl crate::Resettable for LSCH4_HPOINT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
