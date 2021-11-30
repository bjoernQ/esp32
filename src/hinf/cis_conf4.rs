#[doc = "Register `CIS_CONF4` reader"]
pub struct R(crate::R<CIS_CONF4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIS_CONF4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIS_CONF4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIS_CONF4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CIS_CONF4` writer"]
pub struct W(crate::W<CIS_CONF4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CIS_CONF4_SPEC>;
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
impl From<crate::W<CIS_CONF4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CIS_CONF4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CIS_CONF_W4` reader - "]
pub struct CIS_CONF_W4_R(crate::FieldReader<u32, u32>);
impl CIS_CONF_W4_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CIS_CONF_W4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CIS_CONF_W4_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CIS_CONF_W4` writer - "]
pub struct CIS_CONF_W4_W<'a> {
    w: &'a mut W,
}
impl<'a> CIS_CONF_W4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cis_conf_w4(&self) -> CIS_CONF_W4_R {
        CIS_CONF_W4_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cis_conf_w4(&mut self) -> CIS_CONF_W4_W {
        CIS_CONF_W4_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cis_conf4](index.html) module"]
pub struct CIS_CONF4_SPEC;
impl crate::RegisterSpec for CIS_CONF4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cis_conf4::R](R) reader structure"]
impl crate::Readable for CIS_CONF4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cis_conf4::W](W) writer structure"]
impl crate::Writable for CIS_CONF4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CIS_CONF4 to value 0xffff_ffff"]
impl crate::Resettable for CIS_CONF4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
