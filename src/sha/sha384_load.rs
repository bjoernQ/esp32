#[doc = "Register `SHA384_LOAD` writer"]
pub struct W(crate::W<SHA384_LOAD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHA384_LOAD_SPEC>;
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
impl From<crate::W<SHA384_LOAD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHA384_LOAD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SHA384_LOAD` writer - Write 1 to finish the SHA-384 operation to calculate the final message hash."]
pub struct SHA384_LOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> SHA384_LOAD_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to finish the SHA-384 operation to calculate the final message hash."]
    #[inline(always)]
    pub fn sha384_load(&mut self) -> SHA384_LOAD_W {
        SHA384_LOAD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sha384_load](index.html) module"]
pub struct SHA384_LOAD_SPEC;
impl crate::RegisterSpec for SHA384_LOAD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [sha384_load::W](W) writer structure"]
impl crate::Writable for SHA384_LOAD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SHA384_LOAD to value 0"]
impl crate::Resettable for SHA384_LOAD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
