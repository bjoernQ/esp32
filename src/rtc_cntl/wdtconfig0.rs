#[doc = "Register `WDTCONFIG0` reader"]
pub struct R(crate::R<WDTCONFIG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDTCONFIG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDTCONFIG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDTCONFIG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDTCONFIG0` writer"]
pub struct W(crate::W<WDTCONFIG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDTCONFIG0_SPEC>;
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
impl From<crate::W<WDTCONFIG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDTCONFIG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDT_PAUSE_IN_SLP` reader - pause WDT in sleep"]
pub struct WDT_PAUSE_IN_SLP_R(crate::FieldReader<bool, bool>);
impl WDT_PAUSE_IN_SLP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WDT_PAUSE_IN_SLP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDT_PAUSE_IN_SLP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDT_PAUSE_IN_SLP` writer - pause WDT in sleep"]
pub struct WDT_PAUSE_IN_SLP_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_PAUSE_IN_SLP_W<'a> {
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
#[doc = "Field `WDT_APPCPU_RESET_EN` reader - enable WDT reset APP CPU"]
pub struct WDT_APPCPU_RESET_EN_R(crate::FieldReader<bool, bool>);
impl WDT_APPCPU_RESET_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WDT_APPCPU_RESET_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDT_APPCPU_RESET_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDT_APPCPU_RESET_EN` writer - enable WDT reset APP CPU"]
pub struct WDT_APPCPU_RESET_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_APPCPU_RESET_EN_W<'a> {
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
#[doc = "Field `WDT_PROCPU_RESET_EN` reader - enable WDT reset PRO CPU"]
pub struct WDT_PROCPU_RESET_EN_R(crate::FieldReader<bool, bool>);
impl WDT_PROCPU_RESET_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WDT_PROCPU_RESET_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDT_PROCPU_RESET_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDT_PROCPU_RESET_EN` writer - enable WDT reset PRO CPU"]
pub struct WDT_PROCPU_RESET_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_PROCPU_RESET_EN_W<'a> {
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
#[doc = "Field `WDT_FLASHBOOT_MOD_EN` reader - enable WDT in flash boot"]
pub struct WDT_FLASHBOOT_MOD_EN_R(crate::FieldReader<bool, bool>);
impl WDT_FLASHBOOT_MOD_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WDT_FLASHBOOT_MOD_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDT_FLASHBOOT_MOD_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDT_FLASHBOOT_MOD_EN` writer - enable WDT in flash boot"]
pub struct WDT_FLASHBOOT_MOD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_FLASHBOOT_MOD_EN_W<'a> {
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
#[doc = "Field `WDT_SYS_RESET_LENGTH` reader - system reset counter length"]
pub struct WDT_SYS_RESET_LENGTH_R(crate::FieldReader<u8, u8>);
impl WDT_SYS_RESET_LENGTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WDT_SYS_RESET_LENGTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDT_SYS_RESET_LENGTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDT_SYS_RESET_LENGTH` writer - system reset counter length"]
pub struct WDT_SYS_RESET_LENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_SYS_RESET_LENGTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | ((value as u32 & 0x07) << 11);
        self.w
    }
}
#[doc = "Field `WDT_CPU_RESET_LENGTH` reader - CPU reset counter length"]
pub struct WDT_CPU_RESET_LENGTH_R(crate::FieldReader<u8, u8>);
impl WDT_CPU_RESET_LENGTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WDT_CPU_RESET_LENGTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDT_CPU_RESET_LENGTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDT_CPU_RESET_LENGTH` writer - CPU reset counter length"]
pub struct WDT_CPU_RESET_LENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_CPU_RESET_LENGTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 14)) | ((value as u32 & 0x07) << 14);
        self.w
    }
}
#[doc = "Field `WDT_LEVEL_INT_EN` reader - N/A"]
pub struct WDT_LEVEL_INT_EN_R(crate::FieldReader<bool, bool>);
impl WDT_LEVEL_INT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WDT_LEVEL_INT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDT_LEVEL_INT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDT_LEVEL_INT_EN` writer - N/A"]
pub struct WDT_LEVEL_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_LEVEL_INT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `WDT_EDGE_INT_EN` reader - N/A"]
pub struct WDT_EDGE_INT_EN_R(crate::FieldReader<bool, bool>);
impl WDT_EDGE_INT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WDT_EDGE_INT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDT_EDGE_INT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDT_EDGE_INT_EN` writer - N/A"]
pub struct WDT_EDGE_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_EDGE_INT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `WDT_STG3` reader - 1: interrupt stage en 2: CPU reset stage en 3: system reset stage en 4: RTC reset stage en"]
pub struct WDT_STG3_R(crate::FieldReader<u8, u8>);
impl WDT_STG3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WDT_STG3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDT_STG3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDT_STG3` writer - 1: interrupt stage en 2: CPU reset stage en 3: system reset stage en 4: RTC reset stage en"]
pub struct WDT_STG3_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_STG3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 19)) | ((value as u32 & 0x07) << 19);
        self.w
    }
}
#[doc = "Field `WDT_STG2` reader - 1: interrupt stage en 2: CPU reset stage en 3: system reset stage en 4: RTC reset stage en"]
pub struct WDT_STG2_R(crate::FieldReader<u8, u8>);
impl WDT_STG2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WDT_STG2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDT_STG2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDT_STG2` writer - 1: interrupt stage en 2: CPU reset stage en 3: system reset stage en 4: RTC reset stage en"]
pub struct WDT_STG2_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_STG2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 22)) | ((value as u32 & 0x07) << 22);
        self.w
    }
}
#[doc = "Field `WDT_STG1` reader - 1: interrupt stage en 2: CPU reset stage en 3: system reset stage en 4: RTC reset stage en"]
pub struct WDT_STG1_R(crate::FieldReader<u8, u8>);
impl WDT_STG1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WDT_STG1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDT_STG1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDT_STG1` writer - 1: interrupt stage en 2: CPU reset stage en 3: system reset stage en 4: RTC reset stage en"]
pub struct WDT_STG1_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_STG1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 25)) | ((value as u32 & 0x07) << 25);
        self.w
    }
}
#[doc = "Field `WDT_STG0` reader - 1: interrupt stage en 2: CPU reset stage en 3: system reset stage en 4: RTC reset stage en"]
pub struct WDT_STG0_R(crate::FieldReader<u8, u8>);
impl WDT_STG0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WDT_STG0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDT_STG0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDT_STG0` writer - 1: interrupt stage en 2: CPU reset stage en 3: system reset stage en 4: RTC reset stage en"]
pub struct WDT_STG0_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_STG0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | ((value as u32 & 0x07) << 28);
        self.w
    }
}
#[doc = "Field `WDT_EN` reader - enable RTC WDT"]
pub struct WDT_EN_R(crate::FieldReader<bool, bool>);
impl WDT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WDT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDT_EN` writer - enable RTC WDT"]
pub struct WDT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_EN_W<'a> {
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
    #[doc = "Bit 7 - pause WDT in sleep"]
    #[inline(always)]
    pub fn wdt_pause_in_slp(&self) -> WDT_PAUSE_IN_SLP_R {
        WDT_PAUSE_IN_SLP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - enable WDT reset APP CPU"]
    #[inline(always)]
    pub fn wdt_appcpu_reset_en(&self) -> WDT_APPCPU_RESET_EN_R {
        WDT_APPCPU_RESET_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - enable WDT reset PRO CPU"]
    #[inline(always)]
    pub fn wdt_procpu_reset_en(&self) -> WDT_PROCPU_RESET_EN_R {
        WDT_PROCPU_RESET_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - enable WDT in flash boot"]
    #[inline(always)]
    pub fn wdt_flashboot_mod_en(&self) -> WDT_FLASHBOOT_MOD_EN_R {
        WDT_FLASHBOOT_MOD_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 11:13 - system reset counter length"]
    #[inline(always)]
    pub fn wdt_sys_reset_length(&self) -> WDT_SYS_RESET_LENGTH_R {
        WDT_SYS_RESET_LENGTH_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    #[doc = "Bits 14:16 - CPU reset counter length"]
    #[inline(always)]
    pub fn wdt_cpu_reset_length(&self) -> WDT_CPU_RESET_LENGTH_R {
        WDT_CPU_RESET_LENGTH_R::new(((self.bits >> 14) & 0x07) as u8)
    }
    #[doc = "Bit 17 - N/A"]
    #[inline(always)]
    pub fn wdt_level_int_en(&self) -> WDT_LEVEL_INT_EN_R {
        WDT_LEVEL_INT_EN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - N/A"]
    #[inline(always)]
    pub fn wdt_edge_int_en(&self) -> WDT_EDGE_INT_EN_R {
        WDT_EDGE_INT_EN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 19:21 - 1: interrupt stage en 2: CPU reset stage en 3: system reset stage en 4: RTC reset stage en"]
    #[inline(always)]
    pub fn wdt_stg3(&self) -> WDT_STG3_R {
        WDT_STG3_R::new(((self.bits >> 19) & 0x07) as u8)
    }
    #[doc = "Bits 22:24 - 1: interrupt stage en 2: CPU reset stage en 3: system reset stage en 4: RTC reset stage en"]
    #[inline(always)]
    pub fn wdt_stg2(&self) -> WDT_STG2_R {
        WDT_STG2_R::new(((self.bits >> 22) & 0x07) as u8)
    }
    #[doc = "Bits 25:27 - 1: interrupt stage en 2: CPU reset stage en 3: system reset stage en 4: RTC reset stage en"]
    #[inline(always)]
    pub fn wdt_stg1(&self) -> WDT_STG1_R {
        WDT_STG1_R::new(((self.bits >> 25) & 0x07) as u8)
    }
    #[doc = "Bits 28:30 - 1: interrupt stage en 2: CPU reset stage en 3: system reset stage en 4: RTC reset stage en"]
    #[inline(always)]
    pub fn wdt_stg0(&self) -> WDT_STG0_R {
        WDT_STG0_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    #[doc = "Bit 31 - enable RTC WDT"]
    #[inline(always)]
    pub fn wdt_en(&self) -> WDT_EN_R {
        WDT_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - pause WDT in sleep"]
    #[inline(always)]
    pub fn wdt_pause_in_slp(&mut self) -> WDT_PAUSE_IN_SLP_W {
        WDT_PAUSE_IN_SLP_W { w: self }
    }
    #[doc = "Bit 8 - enable WDT reset APP CPU"]
    #[inline(always)]
    pub fn wdt_appcpu_reset_en(&mut self) -> WDT_APPCPU_RESET_EN_W {
        WDT_APPCPU_RESET_EN_W { w: self }
    }
    #[doc = "Bit 9 - enable WDT reset PRO CPU"]
    #[inline(always)]
    pub fn wdt_procpu_reset_en(&mut self) -> WDT_PROCPU_RESET_EN_W {
        WDT_PROCPU_RESET_EN_W { w: self }
    }
    #[doc = "Bit 10 - enable WDT in flash boot"]
    #[inline(always)]
    pub fn wdt_flashboot_mod_en(&mut self) -> WDT_FLASHBOOT_MOD_EN_W {
        WDT_FLASHBOOT_MOD_EN_W { w: self }
    }
    #[doc = "Bits 11:13 - system reset counter length"]
    #[inline(always)]
    pub fn wdt_sys_reset_length(&mut self) -> WDT_SYS_RESET_LENGTH_W {
        WDT_SYS_RESET_LENGTH_W { w: self }
    }
    #[doc = "Bits 14:16 - CPU reset counter length"]
    #[inline(always)]
    pub fn wdt_cpu_reset_length(&mut self) -> WDT_CPU_RESET_LENGTH_W {
        WDT_CPU_RESET_LENGTH_W { w: self }
    }
    #[doc = "Bit 17 - N/A"]
    #[inline(always)]
    pub fn wdt_level_int_en(&mut self) -> WDT_LEVEL_INT_EN_W {
        WDT_LEVEL_INT_EN_W { w: self }
    }
    #[doc = "Bit 18 - N/A"]
    #[inline(always)]
    pub fn wdt_edge_int_en(&mut self) -> WDT_EDGE_INT_EN_W {
        WDT_EDGE_INT_EN_W { w: self }
    }
    #[doc = "Bits 19:21 - 1: interrupt stage en 2: CPU reset stage en 3: system reset stage en 4: RTC reset stage en"]
    #[inline(always)]
    pub fn wdt_stg3(&mut self) -> WDT_STG3_W {
        WDT_STG3_W { w: self }
    }
    #[doc = "Bits 22:24 - 1: interrupt stage en 2: CPU reset stage en 3: system reset stage en 4: RTC reset stage en"]
    #[inline(always)]
    pub fn wdt_stg2(&mut self) -> WDT_STG2_W {
        WDT_STG2_W { w: self }
    }
    #[doc = "Bits 25:27 - 1: interrupt stage en 2: CPU reset stage en 3: system reset stage en 4: RTC reset stage en"]
    #[inline(always)]
    pub fn wdt_stg1(&mut self) -> WDT_STG1_W {
        WDT_STG1_W { w: self }
    }
    #[doc = "Bits 28:30 - 1: interrupt stage en 2: CPU reset stage en 3: system reset stage en 4: RTC reset stage en"]
    #[inline(always)]
    pub fn wdt_stg0(&mut self) -> WDT_STG0_W {
        WDT_STG0_W { w: self }
    }
    #[doc = "Bit 31 - enable RTC WDT"]
    #[inline(always)]
    pub fn wdt_en(&mut self) -> WDT_EN_W {
        WDT_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtconfig0](index.html) module"]
pub struct WDTCONFIG0_SPEC;
impl crate::RegisterSpec for WDTCONFIG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdtconfig0::R](R) reader structure"]
impl crate::Readable for WDTCONFIG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdtconfig0::W](W) writer structure"]
impl crate::Writable for WDTCONFIG0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WDTCONFIG0 to value 0x4c80"]
impl crate::Resettable for WDTCONFIG0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4c80
    }
}
