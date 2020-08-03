winrt::build!(
    dependencies
        os
    types
        windows::ui::xaml::hosting::{DesktopWindowXamlSource, IDesktopWindowXamlSourceFactory, WindowsXamlManager}
        windows::ui::xaml::controls::{ScrollViewer, StackPanel, TextBox, TextBlock}
);

fn main() {
    build()
}