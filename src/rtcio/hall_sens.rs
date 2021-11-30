#[doc = "Register `HALL_SENS` reader"]
pub struct R(crate::R<HALL_SENS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HALL_SENS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HALL_SENS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HALL_SENS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HALL_SENS` writer"]
pub struct W(crate::W<HALL_SENS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HALL_SENS_SPEC>;
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
impl From<crate::W<HALL_SENS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HALL_SENS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HALL_PHASE` reader - Reverse phase of hall sensor"]
pub struct HALL_PHASE_R(crate::FieldReader<bool, bool>);
impl HALL_PHASE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HALL_PHASE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HALL_PHASE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HALL_PHASE` writer - Reverse phase of hall sensor"]
pub struct HALL_PHASE_W<'a> {
    w: &'a mut W,
}
impl<'a> HALL_PHASE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `XPD_HALL` reader - Power on hall sensor and connect to VP and VN"]
pub struct XPD_HALL_R(crate::FieldReader<bool, bool>);
impl XPD_HALL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XPD_HALL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XPD_HALL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XPD_HALL` writer - Power on hall sensor and connect to VP and VN"]
pub struct XPD_HALL_W<'a> {
    w: &'a mut W,
}
impl<'a> XPD_HALL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 30 - Reverse phase of hall sensor"]
    #[inline(always)]
    pub fn hall_phase(&self) -> HALL_PHASE_R {
        HALL_PHASE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Power on hall sensor and connect to VP and VN"]
    #[inline(always)]
    pub fn xpd_hall(&self) -> XPD_HALL_R {
        XPD_HALL_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 30 - Reverse phase of hall sensor"]
    #[inline(always)]
    pub fn hall_phase(&mut self) -> HALL_PHASE_W {
        HALL_PHASE_W { w: self }
    }
    #[doc = "Bit 31 - Power on hall sensor and connect to VP and VN"]
    #[inline(always)]
    pub fn xpd_hall(&mut self) -> XPD_HALL_W {
        XPD_HALL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hall_sens](index.html) module"]
pub struct HALL_SENS_SPEC;
impl crate::RegisterSpec for HALL_SENS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hall_sens::R](R) reader structure"]
impl crate::Readable for HALL_SENS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hall_sens::W](W) writer structure"]
impl crate::Writable for HALL_SENS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HALL_SENS to value 0"]
impl crate::Resettable for HALL_SENS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
