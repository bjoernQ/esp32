#[doc = "Register `SLAVE2` reader"]
pub struct R(crate::R<SLAVE2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLAVE2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLAVE2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLAVE2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLAVE2` writer"]
pub struct W(crate::W<SLAVE2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLAVE2_SPEC>;
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
impl From<crate::W<SLAVE2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLAVE2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLV_RDSTA_DUMMY_CYCLELEN` reader - In the slave mode it is the length in spi_clk cycles of dummy phase for read-status operations. The register value shall be (cycle_num-1)."]
pub struct SLV_RDSTA_DUMMY_CYCLELEN_R(crate::FieldReader<u8, u8>);
impl SLV_RDSTA_DUMMY_CYCLELEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SLV_RDSTA_DUMMY_CYCLELEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_RDSTA_DUMMY_CYCLELEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_RDSTA_DUMMY_CYCLELEN` writer - In the slave mode it is the length in spi_clk cycles of dummy phase for read-status operations. The register value shall be (cycle_num-1)."]
pub struct SLV_RDSTA_DUMMY_CYCLELEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_RDSTA_DUMMY_CYCLELEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `SLV_WRSTA_DUMMY_CYCLELEN` reader - In the slave mode it is the length in spi_clk cycles of dummy phase for write-status operations. The register value shall be (cycle_num-1)."]
pub struct SLV_WRSTA_DUMMY_CYCLELEN_R(crate::FieldReader<u8, u8>);
impl SLV_WRSTA_DUMMY_CYCLELEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SLV_WRSTA_DUMMY_CYCLELEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_WRSTA_DUMMY_CYCLELEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_WRSTA_DUMMY_CYCLELEN` writer - In the slave mode it is the length in spi_clk cycles of dummy phase for write-status operations. The register value shall be (cycle_num-1)."]
pub struct SLV_WRSTA_DUMMY_CYCLELEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_WRSTA_DUMMY_CYCLELEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `SLV_RDBUF_DUMMY_CYCLELEN` reader - In the slave mode it is the length in spi_clk cycles of dummy phase for read-buffer operations. The register value shall be (cycle_num-1)."]
pub struct SLV_RDBUF_DUMMY_CYCLELEN_R(crate::FieldReader<u8, u8>);
impl SLV_RDBUF_DUMMY_CYCLELEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SLV_RDBUF_DUMMY_CYCLELEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_RDBUF_DUMMY_CYCLELEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_RDBUF_DUMMY_CYCLELEN` writer - In the slave mode it is the length in spi_clk cycles of dummy phase for read-buffer operations. The register value shall be (cycle_num-1)."]
pub struct SLV_RDBUF_DUMMY_CYCLELEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_RDBUF_DUMMY_CYCLELEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `SLV_WRBUF_DUMMY_CYCLELEN` reader - In the slave mode it is the length in spi_clk cycles of dummy phase for write-buffer operations. The register value shall be (cycle_num-1)."]
pub struct SLV_WRBUF_DUMMY_CYCLELEN_R(crate::FieldReader<u8, u8>);
impl SLV_WRBUF_DUMMY_CYCLELEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SLV_WRBUF_DUMMY_CYCLELEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_WRBUF_DUMMY_CYCLELEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_WRBUF_DUMMY_CYCLELEN` writer - In the slave mode it is the length in spi_clk cycles of dummy phase for write-buffer operations. The register value shall be (cycle_num-1)."]
pub struct SLV_WRBUF_DUMMY_CYCLELEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_WRBUF_DUMMY_CYCLELEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - In the slave mode it is the length in spi_clk cycles of dummy phase for read-status operations. The register value shall be (cycle_num-1)."]
    #[inline(always)]
    pub fn slv_rdsta_dummy_cyclelen(&self) -> SLV_RDSTA_DUMMY_CYCLELEN_R {
        SLV_RDSTA_DUMMY_CYCLELEN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - In the slave mode it is the length in spi_clk cycles of dummy phase for write-status operations. The register value shall be (cycle_num-1)."]
    #[inline(always)]
    pub fn slv_wrsta_dummy_cyclelen(&self) -> SLV_WRSTA_DUMMY_CYCLELEN_R {
        SLV_WRSTA_DUMMY_CYCLELEN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - In the slave mode it is the length in spi_clk cycles of dummy phase for read-buffer operations. The register value shall be (cycle_num-1)."]
    #[inline(always)]
    pub fn slv_rdbuf_dummy_cyclelen(&self) -> SLV_RDBUF_DUMMY_CYCLELEN_R {
        SLV_RDBUF_DUMMY_CYCLELEN_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - In the slave mode it is the length in spi_clk cycles of dummy phase for write-buffer operations. The register value shall be (cycle_num-1)."]
    #[inline(always)]
    pub fn slv_wrbuf_dummy_cyclelen(&self) -> SLV_WRBUF_DUMMY_CYCLELEN_R {
        SLV_WRBUF_DUMMY_CYCLELEN_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - In the slave mode it is the length in spi_clk cycles of dummy phase for read-status operations. The register value shall be (cycle_num-1)."]
    #[inline(always)]
    pub fn slv_rdsta_dummy_cyclelen(&mut self) -> SLV_RDSTA_DUMMY_CYCLELEN_W {
        SLV_RDSTA_DUMMY_CYCLELEN_W { w: self }
    }
    #[doc = "Bits 8:15 - In the slave mode it is the length in spi_clk cycles of dummy phase for write-status operations. The register value shall be (cycle_num-1)."]
    #[inline(always)]
    pub fn slv_wrsta_dummy_cyclelen(&mut self) -> SLV_WRSTA_DUMMY_CYCLELEN_W {
        SLV_WRSTA_DUMMY_CYCLELEN_W { w: self }
    }
    #[doc = "Bits 16:23 - In the slave mode it is the length in spi_clk cycles of dummy phase for read-buffer operations. The register value shall be (cycle_num-1)."]
    #[inline(always)]
    pub fn slv_rdbuf_dummy_cyclelen(&mut self) -> SLV_RDBUF_DUMMY_CYCLELEN_W {
        SLV_RDBUF_DUMMY_CYCLELEN_W { w: self }
    }
    #[doc = "Bits 24:31 - In the slave mode it is the length in spi_clk cycles of dummy phase for write-buffer operations. The register value shall be (cycle_num-1)."]
    #[inline(always)]
    pub fn slv_wrbuf_dummy_cyclelen(&mut self) -> SLV_WRBUF_DUMMY_CYCLELEN_W {
        SLV_WRBUF_DUMMY_CYCLELEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slave2](index.html) module"]
pub struct SLAVE2_SPEC;
impl crate::RegisterSpec for SLAVE2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slave2::R](R) reader structure"]
impl crate::Readable for SLAVE2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slave2::W](W) writer structure"]
impl crate::Writable for SLAVE2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SLAVE2 to value 0"]
impl crate::Resettable for SLAVE2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
