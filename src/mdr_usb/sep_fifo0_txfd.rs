#[doc = "Register `SEP_FIFO0_TXFD` reader"]
pub struct R(crate::R<SEP_FIFO0_TXFD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEP_FIFO0_TXFD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEP_FIFO0_TXFD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEP_FIFO0_TXFD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEP_FIFO0_TXFD` writer"]
pub struct W(crate::W<SEP_FIFO0_TXFD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEP_FIFO0_TXFD_SPEC>;
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
impl From<crate::W<SEP_FIFO0_TXFD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEP_FIFO0_TXFD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXFIFODATA` reader - "]
pub type TXFIFODATA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXFIFODATA` writer - "]
pub type TXFIFODATA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SEP_FIFO0_TXFD_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn txfifodata(&self) -> TXFIFODATA_R {
        TXFIFODATA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn txfifodata(&mut self) -> TXFIFODATA_W<0> {
        TXFIFODATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sep_fifo0_txfd](index.html) module"]
pub struct SEP_FIFO0_TXFD_SPEC;
impl crate::RegisterSpec for SEP_FIFO0_TXFD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sep_fifo0_txfd::R](R) reader structure"]
impl crate::Readable for SEP_FIFO0_TXFD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sep_fifo0_txfd::W](W) writer structure"]
impl crate::Writable for SEP_FIFO0_TXFD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEP_FIFO0_TXFD to value 0"]
impl crate::Resettable for SEP_FIFO0_TXFD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
