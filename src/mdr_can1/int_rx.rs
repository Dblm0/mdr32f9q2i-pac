#[doc = "Register `INT_RX` reader"]
pub struct R(crate::R<INT_RX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_RX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_RX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_RX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_RX` writer"]
pub struct W(crate::W<INT_RX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_RX_SPEC>;
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
impl From<crate::W<INT_RX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_RX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT_RX` reader - "]
pub type INT_RX_R = crate::FieldReader<u32, u32>;
#[doc = "Field `INT_RX` writer - "]
pub type INT_RX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INT_RX_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn int_rx(&self) -> INT_RX_R {
        INT_RX_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn int_rx(&mut self) -> INT_RX_W<0> {
        INT_RX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_rx](index.html) module"]
pub struct INT_RX_SPEC;
impl crate::RegisterSpec for INT_RX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_rx::R](R) reader structure"]
impl crate::Readable for INT_RX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_rx::W](W) writer structure"]
impl crate::Writable for INT_RX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_RX to value 0"]
impl crate::Resettable for INT_RX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
