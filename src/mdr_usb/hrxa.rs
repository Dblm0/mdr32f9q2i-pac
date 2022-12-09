#[doc = "Register `HRXA` reader"]
pub struct R(crate::R<HRXA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HRXA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HRXA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HRXA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HRXA` writer"]
pub struct W(crate::W<HRXA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HRXA_SPEC>;
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
impl From<crate::W<HRXA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HRXA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RADDR` reader - "]
pub type RADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RADDR` writer - "]
pub type RADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HRXA_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn raddr(&self) -> RADDR_R {
        RADDR_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    #[must_use]
    pub fn raddr(&mut self) -> RADDR_W<0> {
        RADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hrxa](index.html) module"]
pub struct HRXA_SPEC;
impl crate::RegisterSpec for HRXA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hrxa::R](R) reader structure"]
impl crate::Readable for HRXA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hrxa::W](W) writer structure"]
impl crate::Writable for HRXA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HRXA to value 0"]
impl crate::Resettable for HRXA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
