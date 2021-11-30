#[doc = "Register `APP_CACHE_LOCK_1_ADDR` reader"]
pub struct R(crate::R<APP_CACHE_LOCK_1_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APP_CACHE_LOCK_1_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APP_CACHE_LOCK_1_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APP_CACHE_LOCK_1_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APP_CACHE_LOCK_1_ADDR` writer"]
pub struct W(crate::W<APP_CACHE_LOCK_1_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APP_CACHE_LOCK_1_ADDR_SPEC>;
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
impl From<crate::W<APP_CACHE_LOCK_1_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APP_CACHE_LOCK_1_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRE` reader - "]
pub struct PRE_R(crate::FieldReader<u16, u16>);
impl PRE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        PRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRE` writer - "]
pub struct PRE_W<'a> {
    w: &'a mut W,
}
impl<'a> PRE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | (value as u32 & 0x3fff);
        self.w
    }
}
#[doc = "Field `MIN` reader - "]
pub struct MIN_R(crate::FieldReader<u8, u8>);
impl MIN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MIN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MIN` writer - "]
pub struct MIN_W<'a> {
    w: &'a mut W,
}
impl<'a> MIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 14)) | ((value as u32 & 0x0f) << 14);
        self.w
    }
}
#[doc = "Field `MAX` reader - "]
pub struct MAX_R(crate::FieldReader<u8, u8>);
impl MAX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MAX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAX` writer - "]
pub struct MAX_W<'a> {
    w: &'a mut W,
}
impl<'a> MAX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 18)) | ((value as u32 & 0x0f) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn pre(&self) -> PRE_R {
        PRE_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 14:17"]
    #[inline(always)]
    pub fn min(&self) -> MIN_R {
        MIN_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bits 18:21"]
    #[inline(always)]
    pub fn max(&self) -> MAX_R {
        MAX_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn pre(&mut self) -> PRE_W {
        PRE_W { w: self }
    }
    #[doc = "Bits 14:17"]
    #[inline(always)]
    pub fn min(&mut self) -> MIN_W {
        MIN_W { w: self }
    }
    #[doc = "Bits 18:21"]
    #[inline(always)]
    pub fn max(&mut self) -> MAX_W {
        MAX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [app_cache_lock_1_addr](index.html) module"]
pub struct APP_CACHE_LOCK_1_ADDR_SPEC;
impl crate::RegisterSpec for APP_CACHE_LOCK_1_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [app_cache_lock_1_addr::R](R) reader structure"]
impl crate::Readable for APP_CACHE_LOCK_1_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [app_cache_lock_1_addr::W](W) writer structure"]
impl crate::Writable for APP_CACHE_LOCK_1_ADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APP_CACHE_LOCK_1_ADDR to value 0"]
impl crate::Resettable for APP_CACHE_LOCK_1_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
