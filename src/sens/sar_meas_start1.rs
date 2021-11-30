#[doc = "Register `SAR_MEAS_START1` reader"]
pub struct R(crate::R<SAR_MEAS_START1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_MEAS_START1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_MEAS_START1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_MEAS_START1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_MEAS_START1` writer"]
pub struct W(crate::W<SAR_MEAS_START1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_MEAS_START1_SPEC>;
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
impl From<crate::W<SAR_MEAS_START1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_MEAS_START1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MEAS1_DATA_SAR` reader - SAR ADC1 data"]
pub struct MEAS1_DATA_SAR_R(crate::FieldReader<u16, u16>);
impl MEAS1_DATA_SAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        MEAS1_DATA_SAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEAS1_DATA_SAR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEAS1_DONE_SAR` reader - SAR ADC1 conversion done indication"]
pub struct MEAS1_DONE_SAR_R(crate::FieldReader<bool, bool>);
impl MEAS1_DONE_SAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEAS1_DONE_SAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEAS1_DONE_SAR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEAS1_START_SAR` reader - SAR ADC1 controller (in RTC) starts conversion only active when reg_meas1_start_force = 1"]
pub struct MEAS1_START_SAR_R(crate::FieldReader<bool, bool>);
impl MEAS1_START_SAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEAS1_START_SAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEAS1_START_SAR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEAS1_START_SAR` writer - SAR ADC1 controller (in RTC) starts conversion only active when reg_meas1_start_force = 1"]
pub struct MEAS1_START_SAR_W<'a> {
    w: &'a mut W,
}
impl<'a> MEAS1_START_SAR_W<'a> {
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
#[doc = "Field `MEAS1_START_FORCE` reader - 1: SAR ADC1 controller (in RTC) is started by SW 0: SAR ADC1 controller is started by ULP-coprocessor"]
pub struct MEAS1_START_FORCE_R(crate::FieldReader<bool, bool>);
impl MEAS1_START_FORCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEAS1_START_FORCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEAS1_START_FORCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEAS1_START_FORCE` writer - 1: SAR ADC1 controller (in RTC) is started by SW 0: SAR ADC1 controller is started by ULP-coprocessor"]
pub struct MEAS1_START_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> MEAS1_START_FORCE_W<'a> {
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
#[doc = "Field `SAR1_EN_PAD` reader - SAR ADC1 pad enable bitmap only active when reg_sar1_en_pad_force = 1"]
pub struct SAR1_EN_PAD_R(crate::FieldReader<u16, u16>);
impl SAR1_EN_PAD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        SAR1_EN_PAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR1_EN_PAD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR1_EN_PAD` writer - SAR ADC1 pad enable bitmap only active when reg_sar1_en_pad_force = 1"]
pub struct SAR1_EN_PAD_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR1_EN_PAD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 19)) | ((value as u32 & 0x0fff) << 19);
        self.w
    }
}
#[doc = "Field `SAR1_EN_PAD_FORCE` reader - 1: SAR ADC1 pad enable bitmap is controlled by SW 0: SAR ADC1 pad enable bitmap is controlled by ULP-coprocessor"]
pub struct SAR1_EN_PAD_FORCE_R(crate::FieldReader<bool, bool>);
impl SAR1_EN_PAD_FORCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SAR1_EN_PAD_FORCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR1_EN_PAD_FORCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR1_EN_PAD_FORCE` writer - 1: SAR ADC1 pad enable bitmap is controlled by SW 0: SAR ADC1 pad enable bitmap is controlled by ULP-coprocessor"]
pub struct SAR1_EN_PAD_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR1_EN_PAD_FORCE_W<'a> {
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
    #[doc = "Bits 0:15 - SAR ADC1 data"]
    #[inline(always)]
    pub fn meas1_data_sar(&self) -> MEAS1_DATA_SAR_R {
        MEAS1_DATA_SAR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - SAR ADC1 conversion done indication"]
    #[inline(always)]
    pub fn meas1_done_sar(&self) -> MEAS1_DONE_SAR_R {
        MEAS1_DONE_SAR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - SAR ADC1 controller (in RTC) starts conversion only active when reg_meas1_start_force = 1"]
    #[inline(always)]
    pub fn meas1_start_sar(&self) -> MEAS1_START_SAR_R {
        MEAS1_START_SAR_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - 1: SAR ADC1 controller (in RTC) is started by SW 0: SAR ADC1 controller is started by ULP-coprocessor"]
    #[inline(always)]
    pub fn meas1_start_force(&self) -> MEAS1_START_FORCE_R {
        MEAS1_START_FORCE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 19:30 - SAR ADC1 pad enable bitmap only active when reg_sar1_en_pad_force = 1"]
    #[inline(always)]
    pub fn sar1_en_pad(&self) -> SAR1_EN_PAD_R {
        SAR1_EN_PAD_R::new(((self.bits >> 19) & 0x0fff) as u16)
    }
    #[doc = "Bit 31 - 1: SAR ADC1 pad enable bitmap is controlled by SW 0: SAR ADC1 pad enable bitmap is controlled by ULP-coprocessor"]
    #[inline(always)]
    pub fn sar1_en_pad_force(&self) -> SAR1_EN_PAD_FORCE_R {
        SAR1_EN_PAD_FORCE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 17 - SAR ADC1 controller (in RTC) starts conversion only active when reg_meas1_start_force = 1"]
    #[inline(always)]
    pub fn meas1_start_sar(&mut self) -> MEAS1_START_SAR_W {
        MEAS1_START_SAR_W { w: self }
    }
    #[doc = "Bit 18 - 1: SAR ADC1 controller (in RTC) is started by SW 0: SAR ADC1 controller is started by ULP-coprocessor"]
    #[inline(always)]
    pub fn meas1_start_force(&mut self) -> MEAS1_START_FORCE_W {
        MEAS1_START_FORCE_W { w: self }
    }
    #[doc = "Bits 19:30 - SAR ADC1 pad enable bitmap only active when reg_sar1_en_pad_force = 1"]
    #[inline(always)]
    pub fn sar1_en_pad(&mut self) -> SAR1_EN_PAD_W {
        SAR1_EN_PAD_W { w: self }
    }
    #[doc = "Bit 31 - 1: SAR ADC1 pad enable bitmap is controlled by SW 0: SAR ADC1 pad enable bitmap is controlled by ULP-coprocessor"]
    #[inline(always)]
    pub fn sar1_en_pad_force(&mut self) -> SAR1_EN_PAD_FORCE_W {
        SAR1_EN_PAD_FORCE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_meas_start1](index.html) module"]
pub struct SAR_MEAS_START1_SPEC;
impl crate::RegisterSpec for SAR_MEAS_START1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_meas_start1::R](R) reader structure"]
impl crate::Readable for SAR_MEAS_START1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_meas_start1::W](W) writer structure"]
impl crate::Writable for SAR_MEAS_START1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAR_MEAS_START1 to value 0"]
impl crate::Resettable for SAR_MEAS_START1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
