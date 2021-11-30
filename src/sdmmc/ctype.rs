#[doc = "Register `CTYPE` reader"]
pub struct R(crate::R<CTYPE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTYPE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTYPE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTYPE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTYPE` writer"]
pub struct W(crate::W<CTYPE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTYPE_SPEC>;
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
impl From<crate::W<CTYPE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTYPE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CARD_WIDTH4` reader - One bit per card indicates if card is 1-bit or 4-bit mode. 0: 1-bit mode; 1: 4-bit mode. Bit\\[1:0\\]
correspond to card\\[1:0\\]
respectively."]
pub struct CARD_WIDTH4_R(crate::FieldReader<u8, u8>);
impl CARD_WIDTH4_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CARD_WIDTH4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CARD_WIDTH4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CARD_WIDTH4` writer - One bit per card indicates if card is 1-bit or 4-bit mode. 0: 1-bit mode; 1: 4-bit mode. Bit\\[1:0\\]
correspond to card\\[1:0\\]
respectively."]
pub struct CARD_WIDTH4_W<'a> {
    w: &'a mut W,
}
impl<'a> CARD_WIDTH4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `CARD_WIDTH8` reader - One bit per card indicates if card is in 8-bit mode. 0: Non 8-bit mode; 1: 8-bit mode. Bit\\[17:16\\]
correspond to card\\[1:0\\]
respectively."]
pub struct CARD_WIDTH8_R(crate::FieldReader<u8, u8>);
impl CARD_WIDTH8_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CARD_WIDTH8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CARD_WIDTH8_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CARD_WIDTH8` writer - One bit per card indicates if card is in 8-bit mode. 0: Non 8-bit mode; 1: 8-bit mode. Bit\\[17:16\\]
correspond to card\\[1:0\\]
respectively."]
pub struct CARD_WIDTH8_W<'a> {
    w: &'a mut W,
}
impl<'a> CARD_WIDTH8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - One bit per card indicates if card is 1-bit or 4-bit mode. 0: 1-bit mode; 1: 4-bit mode. Bit\\[1:0\\]
correspond to card\\[1:0\\]
respectively."]
    #[inline(always)]
    pub fn card_width4(&self) -> CARD_WIDTH4_R {
        CARD_WIDTH4_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - One bit per card indicates if card is in 8-bit mode. 0: Non 8-bit mode; 1: 8-bit mode. Bit\\[17:16\\]
correspond to card\\[1:0\\]
respectively."]
    #[inline(always)]
    pub fn card_width8(&self) -> CARD_WIDTH8_R {
        CARD_WIDTH8_R::new(((self.bits >> 16) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - One bit per card indicates if card is 1-bit or 4-bit mode. 0: 1-bit mode; 1: 4-bit mode. Bit\\[1:0\\]
correspond to card\\[1:0\\]
respectively."]
    #[inline(always)]
    pub fn card_width4(&mut self) -> CARD_WIDTH4_W {
        CARD_WIDTH4_W { w: self }
    }
    #[doc = "Bits 16:17 - One bit per card indicates if card is in 8-bit mode. 0: Non 8-bit mode; 1: 8-bit mode. Bit\\[17:16\\]
correspond to card\\[1:0\\]
respectively."]
    #[inline(always)]
    pub fn card_width8(&mut self) -> CARD_WIDTH8_W {
        CARD_WIDTH8_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Card bus width configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctype](index.html) module"]
pub struct CTYPE_SPEC;
impl crate::RegisterSpec for CTYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctype::R](R) reader structure"]
impl crate::Readable for CTYPE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctype::W](W) writer structure"]
impl crate::Writable for CTYPE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTYPE to value 0"]
impl crate::Resettable for CTYPE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
