#[doc = "Register `HOST_SLCHOST_CONF_W4` reader"]
pub struct R(crate::R<HOST_SLCHOST_CONF_W4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOST_SLCHOST_CONF_W4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOST_SLCHOST_CONF_W4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOST_SLCHOST_CONF_W4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HOST_SLCHOST_CONF_W4` writer"]
pub struct W(crate::W<HOST_SLCHOST_CONF_W4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HOST_SLCHOST_CONF_W4_SPEC>;
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
impl From<crate::W<HOST_SLCHOST_CONF_W4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HOST_SLCHOST_CONF_W4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HOST_SLCHOST_CONF16` reader - SLC timeout value"]
pub struct HOST_SLCHOST_CONF16_R(crate::FieldReader<u8, u8>);
impl HOST_SLCHOST_CONF16_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        HOST_SLCHOST_CONF16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOST_SLCHOST_CONF16_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOST_SLCHOST_CONF16` writer - SLC timeout value"]
pub struct HOST_SLCHOST_CONF16_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `HOST_SLCHOST_CONF17` reader - SLC timeout enable"]
pub struct HOST_SLCHOST_CONF17_R(crate::FieldReader<u8, u8>);
impl HOST_SLCHOST_CONF17_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        HOST_SLCHOST_CONF17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOST_SLCHOST_CONF17_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOST_SLCHOST_CONF17` writer - SLC timeout enable"]
pub struct HOST_SLCHOST_CONF17_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF17_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `HOST_SLCHOST_CONF18` reader - "]
pub struct HOST_SLCHOST_CONF18_R(crate::FieldReader<u8, u8>);
impl HOST_SLCHOST_CONF18_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        HOST_SLCHOST_CONF18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOST_SLCHOST_CONF18_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOST_SLCHOST_CONF18` writer - "]
pub struct HOST_SLCHOST_CONF18_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF18_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `HOST_SLCHOST_CONF19` reader - Interrupt to target CPU"]
pub struct HOST_SLCHOST_CONF19_R(crate::FieldReader<u8, u8>);
impl HOST_SLCHOST_CONF19_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        HOST_SLCHOST_CONF19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOST_SLCHOST_CONF19_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOST_SLCHOST_CONF19` writer - Interrupt to target CPU"]
pub struct HOST_SLCHOST_CONF19_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF19_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - SLC timeout value"]
    #[inline(always)]
    pub fn host_slchost_conf16(&self) -> HOST_SLCHOST_CONF16_R {
        HOST_SLCHOST_CONF16_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - SLC timeout enable"]
    #[inline(always)]
    pub fn host_slchost_conf17(&self) -> HOST_SLCHOST_CONF17_R {
        HOST_SLCHOST_CONF17_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn host_slchost_conf18(&self) -> HOST_SLCHOST_CONF18_R {
        HOST_SLCHOST_CONF18_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt to target CPU"]
    #[inline(always)]
    pub fn host_slchost_conf19(&self) -> HOST_SLCHOST_CONF19_R {
        HOST_SLCHOST_CONF19_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SLC timeout value"]
    #[inline(always)]
    pub fn host_slchost_conf16(&mut self) -> HOST_SLCHOST_CONF16_W {
        HOST_SLCHOST_CONF16_W { w: self }
    }
    #[doc = "Bits 8:15 - SLC timeout enable"]
    #[inline(always)]
    pub fn host_slchost_conf17(&mut self) -> HOST_SLCHOST_CONF17_W {
        HOST_SLCHOST_CONF17_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn host_slchost_conf18(&mut self) -> HOST_SLCHOST_CONF18_W {
        HOST_SLCHOST_CONF18_W { w: self }
    }
    #[doc = "Bits 24:31 - Interrupt to target CPU"]
    #[inline(always)]
    pub fn host_slchost_conf19(&mut self) -> HOST_SLCHOST_CONF19_W {
        HOST_SLCHOST_CONF19_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_slchost_conf_w4](index.html) module"]
pub struct HOST_SLCHOST_CONF_W4_SPEC;
impl crate::RegisterSpec for HOST_SLCHOST_CONF_W4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [host_slchost_conf_w4::R](R) reader structure"]
impl crate::Readable for HOST_SLCHOST_CONF_W4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [host_slchost_conf_w4::W](W) writer structure"]
impl crate::Writable for HOST_SLCHOST_CONF_W4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HOST_SLCHOST_CONF_W4 to value 0x01ff"]
impl crate::Resettable for HOST_SLCHOST_CONF_W4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01ff
    }
}
