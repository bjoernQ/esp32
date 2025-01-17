#[doc = "Register `APPCPU_CTRL_C` reader"]
pub struct R(crate::R<APPCPU_CTRL_C_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APPCPU_CTRL_C_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APPCPU_CTRL_C_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APPCPU_CTRL_C_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APPCPU_CTRL_C` writer"]
pub struct W(crate::W<APPCPU_CTRL_C_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APPCPU_CTRL_C_SPEC>;
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
impl From<crate::W<APPCPU_CTRL_C_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APPCPU_CTRL_C_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `APPCPU_RUNSTALL` reader - "]
pub struct APPCPU_RUNSTALL_R(crate::FieldReader<bool, bool>);
impl APPCPU_RUNSTALL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APPCPU_RUNSTALL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APPCPU_RUNSTALL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APPCPU_RUNSTALL` writer - "]
pub struct APPCPU_RUNSTALL_W<'a> {
    w: &'a mut W,
}
impl<'a> APPCPU_RUNSTALL_W<'a> {
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
    pub fn appcpu_runstall(&self) -> APPCPU_RUNSTALL_R {
        APPCPU_RUNSTALL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn appcpu_runstall(&mut self) -> APPCPU_RUNSTALL_W {
        APPCPU_RUNSTALL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [appcpu_ctrl_c](index.html) module"]
pub struct APPCPU_CTRL_C_SPEC;
impl crate::RegisterSpec for APPCPU_CTRL_C_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [appcpu_ctrl_c::R](R) reader structure"]
impl crate::Readable for APPCPU_CTRL_C_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [appcpu_ctrl_c::W](W) writer structure"]
impl crate::Writable for APPCPU_CTRL_C_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APPCPU_CTRL_C to value 0"]
impl crate::Resettable for APPCPU_CTRL_C_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
