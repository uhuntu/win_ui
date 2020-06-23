use winrt::RawPtr;
use bindings::windows::ui::xaml::hosting::DesktopWindowXamlSource;

#[link(name = "windowsapp")]
extern "stdcall" {
    fn RoInitialize(init_type: RoInitType) -> winrt::ErrorCode;
}
#[allow(dead_code)]
#[repr(i32)]
pub enum RoInitType {
    MultiThreaded = 0,
    SingleThreaded = 1,
}

pub fn ro_initialize(init_type: RoInitType) -> winrt::Result<()> {
    unsafe { RoInitialize(init_type).ok() }
}

#[repr(C)]
pub struct abi_IDesktopWindowXamlSourceNative {
    __base: [usize; 3],
    attach_to_window: extern "system" fn (
        winrt::NonNullRawComPtr<IDesktopWindowXamlSourceNative>,
        RawPtr, //HWND
    ) -> winrt::ErrorCode,
    get_window_handle: extern "system" fn (
        winrt::NonNullRawComPtr<IDesktopWindowXamlSourceNative>,
        *mut RawPtr, //HWND
    ) -> winrt::ErrorCode,
}

unsafe impl winrt::ComInterface for IDesktopWindowXamlSourceNative {
    type VTable = abi_IDesktopWindowXamlSourceNative;
    fn iid() -> ::winrt::Guid {
        ::winrt::Guid::from_values(
            1019015615,
            12150,
            20124,
            [150, 171, 232, 75, 55, 151, 37, 84]
        )
    }
}

unsafe impl winrt::AbiTransferable for IDesktopWindowXamlSourceNative {
    type Abi = winrt::RawComPtr<IDesktopWindowXamlSourceNative>;
    fn get_abi(&self) -> Self::Abi {
        <::winrt::ComPtr<IDesktopWindowXamlSourceNative> as winrt::AbiTransferable>::get_abi(
            &self.ptr,
        )
    }
    fn set_abi(&mut self) -> *mut Self::Abi {
        <winrt::ComPtr<IDesktopWindowXamlSourceNative> as winrt::AbiTransferable>::set_abi(
            &mut self.ptr,
        )
    }
}

#[repr(transparent)]
#[derive(Default)] 
pub struct IDesktopWindowXamlSourceNative {
    ptr: winrt::ComPtr<IDesktopWindowXamlSourceNative>,
}

impl IDesktopWindowXamlSourceNative {
    pub fn attach_to_window(
        &self,
        hwnd: RawPtr,
    ) -> winrt::Result<()> {
        use winrt::AbiTransferable;
        let this = self.get_abi().expect("The `this` pointer was null when calling method");
        #[allow(unused_unsafe)]
        unsafe {
            (this.vtable().attach_to_window)(this, hwnd).ok()
        }
    }

    pub fn get_window_handle(
        &self,
    ) -> winrt::Result<RawPtr> {
        use winrt::AbiTransferable;

        let this = self.get_abi().expect("The `this` pointer was null when calling method");        
        #[allow(unused_unsafe)]
        unsafe {
            let mut result = std::ptr::null_mut();
            (this.vtable().get_window_handle)(this, &mut result).and_then(|| result)
        }

    }
}

impl std::clone::Clone for IDesktopWindowXamlSourceNative {
    fn clone(&self) -> Self {
        Self {
            ptr: self.ptr.clone()
        }
    }
}

impl From<&DesktopWindowXamlSource> for IDesktopWindowXamlSourceNative {
    fn from(value: &DesktopWindowXamlSource) -> IDesktopWindowXamlSourceNative {
        <DesktopWindowXamlSource as ::winrt::ComInterface>::query(value)
    }
}

impl From<DesktopWindowXamlSource> for IDesktopWindowXamlSourceNative {
    fn from(value: DesktopWindowXamlSource) -> IDesktopWindowXamlSourceNative {
        std::convert::From::from(&value)
    }
}