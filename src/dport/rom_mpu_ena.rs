#[doc = "Register `ROM_MPU_ENA` reader"]
pub struct R(crate::R<ROM_MPU_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ROM_MPU_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ROM_MPU_ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ROM_MPU_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ROM_MPU_ENA` writer"]
pub struct W(crate::W<ROM_MPU_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ROM_MPU_ENA_SPEC>;
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
impl From<crate::W<ROM_MPU_ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ROM_MPU_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SHARE_ROM_MPU_ENA` reader - "]
pub struct SHARE_ROM_MPU_ENA_R(crate::FieldReader<bool, bool>);
impl SHARE_ROM_MPU_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SHARE_ROM_MPU_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SHARE_ROM_MPU_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SHARE_ROM_MPU_ENA` writer - "]
pub struct SHARE_ROM_MPU_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SHARE_ROM_MPU_ENA_W<'a> {
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
#[doc = "Field `PRO_ROM_MPU_ENA` reader - "]
pub struct PRO_ROM_MPU_ENA_R(crate::FieldReader<bool, bool>);
impl PRO_ROM_MPU_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_ROM_MPU_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_ROM_MPU_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_ROM_MPU_ENA` writer - "]
pub struct PRO_ROM_MPU_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_ROM_MPU_ENA_W<'a> {
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
#[doc = "Field `APP_ROM_MPU_ENA` reader - "]
pub struct APP_ROM_MPU_ENA_R(crate::FieldReader<bool, bool>);
impl APP_ROM_MPU_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APP_ROM_MPU_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APP_ROM_MPU_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APP_ROM_MPU_ENA` writer - "]
pub struct APP_ROM_MPU_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_ROM_MPU_ENA_W<'a> {
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
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn share_rom_mpu_ena(&self) -> SHARE_ROM_MPU_ENA_R {
        SHARE_ROM_MPU_ENA_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pro_rom_mpu_ena(&self) -> PRO_ROM_MPU_ENA_R {
        PRO_ROM_MPU_ENA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn app_rom_mpu_ena(&self) -> APP_ROM_MPU_ENA_R {
        APP_ROM_MPU_ENA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn share_rom_mpu_ena(&mut self) -> SHARE_ROM_MPU_ENA_W {
        SHARE_ROM_MPU_ENA_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pro_rom_mpu_ena(&mut self) -> PRO_ROM_MPU_ENA_W {
        PRO_ROM_MPU_ENA_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn app_rom_mpu_ena(&mut self) -> APP_ROM_MPU_ENA_W {
        APP_ROM_MPU_ENA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rom_mpu_ena](index.html) module"]
pub struct ROM_MPU_ENA_SPEC;
impl crate::RegisterSpec for ROM_MPU_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rom_mpu_ena::R](R) reader structure"]
impl crate::Readable for ROM_MPU_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rom_mpu_ena::W](W) writer structure"]
impl crate::Writable for ROM_MPU_ENA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ROM_MPU_ENA to value 0"]
impl crate::Resettable for ROM_MPU_ENA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
