#[doc = "Register `USB_CLOCK` reader"]
pub struct R(crate::R<USB_CLOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB_CLOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB_CLOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB_CLOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USB_CLOCK` writer"]
pub struct W(crate::W<USB_CLOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB_CLOCK_SPEC>;
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
impl From<crate::W<USB_CLOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB_CLOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_C1_SEL` reader - "]
pub type USB_C1_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USB_C1_SEL` writer - "]
pub type USB_C1_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, USB_CLOCK_SPEC, u8, u8, 2, O>;
#[doc = "Field `USB_C2_SEL` reader - "]
pub type USB_C2_SEL_R = crate::BitReader<bool>;
#[doc = "Field `USB_C2_SEL` writer - "]
pub type USB_C2_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_CLOCK_SPEC, bool, O>;
#[doc = "Field `USB_C3_SEL` reader - "]
pub type USB_C3_SEL_R = crate::BitReader<bool>;
#[doc = "Field `USB_C3_SEL` writer - "]
pub type USB_C3_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_CLOCK_SPEC, bool, O>;
#[doc = "Field `USB_CLK_EN` reader - "]
pub type USB_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `USB_CLK_EN` writer - "]
pub type USB_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_CLOCK_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn usb_c1_sel(&self) -> USB_C1_SEL_R {
        USB_C1_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn usb_c2_sel(&self) -> USB_C2_SEL_R {
        USB_C2_SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn usb_c3_sel(&self) -> USB_C3_SEL_R {
        USB_C3_SEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn usb_clk_en(&self) -> USB_CLK_EN_R {
        USB_CLK_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn usb_c1_sel(&mut self) -> USB_C1_SEL_W<0> {
        USB_C1_SEL_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn usb_c2_sel(&mut self) -> USB_C2_SEL_W<2> {
        USB_C2_SEL_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn usb_c3_sel(&mut self) -> USB_C3_SEL_W<4> {
        USB_C3_SEL_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn usb_clk_en(&mut self) -> USB_CLK_EN_W<8> {
        USB_CLK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_clock](index.html) module"]
pub struct USB_CLOCK_SPEC;
impl crate::RegisterSpec for USB_CLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb_clock::R](R) reader structure"]
impl crate::Readable for USB_CLOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb_clock::W](W) writer structure"]
impl crate::Writable for USB_CLOCK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USB_CLOCK to value 0"]
impl crate::Resettable for USB_CLOCK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
