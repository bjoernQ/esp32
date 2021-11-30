#[doc = "Register `PRO_DPORT_APB_MASK1` reader"]
pub struct R(crate::R<PRO_DPORT_APB_MASK1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_DPORT_APB_MASK1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_DPORT_APB_MASK1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_DPORT_APB_MASK1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRO_DPORT_APB_MASK1` writer"]
pub struct W(crate::W<PRO_DPORT_APB_MASK1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRO_DPORT_APB_MASK1_SPEC>;
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
impl From<crate::W<PRO_DPORT_APB_MASK1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRO_DPORT_APB_MASK1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRODPORT_APB_MASK1` reader - "]
pub struct PRODPORT_APB_MASK1_R(crate::FieldReader<u32, u32>);
impl PRODPORT_APB_MASK1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PRODPORT_APB_MASK1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRODPORT_APB_MASK1_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRODPORT_APB_MASK1` writer - "]
pub struct PRODPORT_APB_MASK1_W<'a> {
    w: &'a mut W,
}
impl<'a> PRODPORT_APB_MASK1_W<'a> {
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
    pub fn prodport_apb_mask1(&self) -> PRODPORT_APB_MASK1_R {
        PRODPORT_APB_MASK1_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn prodport_apb_mask1(&mut self) -> PRODPORT_APB_MASK1_W {
        PRODPORT_APB_MASK1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_dport_apb_mask1](index.html) module"]
pub struct PRO_DPORT_APB_MASK1_SPEC;
impl crate::RegisterSpec for PRO_DPORT_APB_MASK1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_dport_apb_mask1::R](R) reader structure"]
impl crate::Readable for PRO_DPORT_APB_MASK1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pro_dport_apb_mask1::W](W) writer structure"]
impl crate::Writable for PRO_DPORT_APB_MASK1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRO_DPORT_APB_MASK1 to value 0"]
impl crate::Resettable for PRO_DPORT_APB_MASK1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
