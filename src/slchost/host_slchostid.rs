#[doc = "Register `HOST_SLCHOSTID` reader"]
pub struct R(crate::R<HOST_SLCHOSTID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOST_SLCHOSTID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOST_SLCHOSTID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOST_SLCHOSTID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HOST_SLCHOSTID` writer"]
pub struct W(crate::W<HOST_SLCHOSTID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HOST_SLCHOSTID_SPEC>;
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
impl From<crate::W<HOST_SLCHOSTID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HOST_SLCHOSTID_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HOST_SLCHOST_ID` reader - "]
pub struct HOST_SLCHOST_ID_R(crate::FieldReader<u32, u32>);
impl HOST_SLCHOST_ID_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        HOST_SLCHOST_ID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOST_SLCHOST_ID_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOST_SLCHOST_ID` writer - "]
pub struct HOST_SLCHOST_ID_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_ID_W<'a> {
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
    pub fn host_slchost_id(&self) -> HOST_SLCHOST_ID_R {
        HOST_SLCHOST_ID_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn host_slchost_id(&mut self) -> HOST_SLCHOST_ID_W {
        HOST_SLCHOST_ID_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_slchostid](index.html) module"]
pub struct HOST_SLCHOSTID_SPEC;
impl crate::RegisterSpec for HOST_SLCHOSTID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [host_slchostid::R](R) reader structure"]
impl crate::Readable for HOST_SLCHOSTID_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [host_slchostid::W](W) writer structure"]
impl crate::Writable for HOST_SLCHOSTID_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HOST_SLCHOSTID to value 0x0600"]
impl crate::Resettable for HOST_SLCHOSTID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0600
    }
}
