#[doc = "Register `FAULT_DETECT` reader"]
pub struct R(crate::R<FAULT_DETECT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FAULT_DETECT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FAULT_DETECT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FAULT_DETECT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FAULT_DETECT` writer"]
pub struct W(crate::W<FAULT_DETECT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FAULT_DETECT_SPEC>;
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
impl From<crate::W<FAULT_DETECT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FAULT_DETECT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `F0_EN` reader - "]
pub struct F0_EN_R(crate::FieldReader<bool, bool>);
impl F0_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        F0_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for F0_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `F0_EN` writer - "]
pub struct F0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> F0_EN_W<'a> {
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
#[doc = "Field `F1_EN` reader - "]
pub struct F1_EN_R(crate::FieldReader<bool, bool>);
impl F1_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        F1_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for F1_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `F1_EN` writer - "]
pub struct F1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> F1_EN_W<'a> {
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
#[doc = "Field `F2_EN` reader - "]
pub struct F2_EN_R(crate::FieldReader<bool, bool>);
impl F2_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        F2_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for F2_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `F2_EN` writer - "]
pub struct F2_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> F2_EN_W<'a> {
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
#[doc = "Field `F0_POLE` reader - "]
pub struct F0_POLE_R(crate::FieldReader<bool, bool>);
impl F0_POLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        F0_POLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for F0_POLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `F0_POLE` writer - "]
pub struct F0_POLE_W<'a> {
    w: &'a mut W,
}
impl<'a> F0_POLE_W<'a> {
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
#[doc = "Field `F1_POLE` reader - "]
pub struct F1_POLE_R(crate::FieldReader<bool, bool>);
impl F1_POLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        F1_POLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for F1_POLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `F1_POLE` writer - "]
pub struct F1_POLE_W<'a> {
    w: &'a mut W,
}
impl<'a> F1_POLE_W<'a> {
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
#[doc = "Field `F2_POLE` reader - "]
pub struct F2_POLE_R(crate::FieldReader<bool, bool>);
impl F2_POLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        F2_POLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for F2_POLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `F2_POLE` writer - "]
pub struct F2_POLE_W<'a> {
    w: &'a mut W,
}
impl<'a> F2_POLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `EVENT_F0` reader - "]
pub struct EVENT_F0_R(crate::FieldReader<bool, bool>);
impl EVENT_F0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EVENT_F0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EVENT_F0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EVENT_F1` reader - "]
pub struct EVENT_F1_R(crate::FieldReader<bool, bool>);
impl EVENT_F1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EVENT_F1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EVENT_F1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EVENT_F2` reader - "]
pub struct EVENT_F2_R(crate::FieldReader<bool, bool>);
impl EVENT_F2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EVENT_F2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EVENT_F2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn f0_en(&self) -> F0_EN_R {
        F0_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn f1_en(&self) -> F1_EN_R {
        F1_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn f2_en(&self) -> F2_EN_R {
        F2_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn f0_pole(&self) -> F0_POLE_R {
        F0_POLE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn f1_pole(&self) -> F1_POLE_R {
        F1_POLE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn f2_pole(&self) -> F2_POLE_R {
        F2_POLE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn event_f0(&self) -> EVENT_F0_R {
        EVENT_F0_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn event_f1(&self) -> EVENT_F1_R {
        EVENT_F1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn event_f2(&self) -> EVENT_F2_R {
        EVENT_F2_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn f0_en(&mut self) -> F0_EN_W {
        F0_EN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn f1_en(&mut self) -> F1_EN_W {
        F1_EN_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn f2_en(&mut self) -> F2_EN_W {
        F2_EN_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn f0_pole(&mut self) -> F0_POLE_W {
        F0_POLE_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn f1_pole(&mut self) -> F1_POLE_W {
        F1_POLE_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn f2_pole(&mut self) -> F2_POLE_W {
        F2_POLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fault_detect](index.html) module"]
pub struct FAULT_DETECT_SPEC;
impl crate::RegisterSpec for FAULT_DETECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fault_detect::R](R) reader structure"]
impl crate::Readable for FAULT_DETECT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fault_detect::W](W) writer structure"]
impl crate::Writable for FAULT_DETECT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FAULT_DETECT to value 0"]
impl crate::Resettable for FAULT_DETECT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
