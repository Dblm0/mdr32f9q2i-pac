#[doc = "Register `SEP0_STS` reader"]
pub struct R(crate::R<SEP0_STS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEP0_STS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEP0_STS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEP0_STS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEP0_STS` writer"]
pub struct W(crate::W<SEP0_STS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEP0_STS_SPEC>;
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
impl From<crate::W<SEP0_STS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEP0_STS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCCRCERR` reader - "]
pub type SCCRCERR_R = crate::BitReader<bool>;
#[doc = "Field `SCCRCERR` writer - "]
pub type SCCRCERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SEP0_STS_SPEC, bool, O>;
#[doc = "Field `SCBSERR` reader - "]
pub type SCBSERR_R = crate::BitReader<bool>;
#[doc = "Field `SCBSERR` writer - "]
pub type SCBSERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SEP0_STS_SPEC, bool, O>;
#[doc = "Field `SCRXOF` reader - "]
pub type SCRXOF_R = crate::BitReader<bool>;
#[doc = "Field `SCRXOF` writer - "]
pub type SCRXOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SEP0_STS_SPEC, bool, O>;
#[doc = "Field `SCRXTO` reader - "]
pub type SCRXTO_R = crate::BitReader<bool>;
#[doc = "Field `SCRXTO` writer - "]
pub type SCRXTO_W<'a, const O: u8> = crate::BitWriter<'a, u32, SEP0_STS_SPEC, bool, O>;
#[doc = "Field `SCNAKSENT` reader - "]
pub type SCNAKSENT_R = crate::BitReader<bool>;
#[doc = "Field `SCNAKSENT` writer - "]
pub type SCNAKSENT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SEP0_STS_SPEC, bool, O>;
#[doc = "Field `SCSTALLSENT` reader - "]
pub type SCSTALLSENT_R = crate::BitReader<bool>;
#[doc = "Field `SCSTALLSENT` writer - "]
pub type SCSTALLSENT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SEP0_STS_SPEC, bool, O>;
#[doc = "Field `SCACKRXED` reader - "]
pub type SCACKRXED_R = crate::BitReader<bool>;
#[doc = "Field `SCACKRXED` writer - "]
pub type SCACKRXED_W<'a, const O: u8> = crate::BitWriter<'a, u32, SEP0_STS_SPEC, bool, O>;
#[doc = "Field `SCDATASEQ` reader - "]
pub type SCDATASEQ_R = crate::BitReader<bool>;
#[doc = "Field `SCDATASEQ` writer - "]
pub type SCDATASEQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, SEP0_STS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sccrcerr(&self) -> SCCRCERR_R {
        SCCRCERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn scbserr(&self) -> SCBSERR_R {
        SCBSERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn scrxof(&self) -> SCRXOF_R {
        SCRXOF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn scrxto(&self) -> SCRXTO_R {
        SCRXTO_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn scnaksent(&self) -> SCNAKSENT_R {
        SCNAKSENT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn scstallsent(&self) -> SCSTALLSENT_R {
        SCSTALLSENT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn scackrxed(&self) -> SCACKRXED_R {
        SCACKRXED_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn scdataseq(&self) -> SCDATASEQ_R {
        SCDATASEQ_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn sccrcerr(&mut self) -> SCCRCERR_W<0> {
        SCCRCERR_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn scbserr(&mut self) -> SCBSERR_W<1> {
        SCBSERR_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn scrxof(&mut self) -> SCRXOF_W<2> {
        SCRXOF_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn scrxto(&mut self) -> SCRXTO_W<3> {
        SCRXTO_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn scnaksent(&mut self) -> SCNAKSENT_W<4> {
        SCNAKSENT_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn scstallsent(&mut self) -> SCSTALLSENT_W<5> {
        SCSTALLSENT_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn scackrxed(&mut self) -> SCACKRXED_W<6> {
        SCACKRXED_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn scdataseq(&mut self) -> SCDATASEQ_W<7> {
        SCDATASEQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB_SEP Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sep0_sts](index.html) module"]
pub struct SEP0_STS_SPEC;
impl crate::RegisterSpec for SEP0_STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sep0_sts::R](R) reader structure"]
impl crate::Readable for SEP0_STS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sep0_sts::W](W) writer structure"]
impl crate::Writable for SEP0_STS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEP0_STS to value 0"]
impl crate::Resettable for SEP0_STS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
