#[doc = "Register `_1TXFIFO_POP` reader"]
pub struct R(crate::R<_1TXFIFO_POP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<_1TXFIFO_POP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<_1TXFIFO_POP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<_1TXFIFO_POP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `_1TXFIFO_POP` writer"]
pub struct W(crate::W<_1TXFIFO_POP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<_1TXFIFO_POP_SPEC>;
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
impl From<crate::W<_1TXFIFO_POP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<_1TXFIFO_POP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLC1_TXFIFO_RDATA` reader - "]
pub struct SLC1_TXFIFO_RDATA_R(crate::FieldReader<u16, u16>);
impl SLC1_TXFIFO_RDATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        SLC1_TXFIFO_RDATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC1_TXFIFO_RDATA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC1_TXFIFO_POP` reader - "]
pub struct SLC1_TXFIFO_POP_R(crate::FieldReader<bool, bool>);
impl SLC1_TXFIFO_POP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLC1_TXFIFO_POP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC1_TXFIFO_POP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC1_TXFIFO_POP` writer - "]
pub struct SLC1_TXFIFO_POP_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC1_TXFIFO_POP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn slc1_txfifo_rdata(&self) -> SLC1_TXFIFO_RDATA_R {
        SLC1_TXFIFO_RDATA_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slc1_txfifo_pop(&self) -> SLC1_TXFIFO_POP_R {
        SLC1_TXFIFO_POP_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slc1_txfifo_pop(&mut self) -> SLC1_TXFIFO_POP_W {
        SLC1_TXFIFO_POP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_1txfifo_pop](index.html) module"]
pub struct _1TXFIFO_POP_SPEC;
impl crate::RegisterSpec for _1TXFIFO_POP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [_1txfifo_pop::R](R) reader structure"]
impl crate::Readable for _1TXFIFO_POP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [_1txfifo_pop::W](W) writer structure"]
impl crate::Writable for _1TXFIFO_POP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets _1TXFIFO_POP to value 0"]
impl crate::Resettable for _1TXFIFO_POP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
