#[doc = "Register `HOST_SLCHOST_CONF_W8` reader"]
pub struct R(crate::R<HOST_SLCHOST_CONF_W8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOST_SLCHOST_CONF_W8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOST_SLCHOST_CONF_W8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOST_SLCHOST_CONF_W8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HOST_SLCHOST_CONF_W8` writer"]
pub struct W(crate::W<HOST_SLCHOST_CONF_W8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HOST_SLCHOST_CONF_W8_SPEC>;
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
impl From<crate::W<HOST_SLCHOST_CONF_W8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HOST_SLCHOST_CONF_W8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HOST_SLCHOST_CONF32` reader - "]
pub struct HOST_SLCHOST_CONF32_R(crate::FieldReader<u8, u8>);
impl HOST_SLCHOST_CONF32_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        HOST_SLCHOST_CONF32_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOST_SLCHOST_CONF32_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOST_SLCHOST_CONF32` writer - "]
pub struct HOST_SLCHOST_CONF32_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF32_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `HOST_SLCHOST_CONF33` reader - "]
pub struct HOST_SLCHOST_CONF33_R(crate::FieldReader<u8, u8>);
impl HOST_SLCHOST_CONF33_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        HOST_SLCHOST_CONF33_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOST_SLCHOST_CONF33_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOST_SLCHOST_CONF33` writer - "]
pub struct HOST_SLCHOST_CONF33_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF33_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `HOST_SLCHOST_CONF34` reader - "]
pub struct HOST_SLCHOST_CONF34_R(crate::FieldReader<u8, u8>);
impl HOST_SLCHOST_CONF34_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        HOST_SLCHOST_CONF34_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOST_SLCHOST_CONF34_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOST_SLCHOST_CONF34` writer - "]
pub struct HOST_SLCHOST_CONF34_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF34_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `HOST_SLCHOST_CONF35` reader - "]
pub struct HOST_SLCHOST_CONF35_R(crate::FieldReader<u8, u8>);
impl HOST_SLCHOST_CONF35_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        HOST_SLCHOST_CONF35_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOST_SLCHOST_CONF35_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOST_SLCHOST_CONF35` writer - "]
pub struct HOST_SLCHOST_CONF35_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_CONF35_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn host_slchost_conf32(&self) -> HOST_SLCHOST_CONF32_R {
        HOST_SLCHOST_CONF32_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn host_slchost_conf33(&self) -> HOST_SLCHOST_CONF33_R {
        HOST_SLCHOST_CONF33_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn host_slchost_conf34(&self) -> HOST_SLCHOST_CONF34_R {
        HOST_SLCHOST_CONF34_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn host_slchost_conf35(&self) -> HOST_SLCHOST_CONF35_R {
        HOST_SLCHOST_CONF35_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn host_slchost_conf32(&mut self) -> HOST_SLCHOST_CONF32_W {
        HOST_SLCHOST_CONF32_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn host_slchost_conf33(&mut self) -> HOST_SLCHOST_CONF33_W {
        HOST_SLCHOST_CONF33_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn host_slchost_conf34(&mut self) -> HOST_SLCHOST_CONF34_W {
        HOST_SLCHOST_CONF34_W { w: self }
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn host_slchost_conf35(&mut self) -> HOST_SLCHOST_CONF35_W {
        HOST_SLCHOST_CONF35_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_slchost_conf_w8](index.html) module"]
pub struct HOST_SLCHOST_CONF_W8_SPEC;
impl crate::RegisterSpec for HOST_SLCHOST_CONF_W8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [host_slchost_conf_w8::R](R) reader structure"]
impl crate::Readable for HOST_SLCHOST_CONF_W8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [host_slchost_conf_w8::W](W) writer structure"]
impl crate::Writable for HOST_SLCHOST_CONF_W8_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HOST_SLCHOST_CONF_W8 to value 0"]
impl crate::Resettable for HOST_SLCHOST_CONF_W8_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
