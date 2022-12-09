#[doc = "Register `BUF_CON00` reader"]
pub struct R(crate::R<BUF_CON00_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUF_CON00_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUF_CON00_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUF_CON00_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BUF_CON00` writer"]
pub struct W(crate::W<BUF_CON00_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUF_CON00_SPEC>;
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
impl From<crate::W<BUF_CON00_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUF_CON00_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - "]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - "]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BUF_CON00_SPEC, bool, O>;
#[doc = "Field `RX_TXn` reader - "]
pub type RX_TXN_R = crate::BitReader<bool>;
#[doc = "Field `RX_TXn` writer - "]
pub type RX_TXN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BUF_CON00_SPEC, bool, O>;
#[doc = "Field `OVER_EN` reader - "]
pub type OVER_EN_R = crate::BitReader<bool>;
#[doc = "Field `OVER_EN` writer - "]
pub type OVER_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BUF_CON00_SPEC, bool, O>;
#[doc = "Field `RTR_EN` reader - "]
pub type RTR_EN_R = crate::BitReader<bool>;
#[doc = "Field `RTR_EN` writer - "]
pub type RTR_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BUF_CON00_SPEC, bool, O>;
#[doc = "Field `PRIOR_0` reader - "]
pub type PRIOR_0_R = crate::BitReader<bool>;
#[doc = "Field `PRIOR_0` writer - "]
pub type PRIOR_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, BUF_CON00_SPEC, bool, O>;
#[doc = "Field `TX_REQ` reader - "]
pub type TX_REQ_R = crate::BitReader<bool>;
#[doc = "Field `TX_REQ` writer - "]
pub type TX_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, BUF_CON00_SPEC, bool, O>;
#[doc = "Field `RX_FULL` reader - "]
pub type RX_FULL_R = crate::BitReader<bool>;
#[doc = "Field `RX_FULL` writer - "]
pub type RX_FULL_W<'a, const O: u8> = crate::BitWriter<'a, u32, BUF_CON00_SPEC, bool, O>;
#[doc = "Field `OVER_WR` reader - "]
pub type OVER_WR_R = crate::BitReader<bool>;
#[doc = "Field `OVER_WR` writer - "]
pub type OVER_WR_W<'a, const O: u8> = crate::BitWriter<'a, u32, BUF_CON00_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rx_txn(&self) -> RX_TXN_R {
        RX_TXN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn over_en(&self) -> OVER_EN_R {
        OVER_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rtr_en(&self) -> RTR_EN_R {
        RTR_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn prior_0(&self) -> PRIOR_0_R {
        PRIOR_0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tx_req(&self) -> TX_REQ_R {
        TX_REQ_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rx_full(&self) -> RX_FULL_R {
        RX_FULL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn over_wr(&self) -> OVER_WR_R {
        OVER_WR_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn rx_txn(&mut self) -> RX_TXN_W<1> {
        RX_TXN_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn over_en(&mut self) -> OVER_EN_W<2> {
        OVER_EN_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn rtr_en(&mut self) -> RTR_EN_W<3> {
        RTR_EN_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn prior_0(&mut self) -> PRIOR_0_W<4> {
        PRIOR_0_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn tx_req(&mut self) -> TX_REQ_W<5> {
        TX_REQ_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn rx_full(&mut self) -> RX_FULL_W<6> {
        RX_FULL_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn over_wr(&mut self) -> OVER_WR_W<7> {
        OVER_WR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN Buffer Connection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buf_con00](index.html) module"]
pub struct BUF_CON00_SPEC;
impl crate::RegisterSpec for BUF_CON00_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [buf_con00::R](R) reader structure"]
impl crate::Readable for BUF_CON00_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [buf_con00::W](W) writer structure"]
impl crate::Writable for BUF_CON00_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BUF_CON00 to value 0"]
impl crate::Resettable for BUF_CON00_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
