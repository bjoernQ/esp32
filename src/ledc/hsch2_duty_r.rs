#[doc = "Register `HSCH2_DUTY_R` reader"]
pub struct R(crate::R<HSCH2_DUTY_R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSCH2_DUTY_R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSCH2_DUTY_R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSCH2_DUTY_R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSCH2_DUTY_R` writer"]
pub struct W(crate::W<HSCH2_DUTY_R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSCH2_DUTY_R_SPEC>;
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
impl From<crate::W<HSCH2_DUTY_R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSCH2_DUTY_R_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsch2_duty_r](index.html) module"]
pub struct HSCH2_DUTY_R_SPEC;
impl crate::RegisterSpec for HSCH2_DUTY_R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hsch2_duty_r::R](R) reader structure"]
impl crate::Readable for HSCH2_DUTY_R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hsch2_duty_r::W](W) writer structure"]
impl crate::Writable for HSCH2_DUTY_R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HSCH2_DUTY_R to value 0"]
impl crate::Resettable for HSCH2_DUTY_R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
