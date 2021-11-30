#[doc = "Register `CFG_DATA7` reader"]
pub struct R(crate::R<CFG_DATA7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_DATA7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_DATA7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_DATA7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG_DATA7` writer"]
pub struct W(crate::W<CFG_DATA7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_DATA7_SPEC>;
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
impl From<crate::W<CFG_DATA7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_DATA7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PIN_STATE` reader - "]
pub struct PIN_STATE_R(crate::FieldReader<u8, u8>);
impl PIN_STATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PIN_STATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIN_STATE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN_STATE` writer - "]
pub struct PIN_STATE_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN_STATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `CHIP_STATE` reader - "]
pub struct CHIP_STATE_R(crate::FieldReader<u8, u8>);
impl CHIP_STATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CHIP_STATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHIP_STATE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHIP_STATE` writer - "]
pub struct CHIP_STATE_W<'a> {
    w: &'a mut W,
}
impl<'a> CHIP_STATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `SDIO_RST` reader - "]
pub struct SDIO_RST_R(crate::FieldReader<bool, bool>);
impl SDIO_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDIO_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIO_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIO_RST` writer - "]
pub struct SDIO_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `SDIO_IOREADY0` reader - "]
pub struct SDIO_IOREADY0_R(crate::FieldReader<bool, bool>);
impl SDIO_IOREADY0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDIO_IOREADY0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIO_IOREADY0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIO_IOREADY0` writer - "]
pub struct SDIO_IOREADY0_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_IOREADY0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn pin_state(&self) -> PIN_STATE_R {
        PIN_STATE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn chip_state(&self) -> CHIP_STATE_R {
        CHIP_STATE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn sdio_rst(&self) -> SDIO_RST_R {
        SDIO_RST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn sdio_ioready0(&self) -> SDIO_IOREADY0_R {
        SDIO_IOREADY0_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn pin_state(&mut self) -> PIN_STATE_W {
        PIN_STATE_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn chip_state(&mut self) -> CHIP_STATE_W {
        CHIP_STATE_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn sdio_rst(&mut self) -> SDIO_RST_W {
        SDIO_RST_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn sdio_ioready0(&mut self) -> SDIO_IOREADY0_W {
        SDIO_IOREADY0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg_data7](index.html) module"]
pub struct CFG_DATA7_SPEC;
impl crate::RegisterSpec for CFG_DATA7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg_data7::R](R) reader structure"]
impl crate::Readable for CFG_DATA7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg_data7::W](W) writer structure"]
impl crate::Writable for CFG_DATA7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG_DATA7 to value 0x0002_0000"]
impl crate::Resettable for CFG_DATA7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0002_0000
    }
}
