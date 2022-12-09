#[doc = "Register `SEP_FIFO0_RXFDC_H` reader"]
pub struct R(crate::R<SEP_FIFO0_RXFDC_H_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEP_FIFO0_RXFDC_H_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEP_FIFO0_RXFDC_H_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEP_FIFO0_RXFDC_H_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEP_FIFO0_RXFDC_H` writer"]
pub struct W(crate::W<SEP_FIFO0_RXFDC_H_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEP_FIFO0_RXFDC_H_SPEC>;
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
impl From<crate::W<SEP_FIFO0_RXFDC_H_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEP_FIFO0_RXFDC_H_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIFODATACOUNT` reader - "]
pub type FIFODATACOUNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FIFODATACOUNT` writer - "]
pub type FIFODATACOUNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SEP_FIFO0_RXFDC_H_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn fifodatacount(&self) -> FIFODATACOUNT_R {
        FIFODATACOUNT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn fifodatacount(&mut self) -> FIFODATACOUNT_W<0> {
        FIFODATACOUNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sep_fifo0_rxfdc_h](index.html) module"]
pub struct SEP_FIFO0_RXFDC_H_SPEC;
impl crate::RegisterSpec for SEP_FIFO0_RXFDC_H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sep_fifo0_rxfdc_h::R](R) reader structure"]
impl crate::Readable for SEP_FIFO0_RXFDC_H_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sep_fifo0_rxfdc_h::W](W) writer structure"]
impl crate::Writable for SEP_FIFO0_RXFDC_H_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEP_FIFO0_RXFDC_H to value 0"]
impl crate::Resettable for SEP_FIFO0_RXFDC_H_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
