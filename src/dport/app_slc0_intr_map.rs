#[doc = "Register `APP_SLC0_INTR_MAP` reader"]
pub struct R(crate::R<APP_SLC0_INTR_MAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APP_SLC0_INTR_MAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APP_SLC0_INTR_MAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APP_SLC0_INTR_MAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APP_SLC0_INTR_MAP` writer"]
pub struct W(crate::W<APP_SLC0_INTR_MAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APP_SLC0_INTR_MAP_SPEC>;
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
impl From<crate::W<APP_SLC0_INTR_MAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APP_SLC0_INTR_MAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `APP_SLC0_INTR_MAP` reader - "]
pub struct APP_SLC0_INTR_MAP_R(crate::FieldReader<u8, u8>);
impl APP_SLC0_INTR_MAP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        APP_SLC0_INTR_MAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APP_SLC0_INTR_MAP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APP_SLC0_INTR_MAP` writer - "]
pub struct APP_SLC0_INTR_MAP_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_SLC0_INTR_MAP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn app_slc0_intr_map(&self) -> APP_SLC0_INTR_MAP_R {
        APP_SLC0_INTR_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn app_slc0_intr_map(&mut self) -> APP_SLC0_INTR_MAP_W {
        APP_SLC0_INTR_MAP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [app_slc0_intr_map](index.html) module"]
pub struct APP_SLC0_INTR_MAP_SPEC;
impl crate::RegisterSpec for APP_SLC0_INTR_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [app_slc0_intr_map::R](R) reader structure"]
impl crate::Readable for APP_SLC0_INTR_MAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [app_slc0_intr_map::W](W) writer structure"]
impl crate::Writable for APP_SLC0_INTR_MAP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APP_SLC0_INTR_MAP to value 0x10"]
impl crate::Resettable for APP_SLC0_INTR_MAP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
