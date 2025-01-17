#[doc = "Register `HOST_SLC0HOST_TOKEN_WDATA` reader"]
pub struct R(crate::R<HOST_SLC0HOST_TOKEN_WDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOST_SLC0HOST_TOKEN_WDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOST_SLC0HOST_TOKEN_WDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOST_SLC0HOST_TOKEN_WDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HOST_SLC0HOST_TOKEN_WDATA` writer"]
pub struct W(crate::W<HOST_SLC0HOST_TOKEN_WDATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HOST_SLC0HOST_TOKEN_WDATA_SPEC>;
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
impl From<crate::W<HOST_SLC0HOST_TOKEN_WDATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HOST_SLC0HOST_TOKEN_WDATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HOST_SLC0HOST_TOKEN0_WD` reader - "]
pub struct HOST_SLC0HOST_TOKEN0_WD_R(crate::FieldReader<u16, u16>);
impl HOST_SLC0HOST_TOKEN0_WD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        HOST_SLC0HOST_TOKEN0_WD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOST_SLC0HOST_TOKEN0_WD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOST_SLC0HOST_TOKEN0_WD` writer - "]
pub struct HOST_SLC0HOST_TOKEN0_WD_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC0HOST_TOKEN0_WD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
#[doc = "Field `HOST_SLC0HOST_TOKEN1_WD` reader - "]
pub struct HOST_SLC0HOST_TOKEN1_WD_R(crate::FieldReader<u16, u16>);
impl HOST_SLC0HOST_TOKEN1_WD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        HOST_SLC0HOST_TOKEN1_WD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOST_SLC0HOST_TOKEN1_WD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOST_SLC0HOST_TOKEN1_WD` writer - "]
pub struct HOST_SLC0HOST_TOKEN1_WD_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC0HOST_TOKEN1_WD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | ((value as u32 & 0x0fff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn host_slc0host_token0_wd(&self) -> HOST_SLC0HOST_TOKEN0_WD_R {
        HOST_SLC0HOST_TOKEN0_WD_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn host_slc0host_token1_wd(&self) -> HOST_SLC0HOST_TOKEN1_WD_R {
        HOST_SLC0HOST_TOKEN1_WD_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn host_slc0host_token0_wd(&mut self) -> HOST_SLC0HOST_TOKEN0_WD_W {
        HOST_SLC0HOST_TOKEN0_WD_W { w: self }
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn host_slc0host_token1_wd(&mut self) -> HOST_SLC0HOST_TOKEN1_WD_W {
        HOST_SLC0HOST_TOKEN1_WD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_slc0host_token_wdata](index.html) module"]
pub struct HOST_SLC0HOST_TOKEN_WDATA_SPEC;
impl crate::RegisterSpec for HOST_SLC0HOST_TOKEN_WDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [host_slc0host_token_wdata::R](R) reader structure"]
impl crate::Readable for HOST_SLC0HOST_TOKEN_WDATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [host_slc0host_token_wdata::W](W) writer structure"]
impl crate::Writable for HOST_SLC0HOST_TOKEN_WDATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HOST_SLC0HOST_TOKEN_WDATA to value 0"]
impl crate::Resettable for HOST_SLC0HOST_TOKEN_WDATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
