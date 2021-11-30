#[doc = "Register `PRO_INTRUSION_CTRL` reader"]
pub struct R(crate::R<PRO_INTRUSION_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_INTRUSION_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_INTRUSION_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_INTRUSION_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRO_INTRUSION_CTRL` writer"]
pub struct W(crate::W<PRO_INTRUSION_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRO_INTRUSION_CTRL_SPEC>;
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
impl From<crate::W<PRO_INTRUSION_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRO_INTRUSION_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRO_INTRUSION_RECORD_RESET_N` reader - "]
pub struct PRO_INTRUSION_RECORD_RESET_N_R(crate::FieldReader<bool, bool>);
impl PRO_INTRUSION_RECORD_RESET_N_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRO_INTRUSION_RECORD_RESET_N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_INTRUSION_RECORD_RESET_N_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_INTRUSION_RECORD_RESET_N` writer - "]
pub struct PRO_INTRUSION_RECORD_RESET_N_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_INTRUSION_RECORD_RESET_N_W<'a> {
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
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pro_intrusion_record_reset_n(&self) -> PRO_INTRUSION_RECORD_RESET_N_R {
        PRO_INTRUSION_RECORD_RESET_N_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pro_intrusion_record_reset_n(&mut self) -> PRO_INTRUSION_RECORD_RESET_N_W {
        PRO_INTRUSION_RECORD_RESET_N_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_intrusion_ctrl](index.html) module"]
pub struct PRO_INTRUSION_CTRL_SPEC;
impl crate::RegisterSpec for PRO_INTRUSION_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_intrusion_ctrl::R](R) reader structure"]
impl crate::Readable for PRO_INTRUSION_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pro_intrusion_ctrl::W](W) writer structure"]
impl crate::Writable for PRO_INTRUSION_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRO_INTRUSION_CTRL to value 0x01"]
impl crate::Resettable for PRO_INTRUSION_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
