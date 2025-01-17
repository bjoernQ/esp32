#[doc = "Register `HOST_SLCHOST_RDCLR0` reader"]
pub struct R(crate::R<HOST_SLCHOST_RDCLR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOST_SLCHOST_RDCLR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOST_SLCHOST_RDCLR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOST_SLCHOST_RDCLR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HOST_SLCHOST_RDCLR0` writer"]
pub struct W(crate::W<HOST_SLCHOST_RDCLR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HOST_SLCHOST_RDCLR0_SPEC>;
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
impl From<crate::W<HOST_SLCHOST_RDCLR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HOST_SLCHOST_RDCLR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HOST_SLCHOST_SLC0_BIT7_CLRADDR` reader - "]
pub struct HOST_SLCHOST_SLC0_BIT7_CLRADDR_R(crate::FieldReader<u16, u16>);
impl HOST_SLCHOST_SLC0_BIT7_CLRADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        HOST_SLCHOST_SLC0_BIT7_CLRADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOST_SLCHOST_SLC0_BIT7_CLRADDR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOST_SLCHOST_SLC0_BIT7_CLRADDR` writer - "]
pub struct HOST_SLCHOST_SLC0_BIT7_CLRADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_SLC0_BIT7_CLRADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | (value as u32 & 0x01ff);
        self.w
    }
}
#[doc = "Field `HOST_SLCHOST_SLC0_BIT6_CLRADDR` reader - "]
pub struct HOST_SLCHOST_SLC0_BIT6_CLRADDR_R(crate::FieldReader<u16, u16>);
impl HOST_SLCHOST_SLC0_BIT6_CLRADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        HOST_SLCHOST_SLC0_BIT6_CLRADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOST_SLCHOST_SLC0_BIT6_CLRADDR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOST_SLCHOST_SLC0_BIT6_CLRADDR` writer - "]
pub struct HOST_SLCHOST_SLC0_BIT6_CLRADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SLCHOST_SLC0_BIT6_CLRADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 9)) | ((value as u32 & 0x01ff) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn host_slchost_slc0_bit7_clraddr(&self) -> HOST_SLCHOST_SLC0_BIT7_CLRADDR_R {
        HOST_SLCHOST_SLC0_BIT7_CLRADDR_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:17"]
    #[inline(always)]
    pub fn host_slchost_slc0_bit6_clraddr(&self) -> HOST_SLCHOST_SLC0_BIT6_CLRADDR_R {
        HOST_SLCHOST_SLC0_BIT6_CLRADDR_R::new(((self.bits >> 9) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn host_slchost_slc0_bit7_clraddr(&mut self) -> HOST_SLCHOST_SLC0_BIT7_CLRADDR_W {
        HOST_SLCHOST_SLC0_BIT7_CLRADDR_W { w: self }
    }
    #[doc = "Bits 9:17"]
    #[inline(always)]
    pub fn host_slchost_slc0_bit6_clraddr(&mut self) -> HOST_SLCHOST_SLC0_BIT6_CLRADDR_W {
        HOST_SLCHOST_SLC0_BIT6_CLRADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_slchost_rdclr0](index.html) module"]
pub struct HOST_SLCHOST_RDCLR0_SPEC;
impl crate::RegisterSpec for HOST_SLCHOST_RDCLR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [host_slchost_rdclr0::R](R) reader structure"]
impl crate::Readable for HOST_SLCHOST_RDCLR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [host_slchost_rdclr0::W](W) writer structure"]
impl crate::Writable for HOST_SLCHOST_RDCLR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HOST_SLCHOST_RDCLR0 to value 0x0003_c044"]
impl crate::Resettable for HOST_SLCHOST_RDCLR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0003_c044
    }
}
