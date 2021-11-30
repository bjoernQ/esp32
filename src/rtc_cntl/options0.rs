#[doc = "Register `OPTIONS0` reader"]
pub struct R(crate::R<OPTIONS0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPTIONS0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPTIONS0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPTIONS0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPTIONS0` writer"]
pub struct W(crate::W<OPTIONS0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPTIONS0_SPEC>;
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
impl From<crate::W<OPTIONS0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPTIONS0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SW_STALL_APPCPU_C0` reader - {reg_sw_stall_appcpu_c1\\[5:0\\]
reg_sw_stall_appcpu_c0\\[1:0\\]} == 0x86 will stall APP CPU"]
pub struct SW_STALL_APPCPU_C0_R(crate::FieldReader<u8, u8>);
impl SW_STALL_APPCPU_C0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SW_STALL_APPCPU_C0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_STALL_APPCPU_C0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_STALL_APPCPU_C0` writer - {reg_sw_stall_appcpu_c1\\[5:0\\]
reg_sw_stall_appcpu_c0\\[1:0\\]} == 0x86 will stall APP CPU"]
pub struct SW_STALL_APPCPU_C0_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_STALL_APPCPU_C0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `SW_STALL_PROCPU_C0` reader - {reg_sw_stall_procpu_c1\\[5:0\\]
reg_sw_stall_procpu_c0\\[1:0\\]} == 0x86 will stall PRO CPU"]
pub struct SW_STALL_PROCPU_C0_R(crate::FieldReader<u8, u8>);
impl SW_STALL_PROCPU_C0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SW_STALL_PROCPU_C0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_STALL_PROCPU_C0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_STALL_PROCPU_C0` writer - {reg_sw_stall_procpu_c1\\[5:0\\]
reg_sw_stall_procpu_c0\\[1:0\\]} == 0x86 will stall PRO CPU"]
pub struct SW_STALL_PROCPU_C0_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_STALL_PROCPU_C0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `SW_APPCPU_RST` writer - APP CPU SW reset"]
pub struct SW_APPCPU_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_APPCPU_RST_W<'a> {
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
#[doc = "Field `SW_PROCPU_RST` writer - PRO CPU SW reset"]
pub struct SW_PROCPU_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_PROCPU_RST_W<'a> {
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
#[doc = "Field `BB_I2C_FORCE_PD` reader - BB_I2C force power down"]
pub struct BB_I2C_FORCE_PD_R(crate::FieldReader<bool, bool>);
impl BB_I2C_FORCE_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BB_I2C_FORCE_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BB_I2C_FORCE_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BB_I2C_FORCE_PD` writer - BB_I2C force power down"]
pub struct BB_I2C_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> BB_I2C_FORCE_PD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `BB_I2C_FORCE_PU` reader - BB_I2C force power up"]
pub struct BB_I2C_FORCE_PU_R(crate::FieldReader<bool, bool>);
impl BB_I2C_FORCE_PU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BB_I2C_FORCE_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BB_I2C_FORCE_PU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BB_I2C_FORCE_PU` writer - BB_I2C force power up"]
pub struct BB_I2C_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> BB_I2C_FORCE_PU_W<'a> {
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
#[doc = "Field `BBPLL_I2C_FORCE_PD` reader - BB_PLL _I2C force power down"]
pub struct BBPLL_I2C_FORCE_PD_R(crate::FieldReader<bool, bool>);
impl BBPLL_I2C_FORCE_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BBPLL_I2C_FORCE_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BBPLL_I2C_FORCE_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BBPLL_I2C_FORCE_PD` writer - BB_PLL _I2C force power down"]
pub struct BBPLL_I2C_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> BBPLL_I2C_FORCE_PD_W<'a> {
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
#[doc = "Field `BBPLL_I2C_FORCE_PU` reader - BB_PLL_I2C force power up"]
pub struct BBPLL_I2C_FORCE_PU_R(crate::FieldReader<bool, bool>);
impl BBPLL_I2C_FORCE_PU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BBPLL_I2C_FORCE_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BBPLL_I2C_FORCE_PU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BBPLL_I2C_FORCE_PU` writer - BB_PLL_I2C force power up"]
pub struct BBPLL_I2C_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> BBPLL_I2C_FORCE_PU_W<'a> {
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
#[doc = "Field `BBPLL_FORCE_PD` reader - BB_PLL force power down"]
pub struct BBPLL_FORCE_PD_R(crate::FieldReader<bool, bool>);
impl BBPLL_FORCE_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BBPLL_FORCE_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BBPLL_FORCE_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BBPLL_FORCE_PD` writer - BB_PLL force power down"]
pub struct BBPLL_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> BBPLL_FORCE_PD_W<'a> {
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
#[doc = "Field `BBPLL_FORCE_PU` reader - BB_PLL force power up"]
pub struct BBPLL_FORCE_PU_R(crate::FieldReader<bool, bool>);
impl BBPLL_FORCE_PU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BBPLL_FORCE_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BBPLL_FORCE_PU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BBPLL_FORCE_PU` writer - BB_PLL force power up"]
pub struct BBPLL_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> BBPLL_FORCE_PU_W<'a> {
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
#[doc = "Field `XTL_FORCE_PD` reader - crystall force power down"]
pub struct XTL_FORCE_PD_R(crate::FieldReader<bool, bool>);
impl XTL_FORCE_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XTL_FORCE_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTL_FORCE_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTL_FORCE_PD` writer - crystall force power down"]
pub struct XTL_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> XTL_FORCE_PD_W<'a> {
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
#[doc = "Field `XTL_FORCE_PU` reader - crystall force power up"]
pub struct XTL_FORCE_PU_R(crate::FieldReader<bool, bool>);
impl XTL_FORCE_PU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XTL_FORCE_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTL_FORCE_PU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTL_FORCE_PU` writer - crystall force power up"]
pub struct XTL_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> XTL_FORCE_PU_W<'a> {
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
#[doc = "Field `BIAS_SLEEP_FOLW_8M` reader - BIAS_SLEEP follow CK8M"]
pub struct BIAS_SLEEP_FOLW_8M_R(crate::FieldReader<bool, bool>);
impl BIAS_SLEEP_FOLW_8M_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BIAS_SLEEP_FOLW_8M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BIAS_SLEEP_FOLW_8M_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BIAS_SLEEP_FOLW_8M` writer - BIAS_SLEEP follow CK8M"]
pub struct BIAS_SLEEP_FOLW_8M_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_SLEEP_FOLW_8M_W<'a> {
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
#[doc = "Field `BIAS_FORCE_SLEEP` reader - BIAS_SLEEP force sleep"]
pub struct BIAS_FORCE_SLEEP_R(crate::FieldReader<bool, bool>);
impl BIAS_FORCE_SLEEP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BIAS_FORCE_SLEEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BIAS_FORCE_SLEEP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BIAS_FORCE_SLEEP` writer - BIAS_SLEEP force sleep"]
pub struct BIAS_FORCE_SLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_FORCE_SLEEP_W<'a> {
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
#[doc = "Field `BIAS_FORCE_NOSLEEP` reader - BIAS_SLEEP force no sleep"]
pub struct BIAS_FORCE_NOSLEEP_R(crate::FieldReader<bool, bool>);
impl BIAS_FORCE_NOSLEEP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BIAS_FORCE_NOSLEEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BIAS_FORCE_NOSLEEP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BIAS_FORCE_NOSLEEP` writer - BIAS_SLEEP force no sleep"]
pub struct BIAS_FORCE_NOSLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_FORCE_NOSLEEP_W<'a> {
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
#[doc = "Field `BIAS_I2C_FOLW_8M` reader - BIAS_I2C follow CK8M"]
pub struct BIAS_I2C_FOLW_8M_R(crate::FieldReader<bool, bool>);
impl BIAS_I2C_FOLW_8M_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BIAS_I2C_FOLW_8M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BIAS_I2C_FOLW_8M_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BIAS_I2C_FOLW_8M` writer - BIAS_I2C follow CK8M"]
pub struct BIAS_I2C_FOLW_8M_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_I2C_FOLW_8M_W<'a> {
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
#[doc = "Field `BIAS_I2C_FORCE_PD` reader - BIAS_I2C force power down"]
pub struct BIAS_I2C_FORCE_PD_R(crate::FieldReader<bool, bool>);
impl BIAS_I2C_FORCE_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BIAS_I2C_FORCE_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BIAS_I2C_FORCE_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BIAS_I2C_FORCE_PD` writer - BIAS_I2C force power down"]
pub struct BIAS_I2C_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_I2C_FORCE_PD_W<'a> {
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
#[doc = "Field `BIAS_I2C_FORCE_PU` reader - BIAS_I2C force power up"]
pub struct BIAS_I2C_FORCE_PU_R(crate::FieldReader<bool, bool>);
impl BIAS_I2C_FORCE_PU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BIAS_I2C_FORCE_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BIAS_I2C_FORCE_PU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BIAS_I2C_FORCE_PU` writer - BIAS_I2C force power up"]
pub struct BIAS_I2C_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_I2C_FORCE_PU_W<'a> {
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
#[doc = "Field `BIAS_CORE_FOLW_8M` reader - BIAS_CORE follow CK8M"]
pub struct BIAS_CORE_FOLW_8M_R(crate::FieldReader<bool, bool>);
impl BIAS_CORE_FOLW_8M_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BIAS_CORE_FOLW_8M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BIAS_CORE_FOLW_8M_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BIAS_CORE_FOLW_8M` writer - BIAS_CORE follow CK8M"]
pub struct BIAS_CORE_FOLW_8M_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_CORE_FOLW_8M_W<'a> {
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
#[doc = "Field `BIAS_CORE_FORCE_PD` reader - BIAS_CORE force power down"]
pub struct BIAS_CORE_FORCE_PD_R(crate::FieldReader<bool, bool>);
impl BIAS_CORE_FORCE_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BIAS_CORE_FORCE_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BIAS_CORE_FORCE_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BIAS_CORE_FORCE_PD` writer - BIAS_CORE force power down"]
pub struct BIAS_CORE_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_CORE_FORCE_PD_W<'a> {
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
#[doc = "Field `BIAS_CORE_FORCE_PU` reader - BIAS_CORE force power up"]
pub struct BIAS_CORE_FORCE_PU_R(crate::FieldReader<bool, bool>);
impl BIAS_CORE_FORCE_PU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BIAS_CORE_FORCE_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BIAS_CORE_FORCE_PU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BIAS_CORE_FORCE_PU` writer - BIAS_CORE force power up"]
pub struct BIAS_CORE_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_CORE_FORCE_PU_W<'a> {
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
#[doc = "Field `XTL_FORCE_ISO` reader - "]
pub struct XTL_FORCE_ISO_R(crate::FieldReader<bool, bool>);
impl XTL_FORCE_ISO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XTL_FORCE_ISO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTL_FORCE_ISO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTL_FORCE_ISO` writer - "]
pub struct XTL_FORCE_ISO_W<'a> {
    w: &'a mut W,
}
impl<'a> XTL_FORCE_ISO_W<'a> {
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
#[doc = "Field `PLL_FORCE_ISO` reader - "]
pub struct PLL_FORCE_ISO_R(crate::FieldReader<bool, bool>);
impl PLL_FORCE_ISO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PLL_FORCE_ISO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLL_FORCE_ISO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL_FORCE_ISO` writer - "]
pub struct PLL_FORCE_ISO_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_FORCE_ISO_W<'a> {
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
#[doc = "Field `ANALOG_FORCE_ISO` reader - "]
pub struct ANALOG_FORCE_ISO_R(crate::FieldReader<bool, bool>);
impl ANALOG_FORCE_ISO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ANALOG_FORCE_ISO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ANALOG_FORCE_ISO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ANALOG_FORCE_ISO` writer - "]
pub struct ANALOG_FORCE_ISO_W<'a> {
    w: &'a mut W,
}
impl<'a> ANALOG_FORCE_ISO_W<'a> {
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
#[doc = "Field `XTL_FORCE_NOISO` reader - "]
pub struct XTL_FORCE_NOISO_R(crate::FieldReader<bool, bool>);
impl XTL_FORCE_NOISO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XTL_FORCE_NOISO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTL_FORCE_NOISO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTL_FORCE_NOISO` writer - "]
pub struct XTL_FORCE_NOISO_W<'a> {
    w: &'a mut W,
}
impl<'a> XTL_FORCE_NOISO_W<'a> {
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
#[doc = "Field `PLL_FORCE_NOISO` reader - "]
pub struct PLL_FORCE_NOISO_R(crate::FieldReader<bool, bool>);
impl PLL_FORCE_NOISO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PLL_FORCE_NOISO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLL_FORCE_NOISO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL_FORCE_NOISO` writer - "]
pub struct PLL_FORCE_NOISO_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_FORCE_NOISO_W<'a> {
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
#[doc = "Field `ANALOG_FORCE_NOISO` reader - "]
pub struct ANALOG_FORCE_NOISO_R(crate::FieldReader<bool, bool>);
impl ANALOG_FORCE_NOISO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ANALOG_FORCE_NOISO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ANALOG_FORCE_NOISO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ANALOG_FORCE_NOISO` writer - "]
pub struct ANALOG_FORCE_NOISO_W<'a> {
    w: &'a mut W,
}
impl<'a> ANALOG_FORCE_NOISO_W<'a> {
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
#[doc = "Field `DG_WRAP_FORCE_RST` reader - digital wrap force reset in deep sleep"]
pub struct DG_WRAP_FORCE_RST_R(crate::FieldReader<bool, bool>);
impl DG_WRAP_FORCE_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DG_WRAP_FORCE_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DG_WRAP_FORCE_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DG_WRAP_FORCE_RST` writer - digital wrap force reset in deep sleep"]
pub struct DG_WRAP_FORCE_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> DG_WRAP_FORCE_RST_W<'a> {
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
#[doc = "Field `DG_WRAP_FORCE_NORST` reader - digital core force no reset in deep sleep"]
pub struct DG_WRAP_FORCE_NORST_R(crate::FieldReader<bool, bool>);
impl DG_WRAP_FORCE_NORST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DG_WRAP_FORCE_NORST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DG_WRAP_FORCE_NORST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DG_WRAP_FORCE_NORST` writer - digital core force no reset in deep sleep"]
pub struct DG_WRAP_FORCE_NORST_W<'a> {
    w: &'a mut W,
}
impl<'a> DG_WRAP_FORCE_NORST_W<'a> {
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
#[doc = "Field `SW_SYS_RST` writer - SW system reset"]
pub struct SW_SYS_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_SYS_RST_W<'a> {
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
    #[doc = "Bits 0:1 - {reg_sw_stall_appcpu_c1\\[5:0\\]
reg_sw_stall_appcpu_c0\\[1:0\\]} == 0x86 will stall APP CPU"]
    #[inline(always)]
    pub fn sw_stall_appcpu_c0(&self) -> SW_STALL_APPCPU_C0_R {
        SW_STALL_APPCPU_C0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - {reg_sw_stall_procpu_c1\\[5:0\\]
reg_sw_stall_procpu_c0\\[1:0\\]} == 0x86 will stall PRO CPU"]
    #[inline(always)]
    pub fn sw_stall_procpu_c0(&self) -> SW_STALL_PROCPU_C0_R {
        SW_STALL_PROCPU_C0_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 6 - BB_I2C force power down"]
    #[inline(always)]
    pub fn bb_i2c_force_pd(&self) -> BB_I2C_FORCE_PD_R {
        BB_I2C_FORCE_PD_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - BB_I2C force power up"]
    #[inline(always)]
    pub fn bb_i2c_force_pu(&self) -> BB_I2C_FORCE_PU_R {
        BB_I2C_FORCE_PU_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - BB_PLL _I2C force power down"]
    #[inline(always)]
    pub fn bbpll_i2c_force_pd(&self) -> BBPLL_I2C_FORCE_PD_R {
        BBPLL_I2C_FORCE_PD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - BB_PLL_I2C force power up"]
    #[inline(always)]
    pub fn bbpll_i2c_force_pu(&self) -> BBPLL_I2C_FORCE_PU_R {
        BBPLL_I2C_FORCE_PU_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - BB_PLL force power down"]
    #[inline(always)]
    pub fn bbpll_force_pd(&self) -> BBPLL_FORCE_PD_R {
        BBPLL_FORCE_PD_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - BB_PLL force power up"]
    #[inline(always)]
    pub fn bbpll_force_pu(&self) -> BBPLL_FORCE_PU_R {
        BBPLL_FORCE_PU_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - crystall force power down"]
    #[inline(always)]
    pub fn xtl_force_pd(&self) -> XTL_FORCE_PD_R {
        XTL_FORCE_PD_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - crystall force power up"]
    #[inline(always)]
    pub fn xtl_force_pu(&self) -> XTL_FORCE_PU_R {
        XTL_FORCE_PU_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - BIAS_SLEEP follow CK8M"]
    #[inline(always)]
    pub fn bias_sleep_folw_8m(&self) -> BIAS_SLEEP_FOLW_8M_R {
        BIAS_SLEEP_FOLW_8M_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - BIAS_SLEEP force sleep"]
    #[inline(always)]
    pub fn bias_force_sleep(&self) -> BIAS_FORCE_SLEEP_R {
        BIAS_FORCE_SLEEP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - BIAS_SLEEP force no sleep"]
    #[inline(always)]
    pub fn bias_force_nosleep(&self) -> BIAS_FORCE_NOSLEEP_R {
        BIAS_FORCE_NOSLEEP_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - BIAS_I2C follow CK8M"]
    #[inline(always)]
    pub fn bias_i2c_folw_8m(&self) -> BIAS_I2C_FOLW_8M_R {
        BIAS_I2C_FOLW_8M_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - BIAS_I2C force power down"]
    #[inline(always)]
    pub fn bias_i2c_force_pd(&self) -> BIAS_I2C_FORCE_PD_R {
        BIAS_I2C_FORCE_PD_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - BIAS_I2C force power up"]
    #[inline(always)]
    pub fn bias_i2c_force_pu(&self) -> BIAS_I2C_FORCE_PU_R {
        BIAS_I2C_FORCE_PU_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - BIAS_CORE follow CK8M"]
    #[inline(always)]
    pub fn bias_core_folw_8m(&self) -> BIAS_CORE_FOLW_8M_R {
        BIAS_CORE_FOLW_8M_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - BIAS_CORE force power down"]
    #[inline(always)]
    pub fn bias_core_force_pd(&self) -> BIAS_CORE_FORCE_PD_R {
        BIAS_CORE_FORCE_PD_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - BIAS_CORE force power up"]
    #[inline(always)]
    pub fn bias_core_force_pu(&self) -> BIAS_CORE_FORCE_PU_R {
        BIAS_CORE_FORCE_PU_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn xtl_force_iso(&self) -> XTL_FORCE_ISO_R {
        XTL_FORCE_ISO_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn pll_force_iso(&self) -> PLL_FORCE_ISO_R {
        PLL_FORCE_ISO_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn analog_force_iso(&self) -> ANALOG_FORCE_ISO_R {
        ANALOG_FORCE_ISO_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn xtl_force_noiso(&self) -> XTL_FORCE_NOISO_R {
        XTL_FORCE_NOISO_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn pll_force_noiso(&self) -> PLL_FORCE_NOISO_R {
        PLL_FORCE_NOISO_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn analog_force_noiso(&self) -> ANALOG_FORCE_NOISO_R {
        ANALOG_FORCE_NOISO_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - digital wrap force reset in deep sleep"]
    #[inline(always)]
    pub fn dg_wrap_force_rst(&self) -> DG_WRAP_FORCE_RST_R {
        DG_WRAP_FORCE_RST_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - digital core force no reset in deep sleep"]
    #[inline(always)]
    pub fn dg_wrap_force_norst(&self) -> DG_WRAP_FORCE_NORST_R {
        DG_WRAP_FORCE_NORST_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - {reg_sw_stall_appcpu_c1\\[5:0\\]
reg_sw_stall_appcpu_c0\\[1:0\\]} == 0x86 will stall APP CPU"]
    #[inline(always)]
    pub fn sw_stall_appcpu_c0(&mut self) -> SW_STALL_APPCPU_C0_W {
        SW_STALL_APPCPU_C0_W { w: self }
    }
    #[doc = "Bits 2:3 - {reg_sw_stall_procpu_c1\\[5:0\\]
reg_sw_stall_procpu_c0\\[1:0\\]} == 0x86 will stall PRO CPU"]
    #[inline(always)]
    pub fn sw_stall_procpu_c0(&mut self) -> SW_STALL_PROCPU_C0_W {
        SW_STALL_PROCPU_C0_W { w: self }
    }
    #[doc = "Bit 4 - APP CPU SW reset"]
    #[inline(always)]
    pub fn sw_appcpu_rst(&mut self) -> SW_APPCPU_RST_W {
        SW_APPCPU_RST_W { w: self }
    }
    #[doc = "Bit 5 - PRO CPU SW reset"]
    #[inline(always)]
    pub fn sw_procpu_rst(&mut self) -> SW_PROCPU_RST_W {
        SW_PROCPU_RST_W { w: self }
    }
    #[doc = "Bit 6 - BB_I2C force power down"]
    #[inline(always)]
    pub fn bb_i2c_force_pd(&mut self) -> BB_I2C_FORCE_PD_W {
        BB_I2C_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 7 - BB_I2C force power up"]
    #[inline(always)]
    pub fn bb_i2c_force_pu(&mut self) -> BB_I2C_FORCE_PU_W {
        BB_I2C_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 8 - BB_PLL _I2C force power down"]
    #[inline(always)]
    pub fn bbpll_i2c_force_pd(&mut self) -> BBPLL_I2C_FORCE_PD_W {
        BBPLL_I2C_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 9 - BB_PLL_I2C force power up"]
    #[inline(always)]
    pub fn bbpll_i2c_force_pu(&mut self) -> BBPLL_I2C_FORCE_PU_W {
        BBPLL_I2C_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 10 - BB_PLL force power down"]
    #[inline(always)]
    pub fn bbpll_force_pd(&mut self) -> BBPLL_FORCE_PD_W {
        BBPLL_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 11 - BB_PLL force power up"]
    #[inline(always)]
    pub fn bbpll_force_pu(&mut self) -> BBPLL_FORCE_PU_W {
        BBPLL_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 12 - crystall force power down"]
    #[inline(always)]
    pub fn xtl_force_pd(&mut self) -> XTL_FORCE_PD_W {
        XTL_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 13 - crystall force power up"]
    #[inline(always)]
    pub fn xtl_force_pu(&mut self) -> XTL_FORCE_PU_W {
        XTL_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 14 - BIAS_SLEEP follow CK8M"]
    #[inline(always)]
    pub fn bias_sleep_folw_8m(&mut self) -> BIAS_SLEEP_FOLW_8M_W {
        BIAS_SLEEP_FOLW_8M_W { w: self }
    }
    #[doc = "Bit 15 - BIAS_SLEEP force sleep"]
    #[inline(always)]
    pub fn bias_force_sleep(&mut self) -> BIAS_FORCE_SLEEP_W {
        BIAS_FORCE_SLEEP_W { w: self }
    }
    #[doc = "Bit 16 - BIAS_SLEEP force no sleep"]
    #[inline(always)]
    pub fn bias_force_nosleep(&mut self) -> BIAS_FORCE_NOSLEEP_W {
        BIAS_FORCE_NOSLEEP_W { w: self }
    }
    #[doc = "Bit 17 - BIAS_I2C follow CK8M"]
    #[inline(always)]
    pub fn bias_i2c_folw_8m(&mut self) -> BIAS_I2C_FOLW_8M_W {
        BIAS_I2C_FOLW_8M_W { w: self }
    }
    #[doc = "Bit 18 - BIAS_I2C force power down"]
    #[inline(always)]
    pub fn bias_i2c_force_pd(&mut self) -> BIAS_I2C_FORCE_PD_W {
        BIAS_I2C_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 19 - BIAS_I2C force power up"]
    #[inline(always)]
    pub fn bias_i2c_force_pu(&mut self) -> BIAS_I2C_FORCE_PU_W {
        BIAS_I2C_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 20 - BIAS_CORE follow CK8M"]
    #[inline(always)]
    pub fn bias_core_folw_8m(&mut self) -> BIAS_CORE_FOLW_8M_W {
        BIAS_CORE_FOLW_8M_W { w: self }
    }
    #[doc = "Bit 21 - BIAS_CORE force power down"]
    #[inline(always)]
    pub fn bias_core_force_pd(&mut self) -> BIAS_CORE_FORCE_PD_W {
        BIAS_CORE_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 22 - BIAS_CORE force power up"]
    #[inline(always)]
    pub fn bias_core_force_pu(&mut self) -> BIAS_CORE_FORCE_PU_W {
        BIAS_CORE_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn xtl_force_iso(&mut self) -> XTL_FORCE_ISO_W {
        XTL_FORCE_ISO_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn pll_force_iso(&mut self) -> PLL_FORCE_ISO_W {
        PLL_FORCE_ISO_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn analog_force_iso(&mut self) -> ANALOG_FORCE_ISO_W {
        ANALOG_FORCE_ISO_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn xtl_force_noiso(&mut self) -> XTL_FORCE_NOISO_W {
        XTL_FORCE_NOISO_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn pll_force_noiso(&mut self) -> PLL_FORCE_NOISO_W {
        PLL_FORCE_NOISO_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn analog_force_noiso(&mut self) -> ANALOG_FORCE_NOISO_W {
        ANALOG_FORCE_NOISO_W { w: self }
    }
    #[doc = "Bit 29 - digital wrap force reset in deep sleep"]
    #[inline(always)]
    pub fn dg_wrap_force_rst(&mut self) -> DG_WRAP_FORCE_RST_W {
        DG_WRAP_FORCE_RST_W { w: self }
    }
    #[doc = "Bit 30 - digital core force no reset in deep sleep"]
    #[inline(always)]
    pub fn dg_wrap_force_norst(&mut self) -> DG_WRAP_FORCE_NORST_W {
        DG_WRAP_FORCE_NORST_W { w: self }
    }
    #[doc = "Bit 31 - SW system reset"]
    #[inline(always)]
    pub fn sw_sys_rst(&mut self) -> SW_SYS_RST_W {
        SW_SYS_RST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [options0](index.html) module"]
pub struct OPTIONS0_SPEC;
impl crate::RegisterSpec for OPTIONS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [options0::R](R) reader structure"]
impl crate::Readable for OPTIONS0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [options0::W](W) writer structure"]
impl crate::Writable for OPTIONS0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OPTIONS0 to value 0x1c49_2000"]
impl crate::Resettable for OPTIONS0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1c49_2000
    }
}
