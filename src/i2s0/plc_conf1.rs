#[doc = "Register `PLC_CONF1` reader"]
pub struct R(crate::R<PLC_CONF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLC_CONF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLC_CONF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLC_CONF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLC_CONF1` writer"]
pub struct W(crate::W<PLC_CONF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLC_CONF1_SPEC>;
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
impl From<crate::W<PLC_CONF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLC_CONF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BAD_CEF_ATTEN_PARA` reader - "]
pub struct BAD_CEF_ATTEN_PARA_R(crate::FieldReader<u8, u8>);
impl BAD_CEF_ATTEN_PARA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BAD_CEF_ATTEN_PARA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BAD_CEF_ATTEN_PARA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BAD_CEF_ATTEN_PARA` writer - "]
pub struct BAD_CEF_ATTEN_PARA_W<'a> {
    w: &'a mut W,
}
impl<'a> BAD_CEF_ATTEN_PARA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `BAD_CEF_ATTEN_PARA_SHIFT` reader - "]
pub struct BAD_CEF_ATTEN_PARA_SHIFT_R(crate::FieldReader<u8, u8>);
impl BAD_CEF_ATTEN_PARA_SHIFT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BAD_CEF_ATTEN_PARA_SHIFT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BAD_CEF_ATTEN_PARA_SHIFT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BAD_CEF_ATTEN_PARA_SHIFT` writer - "]
pub struct BAD_CEF_ATTEN_PARA_SHIFT_W<'a> {
    w: &'a mut W,
}
impl<'a> BAD_CEF_ATTEN_PARA_SHIFT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `BAD_OLA_WIN2_PARA_SHIFT` reader - "]
pub struct BAD_OLA_WIN2_PARA_SHIFT_R(crate::FieldReader<u8, u8>);
impl BAD_OLA_WIN2_PARA_SHIFT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BAD_OLA_WIN2_PARA_SHIFT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BAD_OLA_WIN2_PARA_SHIFT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BAD_OLA_WIN2_PARA_SHIFT` writer - "]
pub struct BAD_OLA_WIN2_PARA_SHIFT_W<'a> {
    w: &'a mut W,
}
impl<'a> BAD_OLA_WIN2_PARA_SHIFT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `BAD_OLA_WIN2_PARA` reader - "]
pub struct BAD_OLA_WIN2_PARA_R(crate::FieldReader<u8, u8>);
impl BAD_OLA_WIN2_PARA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BAD_OLA_WIN2_PARA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BAD_OLA_WIN2_PARA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BAD_OLA_WIN2_PARA` writer - "]
pub struct BAD_OLA_WIN2_PARA_W<'a> {
    w: &'a mut W,
}
impl<'a> BAD_OLA_WIN2_PARA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `SLIDE_WIN_LEN` reader - "]
pub struct SLIDE_WIN_LEN_R(crate::FieldReader<u8, u8>);
impl SLIDE_WIN_LEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SLIDE_WIN_LEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLIDE_WIN_LEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLIDE_WIN_LEN` writer - "]
pub struct SLIDE_WIN_LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLIDE_WIN_LEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn bad_cef_atten_para(&self) -> BAD_CEF_ATTEN_PARA_R {
        BAD_CEF_ATTEN_PARA_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn bad_cef_atten_para_shift(&self) -> BAD_CEF_ATTEN_PARA_SHIFT_R {
        BAD_CEF_ATTEN_PARA_SHIFT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn bad_ola_win2_para_shift(&self) -> BAD_OLA_WIN2_PARA_SHIFT_R {
        BAD_OLA_WIN2_PARA_SHIFT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn bad_ola_win2_para(&self) -> BAD_OLA_WIN2_PARA_R {
        BAD_OLA_WIN2_PARA_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn slide_win_len(&self) -> SLIDE_WIN_LEN_R {
        SLIDE_WIN_LEN_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn bad_cef_atten_para(&mut self) -> BAD_CEF_ATTEN_PARA_W {
        BAD_CEF_ATTEN_PARA_W { w: self }
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn bad_cef_atten_para_shift(&mut self) -> BAD_CEF_ATTEN_PARA_SHIFT_W {
        BAD_CEF_ATTEN_PARA_SHIFT_W { w: self }
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn bad_ola_win2_para_shift(&mut self) -> BAD_OLA_WIN2_PARA_SHIFT_W {
        BAD_OLA_WIN2_PARA_SHIFT_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn bad_ola_win2_para(&mut self) -> BAD_OLA_WIN2_PARA_W {
        BAD_OLA_WIN2_PARA_W { w: self }
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn slide_win_len(&mut self) -> SLIDE_WIN_LEN_W {
        SLIDE_WIN_LEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [plc_conf1](index.html) module"]
pub struct PLC_CONF1_SPEC;
impl crate::RegisterSpec for PLC_CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [plc_conf1::R](R) reader structure"]
impl crate::Readable for PLC_CONF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [plc_conf1::W](W) writer structure"]
impl crate::Writable for PLC_CONF1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLC_CONF1 to value 0xa017_8a05"]
impl crate::Resettable for PLC_CONF1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xa017_8a05
    }
}
