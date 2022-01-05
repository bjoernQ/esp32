#[doc = "Register `SEQ_POSITION` reader"]
pub struct R(crate::R<SEQ_POSITION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEQ_POSITION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEQ_POSITION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEQ_POSITION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEQ_POSITION` writer"]
pub struct W(crate::W<SEQ_POSITION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEQ_POSITION_SPEC>;
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
impl From<crate::W<SEQ_POSITION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEQ_POSITION_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLC0_SEQ_POSITION` reader - "]
pub struct SLC0_SEQ_POSITION_R(crate::FieldReader<u8, u8>);
impl SLC0_SEQ_POSITION_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SLC0_SEQ_POSITION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC0_SEQ_POSITION_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC0_SEQ_POSITION` writer - "]
pub struct SLC0_SEQ_POSITION_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_SEQ_POSITION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `SLC1_SEQ_POSITION` reader - "]
pub struct SLC1_SEQ_POSITION_R(crate::FieldReader<u8, u8>);
impl SLC1_SEQ_POSITION_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SLC1_SEQ_POSITION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC1_SEQ_POSITION_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC1_SEQ_POSITION` writer - "]
pub struct SLC1_SEQ_POSITION_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC1_SEQ_POSITION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn slc0_seq_position(&self) -> SLC0_SEQ_POSITION_R {
        SLC0_SEQ_POSITION_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn slc1_seq_position(&self) -> SLC1_SEQ_POSITION_R {
        SLC1_SEQ_POSITION_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn slc0_seq_position(&mut self) -> SLC0_SEQ_POSITION_W {
        SLC0_SEQ_POSITION_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn slc1_seq_position(&mut self) -> SLC1_SEQ_POSITION_W {
        SLC1_SEQ_POSITION_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seq_position](index.html) module"]
pub struct SEQ_POSITION_SPEC;
impl crate::RegisterSpec for SEQ_POSITION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [seq_position::R](R) reader structure"]
impl crate::Readable for SEQ_POSITION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [seq_position::W](W) writer structure"]
impl crate::Writable for SEQ_POSITION_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SEQ_POSITION to value 0x0509"]
impl crate::Resettable for SEQ_POSITION_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0509
    }
}