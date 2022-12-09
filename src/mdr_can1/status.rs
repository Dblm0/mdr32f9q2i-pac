#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATUS` writer"]
pub struct W(crate::W<STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATUS_SPEC>;
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
impl From<crate::W<STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_READY` reader - "]
pub type RX_READY_R = crate::BitReader<bool>;
#[doc = "Field `RX_READY` writer - "]
pub type RX_READY_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `TX_READY` reader - "]
pub type TX_READY_R = crate::BitReader<bool>;
#[doc = "Field `TX_READY` writer - "]
pub type TX_READY_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `ERROR_OVER` reader - "]
pub type ERROR_OVER_R = crate::BitReader<bool>;
#[doc = "Field `ERROR_OVER` writer - "]
pub type ERROR_OVER_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `BIT_ERR` reader - "]
pub type BIT_ERR_R = crate::BitReader<bool>;
#[doc = "Field `BIT_ERR` writer - "]
pub type BIT_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `BIT_STUFF_ERR` reader - "]
pub type BIT_STUFF_ERR_R = crate::BitReader<bool>;
#[doc = "Field `BIT_STUFF_ERR` writer - "]
pub type BIT_STUFF_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `CRC_ERR` reader - "]
pub type CRC_ERR_R = crate::BitReader<bool>;
#[doc = "Field `CRC_ERR` writer - "]
pub type CRC_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `FRAME_ERR` reader - "]
pub type FRAME_ERR_R = crate::BitReader<bool>;
#[doc = "Field `FRAME_ERR` writer - "]
pub type FRAME_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `ACK_ERR` reader - "]
pub type ACK_ERR_R = crate::BitReader<bool>;
#[doc = "Field `ACK_ERR` writer - "]
pub type ACK_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `IDLOWER` reader - "]
pub type IDLOWER_R = crate::BitReader<bool>;
#[doc = "Field `IDLOWER` writer - "]
pub type IDLOWER_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `ERR_STATUS` reader - "]
pub type ERR_STATUS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ERR_STATUS` writer - "]
pub type ERR_STATUS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STATUS_SPEC, u8, u8, 2, O>;
#[doc = "Field `RX_ERR_CNT8` reader - "]
pub type RX_ERR_CNT8_R = crate::BitReader<bool>;
#[doc = "Field `RX_ERR_CNT8` writer - "]
pub type RX_ERR_CNT8_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `TX_ERR_CNT8` reader - "]
pub type TX_ERR_CNT8_R = crate::BitReader<bool>;
#[doc = "Field `TX_ERR_CNT8` writer - "]
pub type TX_ERR_CNT8_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `RX_ERR_CNT` reader - "]
pub type RX_ERR_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RX_ERR_CNT` writer - "]
pub type RX_ERR_CNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STATUS_SPEC, u8, u8, 8, O>;
#[doc = "Field `TX_ERR_CNT` reader - "]
pub type TX_ERR_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TX_ERR_CNT` writer - "]
pub type TX_ERR_CNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STATUS_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rx_ready(&self) -> RX_READY_R {
        RX_READY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tx_ready(&self) -> TX_READY_R {
        TX_READY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn error_over(&self) -> ERROR_OVER_R {
        ERROR_OVER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn bit_err(&self) -> BIT_ERR_R {
        BIT_ERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn bit_stuff_err(&self) -> BIT_STUFF_ERR_R {
        BIT_STUFF_ERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn crc_err(&self) -> CRC_ERR_R {
        CRC_ERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn frame_err(&self) -> FRAME_ERR_R {
        FRAME_ERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ack_err(&self) -> ACK_ERR_R {
        ACK_ERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn idlower(&self) -> IDLOWER_R {
        IDLOWER_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn err_status(&self) -> ERR_STATUS_R {
        ERR_STATUS_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn rx_err_cnt8(&self) -> RX_ERR_CNT8_R {
        RX_ERR_CNT8_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn tx_err_cnt8(&self) -> TX_ERR_CNT8_R {
        TX_ERR_CNT8_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn rx_err_cnt(&self) -> RX_ERR_CNT_R {
        RX_ERR_CNT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn tx_err_cnt(&self) -> TX_ERR_CNT_R {
        TX_ERR_CNT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn rx_ready(&mut self) -> RX_READY_W<0> {
        RX_READY_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn tx_ready(&mut self) -> TX_READY_W<1> {
        TX_READY_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn error_over(&mut self) -> ERROR_OVER_W<2> {
        ERROR_OVER_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn bit_err(&mut self) -> BIT_ERR_W<3> {
        BIT_ERR_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn bit_stuff_err(&mut self) -> BIT_STUFF_ERR_W<4> {
        BIT_STUFF_ERR_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn crc_err(&mut self) -> CRC_ERR_W<5> {
        CRC_ERR_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn frame_err(&mut self) -> FRAME_ERR_W<6> {
        FRAME_ERR_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn ack_err(&mut self) -> ACK_ERR_W<7> {
        ACK_ERR_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn idlower(&mut self) -> IDLOWER_W<8> {
        IDLOWER_W::new(self)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    #[must_use]
    pub fn err_status(&mut self) -> ERR_STATUS_W<9> {
        ERR_STATUS_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn rx_err_cnt8(&mut self) -> RX_ERR_CNT8_W<11> {
        RX_ERR_CNT8_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn tx_err_cnt8(&mut self) -> TX_ERR_CNT8_W<12> {
        TX_ERR_CNT8_W::new(self)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn rx_err_cnt(&mut self) -> RX_ERR_CNT_W<16> {
        RX_ERR_CNT_W::new(self)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn tx_err_cnt(&mut self) -> TX_ERR_CNT_W<24> {
        TX_ERR_CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [status::W](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
