#[doc = "Register `LSTIMER0_CONF` reader"]
pub struct R(crate::R<LSTIMER0_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LSTIMER0_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LSTIMER0_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LSTIMER0_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LSTIMER0_CONF` writer"]
pub struct W(crate::W<LSTIMER0_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LSTIMER0_CONF_SPEC>;
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
impl From<crate::W<LSTIMER0_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LSTIMER0_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LSTIMER0_DUTY_RES` reader - This register controls the range of the counter in low speed timer0. the counter range is \\[0 2**reg_lstimer0_lim\\]
the max bit width for counter is 20."]
pub struct LSTIMER0_DUTY_RES_R(crate::FieldReader<u8, u8>);
impl LSTIMER0_DUTY_RES_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LSTIMER0_DUTY_RES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSTIMER0_DUTY_RES_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSTIMER0_DUTY_RES` writer - This register controls the range of the counter in low speed timer0. the counter range is \\[0 2**reg_lstimer0_lim\\]
the max bit width for counter is 20."]
pub struct LSTIMER0_DUTY_RES_W<'a> {
    w: &'a mut W,
}
impl<'a> LSTIMER0_DUTY_RES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Field `DIV_NUM_LSTIMER0` reader - This register is used to configure parameter for divider in low speed timer0 the least significant eight bits represent the decimal part."]
pub struct DIV_NUM_LSTIMER0_R(crate::FieldReader<u32, u32>);
impl DIV_NUM_LSTIMER0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        DIV_NUM_LSTIMER0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIV_NUM_LSTIMER0_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIV_NUM_LSTIMER0` writer - This register is used to configure parameter for divider in low speed timer0 the least significant eight bits represent the decimal part."]
pub struct DIV_NUM_LSTIMER0_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_NUM_LSTIMER0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0003_ffff << 5)) | ((value as u32 & 0x0003_ffff) << 5);
        self.w
    }
}
#[doc = "Field `LSTIMER0_PAUSE` reader - This bit is used to pause the counter in low speed timer0."]
pub struct LSTIMER0_PAUSE_R(crate::FieldReader<bool, bool>);
impl LSTIMER0_PAUSE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LSTIMER0_PAUSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSTIMER0_PAUSE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSTIMER0_PAUSE` writer - This bit is used to pause the counter in low speed timer0."]
pub struct LSTIMER0_PAUSE_W<'a> {
    w: &'a mut W,
}
impl<'a> LSTIMER0_PAUSE_W<'a> {
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
#[doc = "Field `LSTIMER0_RST` reader - This bit is used to reset low speed timer0 the counter will be 0 after reset."]
pub struct LSTIMER0_RST_R(crate::FieldReader<bool, bool>);
impl LSTIMER0_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LSTIMER0_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSTIMER0_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSTIMER0_RST` writer - This bit is used to reset low speed timer0 the counter will be 0 after reset."]
pub struct LSTIMER0_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> LSTIMER0_RST_W<'a> {
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
#[doc = "Field `TICK_SEL_LSTIMER0` reader - This bit is used to choose slow_clk or ref_tick for low speed timer0. 1'b1:slow_clk 0:ref_tick"]
pub struct TICK_SEL_LSTIMER0_R(crate::FieldReader<bool, bool>);
impl TICK_SEL_LSTIMER0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TICK_SEL_LSTIMER0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TICK_SEL_LSTIMER0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TICK_SEL_LSTIMER0` writer - This bit is used to choose slow_clk or ref_tick for low speed timer0. 1'b1:slow_clk 0:ref_tick"]
pub struct TICK_SEL_LSTIMER0_W<'a> {
    w: &'a mut W,
}
impl<'a> TICK_SEL_LSTIMER0_W<'a> {
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
#[doc = "Field `LSTIMER0_PARA_UP` reader - Set this bit to update reg_div_num_lstime0 and reg_lstimer0_lim."]
pub struct LSTIMER0_PARA_UP_R(crate::FieldReader<bool, bool>);
impl LSTIMER0_PARA_UP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LSTIMER0_PARA_UP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSTIMER0_PARA_UP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSTIMER0_PARA_UP` writer - Set this bit to update reg_div_num_lstime0 and reg_lstimer0_lim."]
pub struct LSTIMER0_PARA_UP_W<'a> {
    w: &'a mut W,
}
impl<'a> LSTIMER0_PARA_UP_W<'a> {
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
#[doc = "Field `LSTIMER0_LIM` reader - "]
pub struct LSTIMER0_LIM_R(crate::FieldReader<u8, u8>);
impl LSTIMER0_LIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LSTIMER0_LIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSTIMER0_LIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSTIMER0_LIM` writer - "]
pub struct LSTIMER0_LIM_W<'a> {
    w: &'a mut W,
}
impl<'a> LSTIMER0_LIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 31)) | ((value as u32 & 0x1f) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - This register controls the range of the counter in low speed timer0. the counter range is \\[0 2**reg_lstimer0_lim\\]
the max bit width for counter is 20."]
    #[inline(always)]
    pub fn lstimer0_duty_res(&self) -> LSTIMER0_DUTY_RES_R {
        LSTIMER0_DUTY_RES_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:22 - This register is used to configure parameter for divider in low speed timer0 the least significant eight bits represent the decimal part."]
    #[inline(always)]
    pub fn div_num_lstimer0(&self) -> DIV_NUM_LSTIMER0_R {
        DIV_NUM_LSTIMER0_R::new(((self.bits >> 5) & 0x0003_ffff) as u32)
    }
    #[doc = "Bit 23 - This bit is used to pause the counter in low speed timer0."]
    #[inline(always)]
    pub fn lstimer0_pause(&self) -> LSTIMER0_PAUSE_R {
        LSTIMER0_PAUSE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - This bit is used to reset low speed timer0 the counter will be 0 after reset."]
    #[inline(always)]
    pub fn lstimer0_rst(&self) -> LSTIMER0_RST_R {
        LSTIMER0_RST_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - This bit is used to choose slow_clk or ref_tick for low speed timer0. 1'b1:slow_clk 0:ref_tick"]
    #[inline(always)]
    pub fn tick_sel_lstimer0(&self) -> TICK_SEL_LSTIMER0_R {
        TICK_SEL_LSTIMER0_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Set this bit to update reg_div_num_lstime0 and reg_lstimer0_lim."]
    #[inline(always)]
    pub fn lstimer0_para_up(&self) -> LSTIMER0_PARA_UP_R {
        LSTIMER0_PARA_UP_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 31:35"]
    #[inline(always)]
    pub fn lstimer0_lim(&self) -> LSTIMER0_LIM_R {
        LSTIMER0_LIM_R::new(((self.bits >> 31) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - This register controls the range of the counter in low speed timer0. the counter range is \\[0 2**reg_lstimer0_lim\\]
the max bit width for counter is 20."]
    #[inline(always)]
    pub fn lstimer0_duty_res(&mut self) -> LSTIMER0_DUTY_RES_W {
        LSTIMER0_DUTY_RES_W { w: self }
    }
    #[doc = "Bits 5:22 - This register is used to configure parameter for divider in low speed timer0 the least significant eight bits represent the decimal part."]
    #[inline(always)]
    pub fn div_num_lstimer0(&mut self) -> DIV_NUM_LSTIMER0_W {
        DIV_NUM_LSTIMER0_W { w: self }
    }
    #[doc = "Bit 23 - This bit is used to pause the counter in low speed timer0."]
    #[inline(always)]
    pub fn lstimer0_pause(&mut self) -> LSTIMER0_PAUSE_W {
        LSTIMER0_PAUSE_W { w: self }
    }
    #[doc = "Bit 24 - This bit is used to reset low speed timer0 the counter will be 0 after reset."]
    #[inline(always)]
    pub fn lstimer0_rst(&mut self) -> LSTIMER0_RST_W {
        LSTIMER0_RST_W { w: self }
    }
    #[doc = "Bit 25 - This bit is used to choose slow_clk or ref_tick for low speed timer0. 1'b1:slow_clk 0:ref_tick"]
    #[inline(always)]
    pub fn tick_sel_lstimer0(&mut self) -> TICK_SEL_LSTIMER0_W {
        TICK_SEL_LSTIMER0_W { w: self }
    }
    #[doc = "Bit 26 - Set this bit to update reg_div_num_lstime0 and reg_lstimer0_lim."]
    #[inline(always)]
    pub fn lstimer0_para_up(&mut self) -> LSTIMER0_PARA_UP_W {
        LSTIMER0_PARA_UP_W { w: self }
    }
    #[doc = "Bits 31:35"]
    #[inline(always)]
    pub fn lstimer0_lim(&mut self) -> LSTIMER0_LIM_W {
        LSTIMER0_LIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lstimer0_conf](index.html) module"]
pub struct LSTIMER0_CONF_SPEC;
impl crate::RegisterSpec for LSTIMER0_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lstimer0_conf::R](R) reader structure"]
impl crate::Readable for LSTIMER0_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lstimer0_conf::W](W) writer structure"]
impl crate::Writable for LSTIMER0_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LSTIMER0_CONF to value 0x0100_0000"]
impl crate::Resettable for LSTIMER0_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0100_0000
    }
}
