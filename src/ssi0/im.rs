# [doc = "Register `IM` reader"] pub struct R (crate :: R < IM_SPEC >) ; impl core :: ops :: Deref for R { type Target = crate :: R < IM_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl From < crate :: R < IM_SPEC > > for R { # [inline (always)] fn from (reader : crate :: R < IM_SPEC >) -> Self { R (reader) } } # [doc = "Register `IM` writer"] pub struct W (crate :: W < IM_SPEC >) ; impl core :: ops :: Deref for W { type Target = crate :: W < IM_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl core :: ops :: DerefMut for W { # [inline (always)] fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . 0 } } impl From < crate :: W < IM_SPEC > > for W { # [inline (always)] fn from (writer : crate :: W < IM_SPEC >) -> Self { W (writer) } } # [doc = "Field `SSI_IM_RORIM` reader - SSI Receive Overrun Interrupt Mask"] pub type SSI_IM_RORIM_R = crate :: BitReader < bool > ; # [doc = "Field `SSI_IM_RORIM` writer - SSI Receive Overrun Interrupt Mask"] pub type SSI_IM_RORIM_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , IM_SPEC , bool , O > ; # [doc = "Field `SSI_IM_RTIM` reader - SSI Receive Time-Out Interrupt Mask"] pub type SSI_IM_RTIM_R = crate :: BitReader < bool > ; # [doc = "Field `SSI_IM_RTIM` writer - SSI Receive Time-Out Interrupt Mask"] pub type SSI_IM_RTIM_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , IM_SPEC , bool , O > ; # [doc = "Field `SSI_IM_RXIM` reader - SSI Receive FIFO Interrupt Mask"] pub type SSI_IM_RXIM_R = crate :: BitReader < bool > ; # [doc = "Field `SSI_IM_RXIM` writer - SSI Receive FIFO Interrupt Mask"] pub type SSI_IM_RXIM_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , IM_SPEC , bool , O > ; # [doc = "Field `SSI_IM_TXIM` reader - SSI Transmit FIFO Interrupt Mask"] pub type SSI_IM_TXIM_R = crate :: BitReader < bool > ; # [doc = "Field `SSI_IM_TXIM` writer - SSI Transmit FIFO Interrupt Mask"] pub type SSI_IM_TXIM_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , IM_SPEC , bool , O > ; impl R { # [doc = "Bit 0 - SSI Receive Overrun Interrupt Mask"] # [inline (always)] pub fn ssi_im_rorim (& self) -> SSI_IM_RORIM_R { SSI_IM_RORIM_R :: new ((self . bits & 1) != 0) } # [doc = "Bit 1 - SSI Receive Time-Out Interrupt Mask"] # [inline (always)] pub fn ssi_im_rtim (& self) -> SSI_IM_RTIM_R { SSI_IM_RTIM_R :: new (((self . bits >> 1) & 1) != 0) } # [doc = "Bit 2 - SSI Receive FIFO Interrupt Mask"] # [inline (always)] pub fn ssi_im_rxim (& self) -> SSI_IM_RXIM_R { SSI_IM_RXIM_R :: new (((self . bits >> 2) & 1) != 0) } # [doc = "Bit 3 - SSI Transmit FIFO Interrupt Mask"] # [inline (always)] pub fn ssi_im_txim (& self) -> SSI_IM_TXIM_R { SSI_IM_TXIM_R :: new (((self . bits >> 3) & 1) != 0) } } impl W { # [doc = "Bit 0 - SSI Receive Overrun Interrupt Mask"] # [inline (always)] # [must_use] pub fn ssi_im_rorim (& mut self) -> SSI_IM_RORIM_W < 0 > { SSI_IM_RORIM_W :: new (self) } # [doc = "Bit 1 - SSI Receive Time-Out Interrupt Mask"] # [inline (always)] # [must_use] pub fn ssi_im_rtim (& mut self) -> SSI_IM_RTIM_W < 1 > { SSI_IM_RTIM_W :: new (self) } # [doc = "Bit 2 - SSI Receive FIFO Interrupt Mask"] # [inline (always)] # [must_use] pub fn ssi_im_rxim (& mut self) -> SSI_IM_RXIM_W < 2 > { SSI_IM_RXIM_W :: new (self) } # [doc = "Bit 3 - SSI Transmit FIFO Interrupt Mask"] # [inline (always)] # [must_use] pub fn ssi_im_txim (& mut self) -> SSI_IM_TXIM_W < 3 > { SSI_IM_TXIM_W :: new (self) } # [doc = "Writes raw bits to the register."] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . 0 . bits (bits) ; self } } # [doc = "SSI Interrupt Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [im](index.html) module"] pub struct IM_SPEC ; impl crate :: RegisterSpec for IM_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [im::R](R) reader structure"] impl crate :: Readable for IM_SPEC { type Reader = R ; } # [doc = "`write(|w| ..)` method takes [im::W](W) writer structure"] impl crate :: Writable for IM_SPEC { type Writer = W ; const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets IM to value 0"] impl crate :: Resettable for IM_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }