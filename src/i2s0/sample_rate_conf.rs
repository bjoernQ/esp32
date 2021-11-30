#[doc = "Register `SAMPLE_RATE_CONF` reader"]
pub struct R(crate::R<SAMPLE_RATE_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAMPLE_RATE_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAMPLE_RATE_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAMPLE_RATE_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAMPLE_RATE_CONF` writer"]
pub struct W(crate::W<SAMPLE_RATE_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAMPLE_RATE_CONF_SPEC>;
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
impl From<crate::W<SAMPLE_RATE_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAMPLE_RATE_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_BCK_DIV_NUM` reader - "]
pub struct TX_BCK_DIV_NUM_R(crate::FieldReader<u8, u8>);
impl TX_BCK_DIV_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TX_BCK_DIV_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_BCK_DIV_NUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_BCK_DIV_NUM` writer - "]
pub struct TX_BCK_DIV_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_BCK_DIV_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Field `RX_BCK_DIV_NUM` reader - "]
pub struct RX_BCK_DIV_NUM_R(crate::FieldReader<u8, u8>);
impl RX_BCK_DIV_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RX_BCK_DIV_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_BCK_DIV_NUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_BCK_DIV_NUM` writer - "]
pub struct RX_BCK_DIV_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_BCK_DIV_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 6)) | ((value as u32 & 0x3f) << 6);
        self.w
    }
}
#[doc = "Field `TX_BITS_MOD` reader - "]
pub struct TX_BITS_MOD_R(crate::FieldReader<u8, u8>);
impl TX_BITS_MOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TX_BITS_MOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_BITS_MOD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_BITS_MOD` writer - "]
pub struct TX_BITS_MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_BITS_MOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 12)) | ((value as u32 & 0x3f) << 12);
        self.w
    }
}
#[doc = "Field `RX_BITS_MOD` reader - "]
pub struct RX_BITS_MOD_R(crate::FieldReader<u8, u8>);
impl RX_BITS_MOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RX_BITS_MOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_BITS_MOD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_BITS_MOD` writer - "]
pub struct RX_BITS_MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_BITS_MOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 18)) | ((value as u32 & 0x3f) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn tx_bck_div_num(&self) -> TX_BCK_DIV_NUM_R {
        TX_BCK_DIV_NUM_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11"]
    #[inline(always)]
    pub fn rx_bck_div_num(&self) -> RX_BCK_DIV_NUM_R {
        RX_BCK_DIV_NUM_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:17"]
    #[inline(always)]
    pub fn tx_bits_mod(&self) -> TX_BITS_MOD_R {
        TX_BITS_MOD_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 18:23"]
    #[inline(always)]
    pub fn rx_bits_mod(&self) -> RX_BITS_MOD_R {
        RX_BITS_MOD_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn tx_bck_div_num(&mut self) -> TX_BCK_DIV_NUM_W {
        TX_BCK_DIV_NUM_W { w: self }
    }
    #[doc = "Bits 6:11"]
    #[inline(always)]
    pub fn rx_bck_div_num(&mut self) -> RX_BCK_DIV_NUM_W {
        RX_BCK_DIV_NUM_W { w: self }
    }
    #[doc = "Bits 12:17"]
    #[inline(always)]
    pub fn tx_bits_mod(&mut self) -> TX_BITS_MOD_W {
        TX_BITS_MOD_W { w: self }
    }
    #[doc = "Bits 18:23"]
    #[inline(always)]
    pub fn rx_bits_mod(&mut self) -> RX_BITS_MOD_W {
        RX_BITS_MOD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sample_rate_conf](index.html) module"]
pub struct SAMPLE_RATE_CONF_SPEC;
impl crate::RegisterSpec for SAMPLE_RATE_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sample_rate_conf::R](R) reader structure"]
impl crate::Readable for SAMPLE_RATE_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sample_rate_conf::W](W) writer structure"]
impl crate::Writable for SAMPLE_RATE_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAMPLE_RATE_CONF to value 0x0041_0186"]
impl crate::Resettable for SAMPLE_RATE_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0041_0186
    }
}
