#[doc = "Register `HRXFD` reader"]
pub struct R(crate::R<HRXFD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HRXFD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HRXFD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HRXFD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HRXFD` writer"]
pub struct W(crate::W<HRXFD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HRXFD_SPEC>;
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
impl From<crate::W<HRXFD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HRXFD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXFIFODATA` reader - "]
pub type RXFIFODATA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXFIFODATA` writer - "]
pub type RXFIFODATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HRXFD_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn rxfifodata(&self) -> RXFIFODATA_R {
        RXFIFODATA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn rxfifodata(&mut self) -> RXFIFODATA_W<0> {
        RXFIFODATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hrxfd](index.html) module"]
pub struct HRXFD_SPEC;
impl crate::RegisterSpec for HRXFD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hrxfd::R](R) reader structure"]
impl crate::Readable for HRXFD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hrxfd::W](W) writer structure"]
impl crate::Writable for HRXFD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HRXFD to value 0"]
impl crate::Resettable for HRXFD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
