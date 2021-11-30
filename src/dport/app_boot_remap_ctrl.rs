#[doc = "Register `APP_BOOT_REMAP_CTRL` reader"]
pub struct R(crate::R<APP_BOOT_REMAP_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APP_BOOT_REMAP_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APP_BOOT_REMAP_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APP_BOOT_REMAP_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APP_BOOT_REMAP_CTRL` writer"]
pub struct W(crate::W<APP_BOOT_REMAP_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APP_BOOT_REMAP_CTRL_SPEC>;
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
impl From<crate::W<APP_BOOT_REMAP_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APP_BOOT_REMAP_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `APP_BOOT_REMAP` reader - "]
pub struct APP_BOOT_REMAP_R(crate::FieldReader<bool, bool>);
impl APP_BOOT_REMAP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APP_BOOT_REMAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APP_BOOT_REMAP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APP_BOOT_REMAP` writer - "]
pub struct APP_BOOT_REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_BOOT_REMAP_W<'a> {
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
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn app_boot_remap(&self) -> APP_BOOT_REMAP_R {
        APP_BOOT_REMAP_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn app_boot_remap(&mut self) -> APP_BOOT_REMAP_W {
        APP_BOOT_REMAP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [app_boot_remap_ctrl](index.html) module"]
pub struct APP_BOOT_REMAP_CTRL_SPEC;
impl crate::RegisterSpec for APP_BOOT_REMAP_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [app_boot_remap_ctrl::R](R) reader structure"]
impl crate::Readable for APP_BOOT_REMAP_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [app_boot_remap_ctrl::W](W) writer structure"]
impl crate::Writable for APP_BOOT_REMAP_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APP_BOOT_REMAP_CTRL to value 0"]
impl crate::Resettable for APP_BOOT_REMAP_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
