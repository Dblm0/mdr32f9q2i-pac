#[doc = "Register `PLL_CONTROL` reader"]
pub struct R(crate::R<PLL_CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL_CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL_CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL_CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL_CONTROL` writer"]
pub struct W(crate::W<PLL_CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL_CONTROL_SPEC>;
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
impl From<crate::W<PLL_CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL_CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLL_USB_ON` reader - "]
pub type PLL_USB_ON_R = crate::BitReader<bool>;
#[doc = "Field `PLL_USB_ON` writer - "]
pub type PLL_USB_ON_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL_CONTROL_SPEC, bool, O>;
#[doc = "Field `PLL_USB_RLD` reader - "]
pub type PLL_USB_RLD_R = crate::BitReader<bool>;
#[doc = "Field `PLL_USB_RLD` writer - "]
pub type PLL_USB_RLD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL_CONTROL_SPEC, bool, O>;
#[doc = "Field `PLL_CPU_ON` reader - "]
pub type PLL_CPU_ON_R = crate::BitReader<bool>;
#[doc = "Field `PLL_CPU_ON` writer - "]
pub type PLL_CPU_ON_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL_CONTROL_SPEC, bool, O>;
#[doc = "Field `PLL_CPU_PLD` reader - "]
pub type PLL_CPU_PLD_R = crate::BitReader<bool>;
#[doc = "Field `PLL_CPU_PLD` writer - "]
pub type PLL_CPU_PLD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL_CONTROL_SPEC, bool, O>;
#[doc = "Field `PLL_USB_MUL` reader - "]
pub type PLL_USB_MUL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLL_USB_MUL` writer - "]
pub type PLL_USB_MUL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PLL_CONTROL_SPEC, u8, u8, 4, O>;
#[doc = "Field `PLL_CPU_MUL` reader - "]
pub type PLL_CPU_MUL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLL_CPU_MUL` writer - "]
pub type PLL_CPU_MUL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PLL_CONTROL_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pll_usb_on(&self) -> PLL_USB_ON_R {
        PLL_USB_ON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pll_usb_rld(&self) -> PLL_USB_RLD_R {
        PLL_USB_RLD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pll_cpu_on(&self) -> PLL_CPU_ON_R {
        PLL_CPU_ON_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pll_cpu_pld(&self) -> PLL_CPU_PLD_R {
        PLL_CPU_PLD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn pll_usb_mul(&self) -> PLL_USB_MUL_R {
        PLL_USB_MUL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn pll_cpu_mul(&self) -> PLL_CPU_MUL_R {
        PLL_CPU_MUL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pll_usb_on(&mut self) -> PLL_USB_ON_W<0> {
        PLL_USB_ON_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn pll_usb_rld(&mut self) -> PLL_USB_RLD_W<1> {
        PLL_USB_RLD_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn pll_cpu_on(&mut self) -> PLL_CPU_ON_W<2> {
        PLL_CPU_ON_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn pll_cpu_pld(&mut self) -> PLL_CPU_PLD_W<3> {
        PLL_CPU_PLD_W::new(self)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn pll_usb_mul(&mut self) -> PLL_USB_MUL_W<4> {
        PLL_USB_MUL_W::new(self)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    #[must_use]
    pub fn pll_cpu_mul(&mut self) -> PLL_CPU_MUL_W<8> {
        PLL_CPU_MUL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll_control](index.html) module"]
pub struct PLL_CONTROL_SPEC;
impl crate::RegisterSpec for PLL_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll_control::R](R) reader structure"]
impl crate::Readable for PLL_CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll_control::W](W) writer structure"]
impl crate::Writable for PLL_CONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLL_CONTROL to value 0"]
impl crate::Resettable for PLL_CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
