#[doc = "Register `CTRL2` reader"]
pub struct R(crate::R<CTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL2` writer"]
pub struct W(crate::W<CTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL2_SPEC>;
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
impl From<crate::W<CTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SETUP_TIME` reader - (cycles-1) of ¡°prepare¡± phase by spi clock, this bits combined with spi_cs_setup bit."]
pub struct SETUP_TIME_R(crate::FieldReader<u8, u8>);
impl SETUP_TIME_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SETUP_TIME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SETUP_TIME_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETUP_TIME` writer - (cycles-1) of ¡°prepare¡± phase by spi clock, this bits combined with spi_cs_setup bit."]
pub struct SETUP_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> SETUP_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `HOLD_TIME` reader - delay cycles of cs pin by spi clock, this bits combined with spi_cs_hold bit."]
pub struct HOLD_TIME_R(crate::FieldReader<u8, u8>);
impl HOLD_TIME_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        HOLD_TIME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOLD_TIME_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOLD_TIME` writer - delay cycles of cs pin by spi clock, this bits combined with spi_cs_hold bit."]
pub struct HOLD_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> HOLD_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `CK_OUT_LOW_MODE` reader - modify spi clock duty ratio when the value is lager than 8, the bits are combined with spi_clkcnt_N bits and spi_clkcnt_L bits."]
pub struct CK_OUT_LOW_MODE_R(crate::FieldReader<u8, u8>);
impl CK_OUT_LOW_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CK_OUT_LOW_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CK_OUT_LOW_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CK_OUT_LOW_MODE` writer - modify spi clock duty ratio when the value is lager than 8, the bits are combined with spi_clkcnt_N bits and spi_clkcnt_L bits."]
pub struct CK_OUT_LOW_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CK_OUT_LOW_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `CK_OUT_HIGH_MODE` reader - modify spi clock duty ratio when the value is lager than 8, the bits are combined with spi_clkcnt_N bits and spi_clkcnt_H bits."]
pub struct CK_OUT_HIGH_MODE_R(crate::FieldReader<u8, u8>);
impl CK_OUT_HIGH_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CK_OUT_HIGH_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CK_OUT_HIGH_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CK_OUT_HIGH_MODE` writer - modify spi clock duty ratio when the value is lager than 8, the bits are combined with spi_clkcnt_N bits and spi_clkcnt_H bits."]
pub struct CK_OUT_HIGH_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CK_OUT_HIGH_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `MISO_DELAY_MODE` reader - MISO signals are delayed by spi_clk. 0: zero 1: if spi_ck_out_edge or spi_ck_i_edge is set 1 delayed by half cycle else delayed by one cycle 2: if spi_ck_out_edge or spi_ck_i_edge is set 1 delayed by one cycle else delayed by half cycle 3: delayed one cycle"]
pub struct MISO_DELAY_MODE_R(crate::FieldReader<u8, u8>);
impl MISO_DELAY_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MISO_DELAY_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MISO_DELAY_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MISO_DELAY_MODE` writer - MISO signals are delayed by spi_clk. 0: zero 1: if spi_ck_out_edge or spi_ck_i_edge is set 1 delayed by half cycle else delayed by one cycle 2: if spi_ck_out_edge or spi_ck_i_edge is set 1 delayed by one cycle else delayed by half cycle 3: delayed one cycle"]
pub struct MISO_DELAY_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MISO_DELAY_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `MISO_DELAY_NUM` reader - MISO signals are delayed by system clock cycles"]
pub struct MISO_DELAY_NUM_R(crate::FieldReader<u8, u8>);
impl MISO_DELAY_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MISO_DELAY_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MISO_DELAY_NUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MISO_DELAY_NUM` writer - MISO signals are delayed by system clock cycles"]
pub struct MISO_DELAY_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> MISO_DELAY_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | ((value as u32 & 0x07) << 18);
        self.w
    }
}
#[doc = "Field `MOSI_DELAY_MODE` reader - MOSI signals are delayed by spi_clk. 0: zero 1: if spi_ck_out_edge or spi_ck_i_edge is set 1 delayed by half cycle else delayed by one cycle 2: if spi_ck_out_edge or spi_ck_i_edge is set 1 delayed by one cycle else delayed by half cycle 3: delayed one cycle"]
pub struct MOSI_DELAY_MODE_R(crate::FieldReader<u8, u8>);
impl MOSI_DELAY_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MOSI_DELAY_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MOSI_DELAY_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MOSI_DELAY_MODE` writer - MOSI signals are delayed by spi_clk. 0: zero 1: if spi_ck_out_edge or spi_ck_i_edge is set 1 delayed by half cycle else delayed by one cycle 2: if spi_ck_out_edge or spi_ck_i_edge is set 1 delayed by one cycle else delayed by half cycle 3: delayed one cycle"]
pub struct MOSI_DELAY_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MOSI_DELAY_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | ((value as u32 & 0x03) << 21);
        self.w
    }
}
#[doc = "Field `MOSI_DELAY_NUM` reader - MOSI signals are delayed by system clock cycles"]
pub struct MOSI_DELAY_NUM_R(crate::FieldReader<u8, u8>);
impl MOSI_DELAY_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MOSI_DELAY_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MOSI_DELAY_NUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MOSI_DELAY_NUM` writer - MOSI signals are delayed by system clock cycles"]
pub struct MOSI_DELAY_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> MOSI_DELAY_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 23)) | ((value as u32 & 0x07) << 23);
        self.w
    }
}
#[doc = "Field `CS_DELAY_MODE` reader - spi_cs signal is delayed by spi_clk . 0: zero 1: if spi_ck_out_edge or spi_ck_i_edge is set 1 delayed by half cycle else delayed by one cycle 2: if spi_ck_out_edge or spi_ck_i_edge is set 1 delayed by one cycle else delayed by half cycle 3: delayed one cycle"]
pub struct CS_DELAY_MODE_R(crate::FieldReader<u8, u8>);
impl CS_DELAY_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CS_DELAY_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CS_DELAY_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CS_DELAY_MODE` writer - spi_cs signal is delayed by spi_clk . 0: zero 1: if spi_ck_out_edge or spi_ck_i_edge is set 1 delayed by half cycle else delayed by one cycle 2: if spi_ck_out_edge or spi_ck_i_edge is set 1 delayed by one cycle else delayed by half cycle 3: delayed one cycle"]
pub struct CS_DELAY_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CS_DELAY_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | ((value as u32 & 0x03) << 26);
        self.w
    }
}
#[doc = "Field `CS_DELAY_NUM` reader - spi_cs signal is delayed by system clock cycles"]
pub struct CS_DELAY_NUM_R(crate::FieldReader<u8, u8>);
impl CS_DELAY_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CS_DELAY_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CS_DELAY_NUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CS_DELAY_NUM` writer - spi_cs signal is delayed by system clock cycles"]
pub struct CS_DELAY_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> CS_DELAY_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - (cycles-1) of ¡°prepare¡± phase by spi clock, this bits combined with spi_cs_setup bit."]
    #[inline(always)]
    pub fn setup_time(&self) -> SETUP_TIME_R {
        SETUP_TIME_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - delay cycles of cs pin by spi clock, this bits combined with spi_cs_hold bit."]
    #[inline(always)]
    pub fn hold_time(&self) -> HOLD_TIME_R {
        HOLD_TIME_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - modify spi clock duty ratio when the value is lager than 8, the bits are combined with spi_clkcnt_N bits and spi_clkcnt_L bits."]
    #[inline(always)]
    pub fn ck_out_low_mode(&self) -> CK_OUT_LOW_MODE_R {
        CK_OUT_LOW_MODE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - modify spi clock duty ratio when the value is lager than 8, the bits are combined with spi_clkcnt_N bits and spi_clkcnt_H bits."]
    #[inline(always)]
    pub fn ck_out_high_mode(&self) -> CK_OUT_HIGH_MODE_R {
        CK_OUT_HIGH_MODE_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - MISO signals are delayed by spi_clk. 0: zero 1: if spi_ck_out_edge or spi_ck_i_edge is set 1 delayed by half cycle else delayed by one cycle 2: if spi_ck_out_edge or spi_ck_i_edge is set 1 delayed by one cycle else delayed by half cycle 3: delayed one cycle"]
    #[inline(always)]
    pub fn miso_delay_mode(&self) -> MISO_DELAY_MODE_R {
        MISO_DELAY_MODE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:20 - MISO signals are delayed by system clock cycles"]
    #[inline(always)]
    pub fn miso_delay_num(&self) -> MISO_DELAY_NUM_R {
        MISO_DELAY_NUM_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bits 21:22 - MOSI signals are delayed by spi_clk. 0: zero 1: if spi_ck_out_edge or spi_ck_i_edge is set 1 delayed by half cycle else delayed by one cycle 2: if spi_ck_out_edge or spi_ck_i_edge is set 1 delayed by one cycle else delayed by half cycle 3: delayed one cycle"]
    #[inline(always)]
    pub fn mosi_delay_mode(&self) -> MOSI_DELAY_MODE_R {
        MOSI_DELAY_MODE_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bits 23:25 - MOSI signals are delayed by system clock cycles"]
    #[inline(always)]
    pub fn mosi_delay_num(&self) -> MOSI_DELAY_NUM_R {
        MOSI_DELAY_NUM_R::new(((self.bits >> 23) & 0x07) as u8)
    }
    #[doc = "Bits 26:27 - spi_cs signal is delayed by spi_clk . 0: zero 1: if spi_ck_out_edge or spi_ck_i_edge is set 1 delayed by half cycle else delayed by one cycle 2: if spi_ck_out_edge or spi_ck_i_edge is set 1 delayed by one cycle else delayed by half cycle 3: delayed one cycle"]
    #[inline(always)]
    pub fn cs_delay_mode(&self) -> CS_DELAY_MODE_R {
        CS_DELAY_MODE_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:31 - spi_cs signal is delayed by system clock cycles"]
    #[inline(always)]
    pub fn cs_delay_num(&self) -> CS_DELAY_NUM_R {
        CS_DELAY_NUM_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - (cycles-1) of ¡°prepare¡± phase by spi clock, this bits combined with spi_cs_setup bit."]
    #[inline(always)]
    pub fn setup_time(&mut self) -> SETUP_TIME_W {
        SETUP_TIME_W { w: self }
    }
    #[doc = "Bits 4:7 - delay cycles of cs pin by spi clock, this bits combined with spi_cs_hold bit."]
    #[inline(always)]
    pub fn hold_time(&mut self) -> HOLD_TIME_W {
        HOLD_TIME_W { w: self }
    }
    #[doc = "Bits 8:11 - modify spi clock duty ratio when the value is lager than 8, the bits are combined with spi_clkcnt_N bits and spi_clkcnt_L bits."]
    #[inline(always)]
    pub fn ck_out_low_mode(&mut self) -> CK_OUT_LOW_MODE_W {
        CK_OUT_LOW_MODE_W { w: self }
    }
    #[doc = "Bits 12:15 - modify spi clock duty ratio when the value is lager than 8, the bits are combined with spi_clkcnt_N bits and spi_clkcnt_H bits."]
    #[inline(always)]
    pub fn ck_out_high_mode(&mut self) -> CK_OUT_HIGH_MODE_W {
        CK_OUT_HIGH_MODE_W { w: self }
    }
    #[doc = "Bits 16:17 - MISO signals are delayed by spi_clk. 0: zero 1: if spi_ck_out_edge or spi_ck_i_edge is set 1 delayed by half cycle else delayed by one cycle 2: if spi_ck_out_edge or spi_ck_i_edge is set 1 delayed by one cycle else delayed by half cycle 3: delayed one cycle"]
    #[inline(always)]
    pub fn miso_delay_mode(&mut self) -> MISO_DELAY_MODE_W {
        MISO_DELAY_MODE_W { w: self }
    }
    #[doc = "Bits 18:20 - MISO signals are delayed by system clock cycles"]
    #[inline(always)]
    pub fn miso_delay_num(&mut self) -> MISO_DELAY_NUM_W {
        MISO_DELAY_NUM_W { w: self }
    }
    #[doc = "Bits 21:22 - MOSI signals are delayed by spi_clk. 0: zero 1: if spi_ck_out_edge or spi_ck_i_edge is set 1 delayed by half cycle else delayed by one cycle 2: if spi_ck_out_edge or spi_ck_i_edge is set 1 delayed by one cycle else delayed by half cycle 3: delayed one cycle"]
    #[inline(always)]
    pub fn mosi_delay_mode(&mut self) -> MOSI_DELAY_MODE_W {
        MOSI_DELAY_MODE_W { w: self }
    }
    #[doc = "Bits 23:25 - MOSI signals are delayed by system clock cycles"]
    #[inline(always)]
    pub fn mosi_delay_num(&mut self) -> MOSI_DELAY_NUM_W {
        MOSI_DELAY_NUM_W { w: self }
    }
    #[doc = "Bits 26:27 - spi_cs signal is delayed by spi_clk . 0: zero 1: if spi_ck_out_edge or spi_ck_i_edge is set 1 delayed by half cycle else delayed by one cycle 2: if spi_ck_out_edge or spi_ck_i_edge is set 1 delayed by one cycle else delayed by half cycle 3: delayed one cycle"]
    #[inline(always)]
    pub fn cs_delay_mode(&mut self) -> CS_DELAY_MODE_W {
        CS_DELAY_MODE_W { w: self }
    }
    #[doc = "Bits 28:31 - spi_cs signal is delayed by system clock cycles"]
    #[inline(always)]
    pub fn cs_delay_num(&mut self) -> CS_DELAY_NUM_W {
        CS_DELAY_NUM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl2](index.html) module"]
pub struct CTRL2_SPEC;
impl crate::RegisterSpec for CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl2::R](R) reader structure"]
impl crate::Readable for CTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl2::W](W) writer structure"]
impl crate::Writable for CTRL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL2 to value 0x11"]
impl crate::Resettable for CTRL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x11
    }
}
