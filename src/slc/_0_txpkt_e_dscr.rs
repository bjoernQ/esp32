#[doc = "Register `_0_TXPKT_E_DSCR` reader"]
pub struct R(crate::R<_0_TXPKT_E_DSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<_0_TXPKT_E_DSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<_0_TXPKT_E_DSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<_0_TXPKT_E_DSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `_0_TXPKT_E_DSCR` writer"]
pub struct W(crate::W<_0_TXPKT_E_DSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<_0_TXPKT_E_DSCR_SPEC>;
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
impl From<crate::W<_0_TXPKT_E_DSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<_0_TXPKT_E_DSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLC0_TX_PKT_E_DSCR_ADDR` reader - "]
pub struct SLC0_TX_PKT_E_DSCR_ADDR_R(crate::FieldReader<u32, u32>);
impl SLC0_TX_PKT_E_DSCR_ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        SLC0_TX_PKT_E_DSCR_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLC0_TX_PKT_E_DSCR_ADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLC0_TX_PKT_E_DSCR_ADDR` writer - "]
pub struct SLC0_TX_PKT_E_DSCR_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_TX_PKT_E_DSCR_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn slc0_tx_pkt_e_dscr_addr(&self) -> SLC0_TX_PKT_E_DSCR_ADDR_R {
        SLC0_TX_PKT_E_DSCR_ADDR_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn slc0_tx_pkt_e_dscr_addr(&mut self) -> SLC0_TX_PKT_E_DSCR_ADDR_W {
        SLC0_TX_PKT_E_DSCR_ADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_0_txpkt_e_dscr](index.html) module"]
pub struct _0_TXPKT_E_DSCR_SPEC;
impl crate::RegisterSpec for _0_TXPKT_E_DSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [_0_txpkt_e_dscr::R](R) reader structure"]
impl crate::Readable for _0_TXPKT_E_DSCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [_0_txpkt_e_dscr::W](W) writer structure"]
impl crate::Writable for _0_TXPKT_E_DSCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets _0_TXPKT_E_DSCR to value 0"]
impl crate::Resettable for _0_TXPKT_E_DSCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}