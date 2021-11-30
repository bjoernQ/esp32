#[doc = "Register `DT0_FED_CFG` reader"]
pub struct R(crate::R<DT0_FED_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DT0_FED_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DT0_FED_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DT0_FED_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DT0_FED_CFG` writer"]
pub struct W(crate::W<DT0_FED_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DT0_FED_CFG_SPEC>;
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
impl From<crate::W<DT0_FED_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DT0_FED_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DT0_FED` reader - "]
pub struct DT0_FED_R(crate::FieldReader<u16, u16>);
impl DT0_FED_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        DT0_FED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DT0_FED_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DT0_FED` writer - "]
pub struct DT0_FED_W<'a> {
    w: &'a mut W,
}
impl<'a> DT0_FED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn dt0_fed(&self) -> DT0_FED_R {
        DT0_FED_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn dt0_fed(&mut self) -> DT0_FED_W {
        DT0_FED_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dt0_fed_cfg](index.html) module"]
pub struct DT0_FED_CFG_SPEC;
impl crate::RegisterSpec for DT0_FED_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dt0_fed_cfg::R](R) reader structure"]
impl crate::Readable for DT0_FED_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dt0_fed_cfg::W](W) writer structure"]
impl crate::Writable for DT0_FED_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DT0_FED_CFG to value 0"]
impl crate::Resettable for DT0_FED_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
