#[doc = "Register `TIM_CLOCK` reader"]
pub struct R(crate::R<TIM_CLOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM_CLOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM_CLOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM_CLOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM_CLOCK` writer"]
pub struct W(crate::W<TIM_CLOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM_CLOCK_SPEC>;
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
impl From<crate::W<TIM_CLOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM_CLOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM1_BRG` reader - "]
pub type TIM1_BRG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIM1_BRG` writer - "]
pub type TIM1_BRG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM_CLOCK_SPEC, u8, u8, 8, O>;
#[doc = "Field `TIM2_BRG` reader - "]
pub type TIM2_BRG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIM2_BRG` writer - "]
pub type TIM2_BRG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM_CLOCK_SPEC, u8, u8, 8, O>;
#[doc = "Field `TIM3_BRG` reader - "]
pub type TIM3_BRG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIM3_BRG` writer - "]
pub type TIM3_BRG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM_CLOCK_SPEC, u8, u8, 8, O>;
#[doc = "Field `TIM1_CLK_EN` reader - "]
pub type TIM1_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM1_CLK_EN` writer - "]
pub type TIM1_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM_CLOCK_SPEC, bool, O>;
#[doc = "Field `TIM2_CLK_EN` reader - "]
pub type TIM2_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM2_CLK_EN` writer - "]
pub type TIM2_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM_CLOCK_SPEC, bool, O>;
#[doc = "Field `TIM3_CLK_EN` reader - "]
pub type TIM3_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM3_CLK_EN` writer - "]
pub type TIM3_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM_CLOCK_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn tim1_brg(&self) -> TIM1_BRG_R {
        TIM1_BRG_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn tim2_brg(&self) -> TIM2_BRG_R {
        TIM2_BRG_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn tim3_brg(&self) -> TIM3_BRG_R {
        TIM3_BRG_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn tim1_clk_en(&self) -> TIM1_CLK_EN_R {
        TIM1_CLK_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn tim2_clk_en(&self) -> TIM2_CLK_EN_R {
        TIM2_CLK_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn tim3_clk_en(&self) -> TIM3_CLK_EN_R {
        TIM3_CLK_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn tim1_brg(&mut self) -> TIM1_BRG_W<0> {
        TIM1_BRG_W::new(self)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn tim2_brg(&mut self) -> TIM2_BRG_W<8> {
        TIM2_BRG_W::new(self)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn tim3_brg(&mut self) -> TIM3_BRG_W<16> {
        TIM3_BRG_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn tim1_clk_en(&mut self) -> TIM1_CLK_EN_W<24> {
        TIM1_CLK_EN_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn tim2_clk_en(&mut self) -> TIM2_CLK_EN_W<25> {
        TIM2_CLK_EN_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn tim3_clk_en(&mut self) -> TIM3_CLK_EN_W<26> {
        TIM3_CLK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim_clock](index.html) module"]
pub struct TIM_CLOCK_SPEC;
impl crate::RegisterSpec for TIM_CLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tim_clock::R](R) reader structure"]
impl crate::Readable for TIM_CLOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim_clock::W](W) writer structure"]
impl crate::Writable for TIM_CLOCK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIM_CLOCK to value 0"]
impl crate::Resettable for TIM_CLOCK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
