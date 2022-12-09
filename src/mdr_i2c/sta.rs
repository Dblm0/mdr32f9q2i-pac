#[doc = "Register `STA` reader"]
pub struct R(crate::R<STA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STA` writer"]
pub struct W(crate::W<STA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STA_SPEC>;
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
impl From<crate::W<STA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT` reader - "]
pub type INT_R = crate::BitReader<bool>;
#[doc = "Field `INT` writer - "]
pub type INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, STA_SPEC, bool, O>;
#[doc = "Field `TR_PROG` reader - "]
pub type TR_PROG_R = crate::BitReader<bool>;
#[doc = "Field `TR_PROG` writer - "]
pub type TR_PROG_W<'a, const O: u8> = crate::BitWriter<'a, u32, STA_SPEC, bool, O>;
#[doc = "Field `LOST_ARB` reader - "]
pub type LOST_ARB_R = crate::BitReader<bool>;
#[doc = "Field `LOST_ARB` writer - "]
pub type LOST_ARB_W<'a, const O: u8> = crate::BitWriter<'a, u32, STA_SPEC, bool, O>;
#[doc = "Field `BUSY` reader - "]
pub type BUSY_R = crate::BitReader<bool>;
#[doc = "Field `BUSY` writer - "]
pub type BUSY_W<'a, const O: u8> = crate::BitWriter<'a, u32, STA_SPEC, bool, O>;
#[doc = "Field `Rx_ACK` reader - "]
pub type RX_ACK_R = crate::BitReader<bool>;
#[doc = "Field `Rx_ACK` writer - "]
pub type RX_ACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, STA_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn int(&self) -> INT_R {
        INT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tr_prog(&self) -> TR_PROG_R {
        TR_PROG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn lost_arb(&self) -> LOST_ARB_R {
        LOST_ARB_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rx_ack(&self) -> RX_ACK_R {
        RX_ACK_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn int(&mut self) -> INT_W<0> {
        INT_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn tr_prog(&mut self) -> TR_PROG_W<1> {
        TR_PROG_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn lost_arb(&mut self) -> LOST_ARB_W<5> {
        LOST_ARB_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn busy(&mut self) -> BUSY_W<6> {
        BUSY_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn rx_ack(&mut self) -> RX_ACK_W<7> {
        RX_ACK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sta](index.html) module"]
pub struct STA_SPEC;
impl crate::RegisterSpec for STA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sta::R](R) reader structure"]
impl crate::Readable for STA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sta::W](W) writer structure"]
impl crate::Writable for STA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STA to value 0"]
impl crate::Resettable for STA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
