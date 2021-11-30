#[doc = "Register `SLAVE3` reader"]
pub struct R(crate::R<SLAVE3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLAVE3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLAVE3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLAVE3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLAVE3` writer"]
pub struct W(crate::W<SLAVE3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLAVE3_SPEC>;
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
impl From<crate::W<SLAVE3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLAVE3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLV_RDBUF_CMD_VALUE` reader - In the slave mode it is the value of read-buffer command."]
pub struct SLV_RDBUF_CMD_VALUE_R(crate::FieldReader<u8, u8>);
impl SLV_RDBUF_CMD_VALUE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SLV_RDBUF_CMD_VALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_RDBUF_CMD_VALUE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_RDBUF_CMD_VALUE` writer - In the slave mode it is the value of read-buffer command."]
pub struct SLV_RDBUF_CMD_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_RDBUF_CMD_VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `SLV_WRBUF_CMD_VALUE` reader - In the slave mode it is the value of write-buffer command."]
pub struct SLV_WRBUF_CMD_VALUE_R(crate::FieldReader<u8, u8>);
impl SLV_WRBUF_CMD_VALUE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SLV_WRBUF_CMD_VALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_WRBUF_CMD_VALUE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_WRBUF_CMD_VALUE` writer - In the slave mode it is the value of write-buffer command."]
pub struct SLV_WRBUF_CMD_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_WRBUF_CMD_VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `SLV_RDSTA_CMD_VALUE` reader - In the slave mode it is the value of read-status command."]
pub struct SLV_RDSTA_CMD_VALUE_R(crate::FieldReader<u8, u8>);
impl SLV_RDSTA_CMD_VALUE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SLV_RDSTA_CMD_VALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_RDSTA_CMD_VALUE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_RDSTA_CMD_VALUE` writer - In the slave mode it is the value of read-status command."]
pub struct SLV_RDSTA_CMD_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_RDSTA_CMD_VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `SLV_WRSTA_CMD_VALUE` reader - In the slave mode it is the value of write-status command."]
pub struct SLV_WRSTA_CMD_VALUE_R(crate::FieldReader<u8, u8>);
impl SLV_WRSTA_CMD_VALUE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SLV_WRSTA_CMD_VALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_WRSTA_CMD_VALUE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_WRSTA_CMD_VALUE` writer - In the slave mode it is the value of write-status command."]
pub struct SLV_WRSTA_CMD_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_WRSTA_CMD_VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - In the slave mode it is the value of read-buffer command."]
    #[inline(always)]
    pub fn slv_rdbuf_cmd_value(&self) -> SLV_RDBUF_CMD_VALUE_R {
        SLV_RDBUF_CMD_VALUE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - In the slave mode it is the value of write-buffer command."]
    #[inline(always)]
    pub fn slv_wrbuf_cmd_value(&self) -> SLV_WRBUF_CMD_VALUE_R {
        SLV_WRBUF_CMD_VALUE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - In the slave mode it is the value of read-status command."]
    #[inline(always)]
    pub fn slv_rdsta_cmd_value(&self) -> SLV_RDSTA_CMD_VALUE_R {
        SLV_RDSTA_CMD_VALUE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - In the slave mode it is the value of write-status command."]
    #[inline(always)]
    pub fn slv_wrsta_cmd_value(&self) -> SLV_WRSTA_CMD_VALUE_R {
        SLV_WRSTA_CMD_VALUE_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - In the slave mode it is the value of read-buffer command."]
    #[inline(always)]
    pub fn slv_rdbuf_cmd_value(&mut self) -> SLV_RDBUF_CMD_VALUE_W {
        SLV_RDBUF_CMD_VALUE_W { w: self }
    }
    #[doc = "Bits 8:15 - In the slave mode it is the value of write-buffer command."]
    #[inline(always)]
    pub fn slv_wrbuf_cmd_value(&mut self) -> SLV_WRBUF_CMD_VALUE_W {
        SLV_WRBUF_CMD_VALUE_W { w: self }
    }
    #[doc = "Bits 16:23 - In the slave mode it is the value of read-status command."]
    #[inline(always)]
    pub fn slv_rdsta_cmd_value(&mut self) -> SLV_RDSTA_CMD_VALUE_W {
        SLV_RDSTA_CMD_VALUE_W { w: self }
    }
    #[doc = "Bits 24:31 - In the slave mode it is the value of write-status command."]
    #[inline(always)]
    pub fn slv_wrsta_cmd_value(&mut self) -> SLV_WRSTA_CMD_VALUE_W {
        SLV_WRSTA_CMD_VALUE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slave3](index.html) module"]
pub struct SLAVE3_SPEC;
impl crate::RegisterSpec for SLAVE3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slave3::R](R) reader structure"]
impl crate::Readable for SLAVE3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slave3::W](W) writer structure"]
impl crate::Writable for SLAVE3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SLAVE3 to value 0"]
impl crate::Resettable for SLAVE3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
