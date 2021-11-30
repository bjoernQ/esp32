#[doc = "Register `BROWN_OUT` reader"]
pub struct R(crate::R<BROWN_OUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BROWN_OUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BROWN_OUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BROWN_OUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BROWN_OUT` writer"]
pub struct W(crate::W<BROWN_OUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BROWN_OUT_SPEC>;
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
impl From<crate::W<BROWN_OUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BROWN_OUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_MEM_PID_CONF` reader - "]
pub struct RTC_MEM_PID_CONF_R(crate::FieldReader<u8, u8>);
impl RTC_MEM_PID_CONF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RTC_MEM_PID_CONF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_MEM_PID_CONF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_MEM_PID_CONF` writer - "]
pub struct RTC_MEM_PID_CONF_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_MEM_PID_CONF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `RTC_MEM_CRC_START` reader - "]
pub struct RTC_MEM_CRC_START_R(crate::FieldReader<bool, bool>);
impl RTC_MEM_CRC_START_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_MEM_CRC_START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_MEM_CRC_START_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_MEM_CRC_START` writer - "]
pub struct RTC_MEM_CRC_START_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_MEM_CRC_START_W<'a> {
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
#[doc = "Field `RTC_MEM_CRC_ADDR` reader - "]
pub struct RTC_MEM_CRC_ADDR_R(crate::FieldReader<u16, u16>);
impl RTC_MEM_CRC_ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        RTC_MEM_CRC_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_MEM_CRC_ADDR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_MEM_CRC_ADDR` writer - "]
pub struct RTC_MEM_CRC_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_MEM_CRC_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 9)) | ((value as u32 & 0x07ff) << 9);
        self.w
    }
}
#[doc = "Field `CLOSE_FLASH_ENA` reader - enable close flash when brown out happens"]
pub struct CLOSE_FLASH_ENA_R(crate::FieldReader<bool, bool>);
impl CLOSE_FLASH_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLOSE_FLASH_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLOSE_FLASH_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLOSE_FLASH_ENA` writer - enable close flash when brown out happens"]
pub struct CLOSE_FLASH_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CLOSE_FLASH_ENA_W<'a> {
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
#[doc = "Field `PD_RF_ENA` reader - enable power down RF when brown out happens"]
pub struct PD_RF_ENA_R(crate::FieldReader<bool, bool>);
impl PD_RF_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PD_RF_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD_RF_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD_RF_ENA` writer - enable power down RF when brown out happens"]
pub struct PD_RF_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_RF_ENA_W<'a> {
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
#[doc = "Field `RST_WAIT` reader - brown out reset wait cycles"]
pub struct RST_WAIT_R(crate::FieldReader<u16, u16>);
impl RST_WAIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        RST_WAIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RST_WAIT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RST_WAIT` writer - brown out reset wait cycles"]
pub struct RST_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> RST_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | ((value as u32 & 0x03ff) << 16);
        self.w
    }
}
#[doc = "Field `RTC_MEM_CRC_LEN` reader - "]
pub struct RTC_MEM_CRC_LEN_R(crate::FieldReader<u16, u16>);
impl RTC_MEM_CRC_LEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        RTC_MEM_CRC_LEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_MEM_CRC_LEN_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_MEM_CRC_LEN` writer - "]
pub struct RTC_MEM_CRC_LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_MEM_CRC_LEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 20)) | ((value as u32 & 0x07ff) << 20);
        self.w
    }
}
#[doc = "Field `RST_ENA` reader - enable brown out reset"]
pub struct RST_ENA_R(crate::FieldReader<bool, bool>);
impl RST_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RST_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RST_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RST_ENA` writer - enable brown out reset"]
pub struct RST_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> RST_ENA_W<'a> {
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
#[doc = "Field `DBROWN_OUT_THRES` reader - brown out threshold"]
pub struct DBROWN_OUT_THRES_R(crate::FieldReader<u8, u8>);
impl DBROWN_OUT_THRES_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DBROWN_OUT_THRES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBROWN_OUT_THRES_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBROWN_OUT_THRES` writer - brown out threshold"]
pub struct DBROWN_OUT_THRES_W<'a> {
    w: &'a mut W,
}
impl<'a> DBROWN_OUT_THRES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 27)) | ((value as u32 & 0x07) << 27);
        self.w
    }
}
#[doc = "Field `ENA` reader - enable brown out"]
pub struct ENA_R(crate::FieldReader<bool, bool>);
impl ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENA` writer - enable brown out"]
pub struct ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA_W<'a> {
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
#[doc = "Field `DET` reader - brown out detect"]
pub struct DET_R(crate::FieldReader<bool, bool>);
impl DET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_MEM_CRC_FINISH` reader - "]
pub struct RTC_MEM_CRC_FINISH_R(crate::FieldReader<bool, bool>);
impl RTC_MEM_CRC_FINISH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_MEM_CRC_FINISH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_MEM_CRC_FINISH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_MEM_CRC_FINISH` writer - "]
pub struct RTC_MEM_CRC_FINISH_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_MEM_CRC_FINISH_W<'a> {
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
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn rtc_mem_pid_conf(&self) -> RTC_MEM_PID_CONF_R {
        RTC_MEM_PID_CONF_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rtc_mem_crc_start(&self) -> RTC_MEM_CRC_START_R {
        RTC_MEM_CRC_START_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 9:19"]
    #[inline(always)]
    pub fn rtc_mem_crc_addr(&self) -> RTC_MEM_CRC_ADDR_R {
        RTC_MEM_CRC_ADDR_R::new(((self.bits >> 9) & 0x07ff) as u16)
    }
    #[doc = "Bit 14 - enable close flash when brown out happens"]
    #[inline(always)]
    pub fn close_flash_ena(&self) -> CLOSE_FLASH_ENA_R {
        CLOSE_FLASH_ENA_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - enable power down RF when brown out happens"]
    #[inline(always)]
    pub fn pd_rf_ena(&self) -> PD_RF_ENA_R {
        PD_RF_ENA_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:25 - brown out reset wait cycles"]
    #[inline(always)]
    pub fn rst_wait(&self) -> RST_WAIT_R {
        RST_WAIT_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:30"]
    #[inline(always)]
    pub fn rtc_mem_crc_len(&self) -> RTC_MEM_CRC_LEN_R {
        RTC_MEM_CRC_LEN_R::new(((self.bits >> 20) & 0x07ff) as u16)
    }
    #[doc = "Bit 26 - enable brown out reset"]
    #[inline(always)]
    pub fn rst_ena(&self) -> RST_ENA_R {
        RST_ENA_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 27:29 - brown out threshold"]
    #[inline(always)]
    pub fn dbrown_out_thres(&self) -> DBROWN_OUT_THRES_R {
        DBROWN_OUT_THRES_R::new(((self.bits >> 27) & 0x07) as u8)
    }
    #[doc = "Bit 30 - enable brown out"]
    #[inline(always)]
    pub fn ena(&self) -> ENA_R {
        ENA_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - brown out detect"]
    #[inline(always)]
    pub fn det(&self) -> DET_R {
        DET_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rtc_mem_crc_finish(&self) -> RTC_MEM_CRC_FINISH_R {
        RTC_MEM_CRC_FINISH_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn rtc_mem_pid_conf(&mut self) -> RTC_MEM_PID_CONF_W {
        RTC_MEM_PID_CONF_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rtc_mem_crc_start(&mut self) -> RTC_MEM_CRC_START_W {
        RTC_MEM_CRC_START_W { w: self }
    }
    #[doc = "Bits 9:19"]
    #[inline(always)]
    pub fn rtc_mem_crc_addr(&mut self) -> RTC_MEM_CRC_ADDR_W {
        RTC_MEM_CRC_ADDR_W { w: self }
    }
    #[doc = "Bit 14 - enable close flash when brown out happens"]
    #[inline(always)]
    pub fn close_flash_ena(&mut self) -> CLOSE_FLASH_ENA_W {
        CLOSE_FLASH_ENA_W { w: self }
    }
    #[doc = "Bit 15 - enable power down RF when brown out happens"]
    #[inline(always)]
    pub fn pd_rf_ena(&mut self) -> PD_RF_ENA_W {
        PD_RF_ENA_W { w: self }
    }
    #[doc = "Bits 16:25 - brown out reset wait cycles"]
    #[inline(always)]
    pub fn rst_wait(&mut self) -> RST_WAIT_W {
        RST_WAIT_W { w: self }
    }
    #[doc = "Bits 20:30"]
    #[inline(always)]
    pub fn rtc_mem_crc_len(&mut self) -> RTC_MEM_CRC_LEN_W {
        RTC_MEM_CRC_LEN_W { w: self }
    }
    #[doc = "Bit 26 - enable brown out reset"]
    #[inline(always)]
    pub fn rst_ena(&mut self) -> RST_ENA_W {
        RST_ENA_W { w: self }
    }
    #[doc = "Bits 27:29 - brown out threshold"]
    #[inline(always)]
    pub fn dbrown_out_thres(&mut self) -> DBROWN_OUT_THRES_W {
        DBROWN_OUT_THRES_W { w: self }
    }
    #[doc = "Bit 30 - enable brown out"]
    #[inline(always)]
    pub fn ena(&mut self) -> ENA_W {
        ENA_W { w: self }
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rtc_mem_crc_finish(&mut self) -> RTC_MEM_CRC_FINISH_W {
        RTC_MEM_CRC_FINISH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brown_out](index.html) module"]
pub struct BROWN_OUT_SPEC;
impl crate::RegisterSpec for BROWN_OUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [brown_out::R](R) reader structure"]
impl crate::Readable for BROWN_OUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [brown_out::W](W) writer structure"]
impl crate::Writable for BROWN_OUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BROWN_OUT to value 0x13ff_0000"]
impl crate::Resettable for BROWN_OUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x13ff_0000
    }
}
