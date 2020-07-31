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
use winrt::Object;

use std::{process, thread, time::Duration};

use paho_mqtt as mqtt;

// This will attempt to reconnect to the broker. It can be called after
// connection is lost. In this example, we try to reconnect several times,
// with a few second pause between each attempt. A real system might keep
// trying indefinitely, with a backoff, or something like that.
fn try_reconnect(cli: &mqtt::Client) -> bool {
    println!("Connection lost. Waiting to retry connection");
    for _ in 0..12 {
        thread::sleep(Duration::from_millis(5000));
        if cli.reconnect().is_ok() {
            println!("Successfully reconnected");
            return true;
        }
    }
    println!("Unable to reconnect after several attempts.");
    false
}

fn run() -> winrt::Result<()> {
    ro_initialize(RoInitType::MultiThreaded)?;
    let _manager = WindowsXamlManager::initialize_for_current_thread()?;
    let desktop_source =
        winrt::factory::<DesktopWindowXamlSource, IDesktopWindowXamlSourceFactory>()?
            .create_instance(Object::default(), &mut Object::default())?;
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

    let xaml_container = winrt::factory::<StackPanel, IStackPanelFactory>()?
        .create_instance(Object::default(), &mut Object::default())?;

    let tb = winrt::factory::<TextBox, ITextBoxFactory>()?
        .create_instance(Object::default(), &mut Object::default())?;

    xaml_container.children()?.append(&tb)?;
    xaml_container.update_layout()?;
    desktop_source.set_content(&xaml_container)?;

    let host = "tcp://localhost:1883".to_string();

    // Create the client. Use an ID for a persistent session.
    // A real system should try harder to use a unique ID.
    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(host)
        .client_id("rust_sync_consumer")
        .finalize();

    let mut cli = mqtt::Client::new(create_opts).unwrap_or_else(|e| {
        println!("Error creating the client: {:?}", e);
        process::exit(1);
    });

    // Initialize the consumer before connecting
    let rx = cli.start_consuming();

    // Define the set of options for the connection
    let lwt = mqtt::MessageBuilder::new()
        .topic("test")
        .payload("Sync consumer lost connection")
        .finalize();

    let conn_opts = mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(Duration::from_secs(20))
        .clean_session(true)
        .will_message(lwt)
        .finalize();

    let subscriptions = ["test", "hello"];
    let qos = [1, 1];

    // Make the connection to the broker
    println!("Connecting to the MQTT broker...");
    match cli.connect(conn_opts) {
        Ok(rsp) => {
            if let Some(conn_rsp) = rsp.connect_response() {
                println!(
                    "Connected to: '{}' with MQTT version {}",
                    conn_rsp.0, conn_rsp.1
                );
                if !conn_rsp.2 {
                    // Register subscriptions on the server
                    println!("Subscribing to topics, with requested QoS: {:?}...", qos);

                    match cli.subscribe_many(&subscriptions, &qos) {
                        Ok(qosv) => println!("QoS granted: {:?}", qosv),
                        Err(e) => {
                            println!("Error subscribing to topics: {:?}", e);
                            cli.disconnect(None).unwrap();
                            process::exit(1);
                        }
                    }
                }
            }
        }
        Err(e) => {
            println!("Error connecting to the broker: {:?}", e);
            process::exit(1);
        }
    }

    event_loop.run(move |event, _, control_flow| {
        if *control_flow == ControlFlow::Exit {
            // If we're still connected, then disconnect now,
            // otherwise we're already disconnected.
            if cli.is_connected() {
                println!("Disconnecting");
                cli.unsubscribe_many(&subscriptions).unwrap();
                cli.disconnect(None).unwrap();
            }
        } else {
            // If we get a None message, check if we got disconnected,
            // and then try a reconnect.
            for msg in rx.try_iter() {
                if let Some(msg) = msg {
                    println!("{}", msg);
                    xaml_container.children().unwrap().append({
                        let new_text = TextBlock::new().unwrap();
                        new_text.set_text(msg.to_string()).unwrap();
                        new_text
                    }).unwrap();
                } else if cli.is_connected() || !try_reconnect(&cli) {
                    break;
                }
            }
        }

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
