#[doc = "Register `DIG_ISO` reader"]
pub struct R(crate::R<DIG_ISO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIG_ISO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIG_ISO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIG_ISO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIG_ISO` writer"]
pub struct W(crate::W<DIG_ISO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIG_ISO_SPEC>;
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
impl From<crate::W<DIG_ISO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIG_ISO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FORCE_OFF` reader - "]
pub struct FORCE_OFF_R(crate::FieldReader<bool, bool>);
impl FORCE_OFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FORCE_OFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FORCE_OFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FORCE_OFF` writer - "]
pub struct FORCE_OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_OFF_W<'a> {
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
#[doc = "Field `FORCE_ON` reader - "]
pub struct FORCE_ON_R(crate::FieldReader<bool, bool>);
impl FORCE_ON_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FORCE_ON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FORCE_ON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FORCE_ON` writer - "]
pub struct FORCE_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_ON_W<'a> {
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
#[doc = "Field `DG_PAD_AUTOHOLD` reader - read only register to indicate digital pad auto-hold status"]
pub struct DG_PAD_AUTOHOLD_R(crate::FieldReader<bool, bool>);
impl DG_PAD_AUTOHOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DG_PAD_AUTOHOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DG_PAD_AUTOHOLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLR_DG_PAD_AUTOHOLD` writer - wtite only register to clear digital pad auto-hold"]
pub struct CLR_DG_PAD_AUTOHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_DG_PAD_AUTOHOLD_W<'a> {
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
#[doc = "Field `DG_PAD_AUTOHOLD_EN` reader - digital pad enable auto-hold"]
pub struct DG_PAD_AUTOHOLD_EN_R(crate::FieldReader<bool, bool>);
impl DG_PAD_AUTOHOLD_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DG_PAD_AUTOHOLD_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DG_PAD_AUTOHOLD_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DG_PAD_AUTOHOLD_EN` writer - digital pad enable auto-hold"]
pub struct DG_PAD_AUTOHOLD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DG_PAD_AUTOHOLD_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `DG_PAD_FORCE_NOISO` reader - digital pad force no ISO"]
pub struct DG_PAD_FORCE_NOISO_R(crate::FieldReader<bool, bool>);
impl DG_PAD_FORCE_NOISO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DG_PAD_FORCE_NOISO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DG_PAD_FORCE_NOISO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DG_PAD_FORCE_NOISO` writer - digital pad force no ISO"]
pub struct DG_PAD_FORCE_NOISO_W<'a> {
    w: &'a mut W,
}
impl<'a> DG_PAD_FORCE_NOISO_W<'a> {
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
#[doc = "Field `DG_PAD_FORCE_ISO` reader - digital pad force ISO"]
pub struct DG_PAD_FORCE_ISO_R(crate::FieldReader<bool, bool>);
impl DG_PAD_FORCE_ISO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DG_PAD_FORCE_ISO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DG_PAD_FORCE_ISO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DG_PAD_FORCE_ISO` writer - digital pad force ISO"]
pub struct DG_PAD_FORCE_ISO_W<'a> {
    w: &'a mut W,
}
impl<'a> DG_PAD_FORCE_ISO_W<'a> {
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
#[doc = "Field `DG_PAD_FORCE_UNHOLD` reader - digital pad force un-hold"]
pub struct DG_PAD_FORCE_UNHOLD_R(crate::FieldReader<bool, bool>);
impl DG_PAD_FORCE_UNHOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DG_PAD_FORCE_UNHOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DG_PAD_FORCE_UNHOLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DG_PAD_FORCE_UNHOLD` writer - digital pad force un-hold"]
pub struct DG_PAD_FORCE_UNHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> DG_PAD_FORCE_UNHOLD_W<'a> {
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
#[doc = "Field `DG_PAD_FORCE_HOLD` reader - digital pad force hold"]
pub struct DG_PAD_FORCE_HOLD_R(crate::FieldReader<bool, bool>);
impl DG_PAD_FORCE_HOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DG_PAD_FORCE_HOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DG_PAD_FORCE_HOLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DG_PAD_FORCE_HOLD` writer - digital pad force hold"]
pub struct DG_PAD_FORCE_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> DG_PAD_FORCE_HOLD_W<'a> {
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
#[doc = "Field `ROM0_FORCE_ISO` reader - ROM force ISO"]
pub struct ROM0_FORCE_ISO_R(crate::FieldReader<bool, bool>);
impl ROM0_FORCE_ISO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ROM0_FORCE_ISO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROM0_FORCE_ISO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROM0_FORCE_ISO` writer - ROM force ISO"]
pub struct ROM0_FORCE_ISO_W<'a> {
    w: &'a mut W,
}
impl<'a> ROM0_FORCE_ISO_W<'a> {
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
#[doc = "Field `ROM0_FORCE_NOISO` reader - ROM force no ISO"]
pub struct ROM0_FORCE_NOISO_R(crate::FieldReader<bool, bool>);
impl ROM0_FORCE_NOISO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ROM0_FORCE_NOISO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROM0_FORCE_NOISO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROM0_FORCE_NOISO` writer - ROM force no ISO"]
pub struct ROM0_FORCE_NOISO_W<'a> {
    w: &'a mut W,
}
impl<'a> ROM0_FORCE_NOISO_W<'a> {
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
#[doc = "Field `INTER_RAM0_FORCE_ISO` reader - internal SRAM 0 force ISO"]
pub struct INTER_RAM0_FORCE_ISO_R(crate::FieldReader<bool, bool>);
impl INTER_RAM0_FORCE_ISO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTER_RAM0_FORCE_ISO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTER_RAM0_FORCE_ISO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTER_RAM0_FORCE_ISO` writer - internal SRAM 0 force ISO"]
pub struct INTER_RAM0_FORCE_ISO_W<'a> {
    w: &'a mut W,
}
impl<'a> INTER_RAM0_FORCE_ISO_W<'a> {
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
#[doc = "Field `INTER_RAM0_FORCE_NOISO` reader - internal SRAM 0 force no ISO"]
pub struct INTER_RAM0_FORCE_NOISO_R(crate::FieldReader<bool, bool>);
impl INTER_RAM0_FORCE_NOISO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTER_RAM0_FORCE_NOISO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTER_RAM0_FORCE_NOISO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTER_RAM0_FORCE_NOISO` writer - internal SRAM 0 force no ISO"]
pub struct INTER_RAM0_FORCE_NOISO_W<'a> {
    w: &'a mut W,
}
impl<'a> INTER_RAM0_FORCE_NOISO_W<'a> {
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
#[doc = "Field `INTER_RAM1_FORCE_ISO` reader - internal SRAM 1 force ISO"]
pub struct INTER_RAM1_FORCE_ISO_R(crate::FieldReader<bool, bool>);
impl INTER_RAM1_FORCE_ISO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTER_RAM1_FORCE_ISO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTER_RAM1_FORCE_ISO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTER_RAM1_FORCE_ISO` writer - internal SRAM 1 force ISO"]
pub struct INTER_RAM1_FORCE_ISO_W<'a> {
    w: &'a mut W,
}
impl<'a> INTER_RAM1_FORCE_ISO_W<'a> {
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
#[doc = "Field `INTER_RAM1_FORCE_NOISO` reader - internal SRAM 1 force no ISO"]
pub struct INTER_RAM1_FORCE_NOISO_R(crate::FieldReader<bool, bool>);
impl INTER_RAM1_FORCE_NOISO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTER_RAM1_FORCE_NOISO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTER_RAM1_FORCE_NOISO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTER_RAM1_FORCE_NOISO` writer - internal SRAM 1 force no ISO"]
pub struct INTER_RAM1_FORCE_NOISO_W<'a> {
    w: &'a mut W,
}
impl<'a> INTER_RAM1_FORCE_NOISO_W<'a> {
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
#[doc = "Field `INTER_RAM2_FORCE_ISO` reader - internal SRAM 2 force ISO"]
pub struct INTER_RAM2_FORCE_ISO_R(crate::FieldReader<bool, bool>);
impl INTER_RAM2_FORCE_ISO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTER_RAM2_FORCE_ISO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTER_RAM2_FORCE_ISO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTER_RAM2_FORCE_ISO` writer - internal SRAM 2 force ISO"]
pub struct INTER_RAM2_FORCE_ISO_W<'a> {
    w: &'a mut W,
}
impl<'a> INTER_RAM2_FORCE_ISO_W<'a> {
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
#[doc = "Field `INTER_RAM2_FORCE_NOISO` reader - internal SRAM 2 force no ISO"]
pub struct INTER_RAM2_FORCE_NOISO_R(crate::FieldReader<bool, bool>);
impl INTER_RAM2_FORCE_NOISO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTER_RAM2_FORCE_NOISO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTER_RAM2_FORCE_NOISO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTER_RAM2_FORCE_NOISO` writer - internal SRAM 2 force no ISO"]
pub struct INTER_RAM2_FORCE_NOISO_W<'a> {
    w: &'a mut W,
}
impl<'a> INTER_RAM2_FORCE_NOISO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `INTER_RAM3_FORCE_ISO` reader - internal SRAM 3 force ISO"]
pub struct INTER_RAM3_FORCE_ISO_R(crate::FieldReader<bool, bool>);
impl INTER_RAM3_FORCE_ISO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTER_RAM3_FORCE_ISO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTER_RAM3_FORCE_ISO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTER_RAM3_FORCE_ISO` writer - internal SRAM 3 force ISO"]
pub struct INTER_RAM3_FORCE_ISO_W<'a> {
    w: &'a mut W,
}
impl<'a> INTER_RAM3_FORCE_ISO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `INTER_RAM3_FORCE_NOISO` reader - internal SRAM 3 force no ISO"]
pub struct INTER_RAM3_FORCE_NOISO_R(crate::FieldReader<bool, bool>);
impl INTER_RAM3_FORCE_NOISO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTER_RAM3_FORCE_NOISO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTER_RAM3_FORCE_NOISO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTER_RAM3_FORCE_NOISO` writer - internal SRAM 3 force no ISO"]
pub struct INTER_RAM3_FORCE_NOISO_W<'a> {
    w: &'a mut W,
}
impl<'a> INTER_RAM3_FORCE_NOISO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `INTER_RAM4_FORCE_ISO` reader - internal SRAM 4 force ISO"]
pub struct INTER_RAM4_FORCE_ISO_R(crate::FieldReader<bool, bool>);
impl INTER_RAM4_FORCE_ISO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTER_RAM4_FORCE_ISO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTER_RAM4_FORCE_ISO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTER_RAM4_FORCE_ISO` writer - internal SRAM 4 force ISO"]
pub struct INTER_RAM4_FORCE_ISO_W<'a> {
    w: &'a mut W,
}
impl<'a> INTER_RAM4_FORCE_ISO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `INTER_RAM4_FORCE_NOISO` reader - internal SRAM 4 force no ISO"]
pub struct INTER_RAM4_FORCE_NOISO_R(crate::FieldReader<bool, bool>);
impl INTER_RAM4_FORCE_NOISO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTER_RAM4_FORCE_NOISO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTER_RAM4_FORCE_NOISO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTER_RAM4_FORCE_NOISO` writer - internal SRAM 4 force no ISO"]
pub struct INTER_RAM4_FORCE_NOISO_W<'a> {
    w: &'a mut W,
}
impl<'a> INTER_RAM4_FORCE_NOISO_W<'a> {
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
#[doc = "Field `WIFI_FORCE_ISO` reader - wifi force ISO"]
pub struct WIFI_FORCE_ISO_R(crate::FieldReader<bool, bool>);
impl WIFI_FORCE_ISO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WIFI_FORCE_ISO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WIFI_FORCE_ISO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WIFI_FORCE_ISO` writer - wifi force ISO"]
pub struct WIFI_FORCE_ISO_W<'a> {
    w: &'a mut W,
}
impl<'a> WIFI_FORCE_ISO_W<'a> {
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
#[doc = "Field `WIFI_FORCE_NOISO` reader - wifi force no ISO"]
pub struct WIFI_FORCE_NOISO_R(crate::FieldReader<bool, bool>);
impl WIFI_FORCE_NOISO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WIFI_FORCE_NOISO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WIFI_FORCE_NOISO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WIFI_FORCE_NOISO` writer - wifi force no ISO"]
pub struct WIFI_FORCE_NOISO_W<'a> {
    w: &'a mut W,
}
impl<'a> WIFI_FORCE_NOISO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `DG_WRAP_FORCE_ISO` reader - digital core force ISO"]
pub struct DG_WRAP_FORCE_ISO_R(crate::FieldReader<bool, bool>);
impl DG_WRAP_FORCE_ISO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DG_WRAP_FORCE_ISO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DG_WRAP_FORCE_ISO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DG_WRAP_FORCE_ISO` writer - digital core force ISO"]
pub struct DG_WRAP_FORCE_ISO_W<'a> {
    w: &'a mut W,
}
impl<'a> DG_WRAP_FORCE_ISO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `DG_WRAP_FORCE_NOISO` reader - digital core force no ISO"]
pub struct DG_WRAP_FORCE_NOISO_R(crate::FieldReader<bool, bool>);
impl DG_WRAP_FORCE_NOISO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DG_WRAP_FORCE_NOISO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DG_WRAP_FORCE_NOISO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DG_WRAP_FORCE_NOISO` writer - digital core force no ISO"]
pub struct DG_WRAP_FORCE_NOISO_W<'a> {
    w: &'a mut W,
}
impl<'a> DG_WRAP_FORCE_NOISO_W<'a> {
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
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn force_off(&self) -> FORCE_OFF_R {
        FORCE_OFF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn force_on(&self) -> FORCE_ON_R {
        FORCE_ON_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - read only register to indicate digital pad auto-hold status"]
    #[inline(always)]
    pub fn dg_pad_autohold(&self) -> DG_PAD_AUTOHOLD_R {
        DG_PAD_AUTOHOLD_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 11 - digital pad enable auto-hold"]
    #[inline(always)]
    pub fn dg_pad_autohold_en(&self) -> DG_PAD_AUTOHOLD_EN_R {
        DG_PAD_AUTOHOLD_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - digital pad force no ISO"]
    #[inline(always)]
    pub fn dg_pad_force_noiso(&self) -> DG_PAD_FORCE_NOISO_R {
        DG_PAD_FORCE_NOISO_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - digital pad force ISO"]
    #[inline(always)]
    pub fn dg_pad_force_iso(&self) -> DG_PAD_FORCE_ISO_R {
        DG_PAD_FORCE_ISO_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - digital pad force un-hold"]
    #[inline(always)]
    pub fn dg_pad_force_unhold(&self) -> DG_PAD_FORCE_UNHOLD_R {
        DG_PAD_FORCE_UNHOLD_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - digital pad force hold"]
    #[inline(always)]
    pub fn dg_pad_force_hold(&self) -> DG_PAD_FORCE_HOLD_R {
        DG_PAD_FORCE_HOLD_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - ROM force ISO"]
    #[inline(always)]
    pub fn rom0_force_iso(&self) -> ROM0_FORCE_ISO_R {
        ROM0_FORCE_ISO_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - ROM force no ISO"]
    #[inline(always)]
    pub fn rom0_force_noiso(&self) -> ROM0_FORCE_NOISO_R {
        ROM0_FORCE_NOISO_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - internal SRAM 0 force ISO"]
    #[inline(always)]
    pub fn inter_ram0_force_iso(&self) -> INTER_RAM0_FORCE_ISO_R {
        INTER_RAM0_FORCE_ISO_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - internal SRAM 0 force no ISO"]
    #[inline(always)]
    pub fn inter_ram0_force_noiso(&self) -> INTER_RAM0_FORCE_NOISO_R {
        INTER_RAM0_FORCE_NOISO_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - internal SRAM 1 force ISO"]
    #[inline(always)]
    pub fn inter_ram1_force_iso(&self) -> INTER_RAM1_FORCE_ISO_R {
        INTER_RAM1_FORCE_ISO_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - internal SRAM 1 force no ISO"]
    #[inline(always)]
    pub fn inter_ram1_force_noiso(&self) -> INTER_RAM1_FORCE_NOISO_R {
        INTER_RAM1_FORCE_NOISO_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - internal SRAM 2 force ISO"]
    #[inline(always)]
    pub fn inter_ram2_force_iso(&self) -> INTER_RAM2_FORCE_ISO_R {
        INTER_RAM2_FORCE_ISO_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - internal SRAM 2 force no ISO"]
    #[inline(always)]
    pub fn inter_ram2_force_noiso(&self) -> INTER_RAM2_FORCE_NOISO_R {
        INTER_RAM2_FORCE_NOISO_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - internal SRAM 3 force ISO"]
    #[inline(always)]
    pub fn inter_ram3_force_iso(&self) -> INTER_RAM3_FORCE_ISO_R {
        INTER_RAM3_FORCE_ISO_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - internal SRAM 3 force no ISO"]
    #[inline(always)]
    pub fn inter_ram3_force_noiso(&self) -> INTER_RAM3_FORCE_NOISO_R {
        INTER_RAM3_FORCE_NOISO_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - internal SRAM 4 force ISO"]
    #[inline(always)]
    pub fn inter_ram4_force_iso(&self) -> INTER_RAM4_FORCE_ISO_R {
        INTER_RAM4_FORCE_ISO_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - internal SRAM 4 force no ISO"]
    #[inline(always)]
    pub fn inter_ram4_force_noiso(&self) -> INTER_RAM4_FORCE_NOISO_R {
        INTER_RAM4_FORCE_NOISO_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - wifi force ISO"]
    #[inline(always)]
    pub fn wifi_force_iso(&self) -> WIFI_FORCE_ISO_R {
        WIFI_FORCE_ISO_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - wifi force no ISO"]
    #[inline(always)]
    pub fn wifi_force_noiso(&self) -> WIFI_FORCE_NOISO_R {
        WIFI_FORCE_NOISO_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - digital core force ISO"]
    #[inline(always)]
    pub fn dg_wrap_force_iso(&self) -> DG_WRAP_FORCE_ISO_R {
        DG_WRAP_FORCE_ISO_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - digital core force no ISO"]
    #[inline(always)]
    pub fn dg_wrap_force_noiso(&self) -> DG_WRAP_FORCE_NOISO_R {
        DG_WRAP_FORCE_NOISO_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn force_off(&mut self) -> FORCE_OFF_W {
        FORCE_OFF_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn force_on(&mut self) -> FORCE_ON_W {
        FORCE_ON_W { w: self }
    }
    #[doc = "Bit 10 - wtite only register to clear digital pad auto-hold"]
    #[inline(always)]
    pub fn clr_dg_pad_autohold(&mut self) -> CLR_DG_PAD_AUTOHOLD_W {
        CLR_DG_PAD_AUTOHOLD_W { w: self }
    }
    #[doc = "Bit 11 - digital pad enable auto-hold"]
    #[inline(always)]
    pub fn dg_pad_autohold_en(&mut self) -> DG_PAD_AUTOHOLD_EN_W {
        DG_PAD_AUTOHOLD_EN_W { w: self }
    }
    #[doc = "Bit 12 - digital pad force no ISO"]
    #[inline(always)]
    pub fn dg_pad_force_noiso(&mut self) -> DG_PAD_FORCE_NOISO_W {
        DG_PAD_FORCE_NOISO_W { w: self }
    }
    #[doc = "Bit 13 - digital pad force ISO"]
    #[inline(always)]
    pub fn dg_pad_force_iso(&mut self) -> DG_PAD_FORCE_ISO_W {
        DG_PAD_FORCE_ISO_W { w: self }
    }
    #[doc = "Bit 14 - digital pad force un-hold"]
    #[inline(always)]
    pub fn dg_pad_force_unhold(&mut self) -> DG_PAD_FORCE_UNHOLD_W {
        DG_PAD_FORCE_UNHOLD_W { w: self }
    }
    #[doc = "Bit 15 - digital pad force hold"]
    #[inline(always)]
    pub fn dg_pad_force_hold(&mut self) -> DG_PAD_FORCE_HOLD_W {
        DG_PAD_FORCE_HOLD_W { w: self }
    }
    #[doc = "Bit 16 - ROM force ISO"]
    #[inline(always)]
    pub fn rom0_force_iso(&mut self) -> ROM0_FORCE_ISO_W {
        ROM0_FORCE_ISO_W { w: self }
    }
    #[doc = "Bit 17 - ROM force no ISO"]
    #[inline(always)]
    pub fn rom0_force_noiso(&mut self) -> ROM0_FORCE_NOISO_W {
        ROM0_FORCE_NOISO_W { w: self }
    }
    #[doc = "Bit 18 - internal SRAM 0 force ISO"]
    #[inline(always)]
    pub fn inter_ram0_force_iso(&mut self) -> INTER_RAM0_FORCE_ISO_W {
        INTER_RAM0_FORCE_ISO_W { w: self }
    }
    #[doc = "Bit 19 - internal SRAM 0 force no ISO"]
    #[inline(always)]
    pub fn inter_ram0_force_noiso(&mut self) -> INTER_RAM0_FORCE_NOISO_W {
        INTER_RAM0_FORCE_NOISO_W { w: self }
    }
    #[doc = "Bit 20 - internal SRAM 1 force ISO"]
    #[inline(always)]
    pub fn inter_ram1_force_iso(&mut self) -> INTER_RAM1_FORCE_ISO_W {
        INTER_RAM1_FORCE_ISO_W { w: self }
    }
    #[doc = "Bit 21 - internal SRAM 1 force no ISO"]
    #[inline(always)]
    pub fn inter_ram1_force_noiso(&mut self) -> INTER_RAM1_FORCE_NOISO_W {
        INTER_RAM1_FORCE_NOISO_W { w: self }
    }
    #[doc = "Bit 22 - internal SRAM 2 force ISO"]
    #[inline(always)]
    pub fn inter_ram2_force_iso(&mut self) -> INTER_RAM2_FORCE_ISO_W {
        INTER_RAM2_FORCE_ISO_W { w: self }
    }
    #[doc = "Bit 23 - internal SRAM 2 force no ISO"]
    #[inline(always)]
    pub fn inter_ram2_force_noiso(&mut self) -> INTER_RAM2_FORCE_NOISO_W {
        INTER_RAM2_FORCE_NOISO_W { w: self }
    }
    #[doc = "Bit 24 - internal SRAM 3 force ISO"]
    #[inline(always)]
    pub fn inter_ram3_force_iso(&mut self) -> INTER_RAM3_FORCE_ISO_W {
        INTER_RAM3_FORCE_ISO_W { w: self }
    }
    #[doc = "Bit 25 - internal SRAM 3 force no ISO"]
    #[inline(always)]
    pub fn inter_ram3_force_noiso(&mut self) -> INTER_RAM3_FORCE_NOISO_W {
        INTER_RAM3_FORCE_NOISO_W { w: self }
    }
    #[doc = "Bit 26 - internal SRAM 4 force ISO"]
    #[inline(always)]
    pub fn inter_ram4_force_iso(&mut self) -> INTER_RAM4_FORCE_ISO_W {
        INTER_RAM4_FORCE_ISO_W { w: self }
    }
    #[doc = "Bit 27 - internal SRAM 4 force no ISO"]
    #[inline(always)]
    pub fn inter_ram4_force_noiso(&mut self) -> INTER_RAM4_FORCE_NOISO_W {
        INTER_RAM4_FORCE_NOISO_W { w: self }
    }
    #[doc = "Bit 28 - wifi force ISO"]
    #[inline(always)]
    pub fn wifi_force_iso(&mut self) -> WIFI_FORCE_ISO_W {
        WIFI_FORCE_ISO_W { w: self }
    }
    #[doc = "Bit 29 - wifi force no ISO"]
    #[inline(always)]
    pub fn wifi_force_noiso(&mut self) -> WIFI_FORCE_NOISO_W {
        WIFI_FORCE_NOISO_W { w: self }
    }
    #[doc = "Bit 30 - digital core force ISO"]
    #[inline(always)]
    pub fn dg_wrap_force_iso(&mut self) -> DG_WRAP_FORCE_ISO_W {
        DG_WRAP_FORCE_ISO_W { w: self }
    }
    #[doc = "Bit 31 - digital core force no ISO"]
    #[inline(always)]
    pub fn dg_wrap_force_noiso(&mut self) -> DG_WRAP_FORCE_NOISO_W {
        DG_WRAP_FORCE_NOISO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dig_iso](index.html) module"]
pub struct DIG_ISO_SPEC;
impl crate::RegisterSpec for DIG_ISO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dig_iso::R](R) reader structure"]
impl crate::Readable for DIG_ISO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dig_iso::W](W) writer structure"]
impl crate::Writable for DIG_ISO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIG_ISO to value 0xaaaa_5000"]
impl crate::Resettable for DIG_ISO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xaaaa_5000
    }
}
