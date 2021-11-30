#[doc = "Register `CFG_DATA16` reader"]
pub struct R(crate::R<CFG_DATA16_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_DATA16_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_DATA16_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_DATA16_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG_DATA16` writer"]
pub struct W(crate::W<CFG_DATA16_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_DATA16_SPEC>;
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
impl From<crate::W<CFG_DATA16_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_DATA16_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USER_ID_FN2` reader - "]
pub struct USER_ID_FN2_R(crate::FieldReader<u16, u16>);
impl USER_ID_FN2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        USER_ID_FN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USER_ID_FN2_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USER_ID_FN2` writer - "]
pub struct USER_ID_FN2_W<'a> {
    w: &'a mut W,
}
impl<'a> USER_ID_FN2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `DEVICE_ID_FN2` reader - "]
pub struct DEVICE_ID_FN2_R(crate::FieldReader<u16, u16>);
impl DEVICE_ID_FN2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        DEVICE_ID_FN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEVICE_ID_FN2_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEVICE_ID_FN2` writer - "]
pub struct DEVICE_ID_FN2_W<'a> {
    w: &'a mut W,
}
impl<'a> DEVICE_ID_FN2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn user_id_fn2(&self) -> USER_ID_FN2_R {
        USER_ID_FN2_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn device_id_fn2(&self) -> DEVICE_ID_FN2_R {
        DEVICE_ID_FN2_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn user_id_fn2(&mut self) -> USER_ID_FN2_W {
        USER_ID_FN2_W { w: self }
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn device_id_fn2(&mut self) -> DEVICE_ID_FN2_W {
        DEVICE_ID_FN2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg_data16](index.html) module"]
pub struct CFG_DATA16_SPEC;
impl crate::RegisterSpec for CFG_DATA16_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg_data16::R](R) reader structure"]
impl crate::Readable for CFG_DATA16_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg_data16::W](W) writer structure"]
impl crate::Writable for CFG_DATA16_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG_DATA16 to value 0x3333_6666"]
impl crate::Resettable for CFG_DATA16_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3333_6666
    }
}
