#[doc = "Register `GPIO11` reader"]
pub struct R(crate::R<GPIO11_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO11_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO11_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO11_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO11` writer"]
pub struct W(crate::W<GPIO11_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO11_SPEC>;
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
impl From<crate::W<GPIO11_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO11_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MCU_OE` reader - Output enable of the pad in sleep mode. 1: enable output; 0: disable output."]
pub struct MCU_OE_R(crate::FieldReader<bool, bool>);
impl MCU_OE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MCU_OE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCU_OE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCU_OE` writer - Output enable of the pad in sleep mode. 1: enable output; 0: disable output."]
pub struct MCU_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> MCU_OE_W<'a> {
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
#[doc = "Field `SLP_SEL` reader - Sleep mode selection of this pad. Set to 1 to put the pad in sleep mode."]
pub struct SLP_SEL_R(crate::FieldReader<bool, bool>);
impl SLP_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLP_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLP_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLP_SEL` writer - Sleep mode selection of this pad. Set to 1 to put the pad in sleep mode."]
pub struct SLP_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SLP_SEL_W<'a> {
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
#[doc = "Field `MCU_WPD` reader - Pull-down enable of the pad during sleep mode. 1: internal pull-down enabled; 0: internal pull-down disabled."]
pub struct MCU_WPD_R(crate::FieldReader<bool, bool>);
impl MCU_WPD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MCU_WPD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCU_WPD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCU_WPD` writer - Pull-down enable of the pad during sleep mode. 1: internal pull-down enabled; 0: internal pull-down disabled."]
pub struct MCU_WPD_W<'a> {
    w: &'a mut W,
}
impl<'a> MCU_WPD_W<'a> {
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
#[doc = "Field `MCU_IE` reader - Input enable of the pad during sleep mode. 1: input enabled; 0: input disabled."]
pub struct MCU_IE_R(crate::FieldReader<bool, bool>);
impl MCU_IE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MCU_IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCU_IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCU_IE` writer - Input enable of the pad during sleep mode. 1: input enabled; 0: input disabled."]
pub struct MCU_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> MCU_IE_W<'a> {
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
#[doc = "Field `MCU_DRV` reader - Select the drive strength of the pad during sleep mode. A higher value corresponds with a higher strength."]
pub struct MCU_DRV_R(crate::FieldReader<u8, u8>);
impl MCU_DRV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MCU_DRV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCU_DRV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCU_DRV` writer - Select the drive strength of the pad during sleep mode. A higher value corresponds with a higher strength."]
pub struct MCU_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> MCU_DRV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | ((value as u32 & 0x03) << 5);
        self.w
    }
}
#[doc = "Field `FUN_WPD` reader - Pull-down enable of the pad. 1: internal pull-down enabled, 0: internal pull-down disabled. GPIO pins 34-39 are input-only. These pins do not feature an output driver or internal pull- up/pull-down circuitry, therefore, their FUN_WPD is always 0."]
pub struct FUN_WPD_R(crate::FieldReader<bool, bool>);
impl FUN_WPD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FUN_WPD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FUN_WPD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FUN_WPD` writer - Pull-down enable of the pad. 1: internal pull-down enabled, 0: internal pull-down disabled. GPIO pins 34-39 are input-only. These pins do not feature an output driver or internal pull- up/pull-down circuitry, therefore, their FUN_WPD is always 0."]
pub struct FUN_WPD_W<'a> {
    w: &'a mut W,
}
impl<'a> FUN_WPD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `FUN_WPU` reader - Pull-up enable of the pad. 1: internal pull-up enabled; 0: internal pull-up disabled. GPIO pins 34-39 are input-only. These pins do not feature an output driver or internal pull- up/pull- down circuitry, therefore, their FUN_WPU is always 0."]
pub struct FUN_WPU_R(crate::FieldReader<bool, bool>);
impl FUN_WPU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FUN_WPU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FUN_WPU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FUN_WPU` writer - Pull-up enable of the pad. 1: internal pull-up enabled; 0: internal pull-up disabled. GPIO pins 34-39 are input-only. These pins do not feature an output driver or internal pull- up/pull- down circuitry, therefore, their FUN_WPU is always 0."]
pub struct FUN_WPU_W<'a> {
    w: &'a mut W,
}
impl<'a> FUN_WPU_W<'a> {
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
#[doc = "Field `FUN_IE` reader - Input enable of the pad. 1: input enabled; 0: input disabled."]
pub struct FUN_IE_R(crate::FieldReader<bool, bool>);
impl FUN_IE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FUN_IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FUN_IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FUN_IE` writer - Input enable of the pad. 1: input enabled; 0: input disabled."]
pub struct FUN_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> FUN_IE_W<'a> {
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
#[doc = "Field `FUN_DRV` reader - Select the drive strength of the pad. A higher value corresponds with a higher strength. For GPIO34-39, FUN_DRV is always 0. For detailed drive strength, please see note 8 in Table ”Notes on ESP32 Pin Lists”, in ESP32 Datasheet."]
pub struct FUN_DRV_R(crate::FieldReader<u8, u8>);
impl FUN_DRV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FUN_DRV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FUN_DRV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FUN_DRV` writer - Select the drive strength of the pad. A higher value corresponds with a higher strength. For GPIO34-39, FUN_DRV is always 0. For detailed drive strength, please see note 8 in Table ”Notes on ESP32 Pin Lists”, in ESP32 Datasheet."]
pub struct FUN_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> FUN_DRV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Field `MCU_SEL` reader - Select the IO_MUX function for this signal. 0 selects Function 0, 1 selects Function 1, etc."]
pub struct MCU_SEL_R(crate::FieldReader<u8, u8>);
impl MCU_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MCU_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCU_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCU_SEL` writer - Select the IO_MUX function for this signal. 0 selects Function 0, 1 selects Function 1, etc."]
pub struct MCU_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MCU_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Output enable of the pad in sleep mode. 1: enable output; 0: disable output."]
    #[inline(always)]
    pub fn mcu_oe(&self) -> MCU_OE_R {
        MCU_OE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Sleep mode selection of this pad. Set to 1 to put the pad in sleep mode."]
    #[inline(always)]
    pub fn slp_sel(&self) -> SLP_SEL_R {
        SLP_SEL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Pull-down enable of the pad during sleep mode. 1: internal pull-down enabled; 0: internal pull-down disabled."]
    #[inline(always)]
    pub fn mcu_wpd(&self) -> MCU_WPD_R {
        MCU_WPD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Input enable of the pad during sleep mode. 1: input enabled; 0: input disabled."]
    #[inline(always)]
    pub fn mcu_ie(&self) -> MCU_IE_R {
        MCU_IE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - Select the drive strength of the pad during sleep mode. A higher value corresponds with a higher strength."]
    #[inline(always)]
    pub fn mcu_drv(&self) -> MCU_DRV_R {
        MCU_DRV_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Pull-down enable of the pad. 1: internal pull-down enabled, 0: internal pull-down disabled. GPIO pins 34-39 are input-only. These pins do not feature an output driver or internal pull- up/pull-down circuitry, therefore, their FUN_WPD is always 0."]
    #[inline(always)]
    pub fn fun_wpd(&self) -> FUN_WPD_R {
        FUN_WPD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pull-up enable of the pad. 1: internal pull-up enabled; 0: internal pull-up disabled. GPIO pins 34-39 are input-only. These pins do not feature an output driver or internal pull- up/pull- down circuitry, therefore, their FUN_WPU is always 0."]
    #[inline(always)]
    pub fn fun_wpu(&self) -> FUN_WPU_R {
        FUN_WPU_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Input enable of the pad. 1: input enabled; 0: input disabled."]
    #[inline(always)]
    pub fn fun_ie(&self) -> FUN_IE_R {
        FUN_IE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 10:11 - Select the drive strength of the pad. A higher value corresponds with a higher strength. For GPIO34-39, FUN_DRV is always 0. For detailed drive strength, please see note 8 in Table ”Notes on ESP32 Pin Lists”, in ESP32 Datasheet."]
    #[inline(always)]
    pub fn fun_drv(&self) -> FUN_DRV_R {
        FUN_DRV_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:14 - Select the IO_MUX function for this signal. 0 selects Function 0, 1 selects Function 1, etc."]
    #[inline(always)]
    pub fn mcu_sel(&self) -> MCU_SEL_R {
        MCU_SEL_R::new(((self.bits >> 12) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Output enable of the pad in sleep mode. 1: enable output; 0: disable output."]
    #[inline(always)]
    pub fn mcu_oe(&mut self) -> MCU_OE_W {
        MCU_OE_W { w: self }
    }
    #[doc = "Bit 1 - Sleep mode selection of this pad. Set to 1 to put the pad in sleep mode."]
    #[inline(always)]
    pub fn slp_sel(&mut self) -> SLP_SEL_W {
        SLP_SEL_W { w: self }
    }
    #[doc = "Bit 2 - Pull-down enable of the pad during sleep mode. 1: internal pull-down enabled; 0: internal pull-down disabled."]
    #[inline(always)]
    pub fn mcu_wpd(&mut self) -> MCU_WPD_W {
        MCU_WPD_W { w: self }
    }
    #[doc = "Bit 4 - Input enable of the pad during sleep mode. 1: input enabled; 0: input disabled."]
    #[inline(always)]
    pub fn mcu_ie(&mut self) -> MCU_IE_W {
        MCU_IE_W { w: self }
    }
    #[doc = "Bits 5:6 - Select the drive strength of the pad during sleep mode. A higher value corresponds with a higher strength."]
    #[inline(always)]
    pub fn mcu_drv(&mut self) -> MCU_DRV_W {
        MCU_DRV_W { w: self }
    }
    #[doc = "Bit 7 - Pull-down enable of the pad. 1: internal pull-down enabled, 0: internal pull-down disabled. GPIO pins 34-39 are input-only. These pins do not feature an output driver or internal pull- up/pull-down circuitry, therefore, their FUN_WPD is always 0."]
    #[inline(always)]
    pub fn fun_wpd(&mut self) -> FUN_WPD_W {
        FUN_WPD_W { w: self }
    }
    #[doc = "Bit 8 - Pull-up enable of the pad. 1: internal pull-up enabled; 0: internal pull-up disabled. GPIO pins 34-39 are input-only. These pins do not feature an output driver or internal pull- up/pull- down circuitry, therefore, their FUN_WPU is always 0."]
    #[inline(always)]
    pub fn fun_wpu(&mut self) -> FUN_WPU_W {
        FUN_WPU_W { w: self }
    }
    #[doc = "Bit 9 - Input enable of the pad. 1: input enabled; 0: input disabled."]
    #[inline(always)]
    pub fn fun_ie(&mut self) -> FUN_IE_W {
        FUN_IE_W { w: self }
    }
    #[doc = "Bits 10:11 - Select the drive strength of the pad. A higher value corresponds with a higher strength. For GPIO34-39, FUN_DRV is always 0. For detailed drive strength, please see note 8 in Table ”Notes on ESP32 Pin Lists”, in ESP32 Datasheet."]
    #[inline(always)]
    pub fn fun_drv(&mut self) -> FUN_DRV_W {
        FUN_DRV_W { w: self }
    }
    #[doc = "Bits 12:14 - Select the IO_MUX function for this signal. 0 selects Function 0, 1 selects Function 1, etc."]
    #[inline(always)]
    pub fn mcu_sel(&mut self) -> MCU_SEL_W {
        MCU_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio11](index.html) module"]
pub struct GPIO11_SPEC;
impl crate::RegisterSpec for GPIO11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio11::R](R) reader structure"]
impl crate::Readable for GPIO11_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio11::W](W) writer structure"]
impl crate::Writable for GPIO11_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIO11 to value 0"]
impl crate::Resettable for GPIO11_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
