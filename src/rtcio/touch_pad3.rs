#[doc = "Register `TOUCH_PAD3` reader"]
pub struct R(crate::R<TOUCH_PAD3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TOUCH_PAD3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TOUCH_PAD3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TOUCH_PAD3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TOUCH_PAD3` writer"]
pub struct W(crate::W<TOUCH_PAD3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TOUCH_PAD3_SPEC>;
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
impl From<crate::W<TOUCH_PAD3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TOUCH_PAD3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TO_GPIO` reader - connect the rtc pad input to digital pad input Ó0Ó is availbale.MTDO"]
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
#[doc = "Field `TO_GPIO` writer - connect the rtc pad input to digital pad input Ó0Ó is availbale.MTDO"]
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `FUN_IE` reader - the input enable of the pad"]
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
#[doc = "Field `FUN_IE` writer - the input enable of the pad"]
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `SLP_OE` reader - the output enable of the pad in sleep status"]
pub struct SLP_OE_R(crate::FieldReader<bool, bool>);
impl SLP_OE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLP_OE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLP_OE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLP_OE` writer - the output enable of the pad in sleep status"]
pub struct SLP_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLP_OE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `SLP_IE` reader - the input enable of the pad in sleep status"]
pub struct SLP_IE_R(crate::FieldReader<bool, bool>);
impl SLP_IE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLP_IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLP_IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLP_IE` writer - the input enable of the pad in sleep status"]
pub struct SLP_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLP_IE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `SLP_SEL` reader - the sleep status selection signal of the pad"]
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
#[doc = "Field `SLP_SEL` writer - the sleep status selection signal of the pad"]
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `FUN_SEL` reader - the functional selection signal of the pad"]
pub struct FUN_SEL_R(crate::FieldReader<u8, u8>);
impl FUN_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FUN_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FUN_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FUN_SEL` writer - the functional selection signal of the pad"]
pub struct FUN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FUN_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 17)) | ((value as u32 & 0x03) << 17);
        self.w
    }
}
#[doc = "Field `MUX_SEL` reader - Ò1Ó select the digital function Ó0Óslection the rtc function"]
pub struct MUX_SEL_R(crate::FieldReader<bool, bool>);
impl MUX_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MUX_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUX_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUX_SEL` writer - Ò1Ó select the digital function Ó0Óslection the rtc function"]
pub struct MUX_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_SEL_W<'a> {
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
#[doc = "Field `RUE` reader - the pull up enable of the pad"]
pub struct RUE_R(crate::FieldReader<bool, bool>);
impl RUE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RUE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RUE` writer - the pull up enable of the pad"]
pub struct RUE_W<'a> {
    w: &'a mut W,
}
impl<'a> RUE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `RDE` reader - the pull down enable of the pad"]
pub struct RDE_R(crate::FieldReader<bool, bool>);
impl RDE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDE` writer - the pull down enable of the pad"]
pub struct RDE_W<'a> {
    w: &'a mut W,
}
impl<'a> RDE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `DRV` reader - the driver strength of the pad"]
pub struct DRV_R(crate::FieldReader<u8, u8>);
impl DRV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DRV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DRV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DRV` writer - the driver strength of the pad"]
pub struct DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> DRV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 29)) | ((value as u32 & 0x03) << 29);
        self.w
    }
}
#[doc = "Field `HOLD` reader - hold the current value of the output when setting the hold to Ò1Ó"]
pub struct HOLD_R(crate::FieldReader<bool, bool>);
impl HOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOLD` writer - hold the current value of the output when setting the hold to Ò1Ó"]
pub struct HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> HOLD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 12 - connect the rtc pad input to digital pad input Ó0Ó is availbale.MTDO"]
    #[inline(always)]
    pub fn to_gpio(&self) -> TO_GPIO_R {
        TO_GPIO_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - the input enable of the pad"]
    #[inline(always)]
    pub fn fun_ie(&self) -> FUN_IE_R {
        FUN_IE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - the output enable of the pad in sleep status"]
    #[inline(always)]
    pub fn slp_oe(&self) -> SLP_OE_R {
        SLP_OE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - the input enable of the pad in sleep status"]
    #[inline(always)]
    pub fn slp_ie(&self) -> SLP_IE_R {
        SLP_IE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - the sleep status selection signal of the pad"]
    #[inline(always)]
    pub fn slp_sel(&self) -> SLP_SEL_R {
        SLP_SEL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 17:18 - the functional selection signal of the pad"]
    #[inline(always)]
    pub fn fun_sel(&self) -> FUN_SEL_R {
        FUN_SEL_R::new(((self.bits >> 17) & 0x03) as u8)
    }
    #[doc = "Bit 19 - Ò1Ó select the digital function Ó0Óslection the rtc function"]
    #[inline(always)]
    pub fn mux_sel(&self) -> MUX_SEL_R {
        MUX_SEL_R::new(((self.bits >> 19) & 0x01) != 0)
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
    #[doc = "Bit 27 - the pull up enable of the pad"]
    #[inline(always)]
    pub fn rue(&self) -> RUE_R {
        RUE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - the pull down enable of the pad"]
    #[inline(always)]
    pub fn rde(&self) -> RDE_R {
        RDE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bits 29:30 - the driver strength of the pad"]
    #[inline(always)]
    pub fn drv(&self) -> DRV_R {
        DRV_R::new(((self.bits >> 29) & 0x03) as u8)
    }
    #[doc = "Bit 31 - hold the current value of the output when setting the hold to Ò1Ó"]
    #[inline(always)]
    pub fn hold(&self) -> HOLD_R {
        HOLD_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - connect the rtc pad input to digital pad input Ó0Ó is availbale.MTDO"]
    #[inline(always)]
    pub fn to_gpio(&mut self) -> TO_GPIO_W {
        TO_GPIO_W { w: self }
    }
    #[doc = "Bit 13 - the input enable of the pad"]
    #[inline(always)]
    pub fn fun_ie(&mut self) -> FUN_IE_W {
        FUN_IE_W { w: self }
    }
    #[doc = "Bit 14 - the output enable of the pad in sleep status"]
    #[inline(always)]
    pub fn slp_oe(&mut self) -> SLP_OE_W {
        SLP_OE_W { w: self }
    }
    #[doc = "Bit 15 - the input enable of the pad in sleep status"]
    #[inline(always)]
    pub fn slp_ie(&mut self) -> SLP_IE_W {
        SLP_IE_W { w: self }
    }
    #[doc = "Bit 16 - the sleep status selection signal of the pad"]
    #[inline(always)]
    pub fn slp_sel(&mut self) -> SLP_SEL_W {
        SLP_SEL_W { w: self }
    }
    #[doc = "Bits 17:18 - the functional selection signal of the pad"]
    #[inline(always)]
    pub fn fun_sel(&mut self) -> FUN_SEL_W {
        FUN_SEL_W { w: self }
    }
    #[doc = "Bit 19 - Ò1Ó select the digital function Ó0Óslection the rtc function"]
    #[inline(always)]
    pub fn mux_sel(&mut self) -> MUX_SEL_W {
        MUX_SEL_W { w: self }
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
    #[doc = "Bit 27 - the pull up enable of the pad"]
    #[inline(always)]
    pub fn rue(&mut self) -> RUE_W {
        RUE_W { w: self }
    }
    #[doc = "Bit 28 - the pull down enable of the pad"]
    #[inline(always)]
    pub fn rde(&mut self) -> RDE_W {
        RDE_W { w: self }
    }
    #[doc = "Bits 29:30 - the driver strength of the pad"]
    #[inline(always)]
    pub fn drv(&mut self) -> DRV_W {
        DRV_W { w: self }
    }
    #[doc = "Bit 31 - hold the current value of the output when setting the hold to Ò1Ó"]
    #[inline(always)]
    pub fn hold(&mut self) -> HOLD_W {
        HOLD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [touch_pad3](index.html) module"]
pub struct TOUCH_PAD3_SPEC;
impl crate::RegisterSpec for TOUCH_PAD3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [touch_pad3::R](R) reader structure"]
impl crate::Readable for TOUCH_PAD3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [touch_pad3::W](W) writer structure"]
impl crate::Writable for TOUCH_PAD3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TOUCH_PAD3 to value 0x4a00_0000"]
impl crate::Resettable for TOUCH_PAD3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4a00_0000
    }
}
