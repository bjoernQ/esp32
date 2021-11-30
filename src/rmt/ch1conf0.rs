#[doc = "Register `CH1CONF0` reader"]
pub struct R(crate::R<CH1CONF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH1CONF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH1CONF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH1CONF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH1CONF0` writer"]
pub struct W(crate::W<CH1CONF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH1CONF0_SPEC>;
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
impl From<crate::W<CH1CONF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH1CONF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIV_CNT_CH1` reader - This register is used to configure the frequency divider's factor in channel1."]
pub struct DIV_CNT_CH1_R(crate::FieldReader<u8, u8>);
impl DIV_CNT_CH1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIV_CNT_CH1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIV_CNT_CH1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIV_CNT_CH1` writer - This register is used to configure the frequency divider's factor in channel1."]
pub struct DIV_CNT_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_CNT_CH1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `IDLE_THRES_CH1` reader - This register is used to configure the the amount of memory blocks allocated to channel1."]
pub struct IDLE_THRES_CH1_R(crate::FieldReader<u16, u16>);
impl IDLE_THRES_CH1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        IDLE_THRES_CH1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDLE_THRES_CH1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDLE_THRES_CH1` writer - This register is used to configure the the amount of memory blocks allocated to channel1."]
pub struct IDLE_THRES_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLE_THRES_CH1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 8)) | ((value as u32 & 0xffff) << 8);
        self.w
    }
}
#[doc = "Field `MEM_SIZE_CH1` reader - This register is used to configure the the amount of memory blocks allocated to channel1."]
pub struct MEM_SIZE_CH1_R(crate::FieldReader<u8, u8>);
impl MEM_SIZE_CH1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MEM_SIZE_CH1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM_SIZE_CH1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM_SIZE_CH1` writer - This register is used to configure the the amount of memory blocks allocated to channel1."]
pub struct MEM_SIZE_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_SIZE_CH1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `CARRIER_EN_CH1` reader - This is the carrier modulation enable control bit for channel1."]
pub struct CARRIER_EN_CH1_R(crate::FieldReader<bool, bool>);
impl CARRIER_EN_CH1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CARRIER_EN_CH1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CARRIER_EN_CH1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CARRIER_EN_CH1` writer - This is the carrier modulation enable control bit for channel1."]
pub struct CARRIER_EN_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> CARRIER_EN_CH1_W<'a> {
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
#[doc = "Field `CARRIER_OUT_LV_CH1` reader - This bit is used to configure the way carrier wave is modulated for channel1.1'b1:transmit on low output level 1'b0:transmit on high output level."]
pub struct CARRIER_OUT_LV_CH1_R(crate::FieldReader<bool, bool>);
impl CARRIER_OUT_LV_CH1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CARRIER_OUT_LV_CH1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CARRIER_OUT_LV_CH1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CARRIER_OUT_LV_CH1` writer - This bit is used to configure the way carrier wave is modulated for channel1.1'b1:transmit on low output level 1'b0:transmit on high output level."]
pub struct CARRIER_OUT_LV_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> CARRIER_OUT_LV_CH1_W<'a> {
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
impl R {
    #[doc = "Bits 0:7 - This register is used to configure the frequency divider's factor in channel1."]
    #[inline(always)]
    pub fn div_cnt_ch1(&self) -> DIV_CNT_CH1_R {
        DIV_CNT_CH1_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:23 - This register is used to configure the the amount of memory blocks allocated to channel1."]
    #[inline(always)]
    pub fn idle_thres_ch1(&self) -> IDLE_THRES_CH1_R {
        IDLE_THRES_CH1_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
    #[doc = "Bits 24:27 - This register is used to configure the the amount of memory blocks allocated to channel1."]
    #[inline(always)]
    pub fn mem_size_ch1(&self) -> MEM_SIZE_CH1_R {
        MEM_SIZE_CH1_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - This is the carrier modulation enable control bit for channel1."]
    #[inline(always)]
    pub fn carrier_en_ch1(&self) -> CARRIER_EN_CH1_R {
        CARRIER_EN_CH1_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - This bit is used to configure the way carrier wave is modulated for channel1.1'b1:transmit on low output level 1'b0:transmit on high output level."]
    #[inline(always)]
    pub fn carrier_out_lv_ch1(&self) -> CARRIER_OUT_LV_CH1_R {
        CARRIER_OUT_LV_CH1_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - This register is used to configure the frequency divider's factor in channel1."]
    #[inline(always)]
    pub fn div_cnt_ch1(&mut self) -> DIV_CNT_CH1_W {
        DIV_CNT_CH1_W { w: self }
    }
    #[doc = "Bits 8:23 - This register is used to configure the the amount of memory blocks allocated to channel1."]
    #[inline(always)]
    pub fn idle_thres_ch1(&mut self) -> IDLE_THRES_CH1_W {
        IDLE_THRES_CH1_W { w: self }
    }
    #[doc = "Bits 24:27 - This register is used to configure the the amount of memory blocks allocated to channel1."]
    #[inline(always)]
    pub fn mem_size_ch1(&mut self) -> MEM_SIZE_CH1_W {
        MEM_SIZE_CH1_W { w: self }
    }
    #[doc = "Bit 28 - This is the carrier modulation enable control bit for channel1."]
    #[inline(always)]
    pub fn carrier_en_ch1(&mut self) -> CARRIER_EN_CH1_W {
        CARRIER_EN_CH1_W { w: self }
    }
    #[doc = "Bit 29 - This bit is used to configure the way carrier wave is modulated for channel1.1'b1:transmit on low output level 1'b0:transmit on high output level."]
    #[inline(always)]
    pub fn carrier_out_lv_ch1(&mut self) -> CARRIER_OUT_LV_CH1_W {
        CARRIER_OUT_LV_CH1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1conf0](index.html) module"]
pub struct CH1CONF0_SPEC;
impl crate::RegisterSpec for CH1CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch1conf0::R](R) reader structure"]
impl crate::Readable for CH1CONF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch1conf0::W](W) writer structure"]
impl crate::Writable for CH1CONF0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CH1CONF0 to value 0x3110_0002"]
impl crate::Resettable for CH1CONF0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3110_0002
    }
}
