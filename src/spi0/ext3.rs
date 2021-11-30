#[doc = "Register `EXT3` reader"]
pub struct R(crate::R<EXT3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXT3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXT3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXT3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXT3` writer"]
pub struct W(crate::W<EXT3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXT3_SPEC>;
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
impl From<crate::W<EXT3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXT3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT_HOLD_ENA` reader - This register is for two SPI masters to share the same cs clock and data signals. The bits of one SPI are set if the other SPI is busy the SPI will be hold. 1(3): hold at ¡°idle¡± phase 2: hold at ¡°prepare¡± phase."]
pub struct INT_HOLD_ENA_R(crate::FieldReader<u8, u8>);
impl INT_HOLD_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        INT_HOLD_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT_HOLD_ENA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT_HOLD_ENA` writer - This register is for two SPI masters to share the same cs clock and data signals. The bits of one SPI are set if the other SPI is busy the SPI will be hold. 1(3): hold at ¡°idle¡± phase 2: hold at ¡°prepare¡± phase."]
pub struct INT_HOLD_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_HOLD_ENA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - This register is for two SPI masters to share the same cs clock and data signals. The bits of one SPI are set if the other SPI is busy the SPI will be hold. 1(3): hold at ¡°idle¡± phase 2: hold at ¡°prepare¡± phase."]
    #[inline(always)]
    pub fn int_hold_ena(&self) -> INT_HOLD_ENA_R {
        INT_HOLD_ENA_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - This register is for two SPI masters to share the same cs clock and data signals. The bits of one SPI are set if the other SPI is busy the SPI will be hold. 1(3): hold at ¡°idle¡± phase 2: hold at ¡°prepare¡± phase."]
    #[inline(always)]
    pub fn int_hold_ena(&mut self) -> INT_HOLD_ENA_W {
        INT_HOLD_ENA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ext3](index.html) module"]
pub struct EXT3_SPEC;
impl crate::RegisterSpec for EXT3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ext3::R](R) reader structure"]
impl crate::Readable for EXT3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ext3::W](W) writer structure"]
impl crate::Writable for EXT3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXT3 to value 0"]
impl crate::Resettable for EXT3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
