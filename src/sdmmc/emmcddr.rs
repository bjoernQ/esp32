#[doc = "Register `EMMCDDR` reader"]
pub struct R(crate::R<EMMCDDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMMCDDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMMCDDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMMCDDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EMMCDDR` writer"]
pub struct W(crate::W<EMMCDDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMMCDDR_SPEC>;
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
impl From<crate::W<EMMCDDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMMCDDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HALFSTARTBIT` reader - Control for start bit detection mechanism duration of start bit.Each bit refers to one slot.Set this bit to 1 for eMMC4.5 and above,set to 0 for SD applications.For eMMC4.5,start bit can be: 1'b0-Full cycle. 1'b1-less than one full cycle."]
pub struct HALFSTARTBIT_R(crate::FieldReader<u8, u8>);
impl HALFSTARTBIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        HALFSTARTBIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HALFSTARTBIT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HALFSTARTBIT` writer - Control for start bit detection mechanism duration of start bit.Each bit refers to one slot.Set this bit to 1 for eMMC4.5 and above,set to 0 for SD applications.For eMMC4.5,start bit can be: 1'b0-Full cycle. 1'b1-less than one full cycle."]
pub struct HALFSTARTBIT_W<'a> {
    w: &'a mut W,
}
impl<'a> HALFSTARTBIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `HS400_MODE` reader - Set 1 to enable HS400 mode."]
pub struct HS400_MODE_R(crate::FieldReader<bool, bool>);
impl HS400_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HS400_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HS400_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HS400_MODE` writer - Set 1 to enable HS400 mode."]
pub struct HS400_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> HS400_MODE_W<'a> {
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
    #[doc = "Bits 0:1 - Control for start bit detection mechanism duration of start bit.Each bit refers to one slot.Set this bit to 1 for eMMC4.5 and above,set to 0 for SD applications.For eMMC4.5,start bit can be: 1'b0-Full cycle. 1'b1-less than one full cycle."]
    #[inline(always)]
    pub fn halfstartbit(&self) -> HALFSTARTBIT_R {
        HALFSTARTBIT_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 31 - Set 1 to enable HS400 mode."]
    #[inline(always)]
    pub fn hs400_mode(&self) -> HS400_MODE_R {
        HS400_MODE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Control for start bit detection mechanism duration of start bit.Each bit refers to one slot.Set this bit to 1 for eMMC4.5 and above,set to 0 for SD applications.For eMMC4.5,start bit can be: 1'b0-Full cycle. 1'b1-less than one full cycle."]
    #[inline(always)]
    pub fn halfstartbit(&mut self) -> HALFSTARTBIT_W {
        HALFSTARTBIT_W { w: self }
    }
    #[doc = "Bit 31 - Set 1 to enable HS400 mode."]
    #[inline(always)]
    pub fn hs400_mode(&mut self) -> HS400_MODE_W {
        HS400_MODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "eMMC DDR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emmcddr](index.html) module"]
pub struct EMMCDDR_SPEC;
impl crate::RegisterSpec for EMMCDDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emmcddr::R](R) reader structure"]
impl crate::Readable for EMMCDDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emmcddr::W](W) writer structure"]
impl crate::Writable for EMMCDDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EMMCDDR to value 0"]
impl crate::Resettable for EMMCDDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
