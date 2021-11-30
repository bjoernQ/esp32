#[doc = "Register `SLAVE1` reader"]
pub struct R(crate::R<SLAVE1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLAVE1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLAVE1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLAVE1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLAVE1` writer"]
pub struct W(crate::W<SLAVE1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLAVE1_SPEC>;
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
impl From<crate::W<SLAVE1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLAVE1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLV_RDBUF_DUMMY_EN` reader - In the slave mode it is the enable bit of dummy phase for read-buffer operations."]
pub struct SLV_RDBUF_DUMMY_EN_R(crate::FieldReader<bool, bool>);
impl SLV_RDBUF_DUMMY_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLV_RDBUF_DUMMY_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_RDBUF_DUMMY_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_RDBUF_DUMMY_EN` writer - In the slave mode it is the enable bit of dummy phase for read-buffer operations."]
pub struct SLV_RDBUF_DUMMY_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_RDBUF_DUMMY_EN_W<'a> {
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
#[doc = "Field `SLV_WRBUF_DUMMY_EN` reader - In the slave mode it is the enable bit of dummy phase for write-buffer operations."]
pub struct SLV_WRBUF_DUMMY_EN_R(crate::FieldReader<bool, bool>);
impl SLV_WRBUF_DUMMY_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLV_WRBUF_DUMMY_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_WRBUF_DUMMY_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_WRBUF_DUMMY_EN` writer - In the slave mode it is the enable bit of dummy phase for write-buffer operations."]
pub struct SLV_WRBUF_DUMMY_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_WRBUF_DUMMY_EN_W<'a> {
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
#[doc = "Field `SLV_RDSTA_DUMMY_EN` reader - In the slave mode it is the enable bit of dummy phase for read-status operations."]
pub struct SLV_RDSTA_DUMMY_EN_R(crate::FieldReader<bool, bool>);
impl SLV_RDSTA_DUMMY_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLV_RDSTA_DUMMY_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_RDSTA_DUMMY_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_RDSTA_DUMMY_EN` writer - In the slave mode it is the enable bit of dummy phase for read-status operations."]
pub struct SLV_RDSTA_DUMMY_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_RDSTA_DUMMY_EN_W<'a> {
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
#[doc = "Field `SLV_WRSTA_DUMMY_EN` reader - In the slave mode it is the enable bit of dummy phase for write-status operations."]
pub struct SLV_WRSTA_DUMMY_EN_R(crate::FieldReader<bool, bool>);
impl SLV_WRSTA_DUMMY_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLV_WRSTA_DUMMY_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_WRSTA_DUMMY_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_WRSTA_DUMMY_EN` writer - In the slave mode it is the enable bit of dummy phase for write-status operations."]
pub struct SLV_WRSTA_DUMMY_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_WRSTA_DUMMY_EN_W<'a> {
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
#[doc = "Field `SLV_WR_ADDR_BITLEN` reader - In the slave mode it is the address length in bits for write-buffer operation. The register value shall be (bit_num-1)."]
pub struct SLV_WR_ADDR_BITLEN_R(crate::FieldReader<u8, u8>);
impl SLV_WR_ADDR_BITLEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SLV_WR_ADDR_BITLEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_WR_ADDR_BITLEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_WR_ADDR_BITLEN` writer - In the slave mode it is the address length in bits for write-buffer operation. The register value shall be (bit_num-1)."]
pub struct SLV_WR_ADDR_BITLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_WR_ADDR_BITLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 4)) | ((value as u32 & 0x3f) << 4);
        self.w
    }
}
#[doc = "Field `SLV_RD_ADDR_BITLEN` reader - In the slave mode it is the address length in bits for read-buffer operation. The register value shall be (bit_num-1)."]
pub struct SLV_RD_ADDR_BITLEN_R(crate::FieldReader<u8, u8>);
impl SLV_RD_ADDR_BITLEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SLV_RD_ADDR_BITLEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_RD_ADDR_BITLEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_RD_ADDR_BITLEN` writer - In the slave mode it is the address length in bits for read-buffer operation. The register value shall be (bit_num-1)."]
pub struct SLV_RD_ADDR_BITLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_RD_ADDR_BITLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 10)) | ((value as u32 & 0x3f) << 10);
        self.w
    }
}
#[doc = "Field `SLV_STATUS_READBACK` reader - In the slave mode 1:read register of SPI_SLV_WR_STATUS 0: read register of SPI_RD_STATUS."]
pub struct SLV_STATUS_READBACK_R(crate::FieldReader<bool, bool>);
impl SLV_STATUS_READBACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLV_STATUS_READBACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_STATUS_READBACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_STATUS_READBACK` writer - In the slave mode 1:read register of SPI_SLV_WR_STATUS 0: read register of SPI_RD_STATUS."]
pub struct SLV_STATUS_READBACK_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_STATUS_READBACK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `SLV_STATUS_FAST_EN` reader - In the slave mode enable fast read status."]
pub struct SLV_STATUS_FAST_EN_R(crate::FieldReader<bool, bool>);
impl SLV_STATUS_FAST_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLV_STATUS_FAST_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_STATUS_FAST_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_STATUS_FAST_EN` writer - In the slave mode enable fast read status."]
pub struct SLV_STATUS_FAST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_STATUS_FAST_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `SLV_STATUS_BITLEN` reader - In the slave mode it is the length of status bit."]
pub struct SLV_STATUS_BITLEN_R(crate::FieldReader<u8, u8>);
impl SLV_STATUS_BITLEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SLV_STATUS_BITLEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_STATUS_BITLEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_STATUS_BITLEN` writer - In the slave mode it is the length of status bit."]
pub struct SLV_STATUS_BITLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_STATUS_BITLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 27)) | ((value as u32 & 0x1f) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - In the slave mode it is the enable bit of dummy phase for read-buffer operations."]
    #[inline(always)]
    pub fn slv_rdbuf_dummy_en(&self) -> SLV_RDBUF_DUMMY_EN_R {
        SLV_RDBUF_DUMMY_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - In the slave mode it is the enable bit of dummy phase for write-buffer operations."]
    #[inline(always)]
    pub fn slv_wrbuf_dummy_en(&self) -> SLV_WRBUF_DUMMY_EN_R {
        SLV_WRBUF_DUMMY_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - In the slave mode it is the enable bit of dummy phase for read-status operations."]
    #[inline(always)]
    pub fn slv_rdsta_dummy_en(&self) -> SLV_RDSTA_DUMMY_EN_R {
        SLV_RDSTA_DUMMY_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - In the slave mode it is the enable bit of dummy phase for write-status operations."]
    #[inline(always)]
    pub fn slv_wrsta_dummy_en(&self) -> SLV_WRSTA_DUMMY_EN_R {
        SLV_WRSTA_DUMMY_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:9 - In the slave mode it is the address length in bits for write-buffer operation. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn slv_wr_addr_bitlen(&self) -> SLV_WR_ADDR_BITLEN_R {
        SLV_WR_ADDR_BITLEN_R::new(((self.bits >> 4) & 0x3f) as u8)
    }
    #[doc = "Bits 10:15 - In the slave mode it is the address length in bits for read-buffer operation. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn slv_rd_addr_bitlen(&self) -> SLV_RD_ADDR_BITLEN_R {
        SLV_RD_ADDR_BITLEN_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 25 - In the slave mode 1:read register of SPI_SLV_WR_STATUS 0: read register of SPI_RD_STATUS."]
    #[inline(always)]
    pub fn slv_status_readback(&self) -> SLV_STATUS_READBACK_R {
        SLV_STATUS_READBACK_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - In the slave mode enable fast read status."]
    #[inline(always)]
    pub fn slv_status_fast_en(&self) -> SLV_STATUS_FAST_EN_R {
        SLV_STATUS_FAST_EN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 27:31 - In the slave mode it is the length of status bit."]
    #[inline(always)]
    pub fn slv_status_bitlen(&self) -> SLV_STATUS_BITLEN_R {
        SLV_STATUS_BITLEN_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - In the slave mode it is the enable bit of dummy phase for read-buffer operations."]
    #[inline(always)]
    pub fn slv_rdbuf_dummy_en(&mut self) -> SLV_RDBUF_DUMMY_EN_W {
        SLV_RDBUF_DUMMY_EN_W { w: self }
    }
    #[doc = "Bit 1 - In the slave mode it is the enable bit of dummy phase for write-buffer operations."]
    #[inline(always)]
    pub fn slv_wrbuf_dummy_en(&mut self) -> SLV_WRBUF_DUMMY_EN_W {
        SLV_WRBUF_DUMMY_EN_W { w: self }
    }
    #[doc = "Bit 2 - In the slave mode it is the enable bit of dummy phase for read-status operations."]
    #[inline(always)]
    pub fn slv_rdsta_dummy_en(&mut self) -> SLV_RDSTA_DUMMY_EN_W {
        SLV_RDSTA_DUMMY_EN_W { w: self }
    }
    #[doc = "Bit 3 - In the slave mode it is the enable bit of dummy phase for write-status operations."]
    #[inline(always)]
    pub fn slv_wrsta_dummy_en(&mut self) -> SLV_WRSTA_DUMMY_EN_W {
        SLV_WRSTA_DUMMY_EN_W { w: self }
    }
    #[doc = "Bits 4:9 - In the slave mode it is the address length in bits for write-buffer operation. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn slv_wr_addr_bitlen(&mut self) -> SLV_WR_ADDR_BITLEN_W {
        SLV_WR_ADDR_BITLEN_W { w: self }
    }
    #[doc = "Bits 10:15 - In the slave mode it is the address length in bits for read-buffer operation. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn slv_rd_addr_bitlen(&mut self) -> SLV_RD_ADDR_BITLEN_W {
        SLV_RD_ADDR_BITLEN_W { w: self }
    }
    #[doc = "Bit 25 - In the slave mode 1:read register of SPI_SLV_WR_STATUS 0: read register of SPI_RD_STATUS."]
    #[inline(always)]
    pub fn slv_status_readback(&mut self) -> SLV_STATUS_READBACK_W {
        SLV_STATUS_READBACK_W { w: self }
    }
    #[doc = "Bit 26 - In the slave mode enable fast read status."]
    #[inline(always)]
    pub fn slv_status_fast_en(&mut self) -> SLV_STATUS_FAST_EN_W {
        SLV_STATUS_FAST_EN_W { w: self }
    }
    #[doc = "Bits 27:31 - In the slave mode it is the length of status bit."]
    #[inline(always)]
    pub fn slv_status_bitlen(&mut self) -> SLV_STATUS_BITLEN_W {
        SLV_STATUS_BITLEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slave1](index.html) module"]
pub struct SLAVE1_SPEC;
impl crate::RegisterSpec for SLAVE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slave1::R](R) reader structure"]
impl crate::Readable for SLAVE1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slave1::W](W) writer structure"]
impl crate::Writable for SLAVE1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SLAVE1 to value 0x0200_0000"]
impl crate::Resettable for SLAVE1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200_0000
    }
}
