#[doc = "Register `U2_CONF1` reader"]
pub struct R(crate::R<U2_CONF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<U2_CONF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<U2_CONF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<U2_CONF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `U2_CONF1` writer"]
pub struct W(crate::W<U2_CONF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<U2_CONF1_SPEC>;
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
impl From<crate::W<U2_CONF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<U2_CONF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNT_THRES0_U2` reader - This register is used to configure thres0 value for unit2."]
pub struct CNT_THRES0_U2_R(crate::FieldReader<u16, u16>);
impl CNT_THRES0_U2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CNT_THRES0_U2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNT_THRES0_U2_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNT_THRES0_U2` writer - This register is used to configure thres0 value for unit2."]
pub struct CNT_THRES0_U2_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_THRES0_U2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `CNT_THRES1_U2` reader - This register is used to configure thres1 value for unit2."]
pub struct CNT_THRES1_U2_R(crate::FieldReader<u16, u16>);
impl CNT_THRES1_U2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CNT_THRES1_U2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNT_THRES1_U2_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNT_THRES1_U2` writer - This register is used to configure thres1 value for unit2."]
pub struct CNT_THRES1_U2_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_THRES1_U2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - This register is used to configure thres0 value for unit2."]
    #[inline(always)]
    pub fn cnt_thres0_u2(&self) -> CNT_THRES0_U2_R {
        CNT_THRES0_U2_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - This register is used to configure thres1 value for unit2."]
    #[inline(always)]
    pub fn cnt_thres1_u2(&self) -> CNT_THRES1_U2_R {
        CNT_THRES1_U2_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This register is used to configure thres0 value for unit2."]
    #[inline(always)]
    pub fn cnt_thres0_u2(&mut self) -> CNT_THRES0_U2_W {
        CNT_THRES0_U2_W { w: self }
    }
    #[doc = "Bits 16:31 - This register is used to configure thres1 value for unit2."]
    #[inline(always)]
    pub fn cnt_thres1_u2(&mut self) -> CNT_THRES1_U2_W {
        CNT_THRES1_U2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u2_conf1](index.html) module"]
pub struct U2_CONF1_SPEC;
impl crate::RegisterSpec for U2_CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [u2_conf1::R](R) reader structure"]
impl crate::Readable for U2_CONF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [u2_conf1::W](W) writer structure"]
impl crate::Writable for U2_CONF1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets U2_CONF1 to value 0"]
impl crate::Resettable for U2_CONF1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
