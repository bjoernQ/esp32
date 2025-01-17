#[doc = "Register `SAR_SLAVE_ADDR3` reader"]
pub struct R(crate::R<SAR_SLAVE_ADDR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_SLAVE_ADDR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_SLAVE_ADDR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_SLAVE_ADDR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_SLAVE_ADDR3` writer"]
pub struct W(crate::W<SAR_SLAVE_ADDR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_SLAVE_ADDR3_SPEC>;
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
impl From<crate::W<SAR_SLAVE_ADDR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_SLAVE_ADDR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C_SLAVE_ADDR5` reader - "]
pub struct I2C_SLAVE_ADDR5_R(crate::FieldReader<u16, u16>);
impl I2C_SLAVE_ADDR5_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        I2C_SLAVE_ADDR5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_SLAVE_ADDR5_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C_SLAVE_ADDR5` writer - "]
pub struct I2C_SLAVE_ADDR5_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_SLAVE_ADDR5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | (value as u32 & 0x07ff);
        self.w
    }
}
#[doc = "Field `I2C_SLAVE_ADDR4` reader - "]
pub struct I2C_SLAVE_ADDR4_R(crate::FieldReader<u16, u16>);
impl I2C_SLAVE_ADDR4_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        I2C_SLAVE_ADDR4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_SLAVE_ADDR4_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C_SLAVE_ADDR4` writer - "]
pub struct I2C_SLAVE_ADDR4_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_SLAVE_ADDR4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 11)) | ((value as u32 & 0x07ff) << 11);
        self.w
    }
}
#[doc = "Field `TSENS_OUT` reader - temperature sensor data out"]
pub struct TSENS_OUT_R(crate::FieldReader<u8, u8>);
impl TSENS_OUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TSENS_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSENS_OUT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSENS_RDY_OUT` reader - indicate temperature sensor out ready"]
pub struct TSENS_RDY_OUT_R(crate::FieldReader<bool, bool>);
impl TSENS_RDY_OUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TSENS_RDY_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSENS_RDY_OUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn i2c_slave_addr5(&self) -> I2C_SLAVE_ADDR5_R {
        I2C_SLAVE_ADDR5_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:21"]
    #[inline(always)]
    pub fn i2c_slave_addr4(&self) -> I2C_SLAVE_ADDR4_R {
        I2C_SLAVE_ADDR4_R::new(((self.bits >> 11) & 0x07ff) as u16)
    }
    #[doc = "Bits 22:29 - temperature sensor data out"]
    #[inline(always)]
    pub fn tsens_out(&self) -> TSENS_OUT_R {
        TSENS_OUT_R::new(((self.bits >> 22) & 0xff) as u8)
    }
    #[doc = "Bit 30 - indicate temperature sensor out ready"]
    #[inline(always)]
    pub fn tsens_rdy_out(&self) -> TSENS_RDY_OUT_R {
        TSENS_RDY_OUT_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn i2c_slave_addr5(&mut self) -> I2C_SLAVE_ADDR5_W {
        I2C_SLAVE_ADDR5_W { w: self }
    }
    #[doc = "Bits 11:21"]
    #[inline(always)]
    pub fn i2c_slave_addr4(&mut self) -> I2C_SLAVE_ADDR4_W {
        I2C_SLAVE_ADDR4_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_slave_addr3](index.html) module"]
pub struct SAR_SLAVE_ADDR3_SPEC;
impl crate::RegisterSpec for SAR_SLAVE_ADDR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_slave_addr3::R](R) reader structure"]
impl crate::Readable for SAR_SLAVE_ADDR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_slave_addr3::W](W) writer structure"]
impl crate::Writable for SAR_SLAVE_ADDR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAR_SLAVE_ADDR3 to value 0"]
impl crate::Resettable for SAR_SLAVE_ADDR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
