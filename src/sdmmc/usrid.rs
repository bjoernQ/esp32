#[doc = "Register `USRID` reader"]
pub struct R(crate::R<USRID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USRID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USRID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USRID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USRID` writer"]
pub struct W(crate::W<USRID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USRID_SPEC>;
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
impl From<crate::W<USRID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USRID_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USRID` reader - User identification register, value set by user. Can also be used as a scratchpad register by user."]
pub struct USRID_R(crate::FieldReader<u32, u32>);
impl USRID_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        USRID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USRID_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USRID` writer - User identification register, value set by user. Can also be used as a scratchpad register by user."]
pub struct USRID_W<'a> {
    w: &'a mut W,
}
impl<'a> USRID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - User identification register, value set by user. Can also be used as a scratchpad register by user."]
    #[inline(always)]
    pub fn usrid(&self) -> USRID_R {
        USRID_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - User identification register, value set by user. Can also be used as a scratchpad register by user."]
    #[inline(always)]
    pub fn usrid(&mut self) -> USRID_W {
        USRID_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "User ID (scratchpad) register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usrid](index.html) module"]
pub struct USRID_SPEC;
impl crate::RegisterSpec for USRID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usrid::R](R) reader structure"]
impl crate::Readable for USRID_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usrid::W](W) writer structure"]
impl crate::Writable for USRID_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USRID to value 0"]
impl crate::Resettable for USRID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
