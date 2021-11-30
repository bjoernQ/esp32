#[doc = "Register `_1TOKEN0` reader"]
pub struct R(crate::R<_1TOKEN0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<_1TOKEN0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<_1TOKEN0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<_1TOKEN0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `_1TOKEN0` writer"]
pub struct W(crate::W<_1TOKEN0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<_1TOKEN0_SPEC>;
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
impl From<crate::W<_1TOKEN0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<_1TOKEN0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLC1_TOKEN0_WDATA` writer - "]
pub struct SLC1_TOKEN0_WDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC1_TOKEN0_WDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
#[doc = "Field `SLC1_TOKEN0_WR` writer - "]
pub struct SLC1_TOKEN0_WR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC1_TOKEN0_WR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `SLC1_TOKEN0_INC` writer - "]
pub struct SLC1_TOKEN0_INC_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC1_TOKEN0_INC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `SLC1_TOKEN0_INC_MORE` writer - "]
pub struct SLC1_TOKEN0_INC_MORE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC1_TOKEN0_INC_MORE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `SLC1_TOKEN0` reader - "]
pub struct SLC1_TOKEN0_R(crate::FieldReader<u16, u16>);
impl SLC1_TOKEN0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        SLC1_TOKEN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC1_TOKEN0_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn slc1_token0(&self) -> SLC1_TOKEN0_R {
        SLC1_TOKEN0_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn slc1_token0_wdata(&mut self) -> SLC1_TOKEN0_WDATA_W {
        SLC1_TOKEN0_WDATA_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn slc1_token0_wr(&mut self) -> SLC1_TOKEN0_WR_W {
        SLC1_TOKEN0_WR_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn slc1_token0_inc(&mut self) -> SLC1_TOKEN0_INC_W {
        SLC1_TOKEN0_INC_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn slc1_token0_inc_more(&mut self) -> SLC1_TOKEN0_INC_MORE_W {
        SLC1_TOKEN0_INC_MORE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_1token0](index.html) module"]
pub struct _1TOKEN0_SPEC;
impl crate::RegisterSpec for _1TOKEN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [_1token0::R](R) reader structure"]
impl crate::Readable for _1TOKEN0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [_1token0::W](W) writer structure"]
impl crate::Writable for _1TOKEN0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets _1TOKEN0 to value 0"]
impl crate::Resettable for _1TOKEN0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
