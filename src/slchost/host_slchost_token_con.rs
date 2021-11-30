#[doc = "Register `HOST_SLCHOST_TOKEN_CON` writer"]
pub struct W(crate::W<HOST_SLCHOST_TOKEN_CON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HOST_SLCHOST_TOKEN_CON_SPEC>;
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
impl From<crate::W<HOST_SLCHOST_TOKEN_CON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HOST_SLCHOST_TOKEN_CON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HOST_SLC0HOST_TOKEN0_DEC` writer - "]
pub struct HOST_SLC0HOST_TOKEN0_DEC_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC0HOST_TOKEN0_DEC_W<'a> {
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
#[doc = "Field `HOST_SLC0HOST_TOKEN1_DEC` writer - "]
pub struct HOST_SLC0HOST_TOKEN1_DEC_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC0HOST_TOKEN1_DEC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `HOST_SLC0HOST_TOKEN0_WR` writer - "]
pub struct HOST_SLC0HOST_TOKEN0_WR_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC0HOST_TOKEN0_WR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `HOST_SLC0HOST_TOKEN1_WR` writer - "]
pub struct HOST_SLC0HOST_TOKEN1_WR_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC0HOST_TOKEN1_WR_W<'a> {
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
#[doc = "Field `HOST_SLC1HOST_TOKEN0_DEC` writer - "]
pub struct HOST_SLC1HOST_TOKEN0_DEC_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC1HOST_TOKEN0_DEC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `HOST_SLC1HOST_TOKEN1_DEC` writer - "]
pub struct HOST_SLC1HOST_TOKEN1_DEC_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC1HOST_TOKEN1_DEC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `HOST_SLC1HOST_TOKEN0_WR` writer - "]
pub struct HOST_SLC1HOST_TOKEN0_WR_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC1HOST_TOKEN0_WR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `HOST_SLC1HOST_TOKEN1_WR` writer - "]
pub struct HOST_SLC1HOST_TOKEN1_WR_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC1HOST_TOKEN1_WR_W<'a> {
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
#[doc = "Field `HOST_SLC0HOST_LEN_WR` writer - "]
pub struct HOST_SLC0HOST_LEN_WR_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC0HOST_LEN_WR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn host_slc0host_token0_dec(&mut self) -> HOST_SLC0HOST_TOKEN0_DEC_W {
        HOST_SLC0HOST_TOKEN0_DEC_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn host_slc0host_token1_dec(&mut self) -> HOST_SLC0HOST_TOKEN1_DEC_W {
        HOST_SLC0HOST_TOKEN1_DEC_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn host_slc0host_token0_wr(&mut self) -> HOST_SLC0HOST_TOKEN0_WR_W {
        HOST_SLC0HOST_TOKEN0_WR_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn host_slc0host_token1_wr(&mut self) -> HOST_SLC0HOST_TOKEN1_WR_W {
        HOST_SLC0HOST_TOKEN1_WR_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn host_slc1host_token0_dec(&mut self) -> HOST_SLC1HOST_TOKEN0_DEC_W {
        HOST_SLC1HOST_TOKEN0_DEC_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn host_slc1host_token1_dec(&mut self) -> HOST_SLC1HOST_TOKEN1_DEC_W {
        HOST_SLC1HOST_TOKEN1_DEC_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn host_slc1host_token0_wr(&mut self) -> HOST_SLC1HOST_TOKEN0_WR_W {
        HOST_SLC1HOST_TOKEN0_WR_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn host_slc1host_token1_wr(&mut self) -> HOST_SLC1HOST_TOKEN1_WR_W {
        HOST_SLC1HOST_TOKEN1_WR_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn host_slc0host_len_wr(&mut self) -> HOST_SLC0HOST_LEN_WR_W {
        HOST_SLC0HOST_LEN_WR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_slchost_token_con](index.html) module"]
pub struct HOST_SLCHOST_TOKEN_CON_SPEC;
impl crate::RegisterSpec for HOST_SLCHOST_TOKEN_CON_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [host_slchost_token_con::W](W) writer structure"]
impl crate::Writable for HOST_SLCHOST_TOKEN_CON_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HOST_SLCHOST_TOKEN_CON to value 0"]
impl crate::Resettable for HOST_SLCHOST_TOKEN_CON_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
