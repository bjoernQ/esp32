#[doc = "Register `APB_CONF` reader"]
pub struct R(crate::R<APB_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB_CONF` writer"]
pub struct W(crate::W<APB_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB_CONF_SPEC>;
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
impl From<crate::W<APB_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `APB_FIFO_MASK` reader - Set this bit to disable apb fifo access"]
pub struct APB_FIFO_MASK_R(crate::FieldReader<bool, bool>);
impl APB_FIFO_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APB_FIFO_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB_FIFO_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APB_FIFO_MASK` writer - Set this bit to disable apb fifo access"]
pub struct APB_FIFO_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_FIFO_MASK_W<'a> {
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
#[doc = "Field `MEM_TX_WRAP_EN` reader - when datas need to be send is more than channel's mem can store then set this bit to enable reusage of mem this bit is used together with reg_rmt_tx_lim_chn."]
pub struct MEM_TX_WRAP_EN_R(crate::FieldReader<bool, bool>);
impl MEM_TX_WRAP_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEM_TX_WRAP_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM_TX_WRAP_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM_TX_WRAP_EN` writer - when datas need to be send is more than channel's mem can store then set this bit to enable reusage of mem this bit is used together with reg_rmt_tx_lim_chn."]
pub struct MEM_TX_WRAP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_TX_WRAP_EN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Set this bit to disable apb fifo access"]
    #[inline(always)]
    pub fn apb_fifo_mask(&self) -> APB_FIFO_MASK_R {
        APB_FIFO_MASK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - when datas need to be send is more than channel's mem can store then set this bit to enable reusage of mem this bit is used together with reg_rmt_tx_lim_chn."]
    #[inline(always)]
    pub fn mem_tx_wrap_en(&self) -> MEM_TX_WRAP_EN_R {
        MEM_TX_WRAP_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to disable apb fifo access"]
    #[inline(always)]
    pub fn apb_fifo_mask(&mut self) -> APB_FIFO_MASK_W {
        APB_FIFO_MASK_W { w: self }
    }
    #[doc = "Bit 1 - when datas need to be send is more than channel's mem can store then set this bit to enable reusage of mem this bit is used together with reg_rmt_tx_lim_chn."]
    #[inline(always)]
    pub fn mem_tx_wrap_en(&mut self) -> MEM_TX_WRAP_EN_W {
        MEM_TX_WRAP_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_conf](index.html) module"]
pub struct APB_CONF_SPEC;
impl crate::RegisterSpec for APB_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb_conf::R](R) reader structure"]
impl crate::Readable for APB_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb_conf::W](W) writer structure"]
impl crate::Writable for APB_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APB_CONF to value 0"]
impl crate::Resettable for APB_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
