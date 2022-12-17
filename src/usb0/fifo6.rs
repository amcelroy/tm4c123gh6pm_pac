# [doc = "Register `FIFO6` reader"] pub struct R (crate :: R < FIFO6_SPEC >) ; impl core :: ops :: Deref for R { type Target = crate :: R < FIFO6_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl From < crate :: R < FIFO6_SPEC > > for R { # [inline (always)] fn from (reader : crate :: R < FIFO6_SPEC >) -> Self { R (reader) } } # [doc = "Register `FIFO6` writer"] pub struct W (crate :: W < FIFO6_SPEC >) ; impl core :: ops :: Deref for W { type Target = crate :: W < FIFO6_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl core :: ops :: DerefMut for W { # [inline (always)] fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . 0 } } impl From < crate :: W < FIFO6_SPEC > > for W { # [inline (always)] fn from (writer : crate :: W < FIFO6_SPEC >) -> Self { W (writer) } } # [doc = "Field `USB_FIFO6_EPDATA` reader - Endpoint Data"] pub type USB_FIFO6_EPDATA_R = crate :: FieldReader < u32 , u32 > ; # [doc = "Field `USB_FIFO6_EPDATA` writer - Endpoint Data"] pub type USB_FIFO6_EPDATA_W < 'a , const O : u8 > = crate :: FieldWriter < 'a , u32 , FIFO6_SPEC , u32 , u32 , 32 , O > ; impl R { # [doc = "Bits 0:31 - Endpoint Data"] # [inline (always)] pub fn usb_fifo6_epdata (& self) -> USB_FIFO6_EPDATA_R { USB_FIFO6_EPDATA_R :: new (self . bits) } } impl W { # [doc = "Bits 0:31 - Endpoint Data"] # [inline (always)] # [must_use] pub fn usb_fifo6_epdata (& mut self) -> USB_FIFO6_EPDATA_W < 0 > { USB_FIFO6_EPDATA_W :: new (self) } # [doc = "Writes raw bits to the register."] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . 0 . bits (bits) ; self } } # [doc = "USB FIFO Endpoint 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo6](index.html) module"] pub struct FIFO6_SPEC ; impl crate :: RegisterSpec for FIFO6_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [fifo6::R](R) reader structure"] impl crate :: Readable for FIFO6_SPEC { type Reader = R ; } # [doc = "`write(|w| ..)` method takes [fifo6::W](W) writer structure"] impl crate :: Writable for FIFO6_SPEC { type Writer = W ; const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets FIFO6 to value 0"] impl crate :: Resettable for FIFO6_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }