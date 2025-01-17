#[doc = "Register `HSTIMER3_CONF` reader"]
pub struct R(crate::R<HSTIMER3_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSTIMER3_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSTIMER3_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSTIMER3_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSTIMER3_CONF` writer"]
pub struct W(crate::W<HSTIMER3_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSTIMER3_CONF_SPEC>;
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
impl From<crate::W<HSTIMER3_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSTIMER3_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HSTIMER3_DUTY_RES` reader - This register controls the range of the counter in high speed timer3. the counter range is \\[0 2**reg_hstimer3_lim\\]
the max bit width for counter is 20."]
pub struct HSTIMER3_DUTY_RES_R(crate::FieldReader<u8, u8>);
impl HSTIMER3_DUTY_RES_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        HSTIMER3_DUTY_RES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSTIMER3_DUTY_RES_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSTIMER3_DUTY_RES` writer - This register controls the range of the counter in high speed timer3. the counter range is \\[0 2**reg_hstimer3_lim\\]
the max bit width for counter is 20."]
pub struct HSTIMER3_DUTY_RES_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTIMER3_DUTY_RES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Field `DIV_NUM_HSTIMER3` reader - This register is used to configure parameter for divider in high speed timer3 the least significant eight bits represent the decimal part."]
pub struct DIV_NUM_HSTIMER3_R(crate::FieldReader<u32, u32>);
impl DIV_NUM_HSTIMER3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        DIV_NUM_HSTIMER3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIV_NUM_HSTIMER3_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIV_NUM_HSTIMER3` writer - This register is used to configure parameter for divider in high speed timer3 the least significant eight bits represent the decimal part."]
pub struct DIV_NUM_HSTIMER3_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_NUM_HSTIMER3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0003_ffff << 5)) | ((value as u32 & 0x0003_ffff) << 5);
        self.w
    }
}
#[doc = "Field `HSTIMER3_PAUSE` reader - This bit is used to pause the counter in high speed timer3"]
pub struct HSTIMER3_PAUSE_R(crate::FieldReader<bool, bool>);
impl HSTIMER3_PAUSE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HSTIMER3_PAUSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSTIMER3_PAUSE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSTIMER3_PAUSE` writer - This bit is used to pause the counter in high speed timer3"]
pub struct HSTIMER3_PAUSE_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTIMER3_PAUSE_W<'a> {
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
#[doc = "Field `HSTIMER3_RST` reader - This bit is used to reset high speed timer3 the counter will be 0 after reset."]
pub struct HSTIMER3_RST_R(crate::FieldReader<bool, bool>);
impl HSTIMER3_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HSTIMER3_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSTIMER3_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSTIMER3_RST` writer - This bit is used to reset high speed timer3 the counter will be 0 after reset."]
pub struct HSTIMER3_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTIMER3_RST_W<'a> {
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
#[doc = "Field `TICK_SEL_HSTIMER3` reader - This bit is used to choose apb_clk or ref_tick for high speed timer3. 1'b1:apb_clk 0:ref_tick"]
pub struct TICK_SEL_HSTIMER3_R(crate::FieldReader<bool, bool>);
impl TICK_SEL_HSTIMER3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TICK_SEL_HSTIMER3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TICK_SEL_HSTIMER3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TICK_SEL_HSTIMER3` writer - This bit is used to choose apb_clk or ref_tick for high speed timer3. 1'b1:apb_clk 0:ref_tick"]
pub struct TICK_SEL_HSTIMER3_W<'a> {
    w: &'a mut W,
}
impl<'a> TICK_SEL_HSTIMER3_W<'a> {
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
#[doc = "Field `HSTIMER3_LIM` reader - "]
pub struct HSTIMER3_LIM_R(crate::FieldReader<u8, u8>);
impl HSTIMER3_LIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        HSTIMER3_LIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSTIMER3_LIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSTIMER3_LIM` writer - "]
pub struct HSTIMER3_LIM_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTIMER3_LIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 31)) | ((value as u32 & 0x1f) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - This register controls the range of the counter in high speed timer3. the counter range is \\[0 2**reg_hstimer3_lim\\]
the max bit width for counter is 20."]
    #[inline(always)]
    pub fn hstimer3_duty_res(&self) -> HSTIMER3_DUTY_RES_R {
        HSTIMER3_DUTY_RES_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:22 - This register is used to configure parameter for divider in high speed timer3 the least significant eight bits represent the decimal part."]
    #[inline(always)]
    pub fn div_num_hstimer3(&self) -> DIV_NUM_HSTIMER3_R {
        DIV_NUM_HSTIMER3_R::new(((self.bits >> 5) & 0x0003_ffff) as u32)
    }
    #[doc = "Bit 23 - This bit is used to pause the counter in high speed timer3"]
    #[inline(always)]
    pub fn hstimer3_pause(&self) -> HSTIMER3_PAUSE_R {
        HSTIMER3_PAUSE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - This bit is used to reset high speed timer3 the counter will be 0 after reset."]
    #[inline(always)]
    pub fn hstimer3_rst(&self) -> HSTIMER3_RST_R {
        HSTIMER3_RST_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - This bit is used to choose apb_clk or ref_tick for high speed timer3. 1'b1:apb_clk 0:ref_tick"]
    #[inline(always)]
    pub fn tick_sel_hstimer3(&self) -> TICK_SEL_HSTIMER3_R {
        TICK_SEL_HSTIMER3_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 31:35"]
    #[inline(always)]
    pub fn hstimer3_lim(&self) -> HSTIMER3_LIM_R {
        HSTIMER3_LIM_R::new(((self.bits >> 31) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - This register controls the range of the counter in high speed timer3. the counter range is \\[0 2**reg_hstimer3_lim\\]
the max bit width for counter is 20."]
    #[inline(always)]
    pub fn hstimer3_duty_res(&mut self) -> HSTIMER3_DUTY_RES_W {
        HSTIMER3_DUTY_RES_W { w: self }
    }
    #[doc = "Bits 5:22 - This register is used to configure parameter for divider in high speed timer3 the least significant eight bits represent the decimal part."]
    #[inline(always)]
    pub fn div_num_hstimer3(&mut self) -> DIV_NUM_HSTIMER3_W {
        DIV_NUM_HSTIMER3_W { w: self }
    }
    #[doc = "Bit 23 - This bit is used to pause the counter in high speed timer3"]
    #[inline(always)]
    pub fn hstimer3_pause(&mut self) -> HSTIMER3_PAUSE_W {
        HSTIMER3_PAUSE_W { w: self }
    }
    #[doc = "Bit 24 - This bit is used to reset high speed timer3 the counter will be 0 after reset."]
    #[inline(always)]
    pub fn hstimer3_rst(&mut self) -> HSTIMER3_RST_W {
        HSTIMER3_RST_W { w: self }
    }
    #[doc = "Bit 25 - This bit is used to choose apb_clk or ref_tick for high speed timer3. 1'b1:apb_clk 0:ref_tick"]
    #[inline(always)]
    pub fn tick_sel_hstimer3(&mut self) -> TICK_SEL_HSTIMER3_W {
        TICK_SEL_HSTIMER3_W { w: self }
    }
    #[doc = "Bits 31:35"]
    #[inline(always)]
    pub fn hstimer3_lim(&mut self) -> HSTIMER3_LIM_W {
        HSTIMER3_LIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstimer3_conf](index.html) module"]
pub struct HSTIMER3_CONF_SPEC;
impl crate::RegisterSpec for HSTIMER3_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hstimer3_conf::R](R) reader structure"]
impl crate::Readable for HSTIMER3_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hstimer3_conf::W](W) writer structure"]
impl crate::Writable for HSTIMER3_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HSTIMER3_CONF to value 0x0100_0000"]
impl crate::Resettable for HSTIMER3_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0100_0000
    }
}
