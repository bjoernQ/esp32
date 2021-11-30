#[doc = "Register `SAR_SLAVE_ADDR1` reader"]
pub struct R(crate::R<SAR_SLAVE_ADDR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_SLAVE_ADDR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_SLAVE_ADDR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_SLAVE_ADDR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_SLAVE_ADDR1` writer"]
pub struct W(crate::W<SAR_SLAVE_ADDR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_SLAVE_ADDR1_SPEC>;
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
impl From<crate::W<SAR_SLAVE_ADDR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_SLAVE_ADDR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C_SLAVE_ADDR1` reader - "]
pub struct I2C_SLAVE_ADDR1_R(crate::FieldReader<u16, u16>);
impl I2C_SLAVE_ADDR1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        I2C_SLAVE_ADDR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_SLAVE_ADDR1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C_SLAVE_ADDR1` writer - "]
pub struct I2C_SLAVE_ADDR1_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_SLAVE_ADDR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | (value as u32 & 0x07ff);
        self.w
    }
}
#[doc = "Field `I2C_SLAVE_ADDR0` reader - "]
pub struct I2C_SLAVE_ADDR0_R(crate::FieldReader<u16, u16>);
impl I2C_SLAVE_ADDR0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        I2C_SLAVE_ADDR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_SLAVE_ADDR0_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C_SLAVE_ADDR0` writer - "]
pub struct I2C_SLAVE_ADDR0_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_SLAVE_ADDR0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 11)) | ((value as u32 & 0x07ff) << 11);
        self.w
    }
}
#[doc = "Field `MEAS_STATUS` reader - "]
pub struct MEAS_STATUS_R(crate::FieldReader<u8, u8>);
impl MEAS_STATUS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MEAS_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEAS_STATUS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn i2c_slave_addr1(&self) -> I2C_SLAVE_ADDR1_R {
        I2C_SLAVE_ADDR1_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:21"]
    #[inline(always)]
    pub fn i2c_slave_addr0(&self) -> I2C_SLAVE_ADDR0_R {
        I2C_SLAVE_ADDR0_R::new(((self.bits >> 11) & 0x07ff) as u16)
    }
    #[doc = "Bits 22:29"]
    #[inline(always)]
    pub fn meas_status(&self) -> MEAS_STATUS_R {
        MEAS_STATUS_R::new(((self.bits >> 22) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn i2c_slave_addr1(&mut self) -> I2C_SLAVE_ADDR1_W {
        I2C_SLAVE_ADDR1_W { w: self }
    }
    #[doc = "Bits 11:21"]
    #[inline(always)]
    pub fn i2c_slave_addr0(&mut self) -> I2C_SLAVE_ADDR0_W {
        I2C_SLAVE_ADDR0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_slave_addr1](index.html) module"]
pub struct SAR_SLAVE_ADDR1_SPEC;
impl crate::RegisterSpec for SAR_SLAVE_ADDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_slave_addr1::R](R) reader structure"]
impl crate::Readable for SAR_SLAVE_ADDR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_slave_addr1::W](W) writer structure"]
impl crate::Writable for SAR_SLAVE_ADDR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAR_SLAVE_ADDR1 to value 0"]
impl crate::Resettable for SAR_SLAVE_ADDR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
