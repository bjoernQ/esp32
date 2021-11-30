#[doc = "Register `CPU_PERIOD_CONF` reader"]
pub struct R(crate::R<CPU_PERIOD_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU_PERIOD_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPU_PERIOD_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPU_PERIOD_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPU_PERIOD_CONF` writer"]
pub struct W(crate::W<CPU_PERIOD_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPU_PERIOD_CONF_SPEC>;
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
impl From<crate::W<CPU_PERIOD_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPU_PERIOD_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPUSEL_CONF` reader - CPU sel option"]
pub struct CPUSEL_CONF_R(crate::FieldReader<bool, bool>);
impl CPUSEL_CONF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CPUSEL_CONF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPUSEL_CONF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPUSEL_CONF` writer - CPU sel option"]
pub struct CPUSEL_CONF_W<'a> {
    w: &'a mut W,
}
impl<'a> CPUSEL_CONF_W<'a> {
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
#[doc = "Field `CPUPERIOD_SEL` reader - CPU period sel"]
pub struct CPUPERIOD_SEL_R(crate::FieldReader<u8, u8>);
impl CPUPERIOD_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CPUPERIOD_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPUPERIOD_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPUPERIOD_SEL` writer - CPU period sel"]
pub struct CPUPERIOD_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CPUPERIOD_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bit 29 - CPU sel option"]
    #[inline(always)]
    pub fn cpusel_conf(&self) -> CPUSEL_CONF_R {
        CPUSEL_CONF_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 30:31 - CPU period sel"]
    #[inline(always)]
    pub fn cpuperiod_sel(&self) -> CPUPERIOD_SEL_R {
        CPUPERIOD_SEL_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 29 - CPU sel option"]
    #[inline(always)]
    pub fn cpusel_conf(&mut self) -> CPUSEL_CONF_W {
        CPUSEL_CONF_W { w: self }
    }
    #[doc = "Bits 30:31 - CPU period sel"]
    #[inline(always)]
    pub fn cpuperiod_sel(&mut self) -> CPUPERIOD_SEL_W {
        CPUPERIOD_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_period_conf](index.html) module"]
pub struct CPU_PERIOD_CONF_SPEC;
impl crate::RegisterSpec for CPU_PERIOD_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu_period_conf::R](R) reader structure"]
impl crate::Readable for CPU_PERIOD_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpu_period_conf::W](W) writer structure"]
impl crate::Writable for CPU_PERIOD_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CPU_PERIOD_CONF to value 0"]
impl crate::Resettable for CPU_PERIOD_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
