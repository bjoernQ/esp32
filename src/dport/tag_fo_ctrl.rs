#[doc = "Register `TAG_FO_CTRL` reader"]
pub struct R(crate::R<TAG_FO_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAG_FO_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAG_FO_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAG_FO_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TAG_FO_CTRL` writer"]
pub struct W(crate::W<TAG_FO_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TAG_FO_CTRL_SPEC>;
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
impl From<crate::W<TAG_FO_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TAG_FO_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRO_CACHE_TAG_FORCE_ON` reader - "]
pub struct PRO_CACHE_TAG_FORCE_ON_R(crate::FieldReader<bool, bool>);
impl PRO_CACHE_TAG_FORCE_ON_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_CACHE_TAG_FORCE_ON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_CACHE_TAG_FORCE_ON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_CACHE_TAG_FORCE_ON` writer - "]
pub struct PRO_CACHE_TAG_FORCE_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_CACHE_TAG_FORCE_ON_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `PRO_CACHE_TAG_PD` reader - "]
pub struct PRO_CACHE_TAG_PD_R(crate::FieldReader<bool, bool>);
impl PRO_CACHE_TAG_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_CACHE_TAG_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_CACHE_TAG_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_CACHE_TAG_PD` writer - "]
pub struct PRO_CACHE_TAG_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_CACHE_TAG_PD_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `APP_CACHE_TAG_FORCE_ON` reader - "]
pub struct APP_CACHE_TAG_FORCE_ON_R(crate::FieldReader<bool, bool>);
impl APP_CACHE_TAG_FORCE_ON_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APP_CACHE_TAG_FORCE_ON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APP_CACHE_TAG_FORCE_ON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APP_CACHE_TAG_FORCE_ON` writer - "]
pub struct APP_CACHE_TAG_FORCE_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_CACHE_TAG_FORCE_ON_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `APP_CACHE_TAG_PD` reader - "]
pub struct APP_CACHE_TAG_PD_R(crate::FieldReader<bool, bool>);
impl APP_CACHE_TAG_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APP_CACHE_TAG_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APP_CACHE_TAG_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APP_CACHE_TAG_PD` writer - "]
pub struct APP_CACHE_TAG_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_CACHE_TAG_PD_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pro_cache_tag_force_on(&self) -> PRO_CACHE_TAG_FORCE_ON_R {
        PRO_CACHE_TAG_FORCE_ON_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pro_cache_tag_pd(&self) -> PRO_CACHE_TAG_PD_R {
        PRO_CACHE_TAG_PD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn app_cache_tag_force_on(&self) -> APP_CACHE_TAG_FORCE_ON_R {
        APP_CACHE_TAG_FORCE_ON_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn app_cache_tag_pd(&self) -> APP_CACHE_TAG_PD_R {
        APP_CACHE_TAG_PD_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pro_cache_tag_force_on(&mut self) -> PRO_CACHE_TAG_FORCE_ON_W {
        PRO_CACHE_TAG_FORCE_ON_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pro_cache_tag_pd(&mut self) -> PRO_CACHE_TAG_PD_W {
        PRO_CACHE_TAG_PD_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn app_cache_tag_force_on(&mut self) -> APP_CACHE_TAG_FORCE_ON_W {
        APP_CACHE_TAG_FORCE_ON_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn app_cache_tag_pd(&mut self) -> APP_CACHE_TAG_PD_W {
        APP_CACHE_TAG_PD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tag_fo_ctrl](index.html) module"]
pub struct TAG_FO_CTRL_SPEC;
impl crate::RegisterSpec for TAG_FO_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tag_fo_ctrl::R](R) reader structure"]
impl crate::Readable for TAG_FO_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tag_fo_ctrl::W](W) writer structure"]
impl crate::Writable for TAG_FO_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TAG_FO_CTRL to value 0x0101"]
impl crate::Resettable for TAG_FO_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0101
    }
}
