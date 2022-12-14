#[doc = "Register `DO` reader"]
pub struct R(crate::R<DO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DO` writer"]
pub struct W(crate::W<DO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DO_SPEC>;
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
impl From<crate::W<DO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - "]
pub type DATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DATA` writer - "]
pub type DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DO_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<0> {
        DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EEPROM Write Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [do_](index.html) module"]
pub struct DO_SPEC;
impl crate::RegisterSpec for DO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [do_::R](R) reader structure"]
impl crate::Readable for DO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [do_::W](W) writer structure"]
impl crate::Writable for DO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DO to value 0"]
impl crate::Resettable for DO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
