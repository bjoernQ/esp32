#[doc = "Register `HOST_SLCHOST_FUNC2_2` reader"]
pub struct R(crate::R<HOST_SLCHOST_FUNC2_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOST_SLCHOST_FUNC2_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOST_SLCHOST_FUNC2_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOST_SLCHOST_FUNC2_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HOST_SLCHOST_FUNC2_2` writer"]
pub struct W(crate::W<HOST_SLCHOST_FUNC2_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HOST_SLCHOST_FUNC2_2_SPEC>;
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
impl From<crate::W<HOST_SLCHOST_FUNC2_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HOST_SLCHOST_FUNC2_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HOST_SLC_FUNC1_MDSTAT` reader - "]
pub struct HOST_SLC_FUNC1_MDSTAT_R(crate::FieldReader<bool, bool>);
impl HOST_SLC_FUNC1_MDSTAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HOST_SLC_FUNC1_MDSTAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOST_SLC_FUNC1_MDSTAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOST_SLC_FUNC1_MDSTAT` writer - "]
pub struct HOST_SLC_FUNC1_MDSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC_FUNC1_MDSTAT_W<'a> {
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
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn host_slc_func1_mdstat(&self) -> HOST_SLC_FUNC1_MDSTAT_R {
        HOST_SLC_FUNC1_MDSTAT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn host_slc_func1_mdstat(&mut self) -> HOST_SLC_FUNC1_MDSTAT_W {
        HOST_SLC_FUNC1_MDSTAT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_slchost_func2_2](index.html) module"]
pub struct HOST_SLCHOST_FUNC2_2_SPEC;
impl crate::RegisterSpec for HOST_SLCHOST_FUNC2_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [host_slchost_func2_2::R](R) reader structure"]
impl crate::Readable for HOST_SLCHOST_FUNC2_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [host_slchost_func2_2::W](W) writer structure"]
impl crate::Writable for HOST_SLCHOST_FUNC2_2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HOST_SLCHOST_FUNC2_2 to value 0x01"]
impl crate::Resettable for HOST_SLCHOST_FUNC2_2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
