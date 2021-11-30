#[doc = "Register `SLV_WRBUF_DLEN` reader"]
pub struct R(crate::R<SLV_WRBUF_DLEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLV_WRBUF_DLEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLV_WRBUF_DLEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLV_WRBUF_DLEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLV_WRBUF_DLEN` writer"]
pub struct W(crate::W<SLV_WRBUF_DLEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLV_WRBUF_DLEN_SPEC>;
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
impl From<crate::W<SLV_WRBUF_DLEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLV_WRBUF_DLEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLV_WRBUF_DBITLEN` reader - In the slave mode it is the length in bits for write-buffer operations. The register value shall be (bit_num-1)."]
pub struct SLV_WRBUF_DBITLEN_R(crate::FieldReader<u32, u32>);
impl SLV_WRBUF_DBITLEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        SLV_WRBUF_DBITLEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_WRBUF_DBITLEN_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_WRBUF_DBITLEN` writer - In the slave mode it is the length in bits for write-buffer operations. The register value shall be (bit_num-1)."]
pub struct SLV_WRBUF_DBITLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_WRBUF_DBITLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | (value as u32 & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - In the slave mode it is the length in bits for write-buffer operations. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn slv_wrbuf_dbitlen(&self) -> SLV_WRBUF_DBITLEN_R {
        SLV_WRBUF_DBITLEN_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - In the slave mode it is the length in bits for write-buffer operations. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn slv_wrbuf_dbitlen(&mut self) -> SLV_WRBUF_DBITLEN_W {
        SLV_WRBUF_DBITLEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slv_wrbuf_dlen](index.html) module"]
pub struct SLV_WRBUF_DLEN_SPEC;
impl crate::RegisterSpec for SLV_WRBUF_DLEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slv_wrbuf_dlen::R](R) reader structure"]
impl crate::Readable for SLV_WRBUF_DLEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slv_wrbuf_dlen::W](W) writer structure"]
impl crate::Writable for SLV_WRBUF_DLEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SLV_WRBUF_DLEN to value 0"]
impl crate::Resettable for SLV_WRBUF_DLEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
