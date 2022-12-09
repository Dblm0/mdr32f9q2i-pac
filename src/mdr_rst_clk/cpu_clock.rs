#[doc = "Register `CPU_CLOCK` reader"]
pub struct R(crate::R<CPU_CLOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU_CLOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPU_CLOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPU_CLOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPU_CLOCK` writer"]
pub struct W(crate::W<CPU_CLOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPU_CLOCK_SPEC>;
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
impl From<crate::W<CPU_CLOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPU_CLOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPU_C1_SEL` reader - "]
pub type CPU_C1_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CPU_C1_SEL` writer - "]
pub type CPU_C1_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CPU_CLOCK_SPEC, u8, u8, 2, O>;
#[doc = "Field `CPU_C2_SEL` reader - "]
pub type CPU_C2_SEL_R = crate::BitReader<bool>;
#[doc = "Field `CPU_C2_SEL` writer - "]
pub type CPU_C2_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPU_CLOCK_SPEC, bool, O>;
#[doc = "Field `CPU_C3_SEL` reader - "]
pub type CPU_C3_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CPU_C3_SEL` writer - "]
pub type CPU_C3_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CPU_CLOCK_SPEC, u8, u8, 4, O>;
#[doc = "Field `HCLK_SEL` reader - "]
pub type HCLK_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HCLK_SEL` writer - "]
pub type HCLK_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CPU_CLOCK_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn cpu_c1_sel(&self) -> CPU_C1_SEL_R {
        CPU_C1_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cpu_c2_sel(&self) -> CPU_C2_SEL_R {
        CPU_C2_SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn cpu_c3_sel(&self) -> CPU_C3_SEL_R {
        CPU_C3_SEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn hclk_sel(&self) -> HCLK_SEL_R {
        HCLK_SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn cpu_c1_sel(&mut self) -> CPU_C1_SEL_W<0> {
        CPU_C1_SEL_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn cpu_c2_sel(&mut self) -> CPU_C2_SEL_W<2> {
        CPU_C2_SEL_W::new(self)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn cpu_c3_sel(&mut self) -> CPU_C3_SEL_W<4> {
        CPU_C3_SEL_W::new(self)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_sel(&mut self) -> HCLK_SEL_W<8> {
        HCLK_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPU Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_clock](index.html) module"]
pub struct CPU_CLOCK_SPEC;
impl crate::RegisterSpec for CPU_CLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu_clock::R](R) reader structure"]
impl crate::Readable for CPU_CLOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpu_clock::W](W) writer structure"]
impl crate::Writable for CPU_CLOCK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPU_CLOCK to value 0"]
impl crate::Resettable for CPU_CLOCK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
