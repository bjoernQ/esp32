#[doc = "Register `SAR_I2C_CTRL` reader"]
pub struct R(crate::R<SAR_I2C_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_I2C_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_I2C_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_I2C_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_I2C_CTRL` writer"]
pub struct W(crate::W<SAR_I2C_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_I2C_CTRL_SPEC>;
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
impl From<crate::W<SAR_I2C_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_I2C_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAR_I2C_CTRL` reader - I2C control data only active when reg_sar_i2c_start_force = 1"]
pub struct SAR_I2C_CTRL_R(crate::FieldReader<u32, u32>);
impl SAR_I2C_CTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        SAR_I2C_CTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR_I2C_CTRL_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR_I2C_CTRL` writer - I2C control data only active when reg_sar_i2c_start_force = 1"]
pub struct SAR_I2C_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR_I2C_CTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff_ffff) | (value as u32 & 0x0fff_ffff);
        self.w
    }
}
#[doc = "Field `SAR_I2C_START` reader - start I2C only active when reg_sar_i2c_start_force = 1"]
pub struct SAR_I2C_START_R(crate::FieldReader<bool, bool>);
impl SAR_I2C_START_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SAR_I2C_START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR_I2C_START_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR_I2C_START` writer - start I2C only active when reg_sar_i2c_start_force = 1"]
pub struct SAR_I2C_START_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR_I2C_START_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `SAR_I2C_START_FORCE` reader - 1: I2C started by SW 0: I2C started by FSM"]
pub struct SAR_I2C_START_FORCE_R(crate::FieldReader<bool, bool>);
impl SAR_I2C_START_FORCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SAR_I2C_START_FORCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR_I2C_START_FORCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR_I2C_START_FORCE` writer - 1: I2C started by SW 0: I2C started by FSM"]
pub struct SAR_I2C_START_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR_I2C_START_FORCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:27 - I2C control data only active when reg_sar_i2c_start_force = 1"]
    #[inline(always)]
    pub fn sar_i2c_ctrl(&self) -> SAR_I2C_CTRL_R {
        SAR_I2C_CTRL_R::new((self.bits & 0x0fff_ffff) as u32)
    }
    #[doc = "Bit 28 - start I2C only active when reg_sar_i2c_start_force = 1"]
    #[inline(always)]
    pub fn sar_i2c_start(&self) -> SAR_I2C_START_R {
        SAR_I2C_START_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - 1: I2C started by SW 0: I2C started by FSM"]
    #[inline(always)]
    pub fn sar_i2c_start_force(&self) -> SAR_I2C_START_FORCE_R {
        SAR_I2C_START_FORCE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:27 - I2C control data only active when reg_sar_i2c_start_force = 1"]
    #[inline(always)]
    pub fn sar_i2c_ctrl(&mut self) -> SAR_I2C_CTRL_W {
        SAR_I2C_CTRL_W { w: self }
    }
    #[doc = "Bit 28 - start I2C only active when reg_sar_i2c_start_force = 1"]
    #[inline(always)]
    pub fn sar_i2c_start(&mut self) -> SAR_I2C_START_W {
        SAR_I2C_START_W { w: self }
    }
    #[doc = "Bit 29 - 1: I2C started by SW 0: I2C started by FSM"]
    #[inline(always)]
    pub fn sar_i2c_start_force(&mut self) -> SAR_I2C_START_FORCE_W {
        SAR_I2C_START_FORCE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_i2c_ctrl](index.html) module"]
pub struct SAR_I2C_CTRL_SPEC;
impl crate::RegisterSpec for SAR_I2C_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_i2c_ctrl::R](R) reader structure"]
impl crate::Readable for SAR_I2C_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_i2c_ctrl::W](W) writer structure"]
impl crate::Writable for SAR_I2C_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAR_I2C_CTRL to value 0"]
impl crate::Resettable for SAR_I2C_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
