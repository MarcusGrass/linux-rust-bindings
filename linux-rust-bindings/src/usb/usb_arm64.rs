/* automatically generated by rust-bindgen 0.65.1 */

#[repr(C)]
#[derive(Default)]
pub struct __IncompleteArrayField<T>(::core::marker::PhantomData<T>, [T; 0]);
impl<T> __IncompleteArrayField<T> {
    #[inline]
    pub const fn new() -> Self {
        __IncompleteArrayField(::core::marker::PhantomData, [])
    }
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self as *const _ as *const T
    }
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self as *mut _ as *mut T
    }
    #[inline]
    pub unsafe fn as_slice(&self, len: usize) -> &[T] {
        ::core::slice::from_raw_parts(self.as_ptr(), len)
    }
    #[inline]
    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
        ::core::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
    }
}
impl<T> ::core::fmt::Debug for __IncompleteArrayField<T> {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.write_str("__IncompleteArrayField")
    }
}
#[repr(C)]
pub struct __BindgenUnionField<T>(::core::marker::PhantomData<T>);
impl<T> __BindgenUnionField<T> {
    #[inline]
    pub const fn new() -> Self {
        __BindgenUnionField(::core::marker::PhantomData)
    }
    #[inline]
    pub unsafe fn as_ref(&self) -> &T {
        ::core::mem::transmute(self)
    }
    #[inline]
    pub unsafe fn as_mut(&mut self) -> &mut T {
        ::core::mem::transmute(self)
    }
}
impl<T> ::core::default::Default for __BindgenUnionField<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
impl<T> ::core::clone::Clone for __BindgenUnionField<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self::new()
    }
}
impl<T> ::core::marker::Copy for __BindgenUnionField<T> {}
impl<T> ::core::fmt::Debug for __BindgenUnionField<T> {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.write_str("__BindgenUnionField")
    }
}
impl<T> ::core::hash::Hash for __BindgenUnionField<T> {
    fn hash<H: ::core::hash::Hasher>(&self, _state: &mut H) {}
}
impl<T> ::core::cmp::PartialEq for __BindgenUnionField<T> {
    fn eq(&self, _other: &__BindgenUnionField<T>) -> bool {
        true
    }
}
impl<T> ::core::cmp::Eq for __BindgenUnionField<T> {}
pub const USBDEVICE_SUPER_MAGIC: i32 = 40866;
pub const USBDEVFS_MAXDRIVERNAME: i32 = 255;
pub const USBDEVFS_URB_SHORT_NOT_OK: i32 = 1;
pub const USBDEVFS_URB_ISO_ASAP: i32 = 2;
pub const USBDEVFS_URB_BULK_CONTINUATION: i32 = 4;
pub const USBDEVFS_URB_NO_FSBR: i32 = 32;
pub const USBDEVFS_URB_ZERO_PACKET: i32 = 64;
pub const USBDEVFS_URB_NO_INTERRUPT: i32 = 128;
pub const USBDEVFS_URB_TYPE_ISO: i32 = 0;
pub const USBDEVFS_URB_TYPE_INTERRUPT: i32 = 1;
pub const USBDEVFS_URB_TYPE_CONTROL: i32 = 2;
pub const USBDEVFS_URB_TYPE_BULK: i32 = 3;
pub const USBDEVFS_CAP_ZERO_PACKET: i32 = 1;
pub const USBDEVFS_CAP_BULK_CONTINUATION: i32 = 2;
pub const USBDEVFS_CAP_NO_PACKET_SIZE_LIM: i32 = 4;
pub const USBDEVFS_CAP_BULK_SCATTER_GATHER: i32 = 8;
pub const USBDEVFS_CAP_REAP_AFTER_DISCONNECT: i32 = 16;
pub const USBDEVFS_CAP_MMAP: i32 = 32;
pub const USBDEVFS_CAP_DROP_PRIVILEGES: i32 = 64;
pub const USBDEVFS_CAP_CONNINFO_EX: i32 = 128;
pub const USBDEVFS_CAP_SUSPEND: i32 = 256;
pub const USBDEVFS_DISCONNECT_CLAIM_IF_DRIVER: i32 = 1;
pub const USBDEVFS_DISCONNECT_CLAIM_EXCEPT_DRIVER: i32 = 2;
pub const USBIP_URB_SHORT_NOT_OK: i32 = 1;
pub const USBIP_URB_ISO_ASAP: i32 = 2;
pub const USBIP_URB_NO_TRANSFER_DMA_MAP: i32 = 4;
pub const USBIP_URB_ZERO_PACKET: i32 = 64;
pub const USBIP_URB_NO_INTERRUPT: i32 = 128;
pub const USBIP_URB_FREE_BUFFER: i32 = 256;
pub const USBIP_URB_DIR_IN: i32 = 512;
pub const USBIP_URB_DIR_OUT: i32 = 0;
pub const USBIP_URB_DIR_MASK: i32 = 512;
pub const USBIP_URB_DMA_MAP_SINGLE: i32 = 65536;
pub const USBIP_URB_DMA_MAP_PAGE: i32 = 131072;
pub const USBIP_URB_DMA_MAP_SG: i32 = 262144;
pub const USBIP_URB_MAP_LOCAL: i32 = 524288;
pub const USBIP_URB_SETUP_MAP_SINGLE: i32 = 1048576;
pub const USBIP_URB_SETUP_MAP_LOCAL: i32 = 2097152;
pub const USBIP_URB_DMA_SG_COMBINED: i32 = 4194304;
pub const USBIP_URB_ALIGNED_TEMP_BUFFER: i32 = 8388608;
pub const USB_DIR_OUT: i32 = 0;
pub const USB_DIR_IN: i32 = 128;
pub const USB_TYPE_MASK: i32 = 96;
pub const USB_TYPE_STANDARD: i32 = 0;
pub const USB_TYPE_CLASS: i32 = 32;
pub const USB_TYPE_VENDOR: i32 = 64;
pub const USB_TYPE_RESERVED: i32 = 96;
pub const USB_RECIP_MASK: i32 = 31;
pub const USB_RECIP_DEVICE: i32 = 0;
pub const USB_RECIP_INTERFACE: i32 = 1;
pub const USB_RECIP_ENDPOINT: i32 = 2;
pub const USB_RECIP_OTHER: i32 = 3;
pub const USB_RECIP_PORT: i32 = 4;
pub const USB_RECIP_RPIPE: i32 = 5;
pub const USB_REQ_GET_STATUS: i32 = 0;
pub const USB_REQ_CLEAR_FEATURE: i32 = 1;
pub const USB_REQ_SET_FEATURE: i32 = 3;
pub const USB_REQ_SET_ADDRESS: i32 = 5;
pub const USB_REQ_GET_DESCRIPTOR: i32 = 6;
pub const USB_REQ_SET_DESCRIPTOR: i32 = 7;
pub const USB_REQ_GET_CONFIGURATION: i32 = 8;
pub const USB_REQ_SET_CONFIGURATION: i32 = 9;
pub const USB_REQ_GET_INTERFACE: i32 = 10;
pub const USB_REQ_SET_INTERFACE: i32 = 11;
pub const USB_REQ_SYNCH_FRAME: i32 = 12;
pub const USB_REQ_SET_SEL: i32 = 48;
pub const USB_REQ_SET_ISOCH_DELAY: i32 = 49;
pub const USB_REQ_SET_ENCRYPTION: i32 = 13;
pub const USB_REQ_GET_ENCRYPTION: i32 = 14;
pub const USB_REQ_RPIPE_ABORT: i32 = 14;
pub const USB_REQ_SET_HANDSHAKE: i32 = 15;
pub const USB_REQ_RPIPE_RESET: i32 = 15;
pub const USB_REQ_GET_HANDSHAKE: i32 = 16;
pub const USB_REQ_SET_CONNECTION: i32 = 17;
pub const USB_REQ_SET_SECURITY_DATA: i32 = 18;
pub const USB_REQ_GET_SECURITY_DATA: i32 = 19;
pub const USB_REQ_SET_WUSB_DATA: i32 = 20;
pub const USB_REQ_LOOPBACK_DATA_WRITE: i32 = 21;
pub const USB_REQ_LOOPBACK_DATA_READ: i32 = 22;
pub const USB_REQ_SET_INTERFACE_DS: i32 = 23;
pub const USB_REQ_GET_PARTNER_PDO: i32 = 20;
pub const USB_REQ_GET_BATTERY_STATUS: i32 = 21;
pub const USB_REQ_SET_PDO: i32 = 22;
pub const USB_REQ_GET_VDM: i32 = 23;
pub const USB_REQ_SEND_VDM: i32 = 24;
pub const USB_DEVICE_SELF_POWERED: i32 = 0;
pub const USB_DEVICE_REMOTE_WAKEUP: i32 = 1;
pub const USB_DEVICE_TEST_MODE: i32 = 2;
pub const USB_DEVICE_BATTERY: i32 = 2;
pub const USB_DEVICE_B_HNP_ENABLE: i32 = 3;
pub const USB_DEVICE_WUSB_DEVICE: i32 = 3;
pub const USB_DEVICE_A_HNP_SUPPORT: i32 = 4;
pub const USB_DEVICE_A_ALT_HNP_SUPPORT: i32 = 5;
pub const USB_DEVICE_DEBUG_MODE: i32 = 6;
pub const USB_TEST_J: i32 = 1;
pub const USB_TEST_K: i32 = 2;
pub const USB_TEST_SE0_NAK: i32 = 3;
pub const USB_TEST_PACKET: i32 = 4;
pub const USB_TEST_FORCE_ENABLE: i32 = 5;
pub const USB_STATUS_TYPE_STANDARD: i32 = 0;
pub const USB_STATUS_TYPE_PTM: i32 = 1;
pub const USB_DEVICE_U1_ENABLE: i32 = 48;
pub const USB_DEVICE_U2_ENABLE: i32 = 49;
pub const USB_DEVICE_LTM_ENABLE: i32 = 50;
pub const USB_INTRF_FUNC_SUSPEND: i32 = 0;
pub const USB_INTR_FUNC_SUSPEND_OPT_MASK: i32 = 65280;
pub const USB_INTRF_FUNC_SUSPEND_LP: i32 = 256;
pub const USB_INTRF_FUNC_SUSPEND_RW: i32 = 512;
pub const USB_INTRF_STAT_FUNC_RW_CAP: i32 = 1;
pub const USB_INTRF_STAT_FUNC_RW: i32 = 2;
pub const USB_ENDPOINT_HALT: i32 = 0;
pub const USB_DEV_STAT_U1_ENABLED: i32 = 2;
pub const USB_DEV_STAT_U2_ENABLED: i32 = 3;
pub const USB_DEV_STAT_LTM_ENABLED: i32 = 4;
pub const USB_DEVICE_BATTERY_WAKE_MASK: i32 = 40;
pub const USB_DEVICE_OS_IS_PD_AWARE: i32 = 41;
pub const USB_DEVICE_POLICY_MODE: i32 = 42;
pub const USB_PORT_PR_SWAP: i32 = 43;
pub const USB_PORT_GOTO_MIN: i32 = 44;
pub const USB_PORT_RETURN_POWER: i32 = 45;
pub const USB_PORT_ACCEPT_PD_REQUEST: i32 = 46;
pub const USB_PORT_REJECT_PD_REQUEST: i32 = 47;
pub const USB_PORT_PORT_PD_RESET: i32 = 48;
pub const USB_PORT_C_PORT_PD_CHANGE: i32 = 49;
pub const USB_PORT_CABLE_PD_RESET: i32 = 50;
pub const USB_DEVICE_CHARGING_POLICY: i32 = 54;
pub const USB_DT_DEVICE: i32 = 1;
pub const USB_DT_CONFIG: i32 = 2;
pub const USB_DT_STRING: i32 = 3;
pub const USB_DT_INTERFACE: i32 = 4;
pub const USB_DT_ENDPOINT: i32 = 5;
pub const USB_DT_DEVICE_QUALIFIER: i32 = 6;
pub const USB_DT_OTHER_SPEED_CONFIG: i32 = 7;
pub const USB_DT_INTERFACE_POWER: i32 = 8;
pub const USB_DT_OTG: i32 = 9;
pub const USB_DT_DEBUG: i32 = 10;
pub const USB_DT_INTERFACE_ASSOCIATION: i32 = 11;
pub const USB_DT_SECURITY: i32 = 12;
pub const USB_DT_KEY: i32 = 13;
pub const USB_DT_ENCRYPTION_TYPE: i32 = 14;
pub const USB_DT_BOS: i32 = 15;
pub const USB_DT_DEVICE_CAPABILITY: i32 = 16;
pub const USB_DT_WIRELESS_ENDPOINT_COMP: i32 = 17;
pub const USB_DT_WIRE_ADAPTER: i32 = 33;
pub const USB_DT_RPIPE: i32 = 34;
pub const USB_DT_CS_RADIO_CONTROL: i32 = 35;
pub const USB_DT_PIPE_USAGE: i32 = 36;
pub const USB_DT_SS_ENDPOINT_COMP: i32 = 48;
pub const USB_DT_SSP_ISOC_ENDPOINT_COMP: i32 = 49;
pub const USB_DT_CS_DEVICE: i32 = 33;
pub const USB_DT_CS_CONFIG: i32 = 34;
pub const USB_DT_CS_STRING: i32 = 35;
pub const USB_DT_CS_INTERFACE: i32 = 36;
pub const USB_DT_CS_ENDPOINT: i32 = 37;
pub const USB_DT_DEVICE_SIZE: i32 = 18;
pub const USB_CLASS_PER_INTERFACE: i32 = 0;
pub const USB_CLASS_AUDIO: i32 = 1;
pub const USB_CLASS_COMM: i32 = 2;
pub const USB_CLASS_HID: i32 = 3;
pub const USB_CLASS_PHYSICAL: i32 = 5;
pub const USB_CLASS_STILL_IMAGE: i32 = 6;
pub const USB_CLASS_PRINTER: i32 = 7;
pub const USB_CLASS_MASS_STORAGE: i32 = 8;
pub const USB_CLASS_HUB: i32 = 9;
pub const USB_CLASS_CDC_DATA: i32 = 10;
pub const USB_CLASS_CSCID: i32 = 11;
pub const USB_CLASS_CONTENT_SEC: i32 = 13;
pub const USB_CLASS_VIDEO: i32 = 14;
pub const USB_CLASS_WIRELESS_CONTROLLER: i32 = 224;
pub const USB_CLASS_PERSONAL_HEALTHCARE: i32 = 15;
pub const USB_CLASS_AUDIO_VIDEO: i32 = 16;
pub const USB_CLASS_BILLBOARD: i32 = 17;
pub const USB_CLASS_USB_TYPE_C_BRIDGE: i32 = 18;
pub const USB_CLASS_MISC: i32 = 239;
pub const USB_CLASS_APP_SPEC: i32 = 254;
pub const USB_CLASS_VENDOR_SPEC: i32 = 255;
pub const USB_SUBCLASS_VENDOR_SPEC: i32 = 255;
pub const USB_DT_CONFIG_SIZE: i32 = 9;
pub const USB_CONFIG_ATT_ONE: i32 = 128;
pub const USB_CONFIG_ATT_SELFPOWER: i32 = 64;
pub const USB_CONFIG_ATT_WAKEUP: i32 = 32;
pub const USB_CONFIG_ATT_BATTERY: i32 = 16;
pub const USB_MAX_STRING_LEN: i32 = 126;
pub const USB_DT_INTERFACE_SIZE: i32 = 9;
pub const USB_DT_ENDPOINT_SIZE: i32 = 7;
pub const USB_DT_ENDPOINT_AUDIO_SIZE: i32 = 9;
pub const USB_ENDPOINT_NUMBER_MASK: i32 = 15;
pub const USB_ENDPOINT_DIR_MASK: i32 = 128;
pub const USB_ENDPOINT_XFERTYPE_MASK: i32 = 3;
pub const USB_ENDPOINT_XFER_CONTROL: i32 = 0;
pub const USB_ENDPOINT_XFER_ISOC: i32 = 1;
pub const USB_ENDPOINT_XFER_BULK: i32 = 2;
pub const USB_ENDPOINT_XFER_INT: i32 = 3;
pub const USB_ENDPOINT_MAX_ADJUSTABLE: i32 = 128;
pub const USB_ENDPOINT_MAXP_MASK: i32 = 2047;
pub const USB_EP_MAXP_MULT_SHIFT: i32 = 11;
pub const USB_EP_MAXP_MULT_MASK: i32 = 6144;
pub const USB_ENDPOINT_INTRTYPE: i32 = 48;
pub const USB_ENDPOINT_INTR_PERIODIC: i32 = 0;
pub const USB_ENDPOINT_INTR_NOTIFICATION: i32 = 16;
pub const USB_ENDPOINT_SYNCTYPE: i32 = 12;
pub const USB_ENDPOINT_SYNC_NONE: i32 = 0;
pub const USB_ENDPOINT_SYNC_ASYNC: i32 = 4;
pub const USB_ENDPOINT_SYNC_ADAPTIVE: i32 = 8;
pub const USB_ENDPOINT_SYNC_SYNC: i32 = 12;
pub const USB_ENDPOINT_USAGE_MASK: i32 = 48;
pub const USB_ENDPOINT_USAGE_DATA: i32 = 0;
pub const USB_ENDPOINT_USAGE_FEEDBACK: i32 = 16;
pub const USB_ENDPOINT_USAGE_IMPLICIT_FB: i32 = 32;
pub const USB_DT_SSP_ISOC_EP_COMP_SIZE: i32 = 8;
pub const USB_DT_SS_EP_COMP_SIZE: i32 = 6;
pub const USB_OTG_SRP: i32 = 1;
pub const USB_OTG_HNP: i32 = 2;
pub const USB_OTG_ADP: i32 = 4;
pub const USB_DT_INTERFACE_ASSOCIATION_SIZE: i32 = 8;
pub const USB_ENC_TYPE_UNSECURE: i32 = 0;
pub const USB_ENC_TYPE_WIRED: i32 = 1;
pub const USB_ENC_TYPE_CCM_1: i32 = 2;
pub const USB_ENC_TYPE_RSA_1: i32 = 3;
pub const USB_DT_BOS_SIZE: i32 = 5;
pub const USB_CAP_TYPE_WIRELESS_USB: i32 = 1;
pub const USB_WIRELESS_P2P_DRD: i32 = 2;
pub const USB_WIRELESS_BEACON_MASK: i32 = 12;
pub const USB_WIRELESS_BEACON_SELF: i32 = 4;
pub const USB_WIRELESS_BEACON_DIRECTED: i32 = 8;
pub const USB_WIRELESS_BEACON_NONE: i32 = 12;
pub const USB_WIRELESS_PHY_53: i32 = 1;
pub const USB_WIRELESS_PHY_80: i32 = 2;
pub const USB_WIRELESS_PHY_107: i32 = 4;
pub const USB_WIRELESS_PHY_160: i32 = 8;
pub const USB_WIRELESS_PHY_200: i32 = 16;
pub const USB_WIRELESS_PHY_320: i32 = 32;
pub const USB_WIRELESS_PHY_400: i32 = 64;
pub const USB_WIRELESS_PHY_480: i32 = 128;
pub const USB_DT_USB_WIRELESS_CAP_SIZE: i32 = 11;
pub const USB_CAP_TYPE_EXT: i32 = 2;
pub const USB_LPM_SUPPORT: i32 = 2;
pub const USB_BESL_SUPPORT: i32 = 4;
pub const USB_BESL_BASELINE_VALID: i32 = 8;
pub const USB_BESL_DEEP_VALID: i32 = 16;
pub const USB_DT_USB_EXT_CAP_SIZE: i32 = 7;
pub const USB_SS_CAP_TYPE: i32 = 3;
pub const USB_LTM_SUPPORT: i32 = 2;
pub const USB_LOW_SPEED_OPERATION: i32 = 1;
pub const USB_FULL_SPEED_OPERATION: i32 = 2;
pub const USB_HIGH_SPEED_OPERATION: i32 = 4;
pub const USB_5GBPS_OPERATION: i32 = 8;
pub const USB_DT_USB_SS_CAP_SIZE: i32 = 10;
pub const USB_DT_USB_SS_CONTN_ID_SIZE: i32 = 20;
pub const USB_PLAT_DEV_CAP_TYPE: i32 = 5;
pub const USB_SSP_CAP_TYPE: i32 = 10;
pub const USB_SSP_SUBLINK_SPEED_ATTRIBS: i32 = 31;
pub const USB_SSP_SUBLINK_SPEED_IDS: i32 = 480;
pub const USB_SSP_MIN_SUBLINK_SPEED_ATTRIBUTE_ID: i32 = 15;
pub const USB_SSP_MIN_RX_LANE_COUNT: i32 = 3840;
pub const USB_SSP_MIN_TX_LANE_COUNT: i32 = 61440;
pub const USB_SSP_SUBLINK_SPEED_SSID: i32 = 15;
pub const USB_SSP_SUBLINK_SPEED_LSE: i32 = 48;
pub const USB_SSP_SUBLINK_SPEED_LSE_BPS: i32 = 0;
pub const USB_SSP_SUBLINK_SPEED_LSE_KBPS: i32 = 1;
pub const USB_SSP_SUBLINK_SPEED_LSE_MBPS: i32 = 2;
pub const USB_SSP_SUBLINK_SPEED_LSE_GBPS: i32 = 3;
pub const USB_SSP_SUBLINK_SPEED_ST: i32 = 192;
pub const USB_SSP_SUBLINK_SPEED_ST_SYM_RX: i32 = 0;
pub const USB_SSP_SUBLINK_SPEED_ST_ASYM_RX: i32 = 1;
pub const USB_SSP_SUBLINK_SPEED_ST_SYM_TX: i32 = 2;
pub const USB_SSP_SUBLINK_SPEED_ST_ASYM_TX: i32 = 3;
pub const USB_SSP_SUBLINK_SPEED_RSVD: i32 = 16128;
pub const USB_SSP_SUBLINK_SPEED_LP: i32 = 49152;
pub const USB_SSP_SUBLINK_SPEED_LP_SS: i32 = 0;
pub const USB_SSP_SUBLINK_SPEED_LP_SSP: i32 = 1;
pub const USB_SSP_SUBLINK_SPEED_LSM: i32 = 16711680;
pub const USB_PD_POWER_DELIVERY_CAPABILITY: i32 = 6;
pub const USB_PD_BATTERY_INFO_CAPABILITY: i32 = 7;
pub const USB_PD_PD_CONSUMER_PORT_CAPABILITY: i32 = 8;
pub const USB_PD_PD_PROVIDER_PORT_CAPABILITY: i32 = 9;
pub const USB_PD_CAP_BATTERY_CHARGING: i32 = 2;
pub const USB_PD_CAP_USB_PD: i32 = 4;
pub const USB_PD_CAP_PROVIDER: i32 = 8;
pub const USB_PD_CAP_CONSUMER: i32 = 16;
pub const USB_PD_CAP_CHARGING_POLICY: i32 = 32;
pub const USB_PD_CAP_TYPE_C_CURRENT: i32 = 64;
pub const USB_PD_CAP_PWR_AC: i32 = 256;
pub const USB_PD_CAP_PWR_BAT: i32 = 512;
pub const USB_PD_CAP_PWR_USE_V_BUS: i32 = 16384;
pub const USB_PD_CAP_CONSUMER_BC: i32 = 1;
pub const USB_PD_CAP_CONSUMER_PD: i32 = 2;
pub const USB_PD_CAP_CONSUMER_TYPE_C: i32 = 4;
pub const USB_PD_CAP_CONSUMER_UNKNOWN_PEAK_POWER_TIME: i32 = 65535;
pub const USB_PD_CAP_PROVIDER_BC: i32 = 1;
pub const USB_PD_CAP_PROVIDER_PD: i32 = 2;
pub const USB_PD_CAP_PROVIDER_TYPE_C: i32 = 4;
pub const USB_PTM_CAP_TYPE: i32 = 11;
pub const USB_DT_USB_PTM_ID_SIZE: i32 = 3;
pub const USB_ENDPOINT_SWITCH_MASK: i32 = 3;
pub const USB_ENDPOINT_SWITCH_NO: i32 = 0;
pub const USB_ENDPOINT_SWITCH_SWITCH: i32 = 1;
pub const USB_ENDPOINT_SWITCH_SCALE: i32 = 2;
pub const USB3_LPM_DISABLED: i32 = 0;
pub const USB3_LPM_U1_MAX_TIMEOUT: i32 = 127;
pub const USB3_LPM_U2_MAX_TIMEOUT: i32 = 254;
pub const USB3_LPM_DEVICE_INITIATED: i32 = 255;
pub const USB3_LPM_MAX_U1_SEL_PEL: i32 = 255;
pub const USB3_LPM_MAX_U2_SEL_PEL: i32 = 65535;
pub const USB_SELF_POWER_VBUS_MAX_DRAW: i32 = 100;
pub type __u8 = ::core::ffi::c_uchar;
pub type __u16 = ::core::ffi::c_ushort;
pub type __u32 = ::core::ffi::c_uint;
pub type __le16 = __u16;
pub type __le32 = __u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct usbdevfs_ctrltransfer {
    pub bRequestType: __u8,
    pub bRequest: __u8,
    pub wValue: __u16,
    pub wIndex: __u16,
    pub wLength: __u16,
    pub timeout: __u32,
    pub data: *mut ::core::ffi::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct usbdevfs_bulktransfer {
    pub ep: ::core::ffi::c_uint,
    pub len: ::core::ffi::c_uint,
    pub timeout: ::core::ffi::c_uint,
    pub data: *mut ::core::ffi::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct usbdevfs_setinterface {
    pub interface: ::core::ffi::c_uint,
    pub altsetting: ::core::ffi::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct usbdevfs_disconnectsignal {
    pub signr: ::core::ffi::c_uint,
    pub context: *mut ::core::ffi::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct usbdevfs_getdriver {
    pub interface: ::core::ffi::c_uint,
    pub driver: [::core::ffi::c_char; 256usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct usbdevfs_connectinfo {
    pub devnum: ::core::ffi::c_uint,
    pub slow: ::core::ffi::c_uchar,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct usbdevfs_conninfo_ex {
    pub size: __u32,
    pub busnum: __u32,
    pub devnum: __u32,
    pub speed: __u32,
    pub num_ports: __u8,
    pub ports: [__u8; 7usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct usbdevfs_iso_packet_desc {
    pub length: ::core::ffi::c_uint,
    pub actual_length: ::core::ffi::c_uint,
    pub status: ::core::ffi::c_uint,
}
#[repr(C)]
pub struct usbdevfs_urb {
    pub type_: ::core::ffi::c_uchar,
    pub endpoint: ::core::ffi::c_uchar,
    pub status: ::core::ffi::c_int,
    pub flags: ::core::ffi::c_uint,
    pub buffer: *mut ::core::ffi::c_void,
    pub buffer_length: ::core::ffi::c_int,
    pub actual_length: ::core::ffi::c_int,
    pub start_frame: ::core::ffi::c_int,
    pub __bindgen_anon_1: usbdevfs_urb__bindgen_ty_1,
    pub error_count: ::core::ffi::c_int,
    pub signr: ::core::ffi::c_uint,
    pub usercontext: *mut ::core::ffi::c_void,
    pub iso_frame_desc: __IncompleteArrayField<usbdevfs_iso_packet_desc>,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union usbdevfs_urb__bindgen_ty_1 {
    pub number_of_packets: ::core::ffi::c_int,
    pub stream_id: ::core::ffi::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct usbdevfs_ioctl {
    pub ifno: ::core::ffi::c_int,
    pub ioctl_code: ::core::ffi::c_int,
    pub data: *mut ::core::ffi::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct usbdevfs_hub_portinfo {
    pub nports: ::core::ffi::c_char,
    pub port: [::core::ffi::c_char; 127usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct usbdevfs_disconnect_claim {
    pub interface: ::core::ffi::c_uint,
    pub flags: ::core::ffi::c_uint,
    pub driver: [::core::ffi::c_char; 256usize],
}
#[repr(C)]
#[derive(Debug)]
pub struct usbdevfs_streams {
    pub num_streams: ::core::ffi::c_uint,
    pub num_eps: ::core::ffi::c_uint,
    pub eps: __IncompleteArrayField<::core::ffi::c_uchar>,
}
pub const usbip_device_status_SDEV_ST_AVAILABLE: usbip_device_status = 1;
pub const usbip_device_status_SDEV_ST_USED: usbip_device_status = 2;
pub const usbip_device_status_SDEV_ST_ERROR: usbip_device_status = 3;
pub const usbip_device_status_VDEV_ST_NULL: usbip_device_status = 4;
pub const usbip_device_status_VDEV_ST_NOTASSIGNED: usbip_device_status = 5;
pub const usbip_device_status_VDEV_ST_USED: usbip_device_status = 6;
pub const usbip_device_status_VDEV_ST_ERROR: usbip_device_status = 7;
pub type usbip_device_status = ::core::ffi::c_uint;
#[doc = " struct usb_ctrlrequest - SETUP data for a USB device control request\n @bRequestType: matches the USB bmRequestType field\n @bRequest: matches the USB bRequest field\n @wValue: matches the USB wValue field (le16 byte order)\n @wIndex: matches the USB wIndex field (le16 byte order)\n @wLength: matches the USB wLength field (le16 byte order)\n\n This structure is used to send control requests to a USB device.  It matches\n the different fields of the USB 2.0 Spec section 9.3, table 9-2.  See the\n USB spec for a fuller description of the different fields, and what they are\n used for.\n\n Note that the driver for any interface can issue control requests.\n For most devices, interfaces don't coordinate with each other, so\n such requests may be made at any time."]
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct usb_ctrlrequest {
    pub bRequestType: __u8,
    pub bRequest: __u8,
    pub wValue: __le16,
    pub wIndex: __le16,
    pub wLength: __le16,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct usb_descriptor_header {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct usb_device_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bcdUSB: __le16,
    pub bDeviceClass: __u8,
    pub bDeviceSubClass: __u8,
    pub bDeviceProtocol: __u8,
    pub bMaxPacketSize0: __u8,
    pub idVendor: __le16,
    pub idProduct: __le16,
    pub bcdDevice: __le16,
    pub iManufacturer: __u8,
    pub iProduct: __u8,
    pub iSerialNumber: __u8,
    pub bNumConfigurations: __u8,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct usb_config_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub wTotalLength: __le16,
    pub bNumInterfaces: __u8,
    pub bConfigurationValue: __u8,
    pub iConfiguration: __u8,
    pub bmAttributes: __u8,
    pub bMaxPower: __u8,
}
#[repr(C, packed)]
pub struct usb_string_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub __bindgen_anon_1: usb_string_descriptor__bindgen_ty_1,
}
#[repr(C)]
pub struct usb_string_descriptor__bindgen_ty_1 {
    pub legacy_padding: __BindgenUnionField<__le16>,
    pub __bindgen_anon_1: __BindgenUnionField<usb_string_descriptor__bindgen_ty_1__bindgen_ty_1>,
    pub bindgen_union_field: u16,
}
#[repr(C)]
#[derive(Debug)]
pub struct usb_string_descriptor__bindgen_ty_1__bindgen_ty_1 {
    pub __empty_wData: usb_string_descriptor__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1,
    pub wData: __IncompleteArrayField<__le16>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct usb_string_descriptor__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1 {}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct usb_interface_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bInterfaceNumber: __u8,
    pub bAlternateSetting: __u8,
    pub bNumEndpoints: __u8,
    pub bInterfaceClass: __u8,
    pub bInterfaceSubClass: __u8,
    pub bInterfaceProtocol: __u8,
    pub iInterface: __u8,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct usb_endpoint_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bEndpointAddress: __u8,
    pub bmAttributes: __u8,
    pub wMaxPacketSize: __le16,
    pub bInterval: __u8,
    pub bRefresh: __u8,
    pub bSynchAddress: __u8,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct usb_ssp_isoc_ep_comp_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub wReseved: __le16,
    pub dwBytesPerInterval: __le32,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct usb_ss_ep_comp_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bMaxBurst: __u8,
    pub bmAttributes: __u8,
    pub wBytesPerInterval: __le16,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct usb_qualifier_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bcdUSB: __le16,
    pub bDeviceClass: __u8,
    pub bDeviceSubClass: __u8,
    pub bDeviceProtocol: __u8,
    pub bMaxPacketSize0: __u8,
    pub bNumConfigurations: __u8,
    pub bRESERVED: __u8,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct usb_otg_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bmAttributes: __u8,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct usb_otg20_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bmAttributes: __u8,
    pub bcdOTG: __le16,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct usb_debug_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDebugInEndpoint: __u8,
    pub bDebugOutEndpoint: __u8,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct usb_interface_assoc_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bFirstInterface: __u8,
    pub bInterfaceCount: __u8,
    pub bFunctionClass: __u8,
    pub bFunctionSubClass: __u8,
    pub bFunctionProtocol: __u8,
    pub iFunction: __u8,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct usb_security_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub wTotalLength: __le16,
    pub bNumEncryptionTypes: __u8,
}
#[repr(C, packed)]
pub struct usb_key_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub tTKID: [__u8; 3usize],
    pub bReserved: __u8,
    pub bKeyData: __IncompleteArrayField<__u8>,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct usb_encryption_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bEncryptionType: __u8,
    pub bEncryptionValue: __u8,
    pub bAuthKeyIndex: __u8,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct usb_bos_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub wTotalLength: __le16,
    pub bNumDeviceCaps: __u8,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct usb_dev_cap_header {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDevCapabilityType: __u8,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct usb_wireless_cap_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDevCapabilityType: __u8,
    pub bmAttributes: __u8,
    pub wPHYRates: __le16,
    pub bmTFITXPowerInfo: __u8,
    pub bmFFITXPowerInfo: __u8,
    pub bmBandGroup: __le16,
    pub bReserved: __u8,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct usb_ext_cap_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDevCapabilityType: __u8,
    pub bmAttributes: __le32,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct usb_ss_cap_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDevCapabilityType: __u8,
    pub bmAttributes: __u8,
    pub wSpeedSupported: __le16,
    pub bFunctionalitySupport: __u8,
    pub bU1devExitLat: __u8,
    pub bU2DevExitLat: __le16,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct usb_ss_container_id_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDevCapabilityType: __u8,
    pub bReserved: __u8,
    pub ContainerID: [__u8; 16usize],
}
#[repr(C, packed)]
pub struct usb_plat_dev_cap_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDevCapabilityType: __u8,
    pub bReserved: __u8,
    pub UUID: [__u8; 16usize],
    pub CapabilityData: __IncompleteArrayField<__u8>,
}
#[repr(C, packed)]
pub struct usb_ssp_cap_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDevCapabilityType: __u8,
    pub bReserved: __u8,
    pub bmAttributes: __le32,
    pub wFunctionalitySupport: __le16,
    pub wReserved: __le16,
    pub __bindgen_anon_1: usb_ssp_cap_descriptor__bindgen_ty_1,
}
#[repr(C)]
pub struct usb_ssp_cap_descriptor__bindgen_ty_1 {
    pub legacy_padding: __BindgenUnionField<__le32>,
    pub __bindgen_anon_1: __BindgenUnionField<usb_ssp_cap_descriptor__bindgen_ty_1__bindgen_ty_1>,
    pub bindgen_union_field: u32,
}
#[repr(C)]
#[derive(Debug)]
pub struct usb_ssp_cap_descriptor__bindgen_ty_1__bindgen_ty_1 {
    pub __empty_bmSublinkSpeedAttr:
        usb_ssp_cap_descriptor__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1,
    pub bmSublinkSpeedAttr: __IncompleteArrayField<__le32>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct usb_ssp_cap_descriptor__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1 {}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct usb_pd_cap_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDevCapabilityType: __u8,
    pub bReserved: __u8,
    pub bmAttributes: __le32,
    pub bmProviderPorts: __le16,
    pub bmConsumerPorts: __le16,
    pub bcdBCVersion: __le16,
    pub bcdPDVersion: __le16,
    pub bcdUSBTypeCVersion: __le16,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct usb_pd_cap_battery_info_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDevCapabilityType: __u8,
    pub iBattery: __u8,
    pub iSerial: __u8,
    pub iManufacturer: __u8,
    pub bBatteryId: __u8,
    pub bReserved: __u8,
    pub dwChargedThreshold: __le32,
    pub dwWeakThreshold: __le32,
    pub dwBatteryDesignCapacity: __le32,
    pub dwBatteryLastFullchargeCapacity: __le32,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct usb_pd_cap_consumer_port_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDevCapabilityType: __u8,
    pub bReserved: __u8,
    pub bmCapabilities: __u8,
    pub wMinVoltage: __le16,
    pub wMaxVoltage: __le16,
    pub wReserved: __u16,
    pub dwMaxOperatingPower: __le32,
    pub dwMaxPeakPower: __le32,
    pub dwMaxPeakPowerTime: __le32,
}
#[repr(C, packed)]
pub struct usb_pd_cap_provider_port_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDevCapabilityType: __u8,
    pub bReserved1: __u8,
    pub bmCapabilities: __u8,
    pub bNumOfPDObjects: __u8,
    pub bReserved2: __u8,
    pub wPowerDataObject: __IncompleteArrayField<__le32>,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct usb_ptm_cap_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDevCapabilityType: __u8,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct usb_wireless_ep_comp_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bMaxBurst: __u8,
    pub bMaxSequence: __u8,
    pub wMaxStreamDelay: __le16,
    pub wOverTheAirPacketSize: __le16,
    pub bOverTheAirInterval: __u8,
    pub bmCompAttributes: __u8,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct usb_handshake {
    pub bMessageNumber: __u8,
    pub bStatus: __u8,
    pub tTKID: [__u8; 3usize],
    pub bReserved: __u8,
    pub CDID: [__u8; 16usize],
    pub nonce: [__u8; 16usize],
    pub MIC: [__u8; 8usize],
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct usb_connection_context {
    pub CHID: [__u8; 16usize],
    pub CDID: [__u8; 16usize],
    pub CK: [__u8; 16usize],
}
pub const usb_device_speed_USB_SPEED_UNKNOWN: usb_device_speed = 0;
pub const usb_device_speed_USB_SPEED_LOW: usb_device_speed = 1;
pub const usb_device_speed_USB_SPEED_FULL: usb_device_speed = 2;
pub const usb_device_speed_USB_SPEED_HIGH: usb_device_speed = 3;
pub const usb_device_speed_USB_SPEED_WIRELESS: usb_device_speed = 4;
pub const usb_device_speed_USB_SPEED_SUPER: usb_device_speed = 5;
pub const usb_device_speed_USB_SPEED_SUPER_PLUS: usb_device_speed = 6;
pub type usb_device_speed = ::core::ffi::c_uint;
pub const usb_device_state_USB_STATE_NOTATTACHED: usb_device_state = 0;
pub const usb_device_state_USB_STATE_ATTACHED: usb_device_state = 1;
pub const usb_device_state_USB_STATE_POWERED: usb_device_state = 2;
pub const usb_device_state_USB_STATE_RECONNECTING: usb_device_state = 3;
pub const usb_device_state_USB_STATE_UNAUTHENTICATED: usb_device_state = 4;
pub const usb_device_state_USB_STATE_DEFAULT: usb_device_state = 5;
pub const usb_device_state_USB_STATE_ADDRESS: usb_device_state = 6;
pub const usb_device_state_USB_STATE_CONFIGURED: usb_device_state = 7;
pub const usb_device_state_USB_STATE_SUSPENDED: usb_device_state = 8;
pub type usb_device_state = ::core::ffi::c_uint;
pub const usb3_link_state_USB3_LPM_U0: usb3_link_state = 0;
pub const usb3_link_state_USB3_LPM_U1: usb3_link_state = 1;
pub const usb3_link_state_USB3_LPM_U2: usb3_link_state = 2;
pub const usb3_link_state_USB3_LPM_U3: usb3_link_state = 3;
pub type usb3_link_state = ::core::ffi::c_uint;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct usb_set_sel_req {
    pub u1_sel: __u8,
    pub u1_pel: __u8,
    pub u2_sel: __le16,
    pub u2_pel: __le16,
}
