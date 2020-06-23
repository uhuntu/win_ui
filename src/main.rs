mod interop;

use interop::{RoInitType, ro_initialize, IDesktopWindowXamlSourceNative};

use bindings::windows::ui::xaml::{controls::*, hosting::*};
use bindings::microsoft::ui::xaml::controls::*;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::windows::WindowExtWindows,
    window::WindowBuilder,
};

use winrt::Object;
use std::ptr;

fn run() -> winrt::Result<()> {
    ro_initialize(RoInitType::MultiThreaded)?;
    let _ = WindowsXamlManager::initialize_for_current_thread()?;
    let desktop_source = winrt::factory::<DesktopWindowXamlSource, IDesktopWindowXamlSourceFactory>()?.create_instance(Object::default(), &mut Object::default())?;
    let interop: IDesktopWindowXamlSourceNative = desktop_source.clone().into();
    
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    window.set_title("WinUI");
    let win32_window_id = window.id();


    let hwnd = window.hwnd();
    interop.attach_to_window(hwnd)?;
    let hwnd_xaml_island = interop.get_window_handle()?;

    let size = window.inner_size();
    unsafe { crate::SetWindowPos(hwnd_xaml_island, ptr::null_mut(), 0, 0, size.width as i32, size.height as i32, /*SWP_SHOWWINDOW*/ 0x40); }

    let xaml_container = winrt::factory::<StackPanel, IStackPanelFactory>()?.create_instance(Object::default(), &mut Object::default())?;

    let tb = winrt::factory::<TextBox, ITextBoxFactory>()?.create_instance(Object::default(), &mut Object::default())?;
    
    // This causes an error: (exit code: 0xc0000005, STATUS_ACCESS_VIOLATION)
    let _ = winrt::factory::<NumberBox, INumberBoxFactory>()?.create_instance(Object::default(), &mut Object::default())?;

    xaml_container.children()?.append(&tb)?;
    xaml_container.update_layout()?;
    desktop_source.set_content(xaml_container)?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == win32_window_id => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}

fn main() {
    let result = run();

    // We do this for nicer HRESULT printing when errors occur.
    if let Err(error) = result {
        error.code().unwrap();
    }
}

#[link(name = "user32")]
extern "stdcall" {
    fn SetWindowPos(
        hwnd: *mut core::ffi::c_void,
        hwnd_insert_after: *mut core::ffi::c_void,
        x: i32,
        y: i32,
        cx: i32,
        cy: i32,
        flags: u32
    ) -> i32;
}