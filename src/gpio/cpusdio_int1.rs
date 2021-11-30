#[doc = "Register `CPUSDIO_INT1` reader"]
pub struct R(crate::R<CPUSDIO_INT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPUSDIO_INT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPUSDIO_INT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPUSDIO_INT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPUSDIO_INT1` writer"]
pub struct W(crate::W<CPUSDIO_INT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPUSDIO_INT1_SPEC>;
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
impl From<crate::W<CPUSDIO_INT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPUSDIO_INT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDIO_INT_H` reader - SDIO's extent GPIO32~39 interrupt"]
pub struct SDIO_INT_H_R(crate::FieldReader<u8, u8>);
impl SDIO_INT_H_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SDIO_INT_H_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIO_INT_H_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN_PAD_DRIVER` reader - "]
pub struct PIN_PAD_DRIVER_R(crate::FieldReader<bool, bool>);
impl PIN_PAD_DRIVER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIN_PAD_DRIVER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIN_PAD_DRIVER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN_PAD_DRIVER` writer - "]
pub struct PIN_PAD_DRIVER_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN_PAD_DRIVER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `PIN_INT_TYPE` reader - "]
pub struct PIN_INT_TYPE_R(crate::FieldReader<u8, u8>);
impl PIN_INT_TYPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PIN_INT_TYPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIN_INT_TYPE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN_INT_TYPE` writer - "]
pub struct PIN_INT_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN_INT_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 7)) | ((value as u32 & 0x07) << 7);
        self.w
    }
}
#[doc = "Field `PIN_WAKEUP_ENABLE` reader - "]
pub struct PIN_WAKEUP_ENABLE_R(crate::FieldReader<bool, bool>);
impl PIN_WAKEUP_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIN_WAKEUP_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIN_WAKEUP_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN_WAKEUP_ENABLE` writer - "]
pub struct PIN_WAKEUP_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN_WAKEUP_ENABLE_W<'a> {
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
#[doc = "Field `PIN_CONFIG` reader - "]
pub struct PIN_CONFIG_R(crate::FieldReader<u8, u8>);
impl PIN_CONFIG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PIN_CONFIG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIN_CONFIG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN_CONFIG` writer - "]
pub struct PIN_CONFIG_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN_CONFIG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | ((value as u32 & 0x03) << 11);
        self.w
    }
}
#[doc = "Field `PIN_INT_ENA` reader - "]
pub struct PIN_INT_ENA_R(crate::FieldReader<u8, u8>);
impl PIN_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PIN_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIN_INT_ENA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN_INT_ENA` writer - "]
pub struct PIN_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN_INT_ENA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 13)) | ((value as u32 & 0x1f) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - SDIO's extent GPIO32~39 interrupt"]
    #[inline(always)]
    pub fn sdio_int_h(&self) -> SDIO_INT_H_R {
        SDIO_INT_H_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pin_pad_driver(&self) -> PIN_PAD_DRIVER_R {
        PIN_PAD_DRIVER_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 7:9"]
    #[inline(always)]
    pub fn pin_int_type(&self) -> PIN_INT_TYPE_R {
        PIN_INT_TYPE_R::new(((self.bits >> 7) & 0x07) as u8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pin_wakeup_enable(&self) -> PIN_WAKEUP_ENABLE_R {
        PIN_WAKEUP_ENABLE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn pin_config(&self) -> PIN_CONFIG_R {
        PIN_CONFIG_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bits 13:17"]
    #[inline(always)]
    pub fn pin_int_ena(&self) -> PIN_INT_ENA_R {
        PIN_INT_ENA_R::new(((self.bits >> 13) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pin_pad_driver(&mut self) -> PIN_PAD_DRIVER_W {
        PIN_PAD_DRIVER_W { w: self }
    }
    #[doc = "Bits 7:9"]
    #[inline(always)]
    pub fn pin_int_type(&mut self) -> PIN_INT_TYPE_W {
        PIN_INT_TYPE_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pin_wakeup_enable(&mut self) -> PIN_WAKEUP_ENABLE_W {
        PIN_WAKEUP_ENABLE_W { w: self }
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn pin_config(&mut self) -> PIN_CONFIG_W {
        PIN_CONFIG_W { w: self }
    }
    #[doc = "Bits 13:17"]
    #[inline(always)]
    pub fn pin_int_ena(&mut self) -> PIN_INT_ENA_W {
        PIN_INT_ENA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpusdio_int1](index.html) module"]
pub struct CPUSDIO_INT1_SPEC;
impl crate::RegisterSpec for CPUSDIO_INT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpusdio_int1::R](R) reader structure"]
impl crate::Readable for CPUSDIO_INT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpusdio_int1::W](W) writer structure"]
impl crate::Writable for CPUSDIO_INT1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CPUSDIO_INT1 to value 0"]
impl crate::Resettable for CPUSDIO_INT1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
