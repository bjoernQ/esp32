#[doc = "Register `SAR_I2C_IO` reader"]
pub struct R(crate::R<SAR_I2C_IO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_I2C_IO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_I2C_IO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_I2C_IO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_I2C_IO` writer"]
pub struct W(crate::W<SAR_I2C_IO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_I2C_IO_SPEC>;
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
impl From<crate::W<SAR_I2C_IO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_I2C_IO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAR_DEBUG_BIT_SEL` reader - "]
pub struct SAR_DEBUG_BIT_SEL_R(crate::FieldReader<u8, u8>);
impl SAR_DEBUG_BIT_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SAR_DEBUG_BIT_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR_DEBUG_BIT_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR_DEBUG_BIT_SEL` writer - "]
pub struct SAR_DEBUG_BIT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR_DEBUG_BIT_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 23)) | ((value as u32 & 0x1f) << 23);
        self.w
    }
}
#[doc = "Field `SAR_I2C_SCL_SEL` reader - Ò0Ó using TOUCH_PAD\\[0\\]
as i2c clk Ò1Ó using TOUCH_PAD\\[2\\]
as i2c clk"]
pub struct SAR_I2C_SCL_SEL_R(crate::FieldReader<u8, u8>);
impl SAR_I2C_SCL_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SAR_I2C_SCL_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR_I2C_SCL_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR_I2C_SCL_SEL` writer - Ò0Ó using TOUCH_PAD\\[0\\]
as i2c clk Ò1Ó using TOUCH_PAD\\[2\\]
as i2c clk"]
pub struct SAR_I2C_SCL_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR_I2C_SCL_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
#[doc = "Field `SAR_I2C_SDA_SEL` reader - Ò0Ó using TOUCH_PAD\\[1\\]
as i2c sda Ò1Ó using TOUCH_PAD\\[3\\]
as i2c sda"]
pub struct SAR_I2C_SDA_SEL_R(crate::FieldReader<u8, u8>);
impl SAR_I2C_SDA_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SAR_I2C_SDA_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR_I2C_SDA_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR_I2C_SDA_SEL` writer - Ò0Ó using TOUCH_PAD\\[1\\]
as i2c sda Ò1Ó using TOUCH_PAD\\[3\\]
as i2c sda"]
pub struct SAR_I2C_SDA_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR_I2C_SDA_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 23:27"]
    #[inline(always)]
    pub fn sar_debug_bit_sel(&self) -> SAR_DEBUG_BIT_SEL_R {
        SAR_DEBUG_BIT_SEL_R::new(((self.bits >> 23) & 0x1f) as u8)
    }
    #[doc = "Bits 28:29 - Ò0Ó using TOUCH_PAD\\[0\\]
as i2c clk Ò1Ó using TOUCH_PAD\\[2\\]
as i2c clk"]
    #[inline(always)]
    pub fn sar_i2c_scl_sel(&self) -> SAR_I2C_SCL_SEL_R {
        SAR_I2C_SCL_SEL_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - Ò0Ó using TOUCH_PAD\\[1\\]
as i2c sda Ò1Ó using TOUCH_PAD\\[3\\]
as i2c sda"]
    #[inline(always)]
    pub fn sar_i2c_sda_sel(&self) -> SAR_I2C_SDA_SEL_R {
        SAR_I2C_SDA_SEL_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 23:27"]
    #[inline(always)]
    pub fn sar_debug_bit_sel(&mut self) -> SAR_DEBUG_BIT_SEL_W {
        SAR_DEBUG_BIT_SEL_W { w: self }
    }
    #[doc = "Bits 28:29 - Ò0Ó using TOUCH_PAD\\[0\\]
as i2c clk Ò1Ó using TOUCH_PAD\\[2\\]
as i2c clk"]
    #[inline(always)]
    pub fn sar_i2c_scl_sel(&mut self) -> SAR_I2C_SCL_SEL_W {
        SAR_I2C_SCL_SEL_W { w: self }
    }
    #[doc = "Bits 30:31 - Ò0Ó using TOUCH_PAD\\[1\\]
as i2c sda Ò1Ó using TOUCH_PAD\\[3\\]
as i2c sda"]
    #[inline(always)]
    pub fn sar_i2c_sda_sel(&mut self) -> SAR_I2C_SDA_SEL_W {
        SAR_I2C_SDA_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_i2c_io](index.html) module"]
pub struct SAR_I2C_IO_SPEC;
impl crate::RegisterSpec for SAR_I2C_IO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_i2c_io::R](R) reader structure"]
impl crate::Readable for SAR_I2C_IO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_i2c_io::W](W) writer structure"]
impl crate::Writable for SAR_I2C_IO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAR_I2C_IO to value 0"]
impl crate::Resettable for SAR_I2C_IO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
