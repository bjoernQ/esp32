#[doc = "Register `SLAVE_SPI_CONFIG` reader"]
pub struct R(crate::R<SLAVE_SPI_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLAVE_SPI_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLAVE_SPI_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLAVE_SPI_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLAVE_SPI_CONFIG` writer"]
pub struct W(crate::W<SLAVE_SPI_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLAVE_SPI_CONFIG_SPEC>;
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
impl From<crate::W<SLAVE_SPI_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLAVE_SPI_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLAVE_SPI_MASK_PRO` reader - "]
pub struct SLAVE_SPI_MASK_PRO_R(crate::FieldReader<bool, bool>);
impl SLAVE_SPI_MASK_PRO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLAVE_SPI_MASK_PRO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLAVE_SPI_MASK_PRO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLAVE_SPI_MASK_PRO` writer - "]
pub struct SLAVE_SPI_MASK_PRO_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_SPI_MASK_PRO_W<'a> {
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
#[doc = "Field `SLAVE_SPI_MASK_APP` reader - "]
pub struct SLAVE_SPI_MASK_APP_R(crate::FieldReader<bool, bool>);
impl SLAVE_SPI_MASK_APP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLAVE_SPI_MASK_APP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLAVE_SPI_MASK_APP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLAVE_SPI_MASK_APP` writer - "]
pub struct SLAVE_SPI_MASK_APP_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_SPI_MASK_APP_W<'a> {
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
#[doc = "Field `SPI_ENCRYPT_ENABLE` reader - "]
pub struct SPI_ENCRYPT_ENABLE_R(crate::FieldReader<bool, bool>);
impl SPI_ENCRYPT_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPI_ENCRYPT_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_ENCRYPT_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_ENCRYPT_ENABLE` writer - "]
pub struct SPI_ENCRYPT_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_ENCRYPT_ENABLE_W<'a> {
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
#[doc = "Field `SPI_DECRYPT_ENABLE` reader - "]
pub struct SPI_DECRYPT_ENABLE_R(crate::FieldReader<bool, bool>);
impl SPI_DECRYPT_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPI_DECRYPT_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_DECRYPT_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_DECRYPT_ENABLE` writer - "]
pub struct SPI_DECRYPT_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_DECRYPT_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn slave_spi_mask_pro(&self) -> SLAVE_SPI_MASK_PRO_R {
        SLAVE_SPI_MASK_PRO_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn slave_spi_mask_app(&self) -> SLAVE_SPI_MASK_APP_R {
        SLAVE_SPI_MASK_APP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn spi_encrypt_enable(&self) -> SPI_ENCRYPT_ENABLE_R {
        SPI_ENCRYPT_ENABLE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn spi_decrypt_enable(&self) -> SPI_DECRYPT_ENABLE_R {
        SPI_DECRYPT_ENABLE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn slave_spi_mask_pro(&mut self) -> SLAVE_SPI_MASK_PRO_W {
        SLAVE_SPI_MASK_PRO_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn slave_spi_mask_app(&mut self) -> SLAVE_SPI_MASK_APP_W {
        SLAVE_SPI_MASK_APP_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn spi_encrypt_enable(&mut self) -> SPI_ENCRYPT_ENABLE_W {
        SPI_ENCRYPT_ENABLE_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn spi_decrypt_enable(&mut self) -> SPI_DECRYPT_ENABLE_W {
        SPI_DECRYPT_ENABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slave_spi_config](index.html) module"]
pub struct SLAVE_SPI_CONFIG_SPEC;
impl crate::RegisterSpec for SLAVE_SPI_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slave_spi_config::R](R) reader structure"]
impl crate::Readable for SLAVE_SPI_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slave_spi_config::W](W) writer structure"]
impl crate::Writable for SLAVE_SPI_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SLAVE_SPI_CONFIG to value 0"]
impl crate::Resettable for SLAVE_SPI_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
