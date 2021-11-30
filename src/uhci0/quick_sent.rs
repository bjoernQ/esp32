#[doc = "Register `QUICK_SENT` reader"]
pub struct R(crate::R<QUICK_SENT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QUICK_SENT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QUICK_SENT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QUICK_SENT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `QUICK_SENT` writer"]
pub struct W(crate::W<QUICK_SENT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QUICK_SENT_SPEC>;
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
impl From<crate::W<QUICK_SENT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QUICK_SENT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SINGLE_SEND_NUM` reader - The bits are used to choose which short packet"]
pub struct SINGLE_SEND_NUM_R(crate::FieldReader<u8, u8>);
impl SINGLE_SEND_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SINGLE_SEND_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SINGLE_SEND_NUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SINGLE_SEND_NUM` writer - The bits are used to choose which short packet"]
pub struct SINGLE_SEND_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> SINGLE_SEND_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `SINGLE_SEND_EN` reader - Set this bit to enable send a short packet"]
pub struct SINGLE_SEND_EN_R(crate::FieldReader<bool, bool>);
impl SINGLE_SEND_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SINGLE_SEND_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SINGLE_SEND_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SINGLE_SEND_EN` writer - Set this bit to enable send a short packet"]
pub struct SINGLE_SEND_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SINGLE_SEND_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `ALWAYS_SEND_NUM` reader - The bits are used to choose which short packet"]
pub struct ALWAYS_SEND_NUM_R(crate::FieldReader<u8, u8>);
impl ALWAYS_SEND_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ALWAYS_SEND_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALWAYS_SEND_NUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALWAYS_SEND_NUM` writer - The bits are used to choose which short packet"]
pub struct ALWAYS_SEND_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> ALWAYS_SEND_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `ALWAYS_SEND_EN` reader - Set this bit to enable continuously send the same short packet"]
pub struct ALWAYS_SEND_EN_R(crate::FieldReader<bool, bool>);
impl ALWAYS_SEND_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALWAYS_SEND_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALWAYS_SEND_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALWAYS_SEND_EN` writer - Set this bit to enable continuously send the same short packet"]
pub struct ALWAYS_SEND_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALWAYS_SEND_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - The bits are used to choose which short packet"]
    #[inline(always)]
    pub fn single_send_num(&self) -> SINGLE_SEND_NUM_R {
        SINGLE_SEND_NUM_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - Set this bit to enable send a short packet"]
    #[inline(always)]
    pub fn single_send_en(&self) -> SINGLE_SEND_EN_R {
        SINGLE_SEND_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - The bits are used to choose which short packet"]
    #[inline(always)]
    pub fn always_send_num(&self) -> ALWAYS_SEND_NUM_R {
        ALWAYS_SEND_NUM_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - Set this bit to enable continuously send the same short packet"]
    #[inline(always)]
    pub fn always_send_en(&self) -> ALWAYS_SEND_EN_R {
        ALWAYS_SEND_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - The bits are used to choose which short packet"]
    #[inline(always)]
    pub fn single_send_num(&mut self) -> SINGLE_SEND_NUM_W {
        SINGLE_SEND_NUM_W { w: self }
    }
    #[doc = "Bit 3 - Set this bit to enable send a short packet"]
    #[inline(always)]
    pub fn single_send_en(&mut self) -> SINGLE_SEND_EN_W {
        SINGLE_SEND_EN_W { w: self }
    }
    #[doc = "Bits 4:6 - The bits are used to choose which short packet"]
    #[inline(always)]
    pub fn always_send_num(&mut self) -> ALWAYS_SEND_NUM_W {
        ALWAYS_SEND_NUM_W { w: self }
    }
    #[doc = "Bit 7 - Set this bit to enable continuously send the same short packet"]
    #[inline(always)]
    pub fn always_send_en(&mut self) -> ALWAYS_SEND_EN_W {
        ALWAYS_SEND_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [quick_sent](index.html) module"]
pub struct QUICK_SENT_SPEC;
impl crate::RegisterSpec for QUICK_SENT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [quick_sent::R](R) reader structure"]
impl crate::Readable for QUICK_SENT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [quick_sent::W](W) writer structure"]
impl crate::Writable for QUICK_SENT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets QUICK_SENT to value 0"]
impl crate::Resettable for QUICK_SENT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
