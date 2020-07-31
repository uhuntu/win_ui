winrt::build!(
    dependencies
        nuget: Microsoft.UI.Xaml
        os
    types
        windows::ui::xaml::hosting::{DesktopWindowXamlSource, IDesktopWindowXamlSourceFactory, WindowsXamlManager}
        windows::ui::xaml::controls::{StackPanel, IStackPanelFactory, TextBox, ITextBoxFactory}
        microsoft::ui::xaml::controls::{NumberBox, INumberBoxFactory}
);

fn main() {
    build()
}