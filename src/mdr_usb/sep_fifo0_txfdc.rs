#[doc = "Register `SEP_FIFO0_TXFDC` reader"]
pub struct R(crate::R<SEP_FIFO0_TXFDC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEP_FIFO0_TXFDC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEP_FIFO0_TXFDC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEP_FIFO0_TXFDC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEP_FIFO0_TXFDC` writer"]
pub struct W(crate::W<SEP_FIFO0_TXFDC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEP_FIFO0_TXFDC_SPEC>;
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
impl From<crate::W<SEP_FIFO0_TXFDC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEP_FIFO0_TXFDC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIFOFORCEEMPTY` reader - "]
pub type FIFOFORCEEMPTY_R = crate::BitReader<bool>;
#[doc = "Field `FIFOFORCEEMPTY` writer - "]
pub type FIFOFORCEEMPTY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEP_FIFO0_TXFDC_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn fifoforceempty(&self) -> FIFOFORCEEMPTY_R {
        FIFOFORCEEMPTY_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn fifoforceempty(&mut self) -> FIFOFORCEEMPTY_W<0> {
        FIFOFORCEEMPTY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sep_fifo0_txfdc](index.html) module"]
pub struct SEP_FIFO0_TXFDC_SPEC;
impl crate::RegisterSpec for SEP_FIFO0_TXFDC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sep_fifo0_txfdc::R](R) reader structure"]
impl crate::Readable for SEP_FIFO0_TXFDC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sep_fifo0_txfdc::W](W) writer structure"]
impl crate::Writable for SEP_FIFO0_TXFDC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEP_FIFO0_TXFDC to value 0"]
impl crate::Resettable for SEP_FIFO0_TXFDC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
