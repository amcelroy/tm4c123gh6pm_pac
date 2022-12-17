# [doc = "Register `POWER` reader"] pub struct R (crate :: R < POWER_SPEC >) ; impl core :: ops :: Deref for R { type Target = crate :: R < POWER_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl From < crate :: R < POWER_SPEC > > for R { # [inline (always)] fn from (reader : crate :: R < POWER_SPEC >) -> Self { R (reader) } } # [doc = "Register `POWER` writer"] pub struct W (crate :: W < POWER_SPEC >) ; impl core :: ops :: Deref for W { type Target = crate :: W < POWER_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl core :: ops :: DerefMut for W { # [inline (always)] fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . 0 } } impl From < crate :: W < POWER_SPEC > > for W { # [inline (always)] fn from (writer : crate :: W < POWER_SPEC >) -> Self { W (writer) } } # [doc = "Field `USB_POWER_PWRDNPHY` reader - Power Down PHY"] pub type USB_POWER_PWRDNPHY_R = crate :: BitReader < bool > ; # [doc = "Field `USB_POWER_PWRDNPHY` writer - Power Down PHY"] pub type USB_POWER_PWRDNPHY_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u8 , POWER_SPEC , bool , O > ; # [doc = "Field `USB_POWER_SUSPEND` reader - SUSPEND Mode"] pub type USB_POWER_SUSPEND_R = crate :: BitReader < bool > ; # [doc = "Field `USB_POWER_SUSPEND` writer - SUSPEND Mode"] pub type USB_POWER_SUSPEND_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u8 , POWER_SPEC , bool , O > ; # [doc = "Field `USB_POWER_RESUME` reader - RESUME Signaling"] pub type USB_POWER_RESUME_R = crate :: BitReader < bool > ; # [doc = "Field `USB_POWER_RESUME` writer - RESUME Signaling"] pub type USB_POWER_RESUME_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u8 , POWER_SPEC , bool , O > ; # [doc = "Field `USB_POWER_RESET` reader - RESET Signaling"] pub type USB_POWER_RESET_R = crate :: BitReader < bool > ; # [doc = "Field `USB_POWER_RESET` writer - RESET Signaling"] pub type USB_POWER_RESET_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u8 , POWER_SPEC , bool , O > ; # [doc = "Field `USB_POWER_SOFTCONN` reader - Soft Connect/Disconnect"] pub type USB_POWER_SOFTCONN_R = crate :: BitReader < bool > ; # [doc = "Field `USB_POWER_SOFTCONN` writer - Soft Connect/Disconnect"] pub type USB_POWER_SOFTCONN_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u8 , POWER_SPEC , bool , O > ; # [doc = "Field `USB_POWER_ISOUP` reader - Isochronous Update"] pub type USB_POWER_ISOUP_R = crate :: BitReader < bool > ; # [doc = "Field `USB_POWER_ISOUP` writer - Isochronous Update"] pub type USB_POWER_ISOUP_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u8 , POWER_SPEC , bool , O > ; impl R { # [doc = "Bit 0 - Power Down PHY"] # [inline (always)] pub fn usb_power_pwrdnphy (& self) -> USB_POWER_PWRDNPHY_R { USB_POWER_PWRDNPHY_R :: new ((self . bits & 1) != 0) } # [doc = "Bit 1 - SUSPEND Mode"] # [inline (always)] pub fn usb_power_suspend (& self) -> USB_POWER_SUSPEND_R { USB_POWER_SUSPEND_R :: new (((self . bits >> 1) & 1) != 0) } # [doc = "Bit 2 - RESUME Signaling"] # [inline (always)] pub fn usb_power_resume (& self) -> USB_POWER_RESUME_R { USB_POWER_RESUME_R :: new (((self . bits >> 2) & 1) != 0) } # [doc = "Bit 3 - RESET Signaling"] # [inline (always)] pub fn usb_power_reset (& self) -> USB_POWER_RESET_R { USB_POWER_RESET_R :: new (((self . bits >> 3) & 1) != 0) } # [doc = "Bit 6 - Soft Connect/Disconnect"] # [inline (always)] pub fn usb_power_softconn (& self) -> USB_POWER_SOFTCONN_R { USB_POWER_SOFTCONN_R :: new (((self . bits >> 6) & 1) != 0) } # [doc = "Bit 7 - Isochronous Update"] # [inline (always)] pub fn usb_power_isoup (& self) -> USB_POWER_ISOUP_R { USB_POWER_ISOUP_R :: new (((self . bits >> 7) & 1) != 0) } } impl W { # [doc = "Bit 0 - Power Down PHY"] # [inline (always)] # [must_use] pub fn usb_power_pwrdnphy (& mut self) -> USB_POWER_PWRDNPHY_W < 0 > { USB_POWER_PWRDNPHY_W :: new (self) } # [doc = "Bit 1 - SUSPEND Mode"] # [inline (always)] # [must_use] pub fn usb_power_suspend (& mut self) -> USB_POWER_SUSPEND_W < 1 > { USB_POWER_SUSPEND_W :: new (self) } # [doc = "Bit 2 - RESUME Signaling"] # [inline (always)] # [must_use] pub fn usb_power_resume (& mut self) -> USB_POWER_RESUME_W < 2 > { USB_POWER_RESUME_W :: new (self) } # [doc = "Bit 3 - RESET Signaling"] # [inline (always)] # [must_use] pub fn usb_power_reset (& mut self) -> USB_POWER_RESET_W < 3 > { USB_POWER_RESET_W :: new (self) } # [doc = "Bit 6 - Soft Connect/Disconnect"] # [inline (always)] # [must_use] pub fn usb_power_softconn (& mut self) -> USB_POWER_SOFTCONN_W < 6 > { USB_POWER_SOFTCONN_W :: new (self) } # [doc = "Bit 7 - Isochronous Update"] # [inline (always)] # [must_use] pub fn usb_power_isoup (& mut self) -> USB_POWER_ISOUP_W < 7 > { USB_POWER_ISOUP_W :: new (self) } # [doc = "Writes raw bits to the register."] # [inline (always)] pub unsafe fn bits (& mut self , bits : u8) -> & mut Self { self . 0 . bits (bits) ; self } } # [doc = "USB Power\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [power](index.html) module"] pub struct POWER_SPEC ; impl crate :: RegisterSpec for POWER_SPEC { type Ux = u8 ; } # [doc = "`read()` method returns [power::R](R) reader structure"] impl crate :: Readable for POWER_SPEC { type Reader = R ; } # [doc = "`write(|w| ..)` method takes [power::W](W) writer structure"] impl crate :: Writable for POWER_SPEC { type Writer = W ; const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets POWER to value 0"] impl crate :: Resettable for POWER_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }