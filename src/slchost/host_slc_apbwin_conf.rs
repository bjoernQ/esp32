#[doc = "Register `HOST_SLC_APBWIN_CONF` reader"]
pub struct R(crate::R<HOST_SLC_APBWIN_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOST_SLC_APBWIN_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOST_SLC_APBWIN_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOST_SLC_APBWIN_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HOST_SLC_APBWIN_CONF` writer"]
pub struct W(crate::W<HOST_SLC_APBWIN_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HOST_SLC_APBWIN_CONF_SPEC>;
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
impl From<crate::W<HOST_SLC_APBWIN_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HOST_SLC_APBWIN_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HOST_SLC_APBWIN_ADDR` reader - "]
pub struct HOST_SLC_APBWIN_ADDR_R(crate::FieldReader<u32, u32>);
impl HOST_SLC_APBWIN_ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        HOST_SLC_APBWIN_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOST_SLC_APBWIN_ADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOST_SLC_APBWIN_ADDR` writer - "]
pub struct HOST_SLC_APBWIN_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC_APBWIN_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff_ffff) | (value as u32 & 0x0fff_ffff);
        self.w
    }
}
#[doc = "Field `HOST_SLC_APBWIN_WR` reader - "]
pub struct HOST_SLC_APBWIN_WR_R(crate::FieldReader<bool, bool>);
impl HOST_SLC_APBWIN_WR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HOST_SLC_APBWIN_WR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOST_SLC_APBWIN_WR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOST_SLC_APBWIN_WR` writer - "]
pub struct HOST_SLC_APBWIN_WR_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC_APBWIN_WR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `HOST_SLC_APBWIN_START` reader - "]
pub struct HOST_SLC_APBWIN_START_R(crate::FieldReader<bool, bool>);
impl HOST_SLC_APBWIN_START_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HOST_SLC_APBWIN_START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOST_SLC_APBWIN_START_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOST_SLC_APBWIN_START` writer - "]
pub struct HOST_SLC_APBWIN_START_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLC_APBWIN_START_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:27"]
    #[inline(always)]
    pub fn host_slc_apbwin_addr(&self) -> HOST_SLC_APBWIN_ADDR_R {
        HOST_SLC_APBWIN_ADDR_R::new((self.bits & 0x0fff_ffff) as u32)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn host_slc_apbwin_wr(&self) -> HOST_SLC_APBWIN_WR_R {
        HOST_SLC_APBWIN_WR_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn host_slc_apbwin_start(&self) -> HOST_SLC_APBWIN_START_R {
        HOST_SLC_APBWIN_START_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:27"]
    #[inline(always)]
    pub fn host_slc_apbwin_addr(&mut self) -> HOST_SLC_APBWIN_ADDR_W {
        HOST_SLC_APBWIN_ADDR_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn host_slc_apbwin_wr(&mut self) -> HOST_SLC_APBWIN_WR_W {
        HOST_SLC_APBWIN_WR_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn host_slc_apbwin_start(&mut self) -> HOST_SLC_APBWIN_START_W {
        HOST_SLC_APBWIN_START_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_slc_apbwin_conf](index.html) module"]
pub struct HOST_SLC_APBWIN_CONF_SPEC;
impl crate::RegisterSpec for HOST_SLC_APBWIN_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [host_slc_apbwin_conf::R](R) reader structure"]
impl crate::Readable for HOST_SLC_APBWIN_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [host_slc_apbwin_conf::W](W) writer structure"]
impl crate::Writable for HOST_SLC_APBWIN_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HOST_SLC_APBWIN_CONF to value 0"]
impl crate::Resettable for HOST_SLC_APBWIN_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
