#[doc = "Register `EXT0` reader"]
pub struct R(crate::R<EXT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXT0` writer"]
pub struct W(crate::W<EXT0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXT0_SPEC>;
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
impl From<crate::W<EXT0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXT0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T_PP_TIME` reader - page program delay time by system clock."]
pub struct T_PP_TIME_R(crate::FieldReader<u16, u16>);
impl T_PP_TIME_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        T_PP_TIME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T_PP_TIME_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T_PP_TIME` writer - page program delay time by system clock."]
pub struct T_PP_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> T_PP_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
#[doc = "Field `T_PP_SHIFT` reader - page program delay time shift ."]
pub struct T_PP_SHIFT_R(crate::FieldReader<u8, u8>);
impl T_PP_SHIFT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        T_PP_SHIFT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T_PP_SHIFT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T_PP_SHIFT` writer - page program delay time shift ."]
pub struct T_PP_SHIFT_W<'a> {
    w: &'a mut W,
}
impl<'a> T_PP_SHIFT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `T_PP_ENA` reader - page program delay enable."]
pub struct T_PP_ENA_R(crate::FieldReader<bool, bool>);
impl T_PP_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        T_PP_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T_PP_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T_PP_ENA` writer - page program delay enable."]
pub struct T_PP_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> T_PP_ENA_W<'a> {
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
    #[doc = "Bits 0:11 - page program delay time by system clock."]
    #[inline(always)]
    pub fn t_pp_time(&self) -> T_PP_TIME_R {
        T_PP_TIME_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:19 - page program delay time shift ."]
    #[inline(always)]
    pub fn t_pp_shift(&self) -> T_PP_SHIFT_R {
        T_PP_SHIFT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - page program delay enable."]
    #[inline(always)]
    pub fn t_pp_ena(&self) -> T_PP_ENA_R {
        T_PP_ENA_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - page program delay time by system clock."]
    #[inline(always)]
    pub fn t_pp_time(&mut self) -> T_PP_TIME_W {
        T_PP_TIME_W { w: self }
    }
    #[doc = "Bits 16:19 - page program delay time shift ."]
    #[inline(always)]
    pub fn t_pp_shift(&mut self) -> T_PP_SHIFT_W {
        T_PP_SHIFT_W { w: self }
    }
    #[doc = "Bit 31 - page program delay enable."]
    #[inline(always)]
    pub fn t_pp_ena(&mut self) -> T_PP_ENA_W {
        T_PP_ENA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ext0](index.html) module"]
pub struct EXT0_SPEC;
impl crate::RegisterSpec for EXT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ext0::R](R) reader structure"]
impl crate::Readable for EXT0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ext0::W](W) writer structure"]
impl crate::Writable for EXT0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXT0 to value 0x800a_0050"]
impl crate::Resettable for EXT0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x800a_0050
    }
}
