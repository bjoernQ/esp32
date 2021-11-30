#[doc = "Register `PD_CONF` reader"]
pub struct R(crate::R<PD_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PD_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PD_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PD_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PD_CONF` writer"]
pub struct W(crate::W<PD_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PD_CONF_SPEC>;
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
impl From<crate::W<PD_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PD_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIFO_FORCE_PD` reader - "]
pub struct FIFO_FORCE_PD_R(crate::FieldReader<bool, bool>);
impl FIFO_FORCE_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FIFO_FORCE_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFO_FORCE_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFO_FORCE_PD` writer - "]
pub struct FIFO_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_FORCE_PD_W<'a> {
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
#[doc = "Field `FIFO_FORCE_PU` reader - "]
pub struct FIFO_FORCE_PU_R(crate::FieldReader<bool, bool>);
impl FIFO_FORCE_PU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FIFO_FORCE_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFO_FORCE_PU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFO_FORCE_PU` writer - "]
pub struct FIFO_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_FORCE_PU_W<'a> {
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
#[doc = "Field `PLC_MEM_FORCE_PD` reader - "]
pub struct PLC_MEM_FORCE_PD_R(crate::FieldReader<bool, bool>);
impl PLC_MEM_FORCE_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PLC_MEM_FORCE_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLC_MEM_FORCE_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLC_MEM_FORCE_PD` writer - "]
pub struct PLC_MEM_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> PLC_MEM_FORCE_PD_W<'a> {
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
#[doc = "Field `PLC_MEM_FORCE_PU` reader - "]
pub struct PLC_MEM_FORCE_PU_R(crate::FieldReader<bool, bool>);
impl PLC_MEM_FORCE_PU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PLC_MEM_FORCE_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLC_MEM_FORCE_PU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLC_MEM_FORCE_PU` writer - "]
pub struct PLC_MEM_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> PLC_MEM_FORCE_PU_W<'a> {
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
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn fifo_force_pd(&self) -> FIFO_FORCE_PD_R {
        FIFO_FORCE_PD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn fifo_force_pu(&self) -> FIFO_FORCE_PU_R {
        FIFO_FORCE_PU_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn plc_mem_force_pd(&self) -> PLC_MEM_FORCE_PD_R {
        PLC_MEM_FORCE_PD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn plc_mem_force_pu(&self) -> PLC_MEM_FORCE_PU_R {
        PLC_MEM_FORCE_PU_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn fifo_force_pd(&mut self) -> FIFO_FORCE_PD_W {
        FIFO_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn fifo_force_pu(&mut self) -> FIFO_FORCE_PU_W {
        FIFO_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn plc_mem_force_pd(&mut self) -> PLC_MEM_FORCE_PD_W {
        PLC_MEM_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn plc_mem_force_pu(&mut self) -> PLC_MEM_FORCE_PU_W {
        PLC_MEM_FORCE_PU_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pd_conf](index.html) module"]
pub struct PD_CONF_SPEC;
impl crate::RegisterSpec for PD_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pd_conf::R](R) reader structure"]
impl crate::Readable for PD_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pd_conf::W](W) writer structure"]
impl crate::Writable for PD_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PD_CONF to value 0x0a"]
impl crate::Resettable for PD_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0a
    }
}
