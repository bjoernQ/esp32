#[doc = "Register `PLC_CONF0` reader"]
pub struct R(crate::R<PLC_CONF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLC_CONF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLC_CONF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLC_CONF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLC_CONF0` writer"]
pub struct W(crate::W<PLC_CONF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLC_CONF0_SPEC>;
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
impl From<crate::W<PLC_CONF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLC_CONF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GOOD_PACK_MAX` reader - "]
pub struct GOOD_PACK_MAX_R(crate::FieldReader<u8, u8>);
impl GOOD_PACK_MAX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GOOD_PACK_MAX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GOOD_PACK_MAX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GOOD_PACK_MAX` writer - "]
pub struct GOOD_PACK_MAX_W<'a> {
    w: &'a mut W,
}
impl<'a> GOOD_PACK_MAX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Field `N_ERR_SEG` reader - "]
pub struct N_ERR_SEG_R(crate::FieldReader<u8, u8>);
impl N_ERR_SEG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        N_ERR_SEG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for N_ERR_SEG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `N_ERR_SEG` writer - "]
pub struct N_ERR_SEG_W<'a> {
    w: &'a mut W,
}
impl<'a> N_ERR_SEG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | ((value as u32 & 0x07) << 6);
        self.w
    }
}
#[doc = "Field `SHIFT_RATE` reader - "]
pub struct SHIFT_RATE_R(crate::FieldReader<u8, u8>);
impl SHIFT_RATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SHIFT_RATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SHIFT_RATE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SHIFT_RATE` writer - "]
pub struct SHIFT_RATE_W<'a> {
    w: &'a mut W,
}
impl<'a> SHIFT_RATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | ((value as u32 & 0x07) << 9);
        self.w
    }
}
#[doc = "Field `MAX_SLIDE_SAMPLE` reader - "]
pub struct MAX_SLIDE_SAMPLE_R(crate::FieldReader<u8, u8>);
impl MAX_SLIDE_SAMPLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MAX_SLIDE_SAMPLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAX_SLIDE_SAMPLE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAX_SLIDE_SAMPLE` writer - "]
pub struct MAX_SLIDE_SAMPLE_W<'a> {
    w: &'a mut W,
}
impl<'a> MAX_SLIDE_SAMPLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 12)) | ((value as u32 & 0xff) << 12);
        self.w
    }
}
#[doc = "Field `PACK_LEN_8K` reader - "]
pub struct PACK_LEN_8K_R(crate::FieldReader<u8, u8>);
impl PACK_LEN_8K_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PACK_LEN_8K_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PACK_LEN_8K_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PACK_LEN_8K` writer - "]
pub struct PACK_LEN_8K_W<'a> {
    w: &'a mut W,
}
impl<'a> PACK_LEN_8K_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 20)) | ((value as u32 & 0x1f) << 20);
        self.w
    }
}
#[doc = "Field `N_MIN_ERR` reader - "]
pub struct N_MIN_ERR_R(crate::FieldReader<u8, u8>);
impl N_MIN_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        N_MIN_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for N_MIN_ERR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `N_MIN_ERR` writer - "]
pub struct N_MIN_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> N_MIN_ERR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 25)) | ((value as u32 & 0x07) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn good_pack_max(&self) -> GOOD_PACK_MAX_R {
        GOOD_PACK_MAX_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:8"]
    #[inline(always)]
    pub fn n_err_seg(&self) -> N_ERR_SEG_R {
        N_ERR_SEG_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn shift_rate(&self) -> SHIFT_RATE_R {
        SHIFT_RATE_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bits 12:19"]
    #[inline(always)]
    pub fn max_slide_sample(&self) -> MAX_SLIDE_SAMPLE_R {
        MAX_SLIDE_SAMPLE_R::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn pack_len_8k(&self) -> PACK_LEN_8K_R {
        PACK_LEN_8K_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:27"]
    #[inline(always)]
    pub fn n_min_err(&self) -> N_MIN_ERR_R {
        N_MIN_ERR_R::new(((self.bits >> 25) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn good_pack_max(&mut self) -> GOOD_PACK_MAX_W {
        GOOD_PACK_MAX_W { w: self }
    }
    #[doc = "Bits 6:8"]
    #[inline(always)]
    pub fn n_err_seg(&mut self) -> N_ERR_SEG_W {
        N_ERR_SEG_W { w: self }
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn shift_rate(&mut self) -> SHIFT_RATE_W {
        SHIFT_RATE_W { w: self }
    }
    #[doc = "Bits 12:19"]
    #[inline(always)]
    pub fn max_slide_sample(&mut self) -> MAX_SLIDE_SAMPLE_W {
        MAX_SLIDE_SAMPLE_W { w: self }
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn pack_len_8k(&mut self) -> PACK_LEN_8K_W {
        PACK_LEN_8K_W { w: self }
    }
    #[doc = "Bits 25:27"]
    #[inline(always)]
    pub fn n_min_err(&mut self) -> N_MIN_ERR_W {
        N_MIN_ERR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [plc_conf0](index.html) module"]
pub struct PLC_CONF0_SPEC;
impl crate::RegisterSpec for PLC_CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [plc_conf0::R](R) reader structure"]
impl crate::Readable for PLC_CONF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [plc_conf0::W](W) writer structure"]
impl crate::Writable for PLC_CONF0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLC_CONF0 to value 0x08a8_0339"]
impl crate::Resettable for PLC_CONF0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x08a8_0339
    }
}
