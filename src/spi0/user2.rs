#[doc = "Register `USER2` reader"]
pub struct R(crate::R<USER2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USER2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USER2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USER2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USER2` writer"]
pub struct W(crate::W<USER2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USER2_SPEC>;
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
impl From<crate::W<USER2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USER2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USR_COMMAND_VALUE` reader - The value of command."]
pub struct USR_COMMAND_VALUE_R(crate::FieldReader<u16, u16>);
impl USR_COMMAND_VALUE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        USR_COMMAND_VALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USR_COMMAND_VALUE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USR_COMMAND_VALUE` writer - The value of command."]
pub struct USR_COMMAND_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> USR_COMMAND_VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `USR_COMMAND_BITLEN` reader - The length in bits of command phase. The register value shall be (bit_num-1)"]
pub struct USR_COMMAND_BITLEN_R(crate::FieldReader<u8, u8>);
impl USR_COMMAND_BITLEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        USR_COMMAND_BITLEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USR_COMMAND_BITLEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USR_COMMAND_BITLEN` writer - The length in bits of command phase. The register value shall be (bit_num-1)"]
pub struct USR_COMMAND_BITLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USR_COMMAND_BITLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - The value of command."]
    #[inline(always)]
    pub fn usr_command_value(&self) -> USR_COMMAND_VALUE_R {
        USR_COMMAND_VALUE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 28:31 - The length in bits of command phase. The register value shall be (bit_num-1)"]
    #[inline(always)]
    pub fn usr_command_bitlen(&self) -> USR_COMMAND_BITLEN_R {
        USR_COMMAND_BITLEN_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - The value of command."]
    #[inline(always)]
    pub fn usr_command_value(&mut self) -> USR_COMMAND_VALUE_W {
        USR_COMMAND_VALUE_W { w: self }
    }
    #[doc = "Bits 28:31 - The length in bits of command phase. The register value shall be (bit_num-1)"]
    #[inline(always)]
    pub fn usr_command_bitlen(&mut self) -> USR_COMMAND_BITLEN_W {
        USR_COMMAND_BITLEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [user2](index.html) module"]
pub struct USER2_SPEC;
impl crate::RegisterSpec for USER2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [user2::R](R) reader structure"]
impl crate::Readable for USER2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [user2::W](W) writer structure"]
impl crate::Writable for USER2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USER2 to value 0x7000_0000"]
impl crate::Resettable for USER2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x7000_0000
    }
}
