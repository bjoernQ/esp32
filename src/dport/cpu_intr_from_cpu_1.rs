#[doc = "Register `CPU_INTR_FROM_CPU_1` reader"]
pub struct R(crate::R<CPU_INTR_FROM_CPU_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU_INTR_FROM_CPU_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPU_INTR_FROM_CPU_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPU_INTR_FROM_CPU_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPU_INTR_FROM_CPU_1` writer"]
pub struct W(crate::W<CPU_INTR_FROM_CPU_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPU_INTR_FROM_CPU_1_SPEC>;
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
impl From<crate::W<CPU_INTR_FROM_CPU_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPU_INTR_FROM_CPU_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPU_INTR_FROM_CPU_1` reader - "]
pub struct CPU_INTR_FROM_CPU_1_R(crate::FieldReader<bool, bool>);
impl CPU_INTR_FROM_CPU_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CPU_INTR_FROM_CPU_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPU_INTR_FROM_CPU_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPU_INTR_FROM_CPU_1` writer - "]
pub struct CPU_INTR_FROM_CPU_1_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU_INTR_FROM_CPU_1_W<'a> {
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
    pub fn cpu_intr_from_cpu_1(&self) -> CPU_INTR_FROM_CPU_1_R {
        CPU_INTR_FROM_CPU_1_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cpu_intr_from_cpu_1(&mut self) -> CPU_INTR_FROM_CPU_1_W {
        CPU_INTR_FROM_CPU_1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_intr_from_cpu_1](index.html) module"]
pub struct CPU_INTR_FROM_CPU_1_SPEC;
impl crate::RegisterSpec for CPU_INTR_FROM_CPU_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu_intr_from_cpu_1::R](R) reader structure"]
impl crate::Readable for CPU_INTR_FROM_CPU_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpu_intr_from_cpu_1::W](W) writer structure"]
impl crate::Writable for CPU_INTR_FROM_CPU_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CPU_INTR_FROM_CPU_1 to value 0"]
impl crate::Resettable for CPU_INTR_FROM_CPU_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
