#[doc = "Register `NAND_CYCLES` reader"]
pub struct R(crate::R<NAND_CYCLES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NAND_CYCLES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NAND_CYCLES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NAND_CYCLES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NAND_CYCLES` writer"]
pub struct W(crate::W<NAND_CYCLES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NAND_CYCLES_SPEC>;
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
impl From<crate::W<NAND_CYCLES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NAND_CYCLES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRC` reader - "]
pub type TRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRC` writer - "]
pub type TRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NAND_CYCLES_SPEC, u8, u8, 4, O>;
#[doc = "Field `TWC` reader - "]
pub type TWC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TWC` writer - "]
pub type TWC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NAND_CYCLES_SPEC, u8, u8, 4, O>;
#[doc = "Field `TREA` reader - "]
pub type TREA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TREA` writer - "]
pub type TREA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NAND_CYCLES_SPEC, u8, u8, 4, O>;
#[doc = "Field `TWP` reader - "]
pub type TWP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TWP` writer - "]
pub type TWP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NAND_CYCLES_SPEC, u8, u8, 4, O>;
#[doc = "Field `TWHR` reader - "]
pub type TWHR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TWHR` writer - "]
pub type TWHR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NAND_CYCLES_SPEC, u8, u8, 4, O>;
#[doc = "Field `TALEA` reader - "]
pub type TALEA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TALEA` writer - "]
pub type TALEA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NAND_CYCLES_SPEC, u8, u8, 4, O>;
#[doc = "Field `TRR` reader - "]
pub type TRR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRR` writer - "]
pub type TRR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NAND_CYCLES_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn trc(&self) -> TRC_R {
        TRC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn twc(&self) -> TWC_R {
        TWC_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn trea(&self) -> TREA_R {
        TREA_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn twp(&self) -> TWP_R {
        TWP_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn twhr(&self) -> TWHR_R {
        TWHR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn talea(&self) -> TALEA_R {
        TALEA_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn trr(&self) -> TRR_R {
        TRR_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn trc(&mut self) -> TRC_W<0> {
        TRC_W::new(self)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn twc(&mut self) -> TWC_W<4> {
        TWC_W::new(self)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    #[must_use]
    pub fn trea(&mut self) -> TREA_W<8> {
        TREA_W::new(self)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    #[must_use]
    pub fn twp(&mut self) -> TWP_W<12> {
        TWP_W::new(self)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    #[must_use]
    pub fn twhr(&mut self) -> TWHR_W<16> {
        TWHR_W::new(self)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    #[must_use]
    pub fn talea(&mut self) -> TALEA_W<20> {
        TALEA_W::new(self)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    #[must_use]
    pub fn trr(&mut self) -> TRR_W<24> {
        TRR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EBC NAND Timing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nand_cycles](index.html) module"]
pub struct NAND_CYCLES_SPEC;
impl crate::RegisterSpec for NAND_CYCLES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nand_cycles::R](R) reader structure"]
impl crate::Readable for NAND_CYCLES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nand_cycles::W](W) writer structure"]
impl crate::Writable for NAND_CYCLES_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NAND_CYCLES to value 0"]
impl crate::Resettable for NAND_CYCLES_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
