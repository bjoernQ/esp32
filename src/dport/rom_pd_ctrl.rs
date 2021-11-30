#[doc = "Register `ROM_PD_CTRL` reader"]
pub struct R(crate::R<ROM_PD_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ROM_PD_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ROM_PD_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ROM_PD_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ROM_PD_CTRL` writer"]
pub struct W(crate::W<ROM_PD_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ROM_PD_CTRL_SPEC>;
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
impl From<crate::W<ROM_PD_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ROM_PD_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRO_ROM_PD` reader - "]
pub struct PRO_ROM_PD_R(crate::FieldReader<bool, bool>);
impl PRO_ROM_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_ROM_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_ROM_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_ROM_PD` writer - "]
pub struct PRO_ROM_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_ROM_PD_W<'a> {
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
#[doc = "Field `APP_ROM_PD` reader - "]
pub struct APP_ROM_PD_R(crate::FieldReader<bool, bool>);
impl APP_ROM_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APP_ROM_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APP_ROM_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APP_ROM_PD` writer - "]
pub struct APP_ROM_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_ROM_PD_W<'a> {
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
#[doc = "Field `SHARE_ROM_PD` reader - "]
pub struct SHARE_ROM_PD_R(crate::FieldReader<u8, u8>);
impl SHARE_ROM_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SHARE_ROM_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SHARE_ROM_PD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SHARE_ROM_PD` writer - "]
pub struct SHARE_ROM_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> SHARE_ROM_PD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 2)) | ((value as u32 & 0x3f) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pro_rom_pd(&self) -> PRO_ROM_PD_R {
        PRO_ROM_PD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn app_rom_pd(&self) -> APP_ROM_PD_R {
        APP_ROM_PD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:7"]
    #[inline(always)]
    pub fn share_rom_pd(&self) -> SHARE_ROM_PD_R {
        SHARE_ROM_PD_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pro_rom_pd(&mut self) -> PRO_ROM_PD_W {
        PRO_ROM_PD_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn app_rom_pd(&mut self) -> APP_ROM_PD_W {
        APP_ROM_PD_W { w: self }
    }
    #[doc = "Bits 2:7"]
    #[inline(always)]
    pub fn share_rom_pd(&mut self) -> SHARE_ROM_PD_W {
        SHARE_ROM_PD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rom_pd_ctrl](index.html) module"]
pub struct ROM_PD_CTRL_SPEC;
impl crate::RegisterSpec for ROM_PD_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rom_pd_ctrl::R](R) reader structure"]
impl crate::Readable for ROM_PD_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rom_pd_ctrl::W](W) writer structure"]
impl crate::Writable for ROM_PD_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ROM_PD_CTRL to value 0"]
impl crate::Resettable for ROM_PD_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
