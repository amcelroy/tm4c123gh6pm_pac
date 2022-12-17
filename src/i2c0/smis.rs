# [doc = "Register `SMIS` reader"] pub struct R (crate :: R < SMIS_SPEC >) ; impl core :: ops :: Deref for R { type Target = crate :: R < SMIS_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl From < crate :: R < SMIS_SPEC > > for R { # [inline (always)] fn from (reader : crate :: R < SMIS_SPEC >) -> Self { R (reader) } } # [doc = "Register `SMIS` writer"] pub struct W (crate :: W < SMIS_SPEC >) ; impl core :: ops :: Deref for W { type Target = crate :: W < SMIS_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl core :: ops :: DerefMut for W { # [inline (always)] fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . 0 } } impl From < crate :: W < SMIS_SPEC > > for W { # [inline (always)] fn from (writer : crate :: W < SMIS_SPEC >) -> Self { W (writer) } } # [doc = "Field `I2C_SMIS_DATAMIS` reader - Data Masked Interrupt Status"] pub type I2C_SMIS_DATAMIS_R = crate :: BitReader < bool > ; # [doc = "Field `I2C_SMIS_DATAMIS` writer - Data Masked Interrupt Status"] pub type I2C_SMIS_DATAMIS_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , SMIS_SPEC , bool , O > ; # [doc = "Field `I2C_SMIS_STARTMIS` reader - Start Condition Masked Interrupt Status"] pub type I2C_SMIS_STARTMIS_R = crate :: BitReader < bool > ; # [doc = "Field `I2C_SMIS_STARTMIS` writer - Start Condition Masked Interrupt Status"] pub type I2C_SMIS_STARTMIS_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , SMIS_SPEC , bool , O > ; # [doc = "Field `I2C_SMIS_STOPMIS` reader - Stop Condition Masked Interrupt Status"] pub type I2C_SMIS_STOPMIS_R = crate :: BitReader < bool > ; # [doc = "Field `I2C_SMIS_STOPMIS` writer - Stop Condition Masked Interrupt Status"] pub type I2C_SMIS_STOPMIS_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , SMIS_SPEC , bool , O > ; impl R { # [doc = "Bit 0 - Data Masked Interrupt Status"] # [inline (always)] pub fn i2c_smis_datamis (& self) -> I2C_SMIS_DATAMIS_R { I2C_SMIS_DATAMIS_R :: new ((self . bits & 1) != 0) } # [doc = "Bit 1 - Start Condition Masked Interrupt Status"] # [inline (always)] pub fn i2c_smis_startmis (& self) -> I2C_SMIS_STARTMIS_R { I2C_SMIS_STARTMIS_R :: new (((self . bits >> 1) & 1) != 0) } # [doc = "Bit 2 - Stop Condition Masked Interrupt Status"] # [inline (always)] pub fn i2c_smis_stopmis (& self) -> I2C_SMIS_STOPMIS_R { I2C_SMIS_STOPMIS_R :: new (((self . bits >> 2) & 1) != 0) } } impl W { # [doc = "Bit 0 - Data Masked Interrupt Status"] # [inline (always)] # [must_use] pub fn i2c_smis_datamis (& mut self) -> I2C_SMIS_DATAMIS_W < 0 > { I2C_SMIS_DATAMIS_W :: new (self) } # [doc = "Bit 1 - Start Condition Masked Interrupt Status"] # [inline (always)] # [must_use] pub fn i2c_smis_startmis (& mut self) -> I2C_SMIS_STARTMIS_W < 1 > { I2C_SMIS_STARTMIS_W :: new (self) } # [doc = "Bit 2 - Stop Condition Masked Interrupt Status"] # [inline (always)] # [must_use] pub fn i2c_smis_stopmis (& mut self) -> I2C_SMIS_STOPMIS_W < 2 > { I2C_SMIS_STOPMIS_W :: new (self) } # [doc = "Writes raw bits to the register."] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . 0 . bits (bits) ; self } } # [doc = "I2C Slave Masked Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smis](index.html) module"] pub struct SMIS_SPEC ; impl crate :: RegisterSpec for SMIS_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [smis::R](R) reader structure"] impl crate :: Readable for SMIS_SPEC { type Reader = R ; } # [doc = "`write(|w| ..)` method takes [smis::W](W) writer structure"] impl crate :: Writable for SMIS_SPEC { type Writer = W ; const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets SMIS to value 0"] impl crate :: Resettable for SMIS_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }