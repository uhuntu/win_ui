winrt::build!(
    dependencies
        os
    types
        windows::ui::xaml::hosting::*
        windows::ui::xaml::controls::*
);

fn main() {
    build()
}