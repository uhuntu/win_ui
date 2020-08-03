winrt::build!(
    dependencies
        os
    types
        windows::ui::xaml::hosting::{DesktopWindowXamlSource, IDesktopWindowXamlSourceFactory, WindowsXamlManager}
        windows::ui::xaml::controls::{StackPanel, IStackPanelFactory, TextBox, ITextBoxFactory}
);

fn main() {
    build()
}
