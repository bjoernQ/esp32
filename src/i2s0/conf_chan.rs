#[doc = "Register `CONF_CHAN` reader"]
pub struct R(crate::R<CONF_CHAN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONF_CHAN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONF_CHAN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONF_CHAN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONF_CHAN` writer"]
pub struct W(crate::W<CONF_CHAN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONF_CHAN_SPEC>;
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
impl From<crate::W<CONF_CHAN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONF_CHAN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_CHAN_MOD` reader - "]
pub struct TX_CHAN_MOD_R(crate::FieldReader<u8, u8>);
impl TX_CHAN_MOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TX_CHAN_MOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_CHAN_MOD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_CHAN_MOD` writer - "]
pub struct TX_CHAN_MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_CHAN_MOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `RX_CHAN_MOD` reader - "]
pub struct RX_CHAN_MOD_R(crate::FieldReader<u8, u8>);
impl RX_CHAN_MOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RX_CHAN_MOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_CHAN_MOD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_CHAN_MOD` writer - "]
pub struct RX_CHAN_MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_CHAN_MOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | ((value as u32 & 0x03) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn tx_chan_mod(&self) -> TX_CHAN_MOD_R {
        TX_CHAN_MOD_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn rx_chan_mod(&self) -> RX_CHAN_MOD_R {
        RX_CHAN_MOD_R::new(((self.bits >> 3) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn tx_chan_mod(&mut self) -> TX_CHAN_MOD_W {
        TX_CHAN_MOD_W { w: self }
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn rx_chan_mod(&mut self) -> RX_CHAN_MOD_W {
        RX_CHAN_MOD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conf_chan](index.html) module"]
pub struct CONF_CHAN_SPEC;
impl crate::RegisterSpec for CONF_CHAN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conf_chan::R](R) reader structure"]
impl crate::Readable for CONF_CHAN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conf_chan::W](W) writer structure"]
impl crate::Writable for CONF_CHAN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONF_CHAN to value 0"]
impl crate::Resettable for CONF_CHAN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
