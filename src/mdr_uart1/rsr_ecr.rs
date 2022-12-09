#[doc = "Register `RSR_ECR` reader"]
pub struct R(crate::R<RSR_ECR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSR_ECR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSR_ECR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSR_ECR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSR_ECR` writer"]
pub struct W(crate::W<RSR_ECR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSR_ECR_SPEC>;
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
impl From<crate::W<RSR_ECR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSR_ECR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FE` reader - "]
pub type FE_R = crate::BitReader<bool>;
#[doc = "Field `FE` writer - "]
pub type FE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSR_ECR_SPEC, bool, O>;
#[doc = "Field `PE` reader - "]
pub type PE_R = crate::BitReader<bool>;
#[doc = "Field `PE` writer - "]
pub type PE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSR_ECR_SPEC, bool, O>;
#[doc = "Field `BE` reader - "]
pub type BE_R = crate::BitReader<bool>;
#[doc = "Field `BE` writer - "]
pub type BE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSR_ECR_SPEC, bool, O>;
#[doc = "Field `OE` reader - "]
pub type OE_R = crate::BitReader<bool>;
#[doc = "Field `OE` writer - "]
pub type OE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSR_ECR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn fe(&self) -> FE_R {
        FE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn be(&self) -> BE_R {
        BE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn oe(&self) -> OE_R {
        OE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn fe(&mut self) -> FE_W<0> {
        FE_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn pe(&mut self) -> PE_W<1> {
        PE_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn be(&mut self) -> BE_W<2> {
        BE_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn oe(&mut self) -> OE_W<3> {
        OE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART RSR Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsr_ecr](index.html) module"]
pub struct RSR_ECR_SPEC;
impl crate::RegisterSpec for RSR_ECR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rsr_ecr::R](R) reader structure"]
impl crate::Readable for RSR_ECR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rsr_ecr::W](W) writer structure"]
impl crate::Writable for RSR_ECR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RSR_ECR to value 0"]
impl crate::Resettable for RSR_ECR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
