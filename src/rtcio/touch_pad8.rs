#[doc = "Register `TOUCH_PAD8` reader"]
pub struct R(crate::R<TOUCH_PAD8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TOUCH_PAD8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TOUCH_PAD8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TOUCH_PAD8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TOUCH_PAD8` writer"]
pub struct W(crate::W<TOUCH_PAD8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TOUCH_PAD8_SPEC>;
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
impl From<crate::W<TOUCH_PAD8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TOUCH_PAD8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TO_GPIO` reader - connect the rtc pad input to digital pad input Ó0Ó is availbale"]
pub struct TO_GPIO_R(crate::FieldReader<bool, bool>);
impl TO_GPIO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TO_GPIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TO_GPIO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TO_GPIO` writer - connect the rtc pad input to digital pad input Ó0Ó is availbale"]
pub struct TO_GPIO_W<'a> {
    w: &'a mut W,
}
impl<'a> TO_GPIO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `XPD` reader - touch sensor power on."]
pub struct XPD_R(crate::FieldReader<bool, bool>);
impl XPD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XPD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XPD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XPD` writer - touch sensor power on."]
pub struct XPD_W<'a> {
    w: &'a mut W,
}
impl<'a> XPD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `TIE_OPT` reader - default touch sensor tie option. 0: tie low 1: tie high."]
pub struct TIE_OPT_R(crate::FieldReader<bool, bool>);
impl TIE_OPT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIE_OPT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIE_OPT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIE_OPT` writer - default touch sensor tie option. 0: tie low 1: tie high."]
pub struct TIE_OPT_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE_OPT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `START` reader - start touch sensor."]
pub struct START_R(crate::FieldReader<bool, bool>);
impl START_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for START_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `START` writer - start touch sensor."]
pub struct START_W<'a> {
    w: &'a mut W,
}
impl<'a> START_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `DAC` reader - touch sensor slope control. 3-bit for each touch panel default 100."]
pub struct DAC_R(crate::FieldReader<u8, u8>);
impl DAC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DAC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAC` writer - touch sensor slope control. 3-bit for each touch panel default 100."]
pub struct DAC_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 23)) | ((value as u32 & 0x07) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bit 19 - connect the rtc pad input to digital pad input Ó0Ó is availbale"]
    #[inline(always)]
    pub fn to_gpio(&self) -> TO_GPIO_R {
        TO_GPIO_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - touch sensor power on."]
    #[inline(always)]
    pub fn xpd(&self) -> XPD_R {
        XPD_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - default touch sensor tie option. 0: tie low 1: tie high."]
    #[inline(always)]
    pub fn tie_opt(&self) -> TIE_OPT_R {
        TIE_OPT_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - start touch sensor."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 23:25 - touch sensor slope control. 3-bit for each touch panel default 100."]
    #[inline(always)]
    pub fn dac(&self) -> DAC_R {
        DAC_R::new(((self.bits >> 23) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 19 - connect the rtc pad input to digital pad input Ó0Ó is availbale"]
    #[inline(always)]
    pub fn to_gpio(&mut self) -> TO_GPIO_W {
        TO_GPIO_W { w: self }
    }
    #[doc = "Bit 20 - touch sensor power on."]
    #[inline(always)]
    pub fn xpd(&mut self) -> XPD_W {
        XPD_W { w: self }
    }
    #[doc = "Bit 21 - default touch sensor tie option. 0: tie low 1: tie high."]
    #[inline(always)]
    pub fn tie_opt(&mut self) -> TIE_OPT_W {
        TIE_OPT_W { w: self }
    }
    #[doc = "Bit 22 - start touch sensor."]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
    #[doc = "Bits 23:25 - touch sensor slope control. 3-bit for each touch panel default 100."]
    #[inline(always)]
    pub fn dac(&mut self) -> DAC_W {
        DAC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [touch_pad8](index.html) module"]
pub struct TOUCH_PAD8_SPEC;
impl crate::RegisterSpec for TOUCH_PAD8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [touch_pad8::R](R) reader structure"]
impl crate::Readable for TOUCH_PAD8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [touch_pad8::W](W) writer structure"]
impl crate::Writable for TOUCH_PAD8_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TOUCH_PAD8 to value 0x0200_0000"]
impl crate::Resettable for TOUCH_PAD8_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200_0000
    }
}
