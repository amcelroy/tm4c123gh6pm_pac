# [doc = "Register `MIMR` reader"] pub struct R (crate :: R < MIMR_SPEC >) ; impl core :: ops :: Deref for R { type Target = crate :: R < MIMR_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl From < crate :: R < MIMR_SPEC > > for R { # [inline (always)] fn from (reader : crate :: R < MIMR_SPEC >) -> Self { R (reader) } } # [doc = "Register `MIMR` writer"] pub struct W (crate :: W < MIMR_SPEC >) ; impl core :: ops :: Deref for W { type Target = crate :: W < MIMR_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl core :: ops :: DerefMut for W { # [inline (always)] fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . 0 } } impl From < crate :: W < MIMR_SPEC > > for W { # [inline (always)] fn from (writer : crate :: W < MIMR_SPEC >) -> Self { W (writer) } } # [doc = "Field `I2C_MIMR_IM` reader - Master Interrupt Mask"] pub type I2C_MIMR_IM_R = crate :: BitReader < bool > ; # [doc = "Field `I2C_MIMR_IM` writer - Master Interrupt Mask"] pub type I2C_MIMR_IM_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , MIMR_SPEC , bool , O > ; # [doc = "Field `I2C_MIMR_CLKIM` reader - Clock Timeout Interrupt Mask"] pub type I2C_MIMR_CLKIM_R = crate :: BitReader < bool > ; # [doc = "Field `I2C_MIMR_CLKIM` writer - Clock Timeout Interrupt Mask"] pub type I2C_MIMR_CLKIM_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , MIMR_SPEC , bool , O > ; impl R { # [doc = "Bit 0 - Master Interrupt Mask"] # [inline (always)] pub fn i2c_mimr_im (& self) -> I2C_MIMR_IM_R { I2C_MIMR_IM_R :: new ((self . bits & 1) != 0) } # [doc = "Bit 1 - Clock Timeout Interrupt Mask"] # [inline (always)] pub fn i2c_mimr_clkim (& self) -> I2C_MIMR_CLKIM_R { I2C_MIMR_CLKIM_R :: new (((self . bits >> 1) & 1) != 0) } } impl W { # [doc = "Bit 0 - Master Interrupt Mask"] # [inline (always)] # [must_use] pub fn i2c_mimr_im (& mut self) -> I2C_MIMR_IM_W < 0 > { I2C_MIMR_IM_W :: new (self) } # [doc = "Bit 1 - Clock Timeout Interrupt Mask"] # [inline (always)] # [must_use] pub fn i2c_mimr_clkim (& mut self) -> I2C_MIMR_CLKIM_W < 1 > { I2C_MIMR_CLKIM_W :: new (self) } # [doc = "Writes raw bits to the register."] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . 0 . bits (bits) ; self } } # [doc = "I2C Master Interrupt Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mimr](index.html) module"] pub struct MIMR_SPEC ; impl crate :: RegisterSpec for MIMR_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [mimr::R](R) reader structure"] impl crate :: Readable for MIMR_SPEC { type Reader = R ; } # [doc = "`write(|w| ..)` method takes [mimr::W](W) writer structure"] impl crate :: Writable for MIMR_SPEC { type Writer = W ; const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets MIMR to value 0"] impl crate :: Resettable for MIMR_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }