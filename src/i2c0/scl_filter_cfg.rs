#[doc = "Register `SCL_FILTER_CFG` reader"]
pub struct R(crate::R<SCL_FILTER_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCL_FILTER_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCL_FILTER_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCL_FILTER_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCL_FILTER_CFG` writer"]
pub struct W(crate::W<SCL_FILTER_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCL_FILTER_CFG_SPEC>;
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
impl From<crate::W<SCL_FILTER_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCL_FILTER_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCL_FILTER_THRES` reader - When input SCL's pulse width is smaller than this register value I2C ignores this pulse."]
pub struct SCL_FILTER_THRES_R(crate::FieldReader<u8, u8>);
impl SCL_FILTER_THRES_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SCL_FILTER_THRES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCL_FILTER_THRES_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCL_FILTER_THRES` writer - When input SCL's pulse width is smaller than this register value I2C ignores this pulse."]
pub struct SCL_FILTER_THRES_W<'a> {
    w: &'a mut W,
}
impl<'a> SCL_FILTER_THRES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `SCL_FILTER_EN` reader - This is the filter enable bit for SCL."]
pub struct SCL_FILTER_EN_R(crate::FieldReader<bool, bool>);
impl SCL_FILTER_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCL_FILTER_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCL_FILTER_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCL_FILTER_EN` writer - This is the filter enable bit for SCL."]
pub struct SCL_FILTER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCL_FILTER_EN_W<'a> {
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
impl R {
    #[doc = "Bits 0:2 - When input SCL's pulse width is smaller than this register value I2C ignores this pulse."]
    #[inline(always)]
    pub fn scl_filter_thres(&self) -> SCL_FILTER_THRES_R {
        SCL_FILTER_THRES_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - This is the filter enable bit for SCL."]
    #[inline(always)]
    pub fn scl_filter_en(&self) -> SCL_FILTER_EN_R {
        SCL_FILTER_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - When input SCL's pulse width is smaller than this register value I2C ignores this pulse."]
    #[inline(always)]
    pub fn scl_filter_thres(&mut self) -> SCL_FILTER_THRES_W {
        SCL_FILTER_THRES_W { w: self }
    }
    #[doc = "Bit 3 - This is the filter enable bit for SCL."]
    #[inline(always)]
    pub fn scl_filter_en(&mut self) -> SCL_FILTER_EN_W {
        SCL_FILTER_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scl_filter_cfg](index.html) module"]
pub struct SCL_FILTER_CFG_SPEC;
impl crate::RegisterSpec for SCL_FILTER_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scl_filter_cfg::R](R) reader structure"]
impl crate::Readable for SCL_FILTER_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scl_filter_cfg::W](W) writer structure"]
impl crate::Writable for SCL_FILTER_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCL_FILTER_CFG to value 0x08"]
impl crate::Resettable for SCL_FILTER_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x08
    }
}
