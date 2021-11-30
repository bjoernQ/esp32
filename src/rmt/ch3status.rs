#[doc = "Register `CH3STATUS` reader"]
pub struct R(crate::R<CH3STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH3STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH3STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH3STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `STATUS_CH3` reader - The status for channel3"]
pub struct STATUS_CH3_R(crate::FieldReader<u32, u32>);
impl STATUS_CH3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        STATUS_CH3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STATUS_CH3_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM_WADDR_EX_CH3` reader - The current memory read address of channel3."]
pub struct MEM_WADDR_EX_CH3_R(crate::FieldReader<u16, u16>);
impl MEM_WADDR_EX_CH3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        MEM_WADDR_EX_CH3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM_WADDR_EX_CH3_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM_RADDR_EX_CH3` reader - The current memory write address of channel3."]
pub struct MEM_RADDR_EX_CH3_R(crate::FieldReader<u16, u16>);
impl MEM_RADDR_EX_CH3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        MEM_RADDR_EX_CH3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM_RADDR_EX_CH3_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STATE_CH3` reader - The channel3 state machine status register.3'h0 : idle, 3'h1 : send, 3'h2 : read memory, 3'h3 : receive, 3'h4 : wait."]
pub struct STATE_CH3_R(crate::FieldReader<u8, u8>);
impl STATE_CH3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        STATE_CH3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STATE_CH3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM_OWNER_ERR_CH3` reader - When channel3 is configured for receive mode, this bit will turn to high level if rmt_mem_owner register is not set to 1."]
pub struct MEM_OWNER_ERR_CH3_R(crate::FieldReader<bool, bool>);
impl MEM_OWNER_ERR_CH3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEM_OWNER_ERR_CH3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM_OWNER_ERR_CH3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM_FULL_CH3` reader - The memory full status bit for channel3 turns to high level when mem_waddr_ex is greater than or equal to the configuration range."]
pub struct MEM_FULL_CH3_R(crate::FieldReader<bool, bool>);
impl MEM_FULL_CH3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEM_FULL_CH3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM_FULL_CH3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM_EMPTY_CH3` reader - The memory empty status bit for channel3. in acyclic mode, this bit turns to high level when mem_raddr_ex is greater than or equal to the configured range."]
pub struct MEM_EMPTY_CH3_R(crate::FieldReader<bool, bool>);
impl MEM_EMPTY_CH3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEM_EMPTY_CH3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM_EMPTY_CH3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APB_MEM_WR_ERR_CH3` reader - The apb write memory status bit for channel3 turns to high level when the apb write address exceeds the configuration range."]
pub struct APB_MEM_WR_ERR_CH3_R(crate::FieldReader<bool, bool>);
impl APB_MEM_WR_ERR_CH3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APB_MEM_WR_ERR_CH3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB_MEM_WR_ERR_CH3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APB_MEM_RD_ERR_CH3` reader - The apb read memory status bit for channel3 turns to high level when the apb read address exceeds the configuration range."]
pub struct APB_MEM_RD_ERR_CH3_R(crate::FieldReader<bool, bool>);
impl APB_MEM_RD_ERR_CH3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APB_MEM_RD_ERR_CH3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB_MEM_RD_ERR_CH3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - The status for channel3"]
    #[inline(always)]
    pub fn status_ch3(&self) -> STATUS_CH3_R {
        STATUS_CH3_R::new(self.bits as u32)
    }
    #[doc = "Bits 0:9 - The current memory read address of channel3."]
    #[inline(always)]
    pub fn mem_waddr_ex_ch3(&self) -> MEM_WADDR_EX_CH3_R {
        MEM_WADDR_EX_CH3_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 12:21 - The current memory write address of channel3."]
    #[inline(always)]
    pub fn mem_raddr_ex_ch3(&self) -> MEM_RADDR_EX_CH3_R {
        MEM_RADDR_EX_CH3_R::new(((self.bits >> 12) & 0x03ff) as u16)
    }
    #[doc = "Bits 24:26 - The channel3 state machine status register.3'h0 : idle, 3'h1 : send, 3'h2 : read memory, 3'h3 : receive, 3'h4 : wait."]
    #[inline(always)]
    pub fn state_ch3(&self) -> STATE_CH3_R {
        STATE_CH3_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bit 27 - When channel3 is configured for receive mode, this bit will turn to high level if rmt_mem_owner register is not set to 1."]
    #[inline(always)]
    pub fn mem_owner_err_ch3(&self) -> MEM_OWNER_ERR_CH3_R {
        MEM_OWNER_ERR_CH3_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - The memory full status bit for channel3 turns to high level when mem_waddr_ex is greater than or equal to the configuration range."]
    #[inline(always)]
    pub fn mem_full_ch3(&self) -> MEM_FULL_CH3_R {
        MEM_FULL_CH3_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - The memory empty status bit for channel3. in acyclic mode, this bit turns to high level when mem_raddr_ex is greater than or equal to the configured range."]
    #[inline(always)]
    pub fn mem_empty_ch3(&self) -> MEM_EMPTY_CH3_R {
        MEM_EMPTY_CH3_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - The apb write memory status bit for channel3 turns to high level when the apb write address exceeds the configuration range."]
    #[inline(always)]
    pub fn apb_mem_wr_err_ch3(&self) -> APB_MEM_WR_ERR_CH3_R {
        APB_MEM_WR_ERR_CH3_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - The apb read memory status bit for channel3 turns to high level when the apb read address exceeds the configuration range."]
    #[inline(always)]
    pub fn apb_mem_rd_err_ch3(&self) -> APB_MEM_RD_ERR_CH3_R {
        APB_MEM_RD_ERR_CH3_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3status](index.html) module"]
pub struct CH3STATUS_SPEC;
impl crate::RegisterSpec for CH3STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch3status::R](R) reader structure"]
impl crate::Readable for CH3STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CH3STATUS to value 0"]
impl crate::Resettable for CH3STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
