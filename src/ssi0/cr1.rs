# [doc = "Register `CR1` reader"] pub struct R (crate :: R < CR1_SPEC >) ; impl core :: ops :: Deref for R { type Target = crate :: R < CR1_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl From < crate :: R < CR1_SPEC > > for R { # [inline (always)] fn from (reader : crate :: R < CR1_SPEC >) -> Self { R (reader) } } # [doc = "Register `CR1` writer"] pub struct W (crate :: W < CR1_SPEC >) ; impl core :: ops :: Deref for W { type Target = crate :: W < CR1_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl core :: ops :: DerefMut for W { # [inline (always)] fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . 0 } } impl From < crate :: W < CR1_SPEC > > for W { # [inline (always)] fn from (writer : crate :: W < CR1_SPEC >) -> Self { W (writer) } } # [doc = "Field `SSI_CR1_LBM` reader - SSI Loopback Mode"] pub type SSI_CR1_LBM_R = crate :: BitReader < bool > ; # [doc = "Field `SSI_CR1_LBM` writer - SSI Loopback Mode"] pub type SSI_CR1_LBM_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , CR1_SPEC , bool , O > ; # [doc = "Field `SSI_CR1_SSE` reader - SSI Synchronous Serial Port Enable"] pub type SSI_CR1_SSE_R = crate :: BitReader < bool > ; # [doc = "Field `SSI_CR1_SSE` writer - SSI Synchronous Serial Port Enable"] pub type SSI_CR1_SSE_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , CR1_SPEC , bool , O > ; # [doc = "Field `SSI_CR1_MS` reader - SSI Master/Slave Select"] pub type SSI_CR1_MS_R = crate :: BitReader < bool > ; # [doc = "Field `SSI_CR1_MS` writer - SSI Master/Slave Select"] pub type SSI_CR1_MS_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , CR1_SPEC , bool , O > ; # [doc = "Field `SSI_CR1_EOT` reader - End of Transmission"] pub type SSI_CR1_EOT_R = crate :: BitReader < bool > ; # [doc = "Field `SSI_CR1_EOT` writer - End of Transmission"] pub type SSI_CR1_EOT_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , CR1_SPEC , bool , O > ; impl R { # [doc = "Bit 0 - SSI Loopback Mode"] # [inline (always)] pub fn ssi_cr1_lbm (& self) -> SSI_CR1_LBM_R { SSI_CR1_LBM_R :: new ((self . bits & 1) != 0) } # [doc = "Bit 1 - SSI Synchronous Serial Port Enable"] # [inline (always)] pub fn ssi_cr1_sse (& self) -> SSI_CR1_SSE_R { SSI_CR1_SSE_R :: new (((self . bits >> 1) & 1) != 0) } # [doc = "Bit 2 - SSI Master/Slave Select"] # [inline (always)] pub fn ssi_cr1_ms (& self) -> SSI_CR1_MS_R { SSI_CR1_MS_R :: new (((self . bits >> 2) & 1) != 0) } # [doc = "Bit 4 - End of Transmission"] # [inline (always)] pub fn ssi_cr1_eot (& self) -> SSI_CR1_EOT_R { SSI_CR1_EOT_R :: new (((self . bits >> 4) & 1) != 0) } } impl W { # [doc = "Bit 0 - SSI Loopback Mode"] # [inline (always)] # [must_use] pub fn ssi_cr1_lbm (& mut self) -> SSI_CR1_LBM_W < 0 > { SSI_CR1_LBM_W :: new (self) } # [doc = "Bit 1 - SSI Synchronous Serial Port Enable"] # [inline (always)] # [must_use] pub fn ssi_cr1_sse (& mut self) -> SSI_CR1_SSE_W < 1 > { SSI_CR1_SSE_W :: new (self) } # [doc = "Bit 2 - SSI Master/Slave Select"] # [inline (always)] # [must_use] pub fn ssi_cr1_ms (& mut self) -> SSI_CR1_MS_W < 2 > { SSI_CR1_MS_W :: new (self) } # [doc = "Bit 4 - End of Transmission"] # [inline (always)] # [must_use] pub fn ssi_cr1_eot (& mut self) -> SSI_CR1_EOT_W < 4 > { SSI_CR1_EOT_W :: new (self) } # [doc = "Writes raw bits to the register."] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . 0 . bits (bits) ; self } } # [doc = "SSI Control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr1](index.html) module"] pub struct CR1_SPEC ; impl crate :: RegisterSpec for CR1_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [cr1::R](R) reader structure"] impl crate :: Readable for CR1_SPEC { type Reader = R ; } # [doc = "`write(|w| ..)` method takes [cr1::W](W) writer structure"] impl crate :: Writable for CR1_SPEC { type Writer = W ; const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets CR1 to value 0"] impl crate :: Resettable for CR1_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }