#[doc = "Register `PER_CLOCK` reader"]
pub struct R(crate::R<PER_CLOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PER_CLOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PER_CLOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PER_CLOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PER_CLOCK` writer"]
pub struct W(crate::W<PER_CLOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PER_CLOCK_SPEC>;
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
impl From<crate::W<PER_CLOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PER_CLOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCLK_CAN1` reader - "]
pub type PCLK_CAN1_R = crate::BitReader<bool>;
#[doc = "Field `PCLK_CAN1` writer - "]
pub type PCLK_CAN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PER_CLOCK_SPEC, bool, O>;
#[doc = "Field `PCLK_CAN2` reader - "]
pub type PCLK_CAN2_R = crate::BitReader<bool>;
#[doc = "Field `PCLK_CAN2` writer - "]
pub type PCLK_CAN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PER_CLOCK_SPEC, bool, O>;
#[doc = "Field `PCLK_USB` reader - "]
pub type PCLK_USB_R = crate::BitReader<bool>;
#[doc = "Field `PCLK_USB` writer - "]
pub type PCLK_USB_W<'a, const O: u8> = crate::BitWriter<'a, u32, PER_CLOCK_SPEC, bool, O>;
#[doc = "Field `PCLK_EEPROM` reader - "]
pub type PCLK_EEPROM_R = crate::BitReader<bool>;
#[doc = "Field `PCLK_EEPROM` writer - "]
pub type PCLK_EEPROM_W<'a, const O: u8> = crate::BitWriter<'a, u32, PER_CLOCK_SPEC, bool, O>;
#[doc = "Field `PCLK_RST_CLK` reader - "]
pub type PCLK_RST_CLK_R = crate::BitReader<bool>;
#[doc = "Field `PCLK_RST_CLK` writer - "]
pub type PCLK_RST_CLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PER_CLOCK_SPEC, bool, O>;
#[doc = "Field `PCLK_DMA` reader - "]
pub type PCLK_DMA_R = crate::BitReader<bool>;
#[doc = "Field `PCLK_DMA` writer - "]
pub type PCLK_DMA_W<'a, const O: u8> = crate::BitWriter<'a, u32, PER_CLOCK_SPEC, bool, O>;
#[doc = "Field `PCLK_UART1` reader - "]
pub type PCLK_UART1_R = crate::BitReader<bool>;
#[doc = "Field `PCLK_UART1` writer - "]
pub type PCLK_UART1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PER_CLOCK_SPEC, bool, O>;
#[doc = "Field `PCLK_UART2` reader - "]
pub type PCLK_UART2_R = crate::BitReader<bool>;
#[doc = "Field `PCLK_UART2` writer - "]
pub type PCLK_UART2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PER_CLOCK_SPEC, bool, O>;
#[doc = "Field `PCLK_SPI1` reader - "]
pub type PCLK_SPI1_R = crate::BitReader<bool>;
#[doc = "Field `PCLK_SPI1` writer - "]
pub type PCLK_SPI1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PER_CLOCK_SPEC, bool, O>;
#[doc = "Field `PCLK_I2C` reader - "]
pub type PCLK_I2C_R = crate::BitReader<bool>;
#[doc = "Field `PCLK_I2C` writer - "]
pub type PCLK_I2C_W<'a, const O: u8> = crate::BitWriter<'a, u32, PER_CLOCK_SPEC, bool, O>;
#[doc = "Field `PCLK_POWER` reader - "]
pub type PCLK_POWER_R = crate::BitReader<bool>;
#[doc = "Field `PCLK_POWER` writer - "]
pub type PCLK_POWER_W<'a, const O: u8> = crate::BitWriter<'a, u32, PER_CLOCK_SPEC, bool, O>;
#[doc = "Field `PCLK_WWDG` reader - "]
pub type PCLK_WWDG_R = crate::BitReader<bool>;
#[doc = "Field `PCLK_WWDG` writer - "]
pub type PCLK_WWDG_W<'a, const O: u8> = crate::BitWriter<'a, u32, PER_CLOCK_SPEC, bool, O>;
#[doc = "Field `PCLK_IWDG` reader - "]
pub type PCLK_IWDG_R = crate::BitReader<bool>;
#[doc = "Field `PCLK_IWDG` writer - "]
pub type PCLK_IWDG_W<'a, const O: u8> = crate::BitWriter<'a, u32, PER_CLOCK_SPEC, bool, O>;
#[doc = "Field `PCLK_TIMER1` reader - "]
pub type PCLK_TIMER1_R = crate::BitReader<bool>;
#[doc = "Field `PCLK_TIMER1` writer - "]
pub type PCLK_TIMER1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PER_CLOCK_SPEC, bool, O>;
#[doc = "Field `PCLK_TIMER2` reader - "]
pub type PCLK_TIMER2_R = crate::BitReader<bool>;
#[doc = "Field `PCLK_TIMER2` writer - "]
pub type PCLK_TIMER2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PER_CLOCK_SPEC, bool, O>;
#[doc = "Field `PCLK_TIMER3` reader - "]
pub type PCLK_TIMER3_R = crate::BitReader<bool>;
#[doc = "Field `PCLK_TIMER3` writer - "]
pub type PCLK_TIMER3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PER_CLOCK_SPEC, bool, O>;
#[doc = "Field `PCLK_ADC` reader - "]
pub type PCLK_ADC_R = crate::BitReader<bool>;
#[doc = "Field `PCLK_ADC` writer - "]
pub type PCLK_ADC_W<'a, const O: u8> = crate::BitWriter<'a, u32, PER_CLOCK_SPEC, bool, O>;
#[doc = "Field `PCLK_DAC` reader - "]
pub type PCLK_DAC_R = crate::BitReader<bool>;
#[doc = "Field `PCLK_DAC` writer - "]
pub type PCLK_DAC_W<'a, const O: u8> = crate::BitWriter<'a, u32, PER_CLOCK_SPEC, bool, O>;
#[doc = "Field `PCLK_COMP` reader - "]
pub type PCLK_COMP_R = crate::BitReader<bool>;
#[doc = "Field `PCLK_COMP` writer - "]
pub type PCLK_COMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PER_CLOCK_SPEC, bool, O>;
#[doc = "Field `PCLK_SPI2` reader - "]
pub type PCLK_SPI2_R = crate::BitReader<bool>;
#[doc = "Field `PCLK_SPI2` writer - "]
pub type PCLK_SPI2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PER_CLOCK_SPEC, bool, O>;
#[doc = "Field `PCLK_PORTA` reader - "]
pub type PCLK_PORTA_R = crate::BitReader<bool>;
#[doc = "Field `PCLK_PORTA` writer - "]
pub type PCLK_PORTA_W<'a, const O: u8> = crate::BitWriter<'a, u32, PER_CLOCK_SPEC, bool, O>;
#[doc = "Field `PCLK_PORTB` reader - "]
pub type PCLK_PORTB_R = crate::BitReader<bool>;
#[doc = "Field `PCLK_PORTB` writer - "]
pub type PCLK_PORTB_W<'a, const O: u8> = crate::BitWriter<'a, u32, PER_CLOCK_SPEC, bool, O>;
#[doc = "Field `PCLK_PORTC` reader - "]
pub type PCLK_PORTC_R = crate::BitReader<bool>;
#[doc = "Field `PCLK_PORTC` writer - "]
pub type PCLK_PORTC_W<'a, const O: u8> = crate::BitWriter<'a, u32, PER_CLOCK_SPEC, bool, O>;
#[doc = "Field `PCLK_PORTD` reader - "]
pub type PCLK_PORTD_R = crate::BitReader<bool>;
#[doc = "Field `PCLK_PORTD` writer - "]
pub type PCLK_PORTD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PER_CLOCK_SPEC, bool, O>;
#[doc = "Field `PCLK_PORTE` reader - "]
pub type PCLK_PORTE_R = crate::BitReader<bool>;
#[doc = "Field `PCLK_PORTE` writer - "]
pub type PCLK_PORTE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PER_CLOCK_SPEC, bool, O>;
#[doc = "Field `PCLK_BKP` reader - "]
pub type PCLK_BKP_R = crate::BitReader<bool>;
#[doc = "Field `PCLK_BKP` writer - "]
pub type PCLK_BKP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PER_CLOCK_SPEC, bool, O>;
#[doc = "Field `PCLK_PORTF` reader - "]
pub type PCLK_PORTF_R = crate::BitReader<bool>;
#[doc = "Field `PCLK_PORTF` writer - "]
pub type PCLK_PORTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, PER_CLOCK_SPEC, bool, O>;
#[doc = "Field `PCLK_EXT_BUS` reader - "]
pub type PCLK_EXT_BUS_R = crate::BitReader<bool>;
#[doc = "Field `PCLK_EXT_BUS` writer - "]
pub type PCLK_EXT_BUS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PER_CLOCK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pclk_can1(&self) -> PCLK_CAN1_R {
        PCLK_CAN1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pclk_can2(&self) -> PCLK_CAN2_R {
        PCLK_CAN2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pclk_usb(&self) -> PCLK_USB_R {
        PCLK_USB_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pclk_eeprom(&self) -> PCLK_EEPROM_R {
        PCLK_EEPROM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pclk_rst_clk(&self) -> PCLK_RST_CLK_R {
        PCLK_RST_CLK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pclk_dma(&self) -> PCLK_DMA_R {
        PCLK_DMA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn pclk_uart1(&self) -> PCLK_UART1_R {
        PCLK_UART1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pclk_uart2(&self) -> PCLK_UART2_R {
        PCLK_UART2_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pclk_spi1(&self) -> PCLK_SPI1_R {
        PCLK_SPI1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pclk_i2c(&self) -> PCLK_I2C_R {
        PCLK_I2C_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pclk_power(&self) -> PCLK_POWER_R {
        PCLK_POWER_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn pclk_wwdg(&self) -> PCLK_WWDG_R {
        PCLK_WWDG_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn pclk_iwdg(&self) -> PCLK_IWDG_R {
        PCLK_IWDG_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn pclk_timer1(&self) -> PCLK_TIMER1_R {
        PCLK_TIMER1_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn pclk_timer2(&self) -> PCLK_TIMER2_R {
        PCLK_TIMER2_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pclk_timer3(&self) -> PCLK_TIMER3_R {
        PCLK_TIMER3_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pclk_adc(&self) -> PCLK_ADC_R {
        PCLK_ADC_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn pclk_dac(&self) -> PCLK_DAC_R {
        PCLK_DAC_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn pclk_comp(&self) -> PCLK_COMP_R {
        PCLK_COMP_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn pclk_spi2(&self) -> PCLK_SPI2_R {
        PCLK_SPI2_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn pclk_porta(&self) -> PCLK_PORTA_R {
        PCLK_PORTA_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pclk_portb(&self) -> PCLK_PORTB_R {
        PCLK_PORTB_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn pclk_portc(&self) -> PCLK_PORTC_R {
        PCLK_PORTC_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn pclk_portd(&self) -> PCLK_PORTD_R {
        PCLK_PORTD_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn pclk_porte(&self) -> PCLK_PORTE_R {
        PCLK_PORTE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn pclk_bkp(&self) -> PCLK_BKP_R {
        PCLK_BKP_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn pclk_portf(&self) -> PCLK_PORTF_R {
        PCLK_PORTF_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn pclk_ext_bus(&self) -> PCLK_EXT_BUS_R {
        PCLK_EXT_BUS_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_can1(&mut self) -> PCLK_CAN1_W<0> {
        PCLK_CAN1_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_can2(&mut self) -> PCLK_CAN2_W<1> {
        PCLK_CAN2_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_usb(&mut self) -> PCLK_USB_W<2> {
        PCLK_USB_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_eeprom(&mut self) -> PCLK_EEPROM_W<3> {
        PCLK_EEPROM_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_rst_clk(&mut self) -> PCLK_RST_CLK_W<4> {
        PCLK_RST_CLK_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_dma(&mut self) -> PCLK_DMA_W<5> {
        PCLK_DMA_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_uart1(&mut self) -> PCLK_UART1_W<6> {
        PCLK_UART1_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_uart2(&mut self) -> PCLK_UART2_W<7> {
        PCLK_UART2_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_spi1(&mut self) -> PCLK_SPI1_W<8> {
        PCLK_SPI1_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_i2c(&mut self) -> PCLK_I2C_W<10> {
        PCLK_I2C_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_power(&mut self) -> PCLK_POWER_W<11> {
        PCLK_POWER_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_wwdg(&mut self) -> PCLK_WWDG_W<12> {
        PCLK_WWDG_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_iwdg(&mut self) -> PCLK_IWDG_W<13> {
        PCLK_IWDG_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_timer1(&mut self) -> PCLK_TIMER1_W<14> {
        PCLK_TIMER1_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_timer2(&mut self) -> PCLK_TIMER2_W<15> {
        PCLK_TIMER2_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_timer3(&mut self) -> PCLK_TIMER3_W<16> {
        PCLK_TIMER3_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_adc(&mut self) -> PCLK_ADC_W<17> {
        PCLK_ADC_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_dac(&mut self) -> PCLK_DAC_W<18> {
        PCLK_DAC_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_comp(&mut self) -> PCLK_COMP_W<19> {
        PCLK_COMP_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_spi2(&mut self) -> PCLK_SPI2_W<20> {
        PCLK_SPI2_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_porta(&mut self) -> PCLK_PORTA_W<21> {
        PCLK_PORTA_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_portb(&mut self) -> PCLK_PORTB_W<22> {
        PCLK_PORTB_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_portc(&mut self) -> PCLK_PORTC_W<23> {
        PCLK_PORTC_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_portd(&mut self) -> PCLK_PORTD_W<24> {
        PCLK_PORTD_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_porte(&mut self) -> PCLK_PORTE_W<25> {
        PCLK_PORTE_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_bkp(&mut self) -> PCLK_BKP_W<27> {
        PCLK_BKP_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_portf(&mut self) -> PCLK_PORTF_W<29> {
        PCLK_PORTF_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_ext_bus(&mut self) -> PCLK_EXT_BUS_W<30> {
        PCLK_EXT_BUS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral Clock Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [per_clock](index.html) module"]
pub struct PER_CLOCK_SPEC;
impl crate::RegisterSpec for PER_CLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [per_clock::R](R) reader structure"]
impl crate::Readable for PER_CLOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [per_clock::W](W) writer structure"]
impl crate::Writable for PER_CLOCK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PER_CLOCK to value 0x10"]
impl crate::Resettable for PER_CLOCK_SPEC {
    const RESET_VALUE: Self::Ux = 0x10;
}
