#[doc = "Register `FIFO_AHB` reader"]
pub struct R(crate::R<FIFO_AHB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFO_AHB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFO_AHB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFO_AHB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFO_AHB` writer"]
pub struct W(crate::W<FIFO_AHB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFO_AHB_SPEC>;
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
impl From<crate::W<FIFO_AHB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFO_AHB_SPEC>) -> Self {
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
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo_ahb](index.html) module"]
pub struct FIFO_AHB_SPEC;
impl crate::RegisterSpec for FIFO_AHB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifo_ahb::R](R) reader structure"]
impl crate::Readable for FIFO_AHB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifo_ahb::W](W) writer structure"]
impl crate::Writable for FIFO_AHB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FIFO_AHB to value 0"]
impl crate::Resettable for FIFO_AHB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
