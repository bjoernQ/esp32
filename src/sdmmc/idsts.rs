#[doc = "Register `IDSTS` reader"]
pub struct R(crate::R<IDSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IDSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IDSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IDSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IDSTS` writer"]
pub struct W(crate::W<IDSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IDSTS_SPEC>;
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
impl From<crate::W<IDSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IDSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TI` reader - Transmit Interrupt. Indicates that data transmission is finished for a descriptor. Writing 1 clears this bit."]
pub struct TI_R(crate::FieldReader<bool, bool>);
impl TI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TI` writer - Transmit Interrupt. Indicates that data transmission is finished for a descriptor. Writing 1 clears this bit."]
pub struct TI_W<'a> {
    w: &'a mut W,
}
impl<'a> TI_W<'a> {
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
#[doc = "Field `RI` reader - Receive Interrupt. Indicates the completion of data reception for a descriptor. Writing 1 clears this bit."]
pub struct RI_R(crate::FieldReader<bool, bool>);
impl RI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RI` writer - Receive Interrupt. Indicates the completion of data reception for a descriptor. Writing 1 clears this bit."]
pub struct RI_W<'a> {
    w: &'a mut W,
}
impl<'a> RI_W<'a> {
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
#[doc = "Field `FBE` reader - Fatal Bus Error Interrupt. Indicates that a Bus Error occurred (IDSTS\\[12:10\\]) . When this bit is set, the DMA disables all its bus accesses. Writing 1 clears this bit."]
pub struct FBE_R(crate::FieldReader<bool, bool>);
impl FBE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FBE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FBE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FBE` writer - Fatal Bus Error Interrupt. Indicates that a Bus Error occurred (IDSTS\\[12:10\\]) . When this bit is set, the DMA disables all its bus accesses. Writing 1 clears this bit."]
pub struct FBE_W<'a> {
    w: &'a mut W,
}
impl<'a> FBE_W<'a> {
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
#[doc = "Field `DU` reader - Descriptor Unavailable Interrupt. This bit is set when the descriptor is unavailable due to OWNER bit = 0 (DES0\\[31\\]
= 0). Writing 1 clears this bit."]
pub struct DU_R(crate::FieldReader<bool, bool>);
impl DU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DU` writer - Descriptor Unavailable Interrupt. This bit is set when the descriptor is unavailable due to OWNER bit = 0 (DES0\\[31\\]
= 0). Writing 1 clears this bit."]
pub struct DU_W<'a> {
    w: &'a mut W,
}
impl<'a> DU_W<'a> {
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
#[doc = "Field `CES` reader - Card Error Summary. Indicates the status of the transaction to/from the card, also present in RINTSTS. Indicates the logical OR of the following bits: EBE : End Bit Error; RTO : Response Timeout/Boot Ack Timeout; RCRC : Response CRC; SBE : Start Bit Error; DRTO : Data Read Timeout/BDS timeout; DCRC : Data CRC for Receive; RE : Response Error. Writing 1 clears this bit. The abort condition of the IDMAC depends on the setting of this CES bit. If the CES bit is enabled, then the IDMAC aborts on a response error."]
pub struct CES_R(crate::FieldReader<bool, bool>);
impl CES_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CES_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CES` writer - Card Error Summary. Indicates the status of the transaction to/from the card, also present in RINTSTS. Indicates the logical OR of the following bits: EBE : End Bit Error; RTO : Response Timeout/Boot Ack Timeout; RCRC : Response CRC; SBE : Start Bit Error; DRTO : Data Read Timeout/BDS timeout; DCRC : Data CRC for Receive; RE : Response Error. Writing 1 clears this bit. The abort condition of the IDMAC depends on the setting of this CES bit. If the CES bit is enabled, then the IDMAC aborts on a response error."]
pub struct CES_W<'a> {
    w: &'a mut W,
}
impl<'a> CES_W<'a> {
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
#[doc = "Field `NIS` reader - Normal Interrupt Summary. Logical OR of the following: IDSTS\\[0\\]
: Transmit Interrupt, IDSTS\\[1\\]
: Receive Interrupt. Only unmasked bits affect this bit. This is a sticky bit and must be cleared each time a corresponding bit that causes NIS to be set is cleared. Writing 1 clears this bit."]
pub struct NIS_R(crate::FieldReader<bool, bool>);
impl NIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NIS` writer - Normal Interrupt Summary. Logical OR of the following: IDSTS\\[0\\]
: Transmit Interrupt, IDSTS\\[1\\]
: Receive Interrupt. Only unmasked bits affect this bit. This is a sticky bit and must be cleared each time a corresponding bit that causes NIS to be set is cleared. Writing 1 clears this bit."]
pub struct NIS_W<'a> {
    w: &'a mut W,
}
impl<'a> NIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `AIS` reader - Abnormal Interrupt Summary. Logical OR of the following: IDSTS\\[2\\]
: Fatal Bus Interrupt, IDSTS\\[4\\]
: DU bit Interrupt. Only unmasked bits affect this bit. This is a sticky bit and must be cleared each time a corresponding bit that causes AIS to be set is cleared. Writing 1 clears this bit."]
pub struct AIS_R(crate::FieldReader<bool, bool>);
impl AIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AIS` writer - Abnormal Interrupt Summary. Logical OR of the following: IDSTS\\[2\\]
: Fatal Bus Interrupt, IDSTS\\[4\\]
: DU bit Interrupt. Only unmasked bits affect this bit. This is a sticky bit and must be cleared each time a corresponding bit that causes AIS to be set is cleared. Writing 1 clears this bit."]
pub struct AIS_W<'a> {
    w: &'a mut W,
}
impl<'a> AIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `FBE_CODE` reader - Fatal Bus Error Code. Indicates the type of error that caused a Bus Error. Valid only when the Fatal Bus Error bit IDSTS\\[2\\]
is set. This field does not generate an interrupt. 001: Host Abort received during transmission; 010: Host Abort received during reception; Others: Reserved."]
pub struct FBE_CODE_R(crate::FieldReader<u8, u8>);
impl FBE_CODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FBE_CODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FBE_CODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FBE_CODE` writer - Fatal Bus Error Code. Indicates the type of error that caused a Bus Error. Valid only when the Fatal Bus Error bit IDSTS\\[2\\]
is set. This field does not generate an interrupt. 001: Host Abort received during transmission; 010: Host Abort received during reception; Others: Reserved."]
pub struct FBE_CODE_W<'a> {
    w: &'a mut W,
}
impl<'a> FBE_CODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 10)) | ((value as u32 & 0x07) << 10);
        self.w
    }
}
#[doc = "Field `FSM` reader - DMAC FSM present state. 0: DMA_IDLE (idle state); 1: DMA_SUSPEND (suspend state); 2: DESC_RD (descriptor reading state); 3: DESC_CHK (descriptor checking state); 4: DMA_RD_REQ_WAIT (read-data request waiting state); 5: DMA_WR_REQ_WAIT (write-data request waiting state); 6: DMA_RD (data-read state); 7: DMA_WR (data-write state); 8: DESC_CLOSE (descriptor close state)."]
pub struct FSM_R(crate::FieldReader<u8, u8>);
impl FSM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FSM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSM` writer - DMAC FSM present state. 0: DMA_IDLE (idle state); 1: DMA_SUSPEND (suspend state); 2: DESC_RD (descriptor reading state); 3: DESC_CHK (descriptor checking state); 4: DMA_RD_REQ_WAIT (read-data request waiting state); 5: DMA_WR_REQ_WAIT (write-data request waiting state); 6: DMA_RD (data-read state); 7: DMA_WR (data-write state); 8: DESC_CLOSE (descriptor close state)."]
pub struct FSM_W<'a> {
    w: &'a mut W,
}
impl<'a> FSM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 13)) | ((value as u32 & 0x0f) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Transmit Interrupt. Indicates that data transmission is finished for a descriptor. Writing 1 clears this bit."]
    #[inline(always)]
    pub fn ti(&self) -> TI_R {
        TI_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Receive Interrupt. Indicates the completion of data reception for a descriptor. Writing 1 clears this bit."]
    #[inline(always)]
    pub fn ri(&self) -> RI_R {
        RI_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Fatal Bus Error Interrupt. Indicates that a Bus Error occurred (IDSTS\\[12:10\\]) . When this bit is set, the DMA disables all its bus accesses. Writing 1 clears this bit."]
    #[inline(always)]
    pub fn fbe(&self) -> FBE_R {
        FBE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Descriptor Unavailable Interrupt. This bit is set when the descriptor is unavailable due to OWNER bit = 0 (DES0\\[31\\]
= 0). Writing 1 clears this bit."]
    #[inline(always)]
    pub fn du(&self) -> DU_R {
        DU_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Card Error Summary. Indicates the status of the transaction to/from the card, also present in RINTSTS. Indicates the logical OR of the following bits: EBE : End Bit Error; RTO : Response Timeout/Boot Ack Timeout; RCRC : Response CRC; SBE : Start Bit Error; DRTO : Data Read Timeout/BDS timeout; DCRC : Data CRC for Receive; RE : Response Error. Writing 1 clears this bit. The abort condition of the IDMAC depends on the setting of this CES bit. If the CES bit is enabled, then the IDMAC aborts on a response error."]
    #[inline(always)]
    pub fn ces(&self) -> CES_R {
        CES_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Normal Interrupt Summary. Logical OR of the following: IDSTS\\[0\\]
: Transmit Interrupt, IDSTS\\[1\\]
: Receive Interrupt. Only unmasked bits affect this bit. This is a sticky bit and must be cleared each time a corresponding bit that causes NIS to be set is cleared. Writing 1 clears this bit."]
    #[inline(always)]
    pub fn nis(&self) -> NIS_R {
        NIS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Abnormal Interrupt Summary. Logical OR of the following: IDSTS\\[2\\]
: Fatal Bus Interrupt, IDSTS\\[4\\]
: DU bit Interrupt. Only unmasked bits affect this bit. This is a sticky bit and must be cleared each time a corresponding bit that causes AIS to be set is cleared. Writing 1 clears this bit."]
    #[inline(always)]
    pub fn ais(&self) -> AIS_R {
        AIS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 10:12 - Fatal Bus Error Code. Indicates the type of error that caused a Bus Error. Valid only when the Fatal Bus Error bit IDSTS\\[2\\]
is set. This field does not generate an interrupt. 001: Host Abort received during transmission; 010: Host Abort received during reception; Others: Reserved."]
    #[inline(always)]
    pub fn fbe_code(&self) -> FBE_CODE_R {
        FBE_CODE_R::new(((self.bits >> 10) & 0x07) as u8)
    }
    #[doc = "Bits 13:16 - DMAC FSM present state. 0: DMA_IDLE (idle state); 1: DMA_SUSPEND (suspend state); 2: DESC_RD (descriptor reading state); 3: DESC_CHK (descriptor checking state); 4: DMA_RD_REQ_WAIT (read-data request waiting state); 5: DMA_WR_REQ_WAIT (write-data request waiting state); 6: DMA_RD (data-read state); 7: DMA_WR (data-write state); 8: DESC_CLOSE (descriptor close state)."]
    #[inline(always)]
    pub fn fsm(&self) -> FSM_R {
        FSM_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Interrupt. Indicates that data transmission is finished for a descriptor. Writing 1 clears this bit."]
    #[inline(always)]
    pub fn ti(&mut self) -> TI_W {
        TI_W { w: self }
    }
    #[doc = "Bit 1 - Receive Interrupt. Indicates the completion of data reception for a descriptor. Writing 1 clears this bit."]
    #[inline(always)]
    pub fn ri(&mut self) -> RI_W {
        RI_W { w: self }
    }
    #[doc = "Bit 2 - Fatal Bus Error Interrupt. Indicates that a Bus Error occurred (IDSTS\\[12:10\\]) . When this bit is set, the DMA disables all its bus accesses. Writing 1 clears this bit."]
    #[inline(always)]
    pub fn fbe(&mut self) -> FBE_W {
        FBE_W { w: self }
    }
    #[doc = "Bit 4 - Descriptor Unavailable Interrupt. This bit is set when the descriptor is unavailable due to OWNER bit = 0 (DES0\\[31\\]
= 0). Writing 1 clears this bit."]
    #[inline(always)]
    pub fn du(&mut self) -> DU_W {
        DU_W { w: self }
    }
    #[doc = "Bit 5 - Card Error Summary. Indicates the status of the transaction to/from the card, also present in RINTSTS. Indicates the logical OR of the following bits: EBE : End Bit Error; RTO : Response Timeout/Boot Ack Timeout; RCRC : Response CRC; SBE : Start Bit Error; DRTO : Data Read Timeout/BDS timeout; DCRC : Data CRC for Receive; RE : Response Error. Writing 1 clears this bit. The abort condition of the IDMAC depends on the setting of this CES bit. If the CES bit is enabled, then the IDMAC aborts on a response error."]
    #[inline(always)]
    pub fn ces(&mut self) -> CES_W {
        CES_W { w: self }
    }
    #[doc = "Bit 8 - Normal Interrupt Summary. Logical OR of the following: IDSTS\\[0\\]
: Transmit Interrupt, IDSTS\\[1\\]
: Receive Interrupt. Only unmasked bits affect this bit. This is a sticky bit and must be cleared each time a corresponding bit that causes NIS to be set is cleared. Writing 1 clears this bit."]
    #[inline(always)]
    pub fn nis(&mut self) -> NIS_W {
        NIS_W { w: self }
    }
    #[doc = "Bit 9 - Abnormal Interrupt Summary. Logical OR of the following: IDSTS\\[2\\]
: Fatal Bus Interrupt, IDSTS\\[4\\]
: DU bit Interrupt. Only unmasked bits affect this bit. This is a sticky bit and must be cleared each time a corresponding bit that causes AIS to be set is cleared. Writing 1 clears this bit."]
    #[inline(always)]
    pub fn ais(&mut self) -> AIS_W {
        AIS_W { w: self }
    }
    #[doc = "Bits 10:12 - Fatal Bus Error Code. Indicates the type of error that caused a Bus Error. Valid only when the Fatal Bus Error bit IDSTS\\[2\\]
is set. This field does not generate an interrupt. 001: Host Abort received during transmission; 010: Host Abort received during reception; Others: Reserved."]
    #[inline(always)]
    pub fn fbe_code(&mut self) -> FBE_CODE_W {
        FBE_CODE_W { w: self }
    }
    #[doc = "Bits 13:16 - DMAC FSM present state. 0: DMA_IDLE (idle state); 1: DMA_SUSPEND (suspend state); 2: DESC_RD (descriptor reading state); 3: DESC_CHK (descriptor checking state); 4: DMA_RD_REQ_WAIT (read-data request waiting state); 5: DMA_WR_REQ_WAIT (write-data request waiting state); 6: DMA_RD (data-read state); 7: DMA_WR (data-write state); 8: DESC_CLOSE (descriptor close state)."]
    #[inline(always)]
    pub fn fsm(&mut self) -> FSM_W {
        FSM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IDMAC status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idsts](index.html) module"]
pub struct IDSTS_SPEC;
impl crate::RegisterSpec for IDSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [idsts::R](R) reader structure"]
impl crate::Readable for IDSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [idsts::W](W) writer structure"]
impl crate::Writable for IDSTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IDSTS to value 0"]
impl crate::Resettable for IDSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
