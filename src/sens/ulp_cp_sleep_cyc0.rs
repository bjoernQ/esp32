#[doc = "Register `ULP_CP_SLEEP_CYC0` reader"]
pub struct R(crate::R<ULP_CP_SLEEP_CYC0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ULP_CP_SLEEP_CYC0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ULP_CP_SLEEP_CYC0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ULP_CP_SLEEP_CYC0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ULP_CP_SLEEP_CYC0` writer"]
pub struct W(crate::W<ULP_CP_SLEEP_CYC0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ULP_CP_SLEEP_CYC0_SPEC>;
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
impl From<crate::W<ULP_CP_SLEEP_CYC0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ULP_CP_SLEEP_CYC0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLEEP_CYCLES_S0` reader - sleep cycles for ULP-coprocessor timer"]
pub struct SLEEP_CYCLES_S0_R(crate::FieldReader<u32, u32>);
impl SLEEP_CYCLES_S0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        SLEEP_CYCLES_S0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLEEP_CYCLES_S0_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLEEP_CYCLES_S0` writer - sleep cycles for ULP-coprocessor timer"]
pub struct SLEEP_CYCLES_S0_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEEP_CYCLES_S0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - sleep cycles for ULP-coprocessor timer"]
    #[inline(always)]
    pub fn sleep_cycles_s0(&self) -> SLEEP_CYCLES_S0_R {
        SLEEP_CYCLES_S0_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - sleep cycles for ULP-coprocessor timer"]
    #[inline(always)]
    pub fn sleep_cycles_s0(&mut self) -> SLEEP_CYCLES_S0_W {
        SLEEP_CYCLES_S0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ulp_cp_sleep_cyc0](index.html) module"]
pub struct ULP_CP_SLEEP_CYC0_SPEC;
impl crate::RegisterSpec for ULP_CP_SLEEP_CYC0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ulp_cp_sleep_cyc0::R](R) reader structure"]
impl crate::Readable for ULP_CP_SLEEP_CYC0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ulp_cp_sleep_cyc0::W](W) writer structure"]
impl crate::Writable for ULP_CP_SLEEP_CYC0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ULP_CP_SLEEP_CYC0 to value 0xc8"]
impl crate::Resettable for ULP_CP_SLEEP_CYC0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xc8
    }
}
