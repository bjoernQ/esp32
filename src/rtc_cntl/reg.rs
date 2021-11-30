#[doc = "Register `REG` reader"]
pub struct R(crate::R<REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REG` writer"]
pub struct W(crate::W<REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG_SPEC>;
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
impl From<crate::W<REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCK_DCAP_FORCE` reader - N/A"]
pub struct SCK_DCAP_FORCE_R(crate::FieldReader<bool, bool>);
impl SCK_DCAP_FORCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCK_DCAP_FORCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCK_DCAP_FORCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCK_DCAP_FORCE` writer - N/A"]
pub struct SCK_DCAP_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> SCK_DCAP_FORCE_W<'a> {
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
#[doc = "Field `DIG_DBIAS_SLP` reader - DIG_REG_DBIAS during sleep"]
pub struct DIG_DBIAS_SLP_R(crate::FieldReader<u8, u8>);
impl DIG_DBIAS_SLP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIG_DBIAS_SLP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIG_DBIAS_SLP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIG_DBIAS_SLP` writer - DIG_REG_DBIAS during sleep"]
pub struct DIG_DBIAS_SLP_W<'a> {
    w: &'a mut W,
}
impl<'a> DIG_DBIAS_SLP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `DIG_DBIAS_WAK` reader - DIG_REG_DBIAS during wakeup"]
pub struct DIG_DBIAS_WAK_R(crate::FieldReader<u8, u8>);
impl DIG_DBIAS_WAK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIG_DBIAS_WAK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIG_DBIAS_WAK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIG_DBIAS_WAK` writer - DIG_REG_DBIAS during wakeup"]
pub struct DIG_DBIAS_WAK_W<'a> {
    w: &'a mut W,
}
impl<'a> DIG_DBIAS_WAK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | ((value as u32 & 0x07) << 11);
        self.w
    }
}
#[doc = "Field `SCK_DCAP` reader - SCK_DCAP"]
pub struct SCK_DCAP_R(crate::FieldReader<u8, u8>);
impl SCK_DCAP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SCK_DCAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCK_DCAP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCK_DCAP` writer - SCK_DCAP"]
pub struct SCK_DCAP_W<'a> {
    w: &'a mut W,
}
impl<'a> SCK_DCAP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 14)) | ((value as u32 & 0xff) << 14);
        self.w
    }
}
#[doc = "Field `DBIAS_SLP` reader - RTC_DBIAS during sleep"]
pub struct DBIAS_SLP_R(crate::FieldReader<u8, u8>);
impl DBIAS_SLP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DBIAS_SLP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBIAS_SLP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBIAS_SLP` writer - RTC_DBIAS during sleep"]
pub struct DBIAS_SLP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBIAS_SLP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 22)) | ((value as u32 & 0x07) << 22);
        self.w
    }
}
#[doc = "Field `DBIAS_WAK` reader - RTC_DBIAS during wakeup"]
pub struct DBIAS_WAK_R(crate::FieldReader<u8, u8>);
impl DBIAS_WAK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DBIAS_WAK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBIAS_WAK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBIAS_WAK` writer - RTC_DBIAS during wakeup"]
pub struct DBIAS_WAK_W<'a> {
    w: &'a mut W,
}
impl<'a> DBIAS_WAK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 25)) | ((value as u32 & 0x07) << 25);
        self.w
    }
}
#[doc = "Field `DBOOST_FORCE_PD` reader - RTC_DBOOST force power down"]
pub struct DBOOST_FORCE_PD_R(crate::FieldReader<bool, bool>);
impl DBOOST_FORCE_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DBOOST_FORCE_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBOOST_FORCE_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBOOST_FORCE_PD` writer - RTC_DBOOST force power down"]
pub struct DBOOST_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> DBOOST_FORCE_PD_W<'a> {
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
#[doc = "Field `DBOOST_FORCE_PU` reader - RTC_DBOOST force power up"]
pub struct DBOOST_FORCE_PU_R(crate::FieldReader<bool, bool>);
impl DBOOST_FORCE_PU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DBOOST_FORCE_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBOOST_FORCE_PU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBOOST_FORCE_PU` writer - RTC_DBOOST force power up"]
pub struct DBOOST_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> DBOOST_FORCE_PU_W<'a> {
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
#[doc = "Field `FORCE_PD` reader - RTC_REG force power down (for RTC_REG power down means decrease the voltage to 0.8v or lower )"]
pub struct FORCE_PD_R(crate::FieldReader<bool, bool>);
impl FORCE_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FORCE_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FORCE_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FORCE_PD` writer - RTC_REG force power down (for RTC_REG power down means decrease the voltage to 0.8v or lower )"]
pub struct FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_PD_W<'a> {
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
#[doc = "Field `FORCE_PU` reader - RTC_REG force power up"]
pub struct FORCE_PU_R(crate::FieldReader<bool, bool>);
impl FORCE_PU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FORCE_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FORCE_PU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FORCE_PU` writer - RTC_REG force power up"]
pub struct FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_PU_W<'a> {
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
    #[doc = "Bit 7 - N/A"]
    #[inline(always)]
    pub fn sck_dcap_force(&self) -> SCK_DCAP_FORCE_R {
        SCK_DCAP_FORCE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - DIG_REG_DBIAS during sleep"]
    #[inline(always)]
    pub fn dig_dbias_slp(&self) -> DIG_DBIAS_SLP_R {
        DIG_DBIAS_SLP_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 11:13 - DIG_REG_DBIAS during wakeup"]
    #[inline(always)]
    pub fn dig_dbias_wak(&self) -> DIG_DBIAS_WAK_R {
        DIG_DBIAS_WAK_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    #[doc = "Bits 14:21 - SCK_DCAP"]
    #[inline(always)]
    pub fn sck_dcap(&self) -> SCK_DCAP_R {
        SCK_DCAP_R::new(((self.bits >> 14) & 0xff) as u8)
    }
    #[doc = "Bits 22:24 - RTC_DBIAS during sleep"]
    #[inline(always)]
    pub fn dbias_slp(&self) -> DBIAS_SLP_R {
        DBIAS_SLP_R::new(((self.bits >> 22) & 0x07) as u8)
    }
    #[doc = "Bits 25:27 - RTC_DBIAS during wakeup"]
    #[inline(always)]
    pub fn dbias_wak(&self) -> DBIAS_WAK_R {
        DBIAS_WAK_R::new(((self.bits >> 25) & 0x07) as u8)
    }
    #[doc = "Bit 28 - RTC_DBOOST force power down"]
    #[inline(always)]
    pub fn dboost_force_pd(&self) -> DBOOST_FORCE_PD_R {
        DBOOST_FORCE_PD_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - RTC_DBOOST force power up"]
    #[inline(always)]
    pub fn dboost_force_pu(&self) -> DBOOST_FORCE_PU_R {
        DBOOST_FORCE_PU_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - RTC_REG force power down (for RTC_REG power down means decrease the voltage to 0.8v or lower )"]
    #[inline(always)]
    pub fn force_pd(&self) -> FORCE_PD_R {
        FORCE_PD_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - RTC_REG force power up"]
    #[inline(always)]
    pub fn force_pu(&self) -> FORCE_PU_R {
        FORCE_PU_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - N/A"]
    #[inline(always)]
    pub fn sck_dcap_force(&mut self) -> SCK_DCAP_FORCE_W {
        SCK_DCAP_FORCE_W { w: self }
    }
    #[doc = "Bits 8:10 - DIG_REG_DBIAS during sleep"]
    #[inline(always)]
    pub fn dig_dbias_slp(&mut self) -> DIG_DBIAS_SLP_W {
        DIG_DBIAS_SLP_W { w: self }
    }
    #[doc = "Bits 11:13 - DIG_REG_DBIAS during wakeup"]
    #[inline(always)]
    pub fn dig_dbias_wak(&mut self) -> DIG_DBIAS_WAK_W {
        DIG_DBIAS_WAK_W { w: self }
    }
    #[doc = "Bits 14:21 - SCK_DCAP"]
    #[inline(always)]
    pub fn sck_dcap(&mut self) -> SCK_DCAP_W {
        SCK_DCAP_W { w: self }
    }
    #[doc = "Bits 22:24 - RTC_DBIAS during sleep"]
    #[inline(always)]
    pub fn dbias_slp(&mut self) -> DBIAS_SLP_W {
        DBIAS_SLP_W { w: self }
    }
    #[doc = "Bits 25:27 - RTC_DBIAS during wakeup"]
    #[inline(always)]
    pub fn dbias_wak(&mut self) -> DBIAS_WAK_W {
        DBIAS_WAK_W { w: self }
    }
    #[doc = "Bit 28 - RTC_DBOOST force power down"]
    #[inline(always)]
    pub fn dboost_force_pd(&mut self) -> DBOOST_FORCE_PD_W {
        DBOOST_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 29 - RTC_DBOOST force power up"]
    #[inline(always)]
    pub fn dboost_force_pu(&mut self) -> DBOOST_FORCE_PU_W {
        DBOOST_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 30 - RTC_REG force power down (for RTC_REG power down means decrease the voltage to 0.8v or lower )"]
    #[inline(always)]
    pub fn force_pd(&mut self) -> FORCE_PD_W {
        FORCE_PD_W { w: self }
    }
    #[doc = "Bit 31 - RTC_REG force power up"]
    #[inline(always)]
    pub fn force_pu(&mut self) -> FORCE_PU_W {
        FORCE_PU_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg](index.html) module"]
pub struct REG_SPEC;
impl crate::RegisterSpec for REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg::R](R) reader structure"]
impl crate::Readable for REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg::W](W) writer structure"]
impl crate::Writable for REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REG to value 0x2900_2400"]
impl crate::Resettable for REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2900_2400
    }
}
