#[doc = "Register `RXD` reader"]
pub struct R(crate::R<RXD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXD` writer"]
pub struct W(crate::W<RXD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXD_SPEC>;
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
impl From<crate::W<RXD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXD` reader - "]
pub type RXD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXD` writer - "]
pub type RXD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RXD_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn rxd(&self) -> RXD_R {
        RXD_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn rxd(&mut self) -> RXD_W<0> {
        RXD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Received Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxd](index.html) module"]
pub struct RXD_SPEC;
impl crate::RegisterSpec for RXD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxd::R](R) reader structure"]
impl crate::Readable for RXD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxd::W](W) writer structure"]
impl crate::Writable for RXD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXD to value 0"]
impl crate::Resettable for RXD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
