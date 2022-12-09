#[doc = "Register `REG_0B` reader"]
pub struct R(crate::R<REG_0B_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG_0B_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG_0B_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG_0B_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REG_0B` writer"]
pub struct W(crate::W<REG_0B_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG_0B_SPEC>;
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
impl From<crate::W<REG_0B_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG_0B_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BKP_REG` reader - "]
pub type BKP_REG_R = crate::FieldReader<u32, u32>;
#[doc = "Field `BKP_REG` writer - "]
pub type BKP_REG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REG_0B_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bkp_reg(&self) -> BKP_REG_R {
        BKP_REG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn bkp_reg(&mut self) -> BKP_REG_W<0> {
        BKP_REG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Backup Register 11\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_0b](index.html) module"]
pub struct REG_0B_SPEC;
impl crate::RegisterSpec for REG_0B_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg_0b::R](R) reader structure"]
impl crate::Readable for REG_0B_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg_0b::W](W) writer structure"]
impl crate::Writable for REG_0B_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REG_0B to value 0"]
impl crate::Resettable for REG_0B_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
