#[doc = "Register `SWFC_CONF` reader"]
pub struct R(crate::R<SWFC_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWFC_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWFC_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWFC_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SWFC_CONF` writer"]
pub struct W(crate::W<SWFC_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWFC_CONF_SPEC>;
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
impl From<crate::W<SWFC_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWFC_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XON_THRESHOLD` reader - when the data amount in receiver's fifo is more than this register value. it will send a xoff char with uart_sw_flow_con_en set to 1."]
pub struct XON_THRESHOLD_R(crate::FieldReader<u8, u8>);
impl XON_THRESHOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        XON_THRESHOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XON_THRESHOLD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XON_THRESHOLD` writer - when the data amount in receiver's fifo is more than this register value. it will send a xoff char with uart_sw_flow_con_en set to 1."]
pub struct XON_THRESHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> XON_THRESHOLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `XOFF_THRESHOLD` reader - When the data amount in receiver's fifo is less than this register value. it will send a xon char with uart_sw_flow_con_en set to 1."]
pub struct XOFF_THRESHOLD_R(crate::FieldReader<u8, u8>);
impl XOFF_THRESHOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        XOFF_THRESHOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XOFF_THRESHOLD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XOFF_THRESHOLD` writer - When the data amount in receiver's fifo is less than this register value. it will send a xon char with uart_sw_flow_con_en set to 1."]
pub struct XOFF_THRESHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> XOFF_THRESHOLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `XON_CHAR` reader - This register stores the xon flow control char."]
pub struct XON_CHAR_R(crate::FieldReader<u8, u8>);
impl XON_CHAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        XON_CHAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XON_CHAR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XON_CHAR` writer - This register stores the xon flow control char."]
pub struct XON_CHAR_W<'a> {
    w: &'a mut W,
}
impl<'a> XON_CHAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `XOFF_CHAR` reader - This register stores the xoff flow control char."]
pub struct XOFF_CHAR_R(crate::FieldReader<u8, u8>);
impl XOFF_CHAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        XOFF_CHAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XOFF_CHAR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XOFF_CHAR` writer - This register stores the xoff flow control char."]
pub struct XOFF_CHAR_W<'a> {
    w: &'a mut W,
}
impl<'a> XOFF_CHAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - when the data amount in receiver's fifo is more than this register value. it will send a xoff char with uart_sw_flow_con_en set to 1."]
    #[inline(always)]
    pub fn xon_threshold(&self) -> XON_THRESHOLD_R {
        XON_THRESHOLD_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - When the data amount in receiver's fifo is less than this register value. it will send a xon char with uart_sw_flow_con_en set to 1."]
    #[inline(always)]
    pub fn xoff_threshold(&self) -> XOFF_THRESHOLD_R {
        XOFF_THRESHOLD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - This register stores the xon flow control char."]
    #[inline(always)]
    pub fn xon_char(&self) -> XON_CHAR_R {
        XON_CHAR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - This register stores the xoff flow control char."]
    #[inline(always)]
    pub fn xoff_char(&self) -> XOFF_CHAR_R {
        XOFF_CHAR_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - when the data amount in receiver's fifo is more than this register value. it will send a xoff char with uart_sw_flow_con_en set to 1."]
    #[inline(always)]
    pub fn xon_threshold(&mut self) -> XON_THRESHOLD_W {
        XON_THRESHOLD_W { w: self }
    }
    #[doc = "Bits 8:15 - When the data amount in receiver's fifo is less than this register value. it will send a xon char with uart_sw_flow_con_en set to 1."]
    #[inline(always)]
    pub fn xoff_threshold(&mut self) -> XOFF_THRESHOLD_W {
        XOFF_THRESHOLD_W { w: self }
    }
    #[doc = "Bits 16:23 - This register stores the xon flow control char."]
    #[inline(always)]
    pub fn xon_char(&mut self) -> XON_CHAR_W {
        XON_CHAR_W { w: self }
    }
    #[doc = "Bits 24:31 - This register stores the xoff flow control char."]
    #[inline(always)]
    pub fn xoff_char(&mut self) -> XOFF_CHAR_W {
        XOFF_CHAR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swfc_conf](index.html) module"]
pub struct SWFC_CONF_SPEC;
impl crate::RegisterSpec for SWFC_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swfc_conf::R](R) reader structure"]
impl crate::Readable for SWFC_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [swfc_conf::W](W) writer structure"]
impl crate::Writable for SWFC_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SWFC_CONF to value 0x1311_e000"]
impl crate::Resettable for SWFC_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1311_e000
    }
}
