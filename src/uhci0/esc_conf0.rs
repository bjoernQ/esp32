#[doc = "Register `ESC_CONF0` reader"]
pub struct R(crate::R<ESC_CONF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ESC_CONF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ESC_CONF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ESC_CONF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ESC_CONF0` writer"]
pub struct W(crate::W<ESC_CONF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ESC_CONF0_SPEC>;
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
impl From<crate::W<ESC_CONF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ESC_CONF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEPER_CHAR` reader - This register stores the seperator char seperator char is used to seperate the data frame."]
pub struct SEPER_CHAR_R(crate::FieldReader<u8, u8>);
impl SEPER_CHAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SEPER_CHAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEPER_CHAR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEPER_CHAR` writer - This register stores the seperator char seperator char is used to seperate the data frame."]
pub struct SEPER_CHAR_W<'a> {
    w: &'a mut W,
}
impl<'a> SEPER_CHAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `SEPER_ESC_CHAR0` reader - This register stores thee first char used to replace seperator char in data."]
pub struct SEPER_ESC_CHAR0_R(crate::FieldReader<u8, u8>);
impl SEPER_ESC_CHAR0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SEPER_ESC_CHAR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEPER_ESC_CHAR0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEPER_ESC_CHAR0` writer - This register stores thee first char used to replace seperator char in data."]
pub struct SEPER_ESC_CHAR0_W<'a> {
    w: &'a mut W,
}
impl<'a> SEPER_ESC_CHAR0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `SEPER_ESC_CHAR1` reader - This register stores the second char used to replace seperator char in data . 0xdc 0xdb replace 0xc0 by default."]
pub struct SEPER_ESC_CHAR1_R(crate::FieldReader<u8, u8>);
impl SEPER_ESC_CHAR1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SEPER_ESC_CHAR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEPER_ESC_CHAR1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEPER_ESC_CHAR1` writer - This register stores the second char used to replace seperator char in data . 0xdc 0xdb replace 0xc0 by default."]
pub struct SEPER_ESC_CHAR1_W<'a> {
    w: &'a mut W,
}
impl<'a> SEPER_ESC_CHAR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - This register stores the seperator char seperator char is used to seperate the data frame."]
    #[inline(always)]
    pub fn seper_char(&self) -> SEPER_CHAR_R {
        SEPER_CHAR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - This register stores thee first char used to replace seperator char in data."]
    #[inline(always)]
    pub fn seper_esc_char0(&self) -> SEPER_ESC_CHAR0_R {
        SEPER_ESC_CHAR0_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - This register stores the second char used to replace seperator char in data . 0xdc 0xdb replace 0xc0 by default."]
    #[inline(always)]
    pub fn seper_esc_char1(&self) -> SEPER_ESC_CHAR1_R {
        SEPER_ESC_CHAR1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - This register stores the seperator char seperator char is used to seperate the data frame."]
    #[inline(always)]
    pub fn seper_char(&mut self) -> SEPER_CHAR_W {
        SEPER_CHAR_W { w: self }
    }
    #[doc = "Bits 8:15 - This register stores thee first char used to replace seperator char in data."]
    #[inline(always)]
    pub fn seper_esc_char0(&mut self) -> SEPER_ESC_CHAR0_W {
        SEPER_ESC_CHAR0_W { w: self }
    }
    #[doc = "Bits 16:23 - This register stores the second char used to replace seperator char in data . 0xdc 0xdb replace 0xc0 by default."]
    #[inline(always)]
    pub fn seper_esc_char1(&mut self) -> SEPER_ESC_CHAR1_W {
        SEPER_ESC_CHAR1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [esc_conf0](index.html) module"]
pub struct ESC_CONF0_SPEC;
impl crate::RegisterSpec for ESC_CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [esc_conf0::R](R) reader structure"]
impl crate::Readable for ESC_CONF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [esc_conf0::W](W) writer structure"]
impl crate::Writable for ESC_CONF0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ESC_CONF0 to value 0x00dc_dbc0"]
impl crate::Resettable for ESC_CONF0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x00dc_dbc0
    }
}
