#[doc = "Register `TOUCH_CFG` reader"]
pub struct R(crate::R<TOUCH_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TOUCH_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TOUCH_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TOUCH_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TOUCH_CFG` writer"]
pub struct W(crate::W<TOUCH_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TOUCH_CFG_SPEC>;
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
impl From<crate::W<TOUCH_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TOUCH_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOUCH_DCUR` reader - touch sensor bias current. Should have option to tie with BIAS_SLEEP(When BIAS_SLEEP this setting is available"]
pub struct TOUCH_DCUR_R(crate::FieldReader<u8, u8>);
impl TOUCH_DCUR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TOUCH_DCUR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_DCUR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_DCUR` writer - touch sensor bias current. Should have option to tie with BIAS_SLEEP(When BIAS_SLEEP this setting is available"]
pub struct TOUCH_DCUR_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_DCUR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 23)) | ((value as u32 & 0x03) << 23);
        self.w
    }
}
#[doc = "Field `TOUCH_DRANGE` reader - touch sensor saw wave voltage range."]
pub struct TOUCH_DRANGE_R(crate::FieldReader<u8, u8>);
impl TOUCH_DRANGE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TOUCH_DRANGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_DRANGE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_DRANGE` writer - touch sensor saw wave voltage range."]
pub struct TOUCH_DRANGE_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_DRANGE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 25)) | ((value as u32 & 0x03) << 25);
        self.w
    }
}
#[doc = "Field `TOUCH_DREFL` reader - touch sensor saw wave bottom voltage."]
pub struct TOUCH_DREFL_R(crate::FieldReader<u8, u8>);
impl TOUCH_DREFL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TOUCH_DREFL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_DREFL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_DREFL` writer - touch sensor saw wave bottom voltage."]
pub struct TOUCH_DREFL_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_DREFL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 27)) | ((value as u32 & 0x03) << 27);
        self.w
    }
}
#[doc = "Field `TOUCH_DREFH` reader - touch sensor saw wave top voltage."]
pub struct TOUCH_DREFH_R(crate::FieldReader<u8, u8>);
impl TOUCH_DREFH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TOUCH_DREFH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_DREFH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_DREFH` writer - touch sensor saw wave top voltage."]
pub struct TOUCH_DREFH_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_DREFH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 29)) | ((value as u32 & 0x03) << 29);
        self.w
    }
}
#[doc = "Field `TOUCH_XPD_BIAS` reader - touch sensor bias power on."]
pub struct TOUCH_XPD_BIAS_R(crate::FieldReader<bool, bool>);
impl TOUCH_XPD_BIAS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOUCH_XPD_BIAS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_XPD_BIAS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_XPD_BIAS` writer - touch sensor bias power on."]
pub struct TOUCH_XPD_BIAS_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_XPD_BIAS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 23:24 - touch sensor bias current. Should have option to tie with BIAS_SLEEP(When BIAS_SLEEP this setting is available"]
    #[inline(always)]
    pub fn touch_dcur(&self) -> TOUCH_DCUR_R {
        TOUCH_DCUR_R::new(((self.bits >> 23) & 0x03) as u8)
    }
    #[doc = "Bits 25:26 - touch sensor saw wave voltage range."]
    #[inline(always)]
    pub fn touch_drange(&self) -> TOUCH_DRANGE_R {
        TOUCH_DRANGE_R::new(((self.bits >> 25) & 0x03) as u8)
    }
    #[doc = "Bits 27:28 - touch sensor saw wave bottom voltage."]
    #[inline(always)]
    pub fn touch_drefl(&self) -> TOUCH_DREFL_R {
        TOUCH_DREFL_R::new(((self.bits >> 27) & 0x03) as u8)
    }
    #[doc = "Bits 29:30 - touch sensor saw wave top voltage."]
    #[inline(always)]
    pub fn touch_drefh(&self) -> TOUCH_DREFH_R {
        TOUCH_DREFH_R::new(((self.bits >> 29) & 0x03) as u8)
    }
    #[doc = "Bit 31 - touch sensor bias power on."]
    #[inline(always)]
    pub fn touch_xpd_bias(&self) -> TOUCH_XPD_BIAS_R {
        TOUCH_XPD_BIAS_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 23:24 - touch sensor bias current. Should have option to tie with BIAS_SLEEP(When BIAS_SLEEP this setting is available"]
    #[inline(always)]
    pub fn touch_dcur(&mut self) -> TOUCH_DCUR_W {
        TOUCH_DCUR_W { w: self }
    }
    #[doc = "Bits 25:26 - touch sensor saw wave voltage range."]
    #[inline(always)]
    pub fn touch_drange(&mut self) -> TOUCH_DRANGE_W {
        TOUCH_DRANGE_W { w: self }
    }
    #[doc = "Bits 27:28 - touch sensor saw wave bottom voltage."]
    #[inline(always)]
    pub fn touch_drefl(&mut self) -> TOUCH_DREFL_W {
        TOUCH_DREFL_W { w: self }
    }
    #[doc = "Bits 29:30 - touch sensor saw wave top voltage."]
    #[inline(always)]
    pub fn touch_drefh(&mut self) -> TOUCH_DREFH_W {
        TOUCH_DREFH_W { w: self }
    }
    #[doc = "Bit 31 - touch sensor bias power on."]
    #[inline(always)]
    pub fn touch_xpd_bias(&mut self) -> TOUCH_XPD_BIAS_W {
        TOUCH_XPD_BIAS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [touch_cfg](index.html) module"]
pub struct TOUCH_CFG_SPEC;
impl crate::RegisterSpec for TOUCH_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [touch_cfg::R](R) reader structure"]
impl crate::Readable for TOUCH_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [touch_cfg::W](W) writer structure"]
impl crate::Writable for TOUCH_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TOUCH_CFG to value 0x6600_0000"]
impl crate::Resettable for TOUCH_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x6600_0000
    }
}
