#[doc = "Register `SHA1_BUSY` writer"]
pub struct W(crate::W<SHA1_BUSY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHA1_BUSY_SPEC>;
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
impl From<crate::W<SHA1_BUSY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHA1_BUSY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SHA1_BUSY` writer - SHA-1 operation status: 1 if the SHA accelerator is processing data, 0 if it is idle."]
pub struct SHA1_BUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> SHA1_BUSY_W<'a> {
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
    #[doc = "Bit 0 - SHA-1 operation status: 1 if the SHA accelerator is processing data, 0 if it is idle."]
    #[inline(always)]
    pub fn sha1_busy(&mut self) -> SHA1_BUSY_W {
        SHA1_BUSY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sha1_busy](index.html) module"]
pub struct SHA1_BUSY_SPEC;
impl crate::RegisterSpec for SHA1_BUSY_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [sha1_busy::W](W) writer structure"]
impl crate::Writable for SHA1_BUSY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SHA1_BUSY to value 0"]
impl crate::Resettable for SHA1_BUSY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
