#[doc = "Register `SAR_START_FORCE` reader"]
pub struct R(crate::R<SAR_START_FORCE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_START_FORCE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_START_FORCE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_START_FORCE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_START_FORCE` writer"]
pub struct W(crate::W<SAR_START_FORCE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_START_FORCE_SPEC>;
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
impl From<crate::W<SAR_START_FORCE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_START_FORCE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAR1_BIT_WIDTH` reader - 00: 9 bit 01: 10 bits 10: 11bits 11: 12bits"]
pub struct SAR1_BIT_WIDTH_R(crate::FieldReader<u8, u8>);
impl SAR1_BIT_WIDTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SAR1_BIT_WIDTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR1_BIT_WIDTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR1_BIT_WIDTH` writer - 00: 9 bit 01: 10 bits 10: 11bits 11: 12bits"]
pub struct SAR1_BIT_WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR1_BIT_WIDTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `SAR2_BIT_WIDTH` reader - 00: 9 bit 01: 10 bits 10: 11bits 11: 12bits"]
pub struct SAR2_BIT_WIDTH_R(crate::FieldReader<u8, u8>);
impl SAR2_BIT_WIDTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SAR2_BIT_WIDTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR2_BIT_WIDTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR2_BIT_WIDTH` writer - 00: 9 bit 01: 10 bits 10: 11bits 11: 12bits"]
pub struct SAR2_BIT_WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR2_BIT_WIDTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `SAR2_EN_TEST` reader - SAR2_EN_TEST only active when reg_sar2_dig_force = 0"]
pub struct SAR2_EN_TEST_R(crate::FieldReader<bool, bool>);
impl SAR2_EN_TEST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SAR2_EN_TEST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR2_EN_TEST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR2_EN_TEST` writer - SAR2_EN_TEST only active when reg_sar2_dig_force = 0"]
pub struct SAR2_EN_TEST_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR2_EN_TEST_W<'a> {
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
#[doc = "Field `SAR2_PWDET_CCT` reader - SAR2_PWDET_CCT PA power detector capacitance tuning."]
pub struct SAR2_PWDET_CCT_R(crate::FieldReader<u8, u8>);
impl SAR2_PWDET_CCT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SAR2_PWDET_CCT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR2_PWDET_CCT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR2_PWDET_CCT` writer - SAR2_PWDET_CCT PA power detector capacitance tuning."]
pub struct SAR2_PWDET_CCT_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR2_PWDET_CCT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | ((value as u32 & 0x07) << 5);
        self.w
    }
}
#[doc = "Field `ULP_CP_FORCE_START_TOP` reader - 1: ULP-coprocessor is started by SW 0: ULP-coprocessor is started by timer"]
pub struct ULP_CP_FORCE_START_TOP_R(crate::FieldReader<bool, bool>);
impl ULP_CP_FORCE_START_TOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ULP_CP_FORCE_START_TOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ULP_CP_FORCE_START_TOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ULP_CP_FORCE_START_TOP` writer - 1: ULP-coprocessor is started by SW 0: ULP-coprocessor is started by timer"]
pub struct ULP_CP_FORCE_START_TOP_W<'a> {
    w: &'a mut W,
}
impl<'a> ULP_CP_FORCE_START_TOP_W<'a> {
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
#[doc = "Field `ULP_CP_START_TOP` reader - Write 1 to start ULP-coprocessor only active when reg_ulp_cp_force_start_top = 1"]
pub struct ULP_CP_START_TOP_R(crate::FieldReader<bool, bool>);
impl ULP_CP_START_TOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ULP_CP_START_TOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ULP_CP_START_TOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ULP_CP_START_TOP` writer - Write 1 to start ULP-coprocessor only active when reg_ulp_cp_force_start_top = 1"]
pub struct ULP_CP_START_TOP_W<'a> {
    w: &'a mut W,
}
impl<'a> ULP_CP_START_TOP_W<'a> {
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
#[doc = "Field `SARCLK_EN` reader - "]
pub struct SARCLK_EN_R(crate::FieldReader<bool, bool>);
impl SARCLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SARCLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SARCLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SARCLK_EN` writer - "]
pub struct SARCLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SARCLK_EN_W<'a> {
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
#[doc = "Field `PC_INIT` reader - initialized PC for ULP-coprocessor"]
pub struct PC_INIT_R(crate::FieldReader<u16, u16>);
impl PC_INIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        PC_INIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PC_INIT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PC_INIT` writer - initialized PC for ULP-coprocessor"]
pub struct PC_INIT_W<'a> {
    w: &'a mut W,
}
impl<'a> PC_INIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 11)) | ((value as u32 & 0x07ff) << 11);
        self.w
    }
}
#[doc = "Field `SAR2_STOP` reader - stop SAR ADC2 conversion"]
pub struct SAR2_STOP_R(crate::FieldReader<bool, bool>);
impl SAR2_STOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SAR2_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR2_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR2_STOP` writer - stop SAR ADC2 conversion"]
pub struct SAR2_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR2_STOP_W<'a> {
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
#[doc = "Field `SAR1_STOP` reader - stop SAR ADC1 conversion"]
pub struct SAR1_STOP_R(crate::FieldReader<bool, bool>);
impl SAR1_STOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SAR1_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR1_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR1_STOP` writer - stop SAR ADC1 conversion"]
pub struct SAR1_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR1_STOP_W<'a> {
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
#[doc = "Field `SAR2_PWDET_EN` reader - N/A"]
pub struct SAR2_PWDET_EN_R(crate::FieldReader<bool, bool>);
impl SAR2_PWDET_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SAR2_PWDET_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR2_PWDET_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR2_PWDET_EN` writer - N/A"]
pub struct SAR2_PWDET_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR2_PWDET_EN_W<'a> {
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
impl R {
    #[doc = "Bits 0:1 - 00: 9 bit 01: 10 bits 10: 11bits 11: 12bits"]
    #[inline(always)]
    pub fn sar1_bit_width(&self) -> SAR1_BIT_WIDTH_R {
        SAR1_BIT_WIDTH_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - 00: 9 bit 01: 10 bits 10: 11bits 11: 12bits"]
    #[inline(always)]
    pub fn sar2_bit_width(&self) -> SAR2_BIT_WIDTH_R {
        SAR2_BIT_WIDTH_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - SAR2_EN_TEST only active when reg_sar2_dig_force = 0"]
    #[inline(always)]
    pub fn sar2_en_test(&self) -> SAR2_EN_TEST_R {
        SAR2_EN_TEST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - SAR2_PWDET_CCT PA power detector capacitance tuning."]
    #[inline(always)]
    pub fn sar2_pwdet_cct(&self) -> SAR2_PWDET_CCT_R {
        SAR2_PWDET_CCT_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bit 8 - 1: ULP-coprocessor is started by SW 0: ULP-coprocessor is started by timer"]
    #[inline(always)]
    pub fn ulp_cp_force_start_top(&self) -> ULP_CP_FORCE_START_TOP_R {
        ULP_CP_FORCE_START_TOP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Write 1 to start ULP-coprocessor only active when reg_ulp_cp_force_start_top = 1"]
    #[inline(always)]
    pub fn ulp_cp_start_top(&self) -> ULP_CP_START_TOP_R {
        ULP_CP_START_TOP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn sarclk_en(&self) -> SARCLK_EN_R {
        SARCLK_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 11:21 - initialized PC for ULP-coprocessor"]
    #[inline(always)]
    pub fn pc_init(&self) -> PC_INIT_R {
        PC_INIT_R::new(((self.bits >> 11) & 0x07ff) as u16)
    }
    #[doc = "Bit 22 - stop SAR ADC2 conversion"]
    #[inline(always)]
    pub fn sar2_stop(&self) -> SAR2_STOP_R {
        SAR2_STOP_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - stop SAR ADC1 conversion"]
    #[inline(always)]
    pub fn sar1_stop(&self) -> SAR1_STOP_R {
        SAR1_STOP_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - N/A"]
    #[inline(always)]
    pub fn sar2_pwdet_en(&self) -> SAR2_PWDET_EN_R {
        SAR2_PWDET_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - 00: 9 bit 01: 10 bits 10: 11bits 11: 12bits"]
    #[inline(always)]
    pub fn sar1_bit_width(&mut self) -> SAR1_BIT_WIDTH_W {
        SAR1_BIT_WIDTH_W { w: self }
    }
    #[doc = "Bits 2:3 - 00: 9 bit 01: 10 bits 10: 11bits 11: 12bits"]
    #[inline(always)]
    pub fn sar2_bit_width(&mut self) -> SAR2_BIT_WIDTH_W {
        SAR2_BIT_WIDTH_W { w: self }
    }
    #[doc = "Bit 4 - SAR2_EN_TEST only active when reg_sar2_dig_force = 0"]
    #[inline(always)]
    pub fn sar2_en_test(&mut self) -> SAR2_EN_TEST_W {
        SAR2_EN_TEST_W { w: self }
    }
    #[doc = "Bits 5:7 - SAR2_PWDET_CCT PA power detector capacitance tuning."]
    #[inline(always)]
    pub fn sar2_pwdet_cct(&mut self) -> SAR2_PWDET_CCT_W {
        SAR2_PWDET_CCT_W { w: self }
    }
    #[doc = "Bit 8 - 1: ULP-coprocessor is started by SW 0: ULP-coprocessor is started by timer"]
    #[inline(always)]
    pub fn ulp_cp_force_start_top(&mut self) -> ULP_CP_FORCE_START_TOP_W {
        ULP_CP_FORCE_START_TOP_W { w: self }
    }
    #[doc = "Bit 9 - Write 1 to start ULP-coprocessor only active when reg_ulp_cp_force_start_top = 1"]
    #[inline(always)]
    pub fn ulp_cp_start_top(&mut self) -> ULP_CP_START_TOP_W {
        ULP_CP_START_TOP_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn sarclk_en(&mut self) -> SARCLK_EN_W {
        SARCLK_EN_W { w: self }
    }
    #[doc = "Bits 11:21 - initialized PC for ULP-coprocessor"]
    #[inline(always)]
    pub fn pc_init(&mut self) -> PC_INIT_W {
        PC_INIT_W { w: self }
    }
    #[doc = "Bit 22 - stop SAR ADC2 conversion"]
    #[inline(always)]
    pub fn sar2_stop(&mut self) -> SAR2_STOP_W {
        SAR2_STOP_W { w: self }
    }
    #[doc = "Bit 23 - stop SAR ADC1 conversion"]
    #[inline(always)]
    pub fn sar1_stop(&mut self) -> SAR1_STOP_W {
        SAR1_STOP_W { w: self }
    }
    #[doc = "Bit 24 - N/A"]
    #[inline(always)]
    pub fn sar2_pwdet_en(&mut self) -> SAR2_PWDET_EN_W {
        SAR2_PWDET_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_start_force](index.html) module"]
pub struct SAR_START_FORCE_SPEC;
impl crate::RegisterSpec for SAR_START_FORCE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_start_force::R](R) reader structure"]
impl crate::Readable for SAR_START_FORCE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_start_force::W](W) writer structure"]
impl crate::Writable for SAR_START_FORCE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAR_START_FORCE to value 0x0f"]
impl crate::Resettable for SAR_START_FORCE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f
    }
}
