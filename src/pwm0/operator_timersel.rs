#[doc = "Register `OPERATOR_TIMERSEL` reader"]
pub struct R(crate::R<OPERATOR_TIMERSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPERATOR_TIMERSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPERATOR_TIMERSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPERATOR_TIMERSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPERATOR_TIMERSEL` writer"]
pub struct W(crate::W<OPERATOR_TIMERSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPERATOR_TIMERSEL_SPEC>;
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
impl From<crate::W<OPERATOR_TIMERSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPERATOR_TIMERSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OPERATOR0_TIMERSEL` reader - "]
pub struct OPERATOR0_TIMERSEL_R(crate::FieldReader<u8, u8>);
impl OPERATOR0_TIMERSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OPERATOR0_TIMERSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPERATOR0_TIMERSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPERATOR0_TIMERSEL` writer - "]
pub struct OPERATOR0_TIMERSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OPERATOR0_TIMERSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `OPERATOR1_TIMERSEL` reader - "]
pub struct OPERATOR1_TIMERSEL_R(crate::FieldReader<u8, u8>);
impl OPERATOR1_TIMERSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OPERATOR1_TIMERSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPERATOR1_TIMERSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPERATOR1_TIMERSEL` writer - "]
pub struct OPERATOR1_TIMERSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OPERATOR1_TIMERSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `OPERATOR2_TIMERSEL` reader - "]
pub struct OPERATOR2_TIMERSEL_R(crate::FieldReader<u8, u8>);
impl OPERATOR2_TIMERSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OPERATOR2_TIMERSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPERATOR2_TIMERSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPERATOR2_TIMERSEL` writer - "]
pub struct OPERATOR2_TIMERSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OPERATOR2_TIMERSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn operator0_timersel(&self) -> OPERATOR0_TIMERSEL_R {
        OPERATOR0_TIMERSEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn operator1_timersel(&self) -> OPERATOR1_TIMERSEL_R {
        OPERATOR1_TIMERSEL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn operator2_timersel(&self) -> OPERATOR2_TIMERSEL_R {
        OPERATOR2_TIMERSEL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn operator0_timersel(&mut self) -> OPERATOR0_TIMERSEL_W {
        OPERATOR0_TIMERSEL_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn operator1_timersel(&mut self) -> OPERATOR1_TIMERSEL_W {
        OPERATOR1_TIMERSEL_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn operator2_timersel(&mut self) -> OPERATOR2_TIMERSEL_W {
        OPERATOR2_TIMERSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [operator_timersel](index.html) module"]
pub struct OPERATOR_TIMERSEL_SPEC;
impl crate::RegisterSpec for OPERATOR_TIMERSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [operator_timersel::R](R) reader structure"]
impl crate::Readable for OPERATOR_TIMERSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [operator_timersel::W](W) writer structure"]
impl crate::Writable for OPERATOR_TIMERSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OPERATOR_TIMERSEL to value 0"]
impl crate::Resettable for OPERATOR_TIMERSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
