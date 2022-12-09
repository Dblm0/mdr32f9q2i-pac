#[doc = "Register `BUF_00_DATAH` reader"]
pub struct R(crate::R<BUF_00_DATAH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUF_00_DATAH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUF_00_DATAH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUF_00_DATAH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BUF_00_DATAH` writer"]
pub struct W(crate::W<BUF_00_DATAH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUF_00_DATAH_SPEC>;
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
impl From<crate::W<BUF_00_DATAH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUF_00_DATAH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DB4` reader - "]
pub type DB4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DB4` writer - "]
pub type DB4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BUF_00_DATAH_SPEC, u8, u8, 8, O>;
#[doc = "Field `DB5` reader - "]
pub type DB5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DB5` writer - "]
pub type DB5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BUF_00_DATAH_SPEC, u8, u8, 8, O>;
#[doc = "Field `DB6` reader - "]
pub type DB6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DB6` writer - "]
pub type DB6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BUF_00_DATAH_SPEC, u8, u8, 8, O>;
#[doc = "Field `DB7` reader - "]
pub type DB7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DB7` writer - "]
pub type DB7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BUF_00_DATAH_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn db4(&self) -> DB4_R {
        DB4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn db5(&self) -> DB5_R {
        DB5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn db6(&self) -> DB6_R {
        DB6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn db7(&self) -> DB7_R {
        DB7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn db4(&mut self) -> DB4_W<0> {
        DB4_W::new(self)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn db5(&mut self) -> DB5_W<8> {
        DB5_W::new(self)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn db6(&mut self) -> DB6_W<16> {
        DB6_W::new(self)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn db7(&mut self) -> DB7_W<24> {
        DB7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN Buffer Data high Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buf_00_datah](index.html) module"]
pub struct BUF_00_DATAH_SPEC;
impl crate::RegisterSpec for BUF_00_DATAH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [buf_00_datah::R](R) reader structure"]
impl crate::Readable for BUF_00_DATAH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [buf_00_datah::W](W) writer structure"]
impl crate::Writable for BUF_00_DATAH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BUF_00_DATAH to value 0"]
impl crate::Resettable for BUF_00_DATAH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
