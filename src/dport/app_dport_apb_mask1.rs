#[doc = "Register `APP_DPORT_APB_MASK1` reader"]
pub struct R(crate::R<APP_DPORT_APB_MASK1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APP_DPORT_APB_MASK1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APP_DPORT_APB_MASK1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APP_DPORT_APB_MASK1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APP_DPORT_APB_MASK1` writer"]
pub struct W(crate::W<APP_DPORT_APB_MASK1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APP_DPORT_APB_MASK1_SPEC>;
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
impl From<crate::W<APP_DPORT_APB_MASK1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APP_DPORT_APB_MASK1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `APPDPORT_APB_MASK1` reader - "]
pub struct APPDPORT_APB_MASK1_R(crate::FieldReader<u32, u32>);
impl APPDPORT_APB_MASK1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        APPDPORT_APB_MASK1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APPDPORT_APB_MASK1_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APPDPORT_APB_MASK1` writer - "]
pub struct APPDPORT_APB_MASK1_W<'a> {
    w: &'a mut W,
}
impl<'a> APPDPORT_APB_MASK1_W<'a> {
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
    pub fn appdport_apb_mask1(&self) -> APPDPORT_APB_MASK1_R {
        APPDPORT_APB_MASK1_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn appdport_apb_mask1(&mut self) -> APPDPORT_APB_MASK1_W {
        APPDPORT_APB_MASK1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [app_dport_apb_mask1](index.html) module"]
pub struct APP_DPORT_APB_MASK1_SPEC;
impl crate::RegisterSpec for APP_DPORT_APB_MASK1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [app_dport_apb_mask1::R](R) reader structure"]
impl crate::Readable for APP_DPORT_APB_MASK1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [app_dport_apb_mask1::W](W) writer structure"]
impl crate::Writable for APP_DPORT_APB_MASK1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APP_DPORT_APB_MASK1 to value 0"]
impl crate::Resettable for APP_DPORT_APB_MASK1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
