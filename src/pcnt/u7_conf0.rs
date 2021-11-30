#[doc = "Register `U7_CONF0` reader"]
pub struct R(crate::R<U7_CONF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<U7_CONF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<U7_CONF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<U7_CONF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `U7_CONF0` writer"]
pub struct W(crate::W<U7_CONF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<U7_CONF0_SPEC>;
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
impl From<crate::W<U7_CONF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<U7_CONF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FILTER_THRES_U7` reader - This register is used to filter pluse whose width is smaller than this value for unit7."]
pub struct FILTER_THRES_U7_R(crate::FieldReader<u16, u16>);
impl FILTER_THRES_U7_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        FILTER_THRES_U7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FILTER_THRES_U7_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FILTER_THRES_U7` writer - This register is used to filter pluse whose width is smaller than this value for unit7."]
pub struct FILTER_THRES_U7_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTER_THRES_U7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
#[doc = "Field `FILTER_EN_U7` reader - This is the enable bit for filtering input signals for unit7."]
pub struct FILTER_EN_U7_R(crate::FieldReader<bool, bool>);
impl FILTER_EN_U7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FILTER_EN_U7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FILTER_EN_U7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FILTER_EN_U7` writer - This is the enable bit for filtering input signals for unit7."]
pub struct FILTER_EN_U7_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTER_EN_U7_W<'a> {
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
#[doc = "Field `THR_ZERO_EN_U7` reader - This is the enable bit for comparing unit7's count with 0 value."]
pub struct THR_ZERO_EN_U7_R(crate::FieldReader<bool, bool>);
impl THR_ZERO_EN_U7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        THR_ZERO_EN_U7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THR_ZERO_EN_U7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THR_ZERO_EN_U7` writer - This is the enable bit for comparing unit7's count with 0 value."]
pub struct THR_ZERO_EN_U7_W<'a> {
    w: &'a mut W,
}
impl<'a> THR_ZERO_EN_U7_W<'a> {
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
#[doc = "Field `THR_H_LIM_EN_U7` reader - This is the enable bit for comparing unit7's count with thr_h_lim value."]
pub struct THR_H_LIM_EN_U7_R(crate::FieldReader<bool, bool>);
impl THR_H_LIM_EN_U7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        THR_H_LIM_EN_U7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THR_H_LIM_EN_U7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THR_H_LIM_EN_U7` writer - This is the enable bit for comparing unit7's count with thr_h_lim value."]
pub struct THR_H_LIM_EN_U7_W<'a> {
    w: &'a mut W,
}
impl<'a> THR_H_LIM_EN_U7_W<'a> {
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
#[doc = "Field `THR_L_LIM_EN_U7` reader - This is the enable bit for comparing unit7's count with thr_l_lim value."]
pub struct THR_L_LIM_EN_U7_R(crate::FieldReader<bool, bool>);
impl THR_L_LIM_EN_U7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        THR_L_LIM_EN_U7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THR_L_LIM_EN_U7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THR_L_LIM_EN_U7` writer - This is the enable bit for comparing unit7's count with thr_l_lim value."]
pub struct THR_L_LIM_EN_U7_W<'a> {
    w: &'a mut W,
}
impl<'a> THR_L_LIM_EN_U7_W<'a> {
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
#[doc = "Field `THR_THRES0_EN_U7` reader - This is the enable bit for comparing unit7's count with thres0 value."]
pub struct THR_THRES0_EN_U7_R(crate::FieldReader<bool, bool>);
impl THR_THRES0_EN_U7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        THR_THRES0_EN_U7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THR_THRES0_EN_U7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THR_THRES0_EN_U7` writer - This is the enable bit for comparing unit7's count with thres0 value."]
pub struct THR_THRES0_EN_U7_W<'a> {
    w: &'a mut W,
}
impl<'a> THR_THRES0_EN_U7_W<'a> {
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
#[doc = "Field `THR_THRES1_EN_U7` reader - This is the enable bit for comparing unit7's count with thres1 value ."]
pub struct THR_THRES1_EN_U7_R(crate::FieldReader<bool, bool>);
impl THR_THRES1_EN_U7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        THR_THRES1_EN_U7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THR_THRES1_EN_U7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THR_THRES1_EN_U7` writer - This is the enable bit for comparing unit7's count with thres1 value ."]
pub struct THR_THRES1_EN_U7_W<'a> {
    w: &'a mut W,
}
impl<'a> THR_THRES1_EN_U7_W<'a> {
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
#[doc = "Field `CH0_NEG_MODE_U7` reader - This register is used to control the mode of channel0's input negedge signal for unit7. 2'd1: increase at the negedge of input signal 2'd2:decrease at the negedge of input signal others:forbidden"]
pub struct CH0_NEG_MODE_U7_R(crate::FieldReader<u8, u8>);
impl CH0_NEG_MODE_U7_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CH0_NEG_MODE_U7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH0_NEG_MODE_U7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH0_NEG_MODE_U7` writer - This register is used to control the mode of channel0's input negedge signal for unit7. 2'd1: increase at the negedge of input signal 2'd2:decrease at the negedge of input signal others:forbidden"]
pub struct CH0_NEG_MODE_U7_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0_NEG_MODE_U7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `CH0_POS_MODE_U7` reader - This register is used to control the mode of channel0's input posedge signal for unit7. 2'd1: increase at the posedge of input signal 2'd2:decrease at the posedge of input signal others:forbidden"]
pub struct CH0_POS_MODE_U7_R(crate::FieldReader<u8, u8>);
impl CH0_POS_MODE_U7_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CH0_POS_MODE_U7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH0_POS_MODE_U7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH0_POS_MODE_U7` writer - This register is used to control the mode of channel0's input posedge signal for unit7. 2'd1: increase at the posedge of input signal 2'd2:decrease at the posedge of input signal others:forbidden"]
pub struct CH0_POS_MODE_U7_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0_POS_MODE_U7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
#[doc = "Field `CH0_HCTRL_MODE_U7` reader - This register is used to control the mode of channel0's high control signal for unit7. 2'd0:increase when control signal is low 2'd1: decrease when control signal is high others:forbidden"]
pub struct CH0_HCTRL_MODE_U7_R(crate::FieldReader<u8, u8>);
impl CH0_HCTRL_MODE_U7_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CH0_HCTRL_MODE_U7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH0_HCTRL_MODE_U7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH0_HCTRL_MODE_U7` writer - This register is used to control the mode of channel0's high control signal for unit7. 2'd0:increase when control signal is low 2'd1: decrease when control signal is high others:forbidden"]
pub struct CH0_HCTRL_MODE_U7_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0_HCTRL_MODE_U7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Field `CH0_LCTRL_MODE_U7` reader - This register is used to control the mode of channel0's low control signal for unit7. 2'd0:increase when control signal is low 2'd1: decrease when control signal is high others:forbidden"]
pub struct CH0_LCTRL_MODE_U7_R(crate::FieldReader<u8, u8>);
impl CH0_LCTRL_MODE_U7_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CH0_LCTRL_MODE_U7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH0_LCTRL_MODE_U7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH0_LCTRL_MODE_U7` writer - This register is used to control the mode of channel0's low control signal for unit7. 2'd0:increase when control signal is low 2'd1: decrease when control signal is high others:forbidden"]
pub struct CH0_LCTRL_MODE_U7_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0_LCTRL_MODE_U7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
#[doc = "Field `CH1_NEG_MODE_U7` reader - This register is used to control the mode of channel1's input negedge signal for unit7. 2'd1: increase at the negedge of input signal 2'd2:decrease at the negedge of input signal others:forbidden"]
pub struct CH1_NEG_MODE_U7_R(crate::FieldReader<u8, u8>);
impl CH1_NEG_MODE_U7_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CH1_NEG_MODE_U7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH1_NEG_MODE_U7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH1_NEG_MODE_U7` writer - This register is used to control the mode of channel1's input negedge signal for unit7. 2'd1: increase at the negedge of input signal 2'd2:decrease at the negedge of input signal others:forbidden"]
pub struct CH1_NEG_MODE_U7_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1_NEG_MODE_U7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Field `CH1_POS_MODE_U7` reader - This register is used to control the mode of channel1's input posedge signal for unit7. 2'd1: increase at the posedge of input signal 2'd2:decrease at the posedge of input signal others:forbidden"]
pub struct CH1_POS_MODE_U7_R(crate::FieldReader<u8, u8>);
impl CH1_POS_MODE_U7_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CH1_POS_MODE_U7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH1_POS_MODE_U7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH1_POS_MODE_U7` writer - This register is used to control the mode of channel1's input posedge signal for unit7. 2'd1: increase at the posedge of input signal 2'd2:decrease at the posedge of input signal others:forbidden"]
pub struct CH1_POS_MODE_U7_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1_POS_MODE_U7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | ((value as u32 & 0x03) << 26);
        self.w
    }
}
#[doc = "Field `CH1_HCTRL_MODE_U7` reader - This register is used to control the mode of channel1's high control signal for unit7. 2'd0:increase when control signal is low 2'd1: decrease when control signal is high others:forbidden"]
pub struct CH1_HCTRL_MODE_U7_R(crate::FieldReader<u8, u8>);
impl CH1_HCTRL_MODE_U7_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CH1_HCTRL_MODE_U7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH1_HCTRL_MODE_U7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH1_HCTRL_MODE_U7` writer - This register is used to control the mode of channel1's high control signal for unit7. 2'd0:increase when control signal is low 2'd1: decrease when control signal is high others:forbidden"]
pub struct CH1_HCTRL_MODE_U7_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1_HCTRL_MODE_U7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
#[doc = "Field `CH1_LCTRL_MODE_U7` reader - This register is used to control the mode of channel1's low control signal for unit7. 2'd0:increase when control signal is low 2'd1: decrease when control signal is high others:forbidden"]
pub struct CH1_LCTRL_MODE_U7_R(crate::FieldReader<u8, u8>);
impl CH1_LCTRL_MODE_U7_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CH1_LCTRL_MODE_U7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH1_LCTRL_MODE_U7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH1_LCTRL_MODE_U7` writer - This register is used to control the mode of channel1's low control signal for unit7. 2'd0:increase when control signal is low 2'd1: decrease when control signal is high others:forbidden"]
pub struct CH1_LCTRL_MODE_U7_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1_LCTRL_MODE_U7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - This register is used to filter pluse whose width is smaller than this value for unit7."]
    #[inline(always)]
    pub fn filter_thres_u7(&self) -> FILTER_THRES_U7_R {
        FILTER_THRES_U7_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - This is the enable bit for filtering input signals for unit7."]
    #[inline(always)]
    pub fn filter_en_u7(&self) -> FILTER_EN_U7_R {
        FILTER_EN_U7_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - This is the enable bit for comparing unit7's count with 0 value."]
    #[inline(always)]
    pub fn thr_zero_en_u7(&self) -> THR_ZERO_EN_U7_R {
        THR_ZERO_EN_U7_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - This is the enable bit for comparing unit7's count with thr_h_lim value."]
    #[inline(always)]
    pub fn thr_h_lim_en_u7(&self) -> THR_H_LIM_EN_U7_R {
        THR_H_LIM_EN_U7_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - This is the enable bit for comparing unit7's count with thr_l_lim value."]
    #[inline(always)]
    pub fn thr_l_lim_en_u7(&self) -> THR_L_LIM_EN_U7_R {
        THR_L_LIM_EN_U7_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - This is the enable bit for comparing unit7's count with thres0 value."]
    #[inline(always)]
    pub fn thr_thres0_en_u7(&self) -> THR_THRES0_EN_U7_R {
        THR_THRES0_EN_U7_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - This is the enable bit for comparing unit7's count with thres1 value ."]
    #[inline(always)]
    pub fn thr_thres1_en_u7(&self) -> THR_THRES1_EN_U7_R {
        THR_THRES1_EN_U7_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - This register is used to control the mode of channel0's input negedge signal for unit7. 2'd1: increase at the negedge of input signal 2'd2:decrease at the negedge of input signal others:forbidden"]
    #[inline(always)]
    pub fn ch0_neg_mode_u7(&self) -> CH0_NEG_MODE_U7_R {
        CH0_NEG_MODE_U7_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - This register is used to control the mode of channel0's input posedge signal for unit7. 2'd1: increase at the posedge of input signal 2'd2:decrease at the posedge of input signal others:forbidden"]
    #[inline(always)]
    pub fn ch0_pos_mode_u7(&self) -> CH0_POS_MODE_U7_R {
        CH0_POS_MODE_U7_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - This register is used to control the mode of channel0's high control signal for unit7. 2'd0:increase when control signal is low 2'd1: decrease when control signal is high others:forbidden"]
    #[inline(always)]
    pub fn ch0_hctrl_mode_u7(&self) -> CH0_HCTRL_MODE_U7_R {
        CH0_HCTRL_MODE_U7_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - This register is used to control the mode of channel0's low control signal for unit7. 2'd0:increase when control signal is low 2'd1: decrease when control signal is high others:forbidden"]
    #[inline(always)]
    pub fn ch0_lctrl_mode_u7(&self) -> CH0_LCTRL_MODE_U7_R {
        CH0_LCTRL_MODE_U7_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - This register is used to control the mode of channel1's input negedge signal for unit7. 2'd1: increase at the negedge of input signal 2'd2:decrease at the negedge of input signal others:forbidden"]
    #[inline(always)]
    pub fn ch1_neg_mode_u7(&self) -> CH1_NEG_MODE_U7_R {
        CH1_NEG_MODE_U7_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - This register is used to control the mode of channel1's input posedge signal for unit7. 2'd1: increase at the posedge of input signal 2'd2:decrease at the posedge of input signal others:forbidden"]
    #[inline(always)]
    pub fn ch1_pos_mode_u7(&self) -> CH1_POS_MODE_U7_R {
        CH1_POS_MODE_U7_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - This register is used to control the mode of channel1's high control signal for unit7. 2'd0:increase when control signal is low 2'd1: decrease when control signal is high others:forbidden"]
    #[inline(always)]
    pub fn ch1_hctrl_mode_u7(&self) -> CH1_HCTRL_MODE_U7_R {
        CH1_HCTRL_MODE_U7_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - This register is used to control the mode of channel1's low control signal for unit7. 2'd0:increase when control signal is low 2'd1: decrease when control signal is high others:forbidden"]
    #[inline(always)]
    pub fn ch1_lctrl_mode_u7(&self) -> CH1_LCTRL_MODE_U7_R {
        CH1_LCTRL_MODE_U7_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - This register is used to filter pluse whose width is smaller than this value for unit7."]
    #[inline(always)]
    pub fn filter_thres_u7(&mut self) -> FILTER_THRES_U7_W {
        FILTER_THRES_U7_W { w: self }
    }
    #[doc = "Bit 10 - This is the enable bit for filtering input signals for unit7."]
    #[inline(always)]
    pub fn filter_en_u7(&mut self) -> FILTER_EN_U7_W {
        FILTER_EN_U7_W { w: self }
    }
    #[doc = "Bit 11 - This is the enable bit for comparing unit7's count with 0 value."]
    #[inline(always)]
    pub fn thr_zero_en_u7(&mut self) -> THR_ZERO_EN_U7_W {
        THR_ZERO_EN_U7_W { w: self }
    }
    #[doc = "Bit 12 - This is the enable bit for comparing unit7's count with thr_h_lim value."]
    #[inline(always)]
    pub fn thr_h_lim_en_u7(&mut self) -> THR_H_LIM_EN_U7_W {
        THR_H_LIM_EN_U7_W { w: self }
    }
    #[doc = "Bit 13 - This is the enable bit for comparing unit7's count with thr_l_lim value."]
    #[inline(always)]
    pub fn thr_l_lim_en_u7(&mut self) -> THR_L_LIM_EN_U7_W {
        THR_L_LIM_EN_U7_W { w: self }
    }
    #[doc = "Bit 14 - This is the enable bit for comparing unit7's count with thres0 value."]
    #[inline(always)]
    pub fn thr_thres0_en_u7(&mut self) -> THR_THRES0_EN_U7_W {
        THR_THRES0_EN_U7_W { w: self }
    }
    #[doc = "Bit 15 - This is the enable bit for comparing unit7's count with thres1 value ."]
    #[inline(always)]
    pub fn thr_thres1_en_u7(&mut self) -> THR_THRES1_EN_U7_W {
        THR_THRES1_EN_U7_W { w: self }
    }
    #[doc = "Bits 16:17 - This register is used to control the mode of channel0's input negedge signal for unit7. 2'd1: increase at the negedge of input signal 2'd2:decrease at the negedge of input signal others:forbidden"]
    #[inline(always)]
    pub fn ch0_neg_mode_u7(&mut self) -> CH0_NEG_MODE_U7_W {
        CH0_NEG_MODE_U7_W { w: self }
    }
    #[doc = "Bits 18:19 - This register is used to control the mode of channel0's input posedge signal for unit7. 2'd1: increase at the posedge of input signal 2'd2:decrease at the posedge of input signal others:forbidden"]
    #[inline(always)]
    pub fn ch0_pos_mode_u7(&mut self) -> CH0_POS_MODE_U7_W {
        CH0_POS_MODE_U7_W { w: self }
    }
    #[doc = "Bits 20:21 - This register is used to control the mode of channel0's high control signal for unit7. 2'd0:increase when control signal is low 2'd1: decrease when control signal is high others:forbidden"]
    #[inline(always)]
    pub fn ch0_hctrl_mode_u7(&mut self) -> CH0_HCTRL_MODE_U7_W {
        CH0_HCTRL_MODE_U7_W { w: self }
    }
    #[doc = "Bits 22:23 - This register is used to control the mode of channel0's low control signal for unit7. 2'd0:increase when control signal is low 2'd1: decrease when control signal is high others:forbidden"]
    #[inline(always)]
    pub fn ch0_lctrl_mode_u7(&mut self) -> CH0_LCTRL_MODE_U7_W {
        CH0_LCTRL_MODE_U7_W { w: self }
    }
    #[doc = "Bits 24:25 - This register is used to control the mode of channel1's input negedge signal for unit7. 2'd1: increase at the negedge of input signal 2'd2:decrease at the negedge of input signal others:forbidden"]
    #[inline(always)]
    pub fn ch1_neg_mode_u7(&mut self) -> CH1_NEG_MODE_U7_W {
        CH1_NEG_MODE_U7_W { w: self }
    }
    #[doc = "Bits 26:27 - This register is used to control the mode of channel1's input posedge signal for unit7. 2'd1: increase at the posedge of input signal 2'd2:decrease at the posedge of input signal others:forbidden"]
    #[inline(always)]
    pub fn ch1_pos_mode_u7(&mut self) -> CH1_POS_MODE_U7_W {
        CH1_POS_MODE_U7_W { w: self }
    }
    #[doc = "Bits 28:29 - This register is used to control the mode of channel1's high control signal for unit7. 2'd0:increase when control signal is low 2'd1: decrease when control signal is high others:forbidden"]
    #[inline(always)]
    pub fn ch1_hctrl_mode_u7(&mut self) -> CH1_HCTRL_MODE_U7_W {
        CH1_HCTRL_MODE_U7_W { w: self }
    }
    #[doc = "Bits 30:31 - This register is used to control the mode of channel1's low control signal for unit7. 2'd0:increase when control signal is low 2'd1: decrease when control signal is high others:forbidden"]
    #[inline(always)]
    pub fn ch1_lctrl_mode_u7(&mut self) -> CH1_LCTRL_MODE_U7_W {
        CH1_LCTRL_MODE_U7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u7_conf0](index.html) module"]
pub struct U7_CONF0_SPEC;
impl crate::RegisterSpec for U7_CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [u7_conf0::R](R) reader structure"]
impl crate::Readable for U7_CONF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [u7_conf0::W](W) writer structure"]
impl crate::Writable for U7_CONF0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets U7_CONF0 to value 0x3c10"]
impl crate::Resettable for U7_CONF0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3c10
    }
}
