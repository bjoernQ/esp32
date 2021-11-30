#[doc = "Register `CH2CONF0` reader"]
pub struct R(crate::R<CH2CONF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH2CONF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH2CONF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH2CONF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH2CONF0` writer"]
pub struct W(crate::W<CH2CONF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH2CONF0_SPEC>;
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
impl From<crate::W<CH2CONF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH2CONF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIV_CNT_CH2` reader - This register is used to configure the frequency divider's factor in channel2."]
pub struct DIV_CNT_CH2_R(crate::FieldReader<u8, u8>);
impl DIV_CNT_CH2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIV_CNT_CH2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIV_CNT_CH2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIV_CNT_CH2` writer - This register is used to configure the frequency divider's factor in channel2."]
pub struct DIV_CNT_CH2_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_CNT_CH2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `IDLE_THRES_CH2` reader - In receive mode when the counter's value is bigger than reg_idle_thres_ch2 then the receive process is done."]
pub struct IDLE_THRES_CH2_R(crate::FieldReader<u16, u16>);
impl IDLE_THRES_CH2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        IDLE_THRES_CH2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDLE_THRES_CH2_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDLE_THRES_CH2` writer - In receive mode when the counter's value is bigger than reg_idle_thres_ch2 then the receive process is done."]
pub struct IDLE_THRES_CH2_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLE_THRES_CH2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 8)) | ((value as u32 & 0xffff) << 8);
        self.w
    }
}
#[doc = "Field `MEM_SIZE_CH2` reader - This register is used to configure the the amount of memory blocks allocated to channel2."]
pub struct MEM_SIZE_CH2_R(crate::FieldReader<u8, u8>);
impl MEM_SIZE_CH2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MEM_SIZE_CH2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM_SIZE_CH2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM_SIZE_CH2` writer - This register is used to configure the the amount of memory blocks allocated to channel2."]
pub struct MEM_SIZE_CH2_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_SIZE_CH2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `CARRIER_EN_CH2` reader - This is the carrier modulation enable control bit for channel2."]
pub struct CARRIER_EN_CH2_R(crate::FieldReader<bool, bool>);
impl CARRIER_EN_CH2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CARRIER_EN_CH2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CARRIER_EN_CH2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CARRIER_EN_CH2` writer - This is the carrier modulation enable control bit for channel2."]
pub struct CARRIER_EN_CH2_W<'a> {
    w: &'a mut W,
}
impl<'a> CARRIER_EN_CH2_W<'a> {
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
#[doc = "Field `CARRIER_OUT_LV_CH2` reader - This bit is used to configure carrier wave's position for channel2.1'b1:add on low level 1'b0:add on high level."]
pub struct CARRIER_OUT_LV_CH2_R(crate::FieldReader<bool, bool>);
impl CARRIER_OUT_LV_CH2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CARRIER_OUT_LV_CH2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CARRIER_OUT_LV_CH2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CARRIER_OUT_LV_CH2` writer - This bit is used to configure carrier wave's position for channel2.1'b1:add on low level 1'b0:add on high level."]
pub struct CARRIER_OUT_LV_CH2_W<'a> {
    w: &'a mut W,
}
impl<'a> CARRIER_OUT_LV_CH2_W<'a> {
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
    #[doc = "Bits 0:7 - This register is used to configure the frequency divider's factor in channel2."]
    #[inline(always)]
    pub fn div_cnt_ch2(&self) -> DIV_CNT_CH2_R {
        DIV_CNT_CH2_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:23 - In receive mode when the counter's value is bigger than reg_idle_thres_ch2 then the receive process is done."]
    #[inline(always)]
    pub fn idle_thres_ch2(&self) -> IDLE_THRES_CH2_R {
        IDLE_THRES_CH2_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
    #[doc = "Bits 24:27 - This register is used to configure the the amount of memory blocks allocated to channel2."]
    #[inline(always)]
    pub fn mem_size_ch2(&self) -> MEM_SIZE_CH2_R {
        MEM_SIZE_CH2_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - This is the carrier modulation enable control bit for channel2."]
    #[inline(always)]
    pub fn carrier_en_ch2(&self) -> CARRIER_EN_CH2_R {
        CARRIER_EN_CH2_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - This bit is used to configure carrier wave's position for channel2.1'b1:add on low level 1'b0:add on high level."]
    #[inline(always)]
    pub fn carrier_out_lv_ch2(&self) -> CARRIER_OUT_LV_CH2_R {
        CARRIER_OUT_LV_CH2_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - This register is used to configure the frequency divider's factor in channel2."]
    #[inline(always)]
    pub fn div_cnt_ch2(&mut self) -> DIV_CNT_CH2_W {
        DIV_CNT_CH2_W { w: self }
    }
    #[doc = "Bits 8:23 - In receive mode when the counter's value is bigger than reg_idle_thres_ch2 then the receive process is done."]
    #[inline(always)]
    pub fn idle_thres_ch2(&mut self) -> IDLE_THRES_CH2_W {
        IDLE_THRES_CH2_W { w: self }
    }
    #[doc = "Bits 24:27 - This register is used to configure the the amount of memory blocks allocated to channel2."]
    #[inline(always)]
    pub fn mem_size_ch2(&mut self) -> MEM_SIZE_CH2_W {
        MEM_SIZE_CH2_W { w: self }
    }
    #[doc = "Bit 28 - This is the carrier modulation enable control bit for channel2."]
    #[inline(always)]
    pub fn carrier_en_ch2(&mut self) -> CARRIER_EN_CH2_W {
        CARRIER_EN_CH2_W { w: self }
    }
    #[doc = "Bit 29 - This bit is used to configure carrier wave's position for channel2.1'b1:add on low level 1'b0:add on high level."]
    #[inline(always)]
    pub fn carrier_out_lv_ch2(&mut self) -> CARRIER_OUT_LV_CH2_W {
        CARRIER_OUT_LV_CH2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2conf0](index.html) module"]
pub struct CH2CONF0_SPEC;
impl crate::RegisterSpec for CH2CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch2conf0::R](R) reader structure"]
impl crate::Readable for CH2CONF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch2conf0::W](W) writer structure"]
impl crate::Writable for CH2CONF0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CH2CONF0 to value 0x3110_0002"]
impl crate::Resettable for CH2CONF0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3110_0002
    }
}
