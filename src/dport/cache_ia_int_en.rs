#[doc = "Register `CACHE_IA_INT_EN` reader"]
pub struct R(crate::R<CACHE_IA_INT_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACHE_IA_INT_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACHE_IA_INT_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACHE_IA_INT_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CACHE_IA_INT_EN` writer"]
pub struct W(crate::W<CACHE_IA_INT_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CACHE_IA_INT_EN_SPEC>;
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
impl From<crate::W<CACHE_IA_INT_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CACHE_IA_INT_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CACHE_IA_INT_EN` reader - Interrupt enable bits for various invalid cache access reasons"]
pub struct CACHE_IA_INT_EN_R(crate::FieldReader<u32, u32>);
impl CACHE_IA_INT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CACHE_IA_INT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CACHE_IA_INT_EN_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CACHE_IA_INT_EN` writer - Interrupt enable bits for various invalid cache access reasons"]
pub struct CACHE_IA_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHE_IA_INT_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff_ffff) | (value as u32 & 0x0fff_ffff);
        self.w
    }
}
#[doc = "Field `CACHE_IA_INT_APP_DROM0` reader - APP CPU invalid access to DROM0 when cache is disabled"]
pub struct CACHE_IA_INT_APP_DROM0_R(crate::FieldReader<bool, bool>);
impl CACHE_IA_INT_APP_DROM0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CACHE_IA_INT_APP_DROM0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CACHE_IA_INT_APP_DROM0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CACHE_IA_INT_APP_DROM0` writer - APP CPU invalid access to DROM0 when cache is disabled"]
pub struct CACHE_IA_INT_APP_DROM0_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHE_IA_INT_APP_DROM0_W<'a> {
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
#[doc = "Field `CACHE_IA_INT_APP_IRAM0` reader - APP CPU invalid access to IRAM0 when cache is disabled"]
pub struct CACHE_IA_INT_APP_IRAM0_R(crate::FieldReader<bool, bool>);
impl CACHE_IA_INT_APP_IRAM0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CACHE_IA_INT_APP_IRAM0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CACHE_IA_INT_APP_IRAM0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CACHE_IA_INT_APP_IRAM0` writer - APP CPU invalid access to IRAM0 when cache is disabled"]
pub struct CACHE_IA_INT_APP_IRAM0_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHE_IA_INT_APP_IRAM0_W<'a> {
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
#[doc = "Field `CACHE_IA_INT_APP_IRAM1` reader - APP CPU invalid access to IRAM1 when cache is disabled"]
pub struct CACHE_IA_INT_APP_IRAM1_R(crate::FieldReader<bool, bool>);
impl CACHE_IA_INT_APP_IRAM1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CACHE_IA_INT_APP_IRAM1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CACHE_IA_INT_APP_IRAM1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CACHE_IA_INT_APP_IRAM1` writer - APP CPU invalid access to IRAM1 when cache is disabled"]
pub struct CACHE_IA_INT_APP_IRAM1_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHE_IA_INT_APP_IRAM1_W<'a> {
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
#[doc = "Field `CACHE_IA_INT_APP_IROM0` reader - APP CPU invalid access to IROM0 when cache is disabled"]
pub struct CACHE_IA_INT_APP_IROM0_R(crate::FieldReader<bool, bool>);
impl CACHE_IA_INT_APP_IROM0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CACHE_IA_INT_APP_IROM0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CACHE_IA_INT_APP_IROM0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CACHE_IA_INT_APP_IROM0` writer - APP CPU invalid access to IROM0 when cache is disabled"]
pub struct CACHE_IA_INT_APP_IROM0_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHE_IA_INT_APP_IROM0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `CACHE_IA_INT_APP_DRAM1` reader - APP CPU invalid access to DRAM1 when cache is disabled"]
pub struct CACHE_IA_INT_APP_DRAM1_R(crate::FieldReader<bool, bool>);
impl CACHE_IA_INT_APP_DRAM1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CACHE_IA_INT_APP_DRAM1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CACHE_IA_INT_APP_DRAM1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CACHE_IA_INT_APP_DRAM1` writer - APP CPU invalid access to DRAM1 when cache is disabled"]
pub struct CACHE_IA_INT_APP_DRAM1_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHE_IA_INT_APP_DRAM1_W<'a> {
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
#[doc = "Field `CACHE_IA_INT_APP_OPPOSITE` reader - APP CPU invalid access to APP CPU cache when cache disabled"]
pub struct CACHE_IA_INT_APP_OPPOSITE_R(crate::FieldReader<bool, bool>);
impl CACHE_IA_INT_APP_OPPOSITE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CACHE_IA_INT_APP_OPPOSITE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CACHE_IA_INT_APP_OPPOSITE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CACHE_IA_INT_APP_OPPOSITE` writer - APP CPU invalid access to APP CPU cache when cache disabled"]
pub struct CACHE_IA_INT_APP_OPPOSITE_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHE_IA_INT_APP_OPPOSITE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `CACHE_IA_INT_PRO_DROM0` reader - PRO CPU invalid access to DROM0 when cache is disabled"]
pub struct CACHE_IA_INT_PRO_DROM0_R(crate::FieldReader<bool, bool>);
impl CACHE_IA_INT_PRO_DROM0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CACHE_IA_INT_PRO_DROM0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CACHE_IA_INT_PRO_DROM0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CACHE_IA_INT_PRO_DROM0` writer - PRO CPU invalid access to DROM0 when cache is disabled"]
pub struct CACHE_IA_INT_PRO_DROM0_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHE_IA_INT_PRO_DROM0_W<'a> {
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
#[doc = "Field `CACHE_IA_INT_PRO_IRAM0` reader - PRO CPU invalid access to IRAM0 when cache is disabled"]
pub struct CACHE_IA_INT_PRO_IRAM0_R(crate::FieldReader<bool, bool>);
impl CACHE_IA_INT_PRO_IRAM0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CACHE_IA_INT_PRO_IRAM0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CACHE_IA_INT_PRO_IRAM0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CACHE_IA_INT_PRO_IRAM0` writer - PRO CPU invalid access to IRAM0 when cache is disabled"]
pub struct CACHE_IA_INT_PRO_IRAM0_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHE_IA_INT_PRO_IRAM0_W<'a> {
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
#[doc = "Field `CACHE_IA_INT_PRO_IRAM1` reader - PRO CPU invalid access to IRAM1 when cache is disabled"]
pub struct CACHE_IA_INT_PRO_IRAM1_R(crate::FieldReader<bool, bool>);
impl CACHE_IA_INT_PRO_IRAM1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CACHE_IA_INT_PRO_IRAM1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CACHE_IA_INT_PRO_IRAM1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CACHE_IA_INT_PRO_IRAM1` writer - PRO CPU invalid access to IRAM1 when cache is disabled"]
pub struct CACHE_IA_INT_PRO_IRAM1_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHE_IA_INT_PRO_IRAM1_W<'a> {
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
#[doc = "Field `CACHE_IA_INT_PRO_IROM0` reader - PRO CPU invalid access to IROM0 when cache is disabled"]
pub struct CACHE_IA_INT_PRO_IROM0_R(crate::FieldReader<bool, bool>);
impl CACHE_IA_INT_PRO_IROM0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CACHE_IA_INT_PRO_IROM0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CACHE_IA_INT_PRO_IROM0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CACHE_IA_INT_PRO_IROM0` writer - PRO CPU invalid access to IROM0 when cache is disabled"]
pub struct CACHE_IA_INT_PRO_IROM0_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHE_IA_INT_PRO_IROM0_W<'a> {
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
#[doc = "Field `CACHE_IA_INT_PRO_DRAM1` reader - PRO CPU invalid access to DRAM1 when cache is disabled"]
pub struct CACHE_IA_INT_PRO_DRAM1_R(crate::FieldReader<bool, bool>);
impl CACHE_IA_INT_PRO_DRAM1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CACHE_IA_INT_PRO_DRAM1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CACHE_IA_INT_PRO_DRAM1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CACHE_IA_INT_PRO_DRAM1` writer - PRO CPU invalid access to DRAM1 when cache is disabled"]
pub struct CACHE_IA_INT_PRO_DRAM1_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHE_IA_INT_PRO_DRAM1_W<'a> {
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
#[doc = "Field `CACHE_IA_INT_PRO_OPPOSITE` reader - PRO CPU invalid access to APP CPU cache when cache disabled"]
pub struct CACHE_IA_INT_PRO_OPPOSITE_R(crate::FieldReader<bool, bool>);
impl CACHE_IA_INT_PRO_OPPOSITE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CACHE_IA_INT_PRO_OPPOSITE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CACHE_IA_INT_PRO_OPPOSITE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CACHE_IA_INT_PRO_OPPOSITE` writer - PRO CPU invalid access to APP CPU cache when cache disabled"]
pub struct CACHE_IA_INT_PRO_OPPOSITE_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHE_IA_INT_PRO_OPPOSITE_W<'a> {
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
impl R {
    #[doc = "Bits 0:27 - Interrupt enable bits for various invalid cache access reasons"]
    #[inline(always)]
    pub fn cache_ia_int_en(&self) -> CACHE_IA_INT_EN_R {
        CACHE_IA_INT_EN_R::new((self.bits & 0x0fff_ffff) as u32)
    }
    #[doc = "Bit 0 - APP CPU invalid access to DROM0 when cache is disabled"]
    #[inline(always)]
    pub fn cache_ia_int_app_drom0(&self) -> CACHE_IA_INT_APP_DROM0_R {
        CACHE_IA_INT_APP_DROM0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - APP CPU invalid access to IRAM0 when cache is disabled"]
    #[inline(always)]
    pub fn cache_ia_int_app_iram0(&self) -> CACHE_IA_INT_APP_IRAM0_R {
        CACHE_IA_INT_APP_IRAM0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - APP CPU invalid access to IRAM1 when cache is disabled"]
    #[inline(always)]
    pub fn cache_ia_int_app_iram1(&self) -> CACHE_IA_INT_APP_IRAM1_R {
        CACHE_IA_INT_APP_IRAM1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - APP CPU invalid access to IROM0 when cache is disabled"]
    #[inline(always)]
    pub fn cache_ia_int_app_irom0(&self) -> CACHE_IA_INT_APP_IROM0_R {
        CACHE_IA_INT_APP_IROM0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - APP CPU invalid access to DRAM1 when cache is disabled"]
    #[inline(always)]
    pub fn cache_ia_int_app_dram1(&self) -> CACHE_IA_INT_APP_DRAM1_R {
        CACHE_IA_INT_APP_DRAM1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - APP CPU invalid access to APP CPU cache when cache disabled"]
    #[inline(always)]
    pub fn cache_ia_int_app_opposite(&self) -> CACHE_IA_INT_APP_OPPOSITE_R {
        CACHE_IA_INT_APP_OPPOSITE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 14 - PRO CPU invalid access to DROM0 when cache is disabled"]
    #[inline(always)]
    pub fn cache_ia_int_pro_drom0(&self) -> CACHE_IA_INT_PRO_DROM0_R {
        CACHE_IA_INT_PRO_DROM0_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - PRO CPU invalid access to IRAM0 when cache is disabled"]
    #[inline(always)]
    pub fn cache_ia_int_pro_iram0(&self) -> CACHE_IA_INT_PRO_IRAM0_R {
        CACHE_IA_INT_PRO_IRAM0_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - PRO CPU invalid access to IRAM1 when cache is disabled"]
    #[inline(always)]
    pub fn cache_ia_int_pro_iram1(&self) -> CACHE_IA_INT_PRO_IRAM1_R {
        CACHE_IA_INT_PRO_IRAM1_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - PRO CPU invalid access to IROM0 when cache is disabled"]
    #[inline(always)]
    pub fn cache_ia_int_pro_irom0(&self) -> CACHE_IA_INT_PRO_IROM0_R {
        CACHE_IA_INT_PRO_IROM0_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - PRO CPU invalid access to DRAM1 when cache is disabled"]
    #[inline(always)]
    pub fn cache_ia_int_pro_dram1(&self) -> CACHE_IA_INT_PRO_DRAM1_R {
        CACHE_IA_INT_PRO_DRAM1_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - PRO CPU invalid access to APP CPU cache when cache disabled"]
    #[inline(always)]
    pub fn cache_ia_int_pro_opposite(&self) -> CACHE_IA_INT_PRO_OPPOSITE_R {
        CACHE_IA_INT_PRO_OPPOSITE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:27 - Interrupt enable bits for various invalid cache access reasons"]
    #[inline(always)]
    pub fn cache_ia_int_en(&mut self) -> CACHE_IA_INT_EN_W {
        CACHE_IA_INT_EN_W { w: self }
    }
    #[doc = "Bit 0 - APP CPU invalid access to DROM0 when cache is disabled"]
    #[inline(always)]
    pub fn cache_ia_int_app_drom0(&mut self) -> CACHE_IA_INT_APP_DROM0_W {
        CACHE_IA_INT_APP_DROM0_W { w: self }
    }
    #[doc = "Bit 1 - APP CPU invalid access to IRAM0 when cache is disabled"]
    #[inline(always)]
    pub fn cache_ia_int_app_iram0(&mut self) -> CACHE_IA_INT_APP_IRAM0_W {
        CACHE_IA_INT_APP_IRAM0_W { w: self }
    }
    #[doc = "Bit 2 - APP CPU invalid access to IRAM1 when cache is disabled"]
    #[inline(always)]
    pub fn cache_ia_int_app_iram1(&mut self) -> CACHE_IA_INT_APP_IRAM1_W {
        CACHE_IA_INT_APP_IRAM1_W { w: self }
    }
    #[doc = "Bit 3 - APP CPU invalid access to IROM0 when cache is disabled"]
    #[inline(always)]
    pub fn cache_ia_int_app_irom0(&mut self) -> CACHE_IA_INT_APP_IROM0_W {
        CACHE_IA_INT_APP_IROM0_W { w: self }
    }
    #[doc = "Bit 4 - APP CPU invalid access to DRAM1 when cache is disabled"]
    #[inline(always)]
    pub fn cache_ia_int_app_dram1(&mut self) -> CACHE_IA_INT_APP_DRAM1_W {
        CACHE_IA_INT_APP_DRAM1_W { w: self }
    }
    #[doc = "Bit 5 - APP CPU invalid access to APP CPU cache when cache disabled"]
    #[inline(always)]
    pub fn cache_ia_int_app_opposite(&mut self) -> CACHE_IA_INT_APP_OPPOSITE_W {
        CACHE_IA_INT_APP_OPPOSITE_W { w: self }
    }
    #[doc = "Bit 14 - PRO CPU invalid access to DROM0 when cache is disabled"]
    #[inline(always)]
    pub fn cache_ia_int_pro_drom0(&mut self) -> CACHE_IA_INT_PRO_DROM0_W {
        CACHE_IA_INT_PRO_DROM0_W { w: self }
    }
    #[doc = "Bit 15 - PRO CPU invalid access to IRAM0 when cache is disabled"]
    #[inline(always)]
    pub fn cache_ia_int_pro_iram0(&mut self) -> CACHE_IA_INT_PRO_IRAM0_W {
        CACHE_IA_INT_PRO_IRAM0_W { w: self }
    }
    #[doc = "Bit 16 - PRO CPU invalid access to IRAM1 when cache is disabled"]
    #[inline(always)]
    pub fn cache_ia_int_pro_iram1(&mut self) -> CACHE_IA_INT_PRO_IRAM1_W {
        CACHE_IA_INT_PRO_IRAM1_W { w: self }
    }
    #[doc = "Bit 17 - PRO CPU invalid access to IROM0 when cache is disabled"]
    #[inline(always)]
    pub fn cache_ia_int_pro_irom0(&mut self) -> CACHE_IA_INT_PRO_IROM0_W {
        CACHE_IA_INT_PRO_IROM0_W { w: self }
    }
    #[doc = "Bit 18 - PRO CPU invalid access to DRAM1 when cache is disabled"]
    #[inline(always)]
    pub fn cache_ia_int_pro_dram1(&mut self) -> CACHE_IA_INT_PRO_DRAM1_W {
        CACHE_IA_INT_PRO_DRAM1_W { w: self }
    }
    #[doc = "Bit 19 - PRO CPU invalid access to APP CPU cache when cache disabled"]
    #[inline(always)]
    pub fn cache_ia_int_pro_opposite(&mut self) -> CACHE_IA_INT_PRO_OPPOSITE_W {
        CACHE_IA_INT_PRO_OPPOSITE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cache_ia_int_en](index.html) module"]
pub struct CACHE_IA_INT_EN_SPEC;
impl crate::RegisterSpec for CACHE_IA_INT_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cache_ia_int_en::R](R) reader structure"]
impl crate::Readable for CACHE_IA_INT_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cache_ia_int_en::W](W) writer structure"]
impl crate::Writable for CACHE_IA_INT_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CACHE_IA_INT_EN to value 0"]
impl crate::Resettable for CACHE_IA_INT_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
