#[doc = "Register `USER1` reader"]
pub struct R(crate::R<USER1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USER1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USER1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USER1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USER1` writer"]
pub struct W(crate::W<USER1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USER1_SPEC>;
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
impl From<crate::W<USER1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USER1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USR_DUMMY_CYCLELEN` reader - The length in spi_clk cycles of dummy phase. The register value shall be (cycle_num-1)."]
pub struct USR_DUMMY_CYCLELEN_R(crate::FieldReader<u8, u8>);
impl USR_DUMMY_CYCLELEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        USR_DUMMY_CYCLELEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USR_DUMMY_CYCLELEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USR_DUMMY_CYCLELEN` writer - The length in spi_clk cycles of dummy phase. The register value shall be (cycle_num-1)."]
pub struct USR_DUMMY_CYCLELEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USR_DUMMY_CYCLELEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `USR_ADDR_BITLEN` reader - The length in bits of address phase. The register value shall be (bit_num-1)."]
pub struct USR_ADDR_BITLEN_R(crate::FieldReader<u8, u8>);
impl USR_ADDR_BITLEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        USR_ADDR_BITLEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USR_ADDR_BITLEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - The length in spi_clk cycles of dummy phase. The register value shall be (cycle_num-1)."]
    #[inline(always)]
    pub fn usr_dummy_cyclelen(&self) -> USR_DUMMY_CYCLELEN_R {
        USR_DUMMY_CYCLELEN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 26:31 - The length in bits of address phase. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn usr_addr_bitlen(&self) -> USR_ADDR_BITLEN_R {
        USR_ADDR_BITLEN_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The length in spi_clk cycles of dummy phase. The register value shall be (cycle_num-1)."]
    #[inline(always)]
    pub fn usr_dummy_cyclelen(&mut self) -> USR_DUMMY_CYCLELEN_W {
        USR_DUMMY_CYCLELEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [user1](index.html) module"]
pub struct USER1_SPEC;
impl crate::RegisterSpec for USER1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [user1::R](R) reader structure"]
impl crate::Readable for USER1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [user1::W](W) writer structure"]
impl crate::Writable for USER1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USER1 to value 0x5c00_0007"]
impl crate::Resettable for USER1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x5c00_0007
    }
}
