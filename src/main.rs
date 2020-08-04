mod interop;

use interop::{ro_initialize, IDesktopWindowXamlSourceNative, RoInitType};

use bindings::windows::ui::xaml::{controls::*, hosting::*};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::windows::WindowExtWindows,
    window::WindowBuilder,
};

use std::ptr;

use winrt::PropertyValue;

fn run() -> winrt::Result<()> {
    ro_initialize(RoInitType::MultiThreaded)?;
    let _manager = WindowsXamlManager::initialize_for_current_thread()?;
    let desktop_source = DesktopWindowXamlSource::new()?;
    let interop: IDesktopWindowXamlSourceNative = desktop_source.clone().into();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    window.set_title("WinUI");
    let win32_window_id = window.id();

    let hwnd = window.hwnd();
    interop.attach_to_window(hwnd)?;
    let hwnd_xaml_island = interop.get_window_handle()?;

    let size = window.inner_size();
    unsafe {
        crate::SetWindowPos(
            hwnd_xaml_island,
            ptr::null_mut(),
            0,
            0,
            size.width as i32,
            size.height as i32,
            /*SWP_SHOWWINDOW*/ 0x40,
        );
    }

    let sp = StackPanel::new()?;
    let tb = TextBox::new()?;
    let sv = ScrollViewer::new()?;
    let lv = ListView::new()?;

    let object = PropertyValue::create_string("hello")?;

    lv.items()?.append(object)?;
    sp.children()?.append(&tb)?;
    sp.children()?.append(&lv)?;
    sp.update_layout()?;
    desktop_source.set_content(&sv)?;
    sv.set_content(&sp)?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == win32_window_id => *control_flow = ControlFlow::Exit,
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                window_id,
            } if window_id == win32_window_id => {
                unsafe {
                    crate::SetWindowPos(
                        hwnd_xaml_island,
                        ptr::null_mut(),
                        0,
                        0,
                        size.width as i32,
                        size.height as i32,
                        /*SWP_SHOWWINDOW*/ 0x40,
                    );
                }
            }
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
        flags: u32,
    ) -> i32;
}
