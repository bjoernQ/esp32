#[doc = "Register `SAR_MEAS_START2` reader"]
pub struct R(crate::R<SAR_MEAS_START2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_MEAS_START2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_MEAS_START2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_MEAS_START2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_MEAS_START2` writer"]
pub struct W(crate::W<SAR_MEAS_START2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_MEAS_START2_SPEC>;
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
impl From<crate::W<SAR_MEAS_START2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_MEAS_START2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MEAS2_DATA_SAR` reader - SAR ADC2 data"]
pub struct MEAS2_DATA_SAR_R(crate::FieldReader<u16, u16>);
impl MEAS2_DATA_SAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        MEAS2_DATA_SAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEAS2_DATA_SAR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEAS2_DONE_SAR` reader - SAR ADC2 conversion done indication"]
pub struct MEAS2_DONE_SAR_R(crate::FieldReader<bool, bool>);
impl MEAS2_DONE_SAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEAS2_DONE_SAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEAS2_DONE_SAR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEAS2_START_SAR` reader - SAR ADC2 controller (in RTC) starts conversion only active when reg_meas2_start_force = 1"]
pub struct MEAS2_START_SAR_R(crate::FieldReader<bool, bool>);
impl MEAS2_START_SAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEAS2_START_SAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEAS2_START_SAR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEAS2_START_SAR` writer - SAR ADC2 controller (in RTC) starts conversion only active when reg_meas2_start_force = 1"]
pub struct MEAS2_START_SAR_W<'a> {
    w: &'a mut W,
}
impl<'a> MEAS2_START_SAR_W<'a> {
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
#[doc = "Field `MEAS2_START_FORCE` reader - 1: SAR ADC2 controller (in RTC) is started by SW 0: SAR ADC2 controller is started by ULP-coprocessor"]
pub struct MEAS2_START_FORCE_R(crate::FieldReader<bool, bool>);
impl MEAS2_START_FORCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEAS2_START_FORCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEAS2_START_FORCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEAS2_START_FORCE` writer - 1: SAR ADC2 controller (in RTC) is started by SW 0: SAR ADC2 controller is started by ULP-coprocessor"]
pub struct MEAS2_START_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> MEAS2_START_FORCE_W<'a> {
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
#[doc = "Field `SAR2_EN_PAD` reader - SAR ADC2 pad enable bitmap only active when reg_sar2_en_pad_force = 1"]
pub struct SAR2_EN_PAD_R(crate::FieldReader<u16, u16>);
impl SAR2_EN_PAD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        SAR2_EN_PAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR2_EN_PAD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR2_EN_PAD` writer - SAR ADC2 pad enable bitmap only active when reg_sar2_en_pad_force = 1"]
pub struct SAR2_EN_PAD_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR2_EN_PAD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 19)) | ((value as u32 & 0x0fff) << 19);
        self.w
    }
}
#[doc = "Field `SAR2_EN_PAD_FORCE` reader - 1: SAR ADC2 pad enable bitmap is controlled by SW 0: SAR ADC2 pad enable bitmap is controlled by ULP-coprocessor"]
pub struct SAR2_EN_PAD_FORCE_R(crate::FieldReader<bool, bool>);
impl SAR2_EN_PAD_FORCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SAR2_EN_PAD_FORCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR2_EN_PAD_FORCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR2_EN_PAD_FORCE` writer - 1: SAR ADC2 pad enable bitmap is controlled by SW 0: SAR ADC2 pad enable bitmap is controlled by ULP-coprocessor"]
pub struct SAR2_EN_PAD_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR2_EN_PAD_FORCE_W<'a> {
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
    #[doc = "Bits 0:15 - SAR ADC2 data"]
    #[inline(always)]
    pub fn meas2_data_sar(&self) -> MEAS2_DATA_SAR_R {
        MEAS2_DATA_SAR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - SAR ADC2 conversion done indication"]
    #[inline(always)]
    pub fn meas2_done_sar(&self) -> MEAS2_DONE_SAR_R {
        MEAS2_DONE_SAR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - SAR ADC2 controller (in RTC) starts conversion only active when reg_meas2_start_force = 1"]
    #[inline(always)]
    pub fn meas2_start_sar(&self) -> MEAS2_START_SAR_R {
        MEAS2_START_SAR_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - 1: SAR ADC2 controller (in RTC) is started by SW 0: SAR ADC2 controller is started by ULP-coprocessor"]
    #[inline(always)]
    pub fn meas2_start_force(&self) -> MEAS2_START_FORCE_R {
        MEAS2_START_FORCE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 19:30 - SAR ADC2 pad enable bitmap only active when reg_sar2_en_pad_force = 1"]
    #[inline(always)]
    pub fn sar2_en_pad(&self) -> SAR2_EN_PAD_R {
        SAR2_EN_PAD_R::new(((self.bits >> 19) & 0x0fff) as u16)
    }
    #[doc = "Bit 31 - 1: SAR ADC2 pad enable bitmap is controlled by SW 0: SAR ADC2 pad enable bitmap is controlled by ULP-coprocessor"]
    #[inline(always)]
    pub fn sar2_en_pad_force(&self) -> SAR2_EN_PAD_FORCE_R {
        SAR2_EN_PAD_FORCE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 17 - SAR ADC2 controller (in RTC) starts conversion only active when reg_meas2_start_force = 1"]
    #[inline(always)]
    pub fn meas2_start_sar(&mut self) -> MEAS2_START_SAR_W {
        MEAS2_START_SAR_W { w: self }
    }
    #[doc = "Bit 18 - 1: SAR ADC2 controller (in RTC) is started by SW 0: SAR ADC2 controller is started by ULP-coprocessor"]
    #[inline(always)]
    pub fn meas2_start_force(&mut self) -> MEAS2_START_FORCE_W {
        MEAS2_START_FORCE_W { w: self }
    }
    #[doc = "Bits 19:30 - SAR ADC2 pad enable bitmap only active when reg_sar2_en_pad_force = 1"]
    #[inline(always)]
    pub fn sar2_en_pad(&mut self) -> SAR2_EN_PAD_W {
        SAR2_EN_PAD_W { w: self }
    }
    #[doc = "Bit 31 - 1: SAR ADC2 pad enable bitmap is controlled by SW 0: SAR ADC2 pad enable bitmap is controlled by ULP-coprocessor"]
    #[inline(always)]
    pub fn sar2_en_pad_force(&mut self) -> SAR2_EN_PAD_FORCE_W {
        SAR2_EN_PAD_FORCE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_meas_start2](index.html) module"]
pub struct SAR_MEAS_START2_SPEC;
impl crate::RegisterSpec for SAR_MEAS_START2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_meas_start2::R](R) reader structure"]
impl crate::Readable for SAR_MEAS_START2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_meas_start2::W](W) writer structure"]
impl crate::Writable for SAR_MEAS_START2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAR_MEAS_START2 to value 0"]
impl crate::Resettable for SAR_MEAS_START2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
