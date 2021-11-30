#[doc = "Register `AHB_LITE_MASK` reader"]
pub struct R(crate::R<AHB_LITE_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB_LITE_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB_LITE_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB_LITE_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB_LITE_MASK` writer"]
pub struct W(crate::W<AHB_LITE_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB_LITE_MASK_SPEC>;
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
impl From<crate::W<AHB_LITE_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB_LITE_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRO` reader - "]
pub struct PRO_R(crate::FieldReader<bool, bool>);
impl PRO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO` writer - "]
pub struct PRO_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_W<'a> {
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
#[doc = "Field `APP` reader - "]
pub struct APP_R(crate::FieldReader<bool, bool>);
impl APP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APP` writer - "]
pub struct APP_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_W<'a> {
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
#[doc = "Field `SDIO` reader - "]
pub struct SDIO_R(crate::FieldReader<bool, bool>);
impl SDIO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIO` writer - "]
pub struct SDIO_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_W<'a> {
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
#[doc = "Field `PRODPORT` reader - "]
pub struct PRODPORT_R(crate::FieldReader<bool, bool>);
impl PRODPORT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRODPORT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRODPORT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRODPORT` writer - "]
pub struct PRODPORT_W<'a> {
    w: &'a mut W,
}
impl<'a> PRODPORT_W<'a> {
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
#[doc = "Field `APPDPORT` reader - "]
pub struct APPDPORT_R(crate::FieldReader<bool, bool>);
impl APPDPORT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APPDPORT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APPDPORT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APPDPORT` writer - "]
pub struct APPDPORT_W<'a> {
    w: &'a mut W,
}
impl<'a> APPDPORT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `AHB_LITE_SDHOST_PID` reader - "]
pub struct AHB_LITE_SDHOST_PID_R(crate::FieldReader<u8, u8>);
impl AHB_LITE_SDHOST_PID_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        AHB_LITE_SDHOST_PID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AHB_LITE_SDHOST_PID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AHB_LITE_SDHOST_PID` writer - "]
pub struct AHB_LITE_SDHOST_PID_W<'a> {
    w: &'a mut W,
}
impl<'a> AHB_LITE_SDHOST_PID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | ((value as u32 & 0x07) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pro(&self) -> PRO_R {
        PRO_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn app(&self) -> APP_R {
        APP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn sdio(&self) -> SDIO_R {
        SDIO_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn prodport(&self) -> PRODPORT_R {
        PRODPORT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn appdport(&self) -> APPDPORT_R {
        APPDPORT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 11:13"]
    #[inline(always)]
    pub fn ahb_lite_sdhost_pid(&self) -> AHB_LITE_SDHOST_PID_R {
        AHB_LITE_SDHOST_PID_R::new(((self.bits >> 11) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pro(&mut self) -> PRO_W {
        PRO_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn app(&mut self) -> APP_W {
        APP_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn sdio(&mut self) -> SDIO_W {
        SDIO_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn prodport(&mut self) -> PRODPORT_W {
        PRODPORT_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn appdport(&mut self) -> APPDPORT_W {
        APPDPORT_W { w: self }
    }
    #[doc = "Bits 11:13"]
    #[inline(always)]
    pub fn ahb_lite_sdhost_pid(&mut self) -> AHB_LITE_SDHOST_PID_W {
        AHB_LITE_SDHOST_PID_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb_lite_mask](index.html) module"]
pub struct AHB_LITE_MASK_SPEC;
impl crate::RegisterSpec for AHB_LITE_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb_lite_mask::R](R) reader structure"]
impl crate::Readable for AHB_LITE_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb_lite_mask::W](W) writer structure"]
impl crate::Writable for AHB_LITE_MASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHB_LITE_MASK to value 0"]
impl crate::Resettable for AHB_LITE_MASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
