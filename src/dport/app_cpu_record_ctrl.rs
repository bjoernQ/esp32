#[doc = "Register `APP_CPU_RECORD_CTRL` reader"]
pub struct R(crate::R<APP_CPU_RECORD_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APP_CPU_RECORD_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APP_CPU_RECORD_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APP_CPU_RECORD_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APP_CPU_RECORD_CTRL` writer"]
pub struct W(crate::W<APP_CPU_RECORD_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APP_CPU_RECORD_CTRL_SPEC>;
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
impl From<crate::W<APP_CPU_RECORD_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APP_CPU_RECORD_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `APP_CPU_RECORD_ENABLE` reader - "]
pub struct APP_CPU_RECORD_ENABLE_R(crate::FieldReader<bool, bool>);
impl APP_CPU_RECORD_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APP_CPU_RECORD_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APP_CPU_RECORD_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APP_CPU_RECORD_ENABLE` writer - "]
pub struct APP_CPU_RECORD_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_CPU_RECORD_ENABLE_W<'a> {
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
#[doc = "Field `APP_CPU_RECORD_DISABLE` reader - "]
pub struct APP_CPU_RECORD_DISABLE_R(crate::FieldReader<bool, bool>);
impl APP_CPU_RECORD_DISABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APP_CPU_RECORD_DISABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APP_CPU_RECORD_DISABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APP_CPU_RECORD_DISABLE` writer - "]
pub struct APP_CPU_RECORD_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_CPU_RECORD_DISABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `APP_CPU_PDEBUG_ENABLE` reader - "]
pub struct APP_CPU_PDEBUG_ENABLE_R(crate::FieldReader<bool, bool>);
impl APP_CPU_PDEBUG_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APP_CPU_PDEBUG_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APP_CPU_PDEBUG_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APP_CPU_PDEBUG_ENABLE` writer - "]
pub struct APP_CPU_PDEBUG_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_CPU_PDEBUG_ENABLE_W<'a> {
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
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn app_cpu_record_enable(&self) -> APP_CPU_RECORD_ENABLE_R {
        APP_CPU_RECORD_ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn app_cpu_record_disable(&self) -> APP_CPU_RECORD_DISABLE_R {
        APP_CPU_RECORD_DISABLE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn app_cpu_pdebug_enable(&self) -> APP_CPU_PDEBUG_ENABLE_R {
        APP_CPU_PDEBUG_ENABLE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn app_cpu_record_enable(&mut self) -> APP_CPU_RECORD_ENABLE_W {
        APP_CPU_RECORD_ENABLE_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn app_cpu_record_disable(&mut self) -> APP_CPU_RECORD_DISABLE_W {
        APP_CPU_RECORD_DISABLE_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn app_cpu_pdebug_enable(&mut self) -> APP_CPU_PDEBUG_ENABLE_W {
        APP_CPU_PDEBUG_ENABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [app_cpu_record_ctrl](index.html) module"]
pub struct APP_CPU_RECORD_CTRL_SPEC;
impl crate::RegisterSpec for APP_CPU_RECORD_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [app_cpu_record_ctrl::R](R) reader structure"]
impl crate::Readable for APP_CPU_RECORD_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [app_cpu_record_ctrl::W](W) writer structure"]
impl crate::Writable for APP_CPU_RECORD_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APP_CPU_RECORD_CTRL to value 0x0100"]
impl crate::Resettable for APP_CPU_RECORD_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0100
    }
}
