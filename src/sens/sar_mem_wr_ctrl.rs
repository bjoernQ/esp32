#[doc = "Register `SAR_MEM_WR_CTRL` reader"]
pub struct R(crate::R<SAR_MEM_WR_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_MEM_WR_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_MEM_WR_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_MEM_WR_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_MEM_WR_CTRL` writer"]
pub struct W(crate::W<SAR_MEM_WR_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_MEM_WR_CTRL_SPEC>;
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
impl From<crate::W<SAR_MEM_WR_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_MEM_WR_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MEM_WR_ADDR_INIT` reader - "]
pub struct MEM_WR_ADDR_INIT_R(crate::FieldReader<u16, u16>);
impl MEM_WR_ADDR_INIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        MEM_WR_ADDR_INIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM_WR_ADDR_INIT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM_WR_ADDR_INIT` writer - "]
pub struct MEM_WR_ADDR_INIT_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_WR_ADDR_INIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | (value as u32 & 0x07ff);
        self.w
    }
}
#[doc = "Field `MEM_WR_ADDR_SIZE` reader - "]
pub struct MEM_WR_ADDR_SIZE_R(crate::FieldReader<u16, u16>);
impl MEM_WR_ADDR_SIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        MEM_WR_ADDR_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM_WR_ADDR_SIZE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM_WR_ADDR_SIZE` writer - "]
pub struct MEM_WR_ADDR_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_WR_ADDR_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 11)) | ((value as u32 & 0x07ff) << 11);
        self.w
    }
}
#[doc = "Field `RTC_MEM_WR_OFFST_CLR` writer - "]
pub struct RTC_MEM_WR_OFFST_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_MEM_WR_OFFST_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn mem_wr_addr_init(&self) -> MEM_WR_ADDR_INIT_R {
        MEM_WR_ADDR_INIT_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:21"]
    #[inline(always)]
    pub fn mem_wr_addr_size(&self) -> MEM_WR_ADDR_SIZE_R {
        MEM_WR_ADDR_SIZE_R::new(((self.bits >> 11) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn mem_wr_addr_init(&mut self) -> MEM_WR_ADDR_INIT_W {
        MEM_WR_ADDR_INIT_W { w: self }
    }
    #[doc = "Bits 11:21"]
    #[inline(always)]
    pub fn mem_wr_addr_size(&mut self) -> MEM_WR_ADDR_SIZE_W {
        MEM_WR_ADDR_SIZE_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn rtc_mem_wr_offst_clr(&mut self) -> RTC_MEM_WR_OFFST_CLR_W {
        RTC_MEM_WR_OFFST_CLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_mem_wr_ctrl](index.html) module"]
pub struct SAR_MEM_WR_CTRL_SPEC;
impl crate::RegisterSpec for SAR_MEM_WR_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_mem_wr_ctrl::R](R) reader structure"]
impl crate::Readable for SAR_MEM_WR_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_mem_wr_ctrl::W](W) writer structure"]
impl crate::Writable for SAR_MEM_WR_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAR_MEM_WR_CTRL to value 0x0010_0200"]
impl crate::Resettable for SAR_MEM_WR_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0010_0200
    }
}
