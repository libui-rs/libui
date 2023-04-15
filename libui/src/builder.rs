/// Creates layout code from a compact, declarative and hierarchical UI description.
///
/// # Example
///
/// For a more example, see the [builder example application](https://github.com/libui-rs/libui/tree/development/libui/examples) in the repository.
///
/// ```no_run
/// extern crate libui;
/// use libui::prelude::*;
///
/// fn main() {
///     let ui = UI::init().unwrap();
///
///     libui::layout! { &ui,
///         let layout = VerticalBox(padded: true) {
///             Compact: let form = Form(padded: true) {
///                 (Compact, "User"): let tb_user = Entry()
///                 (Compact, "Password"): let tb_passwd = Entry()
///             }
///             Stretchy: let bt_submit = Button("Submit")
///         }
///     }
///
///     let mut window = Window::new(&ui, "Builder Example", 320, 200,
///         WindowType::NoMenubar);
///
///     window.set_child(layout);
///     window.show();
///     ui.main();
/// }
/// ```
#[macro_export]
macro_rules! layout {

    // ---------------------- Controls without children -----------------------

    // Button
    [ $ui:expr ,
        let $ctl:ident = Button ( $text:expr )
    ] => [
        #[allow(unused_mut)]
        let mut $ctl = libui::controls::Button::new($text);
    ];

    // Checkbox
    [ $ui:expr ,
        let $ctl:ident = Checkbox ( $text:expr $( , checked: $checked:expr )? )
    ] => [
        #[allow(unused_mut)]
        let mut $ctl = libui::controls::Checkbox::new($text);
        $( $ctl.set_checked($checked); )?
    ];

    // ColorButton
    [ $ui:expr ,
        let $ctl:ident = ColorButton ()
    ] => [
        #[allow(unused_mut)]
        let mut $ctl = libui::controls::ColorButton::new();
    ];

    // Combobox
    [ $ui:expr ,
        let $ctl:ident = Combobox ( $( selected: $selected:expr )? )
        { $( $option:expr ),* }
    ] => [
        #[allow(unused_mut)]
        let mut $ctl = libui::controls::Combobox::new();
        $( $ctl.append($option); )*
        $( $ctl.set_selected($selected); )?
    ];

    // DateTimePicker
    [ $ui:expr ,
        let $ctl:ident = DateTimePicker ( $kind:ident )
    ] => [
        #[allow(unused_mut)]
        let mut $ctl = libui::controls::DateTimePicker::new(
            libui::controls::DateTimePickerKind::$kind);
    ];

    // EditableCombobox
    [ $ui:expr ,
        let $ctl:ident = EditableCombobox ()
        { $( $option:expr ),* }
    ] => [
        #[allow(unused_mut)]
        let mut $ctl = libui::controls::EditableCombobox::new();
        $( $ctl.append($option); )*
    ];

    // Entry
    [ $ui:expr ,
        let $ctl:ident = Entry ()
    ] => [
        #[allow(unused_mut)]
        let mut $ctl = libui::controls::Entry::new();
    ];

    // FontButton
    [ $ui:expr ,
        let $ctl:ident = FontButton ()
    ] => [
        #[allow(unused_mut)]
        let mut $ctl = libui::controls::FontButton::new();
    ];

    // HorizontalSeparator
    [ $ui:expr ,
        let $ctl:ident = HorizontalSeparator ()
    ] => [
        #[allow(unused_mut)]
        let mut $ctl = libui::controls::HorizontalSeparator::new();
    ];

    // Label
    [ $ui:expr ,
        let $ctl:ident = Label ( $text:expr )
    ] => [
        #[allow(unused_mut)]
        let mut $ctl = libui::controls::Label::new($text);
    ];

    // MultilineEntry
    [ $ui:expr ,
        let $ctl:ident = MultilineEntry ()
    ] => [
        #[allow(unused_mut)]
        let mut $ctl = libui::controls::MultilineEntry::new();
    ];

    // PasswordEntry
    [ $ui:expr ,
        let $ctl:ident = PasswordEntry ()
    ] => [
        #[allow(unused_mut)]
        let mut $ctl = libui::controls::PasswordEntry::new();
    ];

    // RadioButtons
    [ $ui:expr ,
        let $ctl:ident = RadioButtons ( $( selected: $selected:expr )? )
        { $( $option:expr ),* }
    ] => [
        #[allow(unused_mut)]
        let mut $ctl = libui::controls::RadioButtons::new();
        $( $ctl.append($option); )*
        $( $ctl.set_selected($selected); )?
    ];

    // SearchEntry
    [ $ui:expr ,
        let $ctl:ident = SearchEntry ()
    ] => [
        #[allow(unused_mut)]
        let mut $ctl = libui::controls::SearchEntry::new();
    ];

    // Slider
    [ $ui:expr ,
        let $ctl:ident = Slider ( $min:expr , $max:expr )
    ] => [
        #[allow(unused_mut)]
        let mut $ctl = libui::controls::Slider::new($min, $max);
    ];

    // Spacer
    [ $ui:expr ,
        let $ctl:ident = Spacer ()
    ] => [
        #[allow(unused_mut)]
        let mut $ctl = libui::controls::Spacer::new();
    ];

    // Spinbox, limited
    [ $ui:expr ,
        let $ctl:ident = Spinbox ( $min:expr , $max:expr )
    ] => [
        #[allow(unused_mut)]
        let mut $ctl = libui::controls::Spinbox::new($min, $max);
    ];

    // Spinbox, unlimited
    [ $ui:expr ,
        let $ctl:ident = Spinbox ()
    ] => [
        #[allow(unused_mut)]
        let mut $ctl = libui::controls::Spinbox::new_unlimited();
    ];

    // ProgressBar
    [ $ui:expr ,
        let $ctl:ident = ProgressBar ()
    ] => [
        #[allow(unused_mut)]
        let mut $ctl = libui::controls::ProgressBar::new();
    ];

    // ----------------- Controls with children (Containers) ------------------

    // Form
    [ $ui:expr ,
        let $ctl:ident = Form ( $( padded: $padded:expr )? )
        { $(
            ( $strategy:ident, $name:expr ) :
            let $child:ident = $type:ident ($($opt:tt)*) $({$($body:tt)*})?
        )* }
    ] => [
        #[allow(unused_mut)]
        let mut $ctl = libui::controls::Form::new();
        $( $ctl.set_padded($padded); )?
        $(
            libui::layout! { $ui, let $child = $type ($($opt)*) $({$($body)*})? }
            $ctl.append($name, $child.clone(),
                    libui::controls::LayoutStrategy::$strategy);
        )*
    ];

    // Group
    [ $ui:expr ,
        let $ctl:ident = Group ( $title:expr $( , margined: $margined:expr )? )
        { $(
            let $child:ident = $type:ident ($($opt:tt)*) $({$($body:tt)*})?
        )? }
    ] => [
        #[allow(unused_mut)]
        let mut $ctl = libui::controls::Group::new($title);
        $( $ctl.set_margined($margined); )?
        $(
            libui::layout! { $ui, let $child = $type ($($opt)*) $({$($body)*})? }
            $ctl.set_child($child.clone());
        )?
    ];

    // HorizontalBox
    [ $ui:expr ,
        let $ctl:ident = HorizontalBox ( $( padded: $padded:expr )? )
        { $(
            $strategy:ident :
            let $child:ident = $type:ident ($($opt:tt)*) $({$($body:tt)*})?
        )* }
    ] => [
        #[allow(unused_mut)]
        let mut $ctl = libui::controls::HorizontalBox::new();
        $( $ctl.set_padded($padded); )?
        $(
            libui::layout! { $ui, let $child = $type ($($opt)*) $({$($body)*})? }
            $ctl.append($child.clone(),
                        libui::controls::LayoutStrategy::$strategy);
        )*
    ];

    // LayoutGrid
    [ $ui:expr ,
        let $ctl:ident = LayoutGrid ( $( padded: $padded:expr )? )
        { $(
            ( $x:expr , $y:expr ) ( $xspan:expr , $yspan:expr )
            $expand:ident ( $halign:ident , $valign:ident ) :
            let $child:ident = $type:ident ($($opt:tt)*) $({$($body:tt)*})?
        )* }
    ] => [
        #[allow(unused_mut)]
        let mut $ctl = libui::controls::LayoutGrid::new();
        $( $ctl.set_padded($padded); )?
        $(
            libui::layout! { $ui, let $child = $type ($($opt)*) $({$($body)*})? }
            $ctl.append($child.clone(), $x, $y, $xspan, $yspan,
                        libui::controls::GridExpand::$expand,
                        libui::controls::GridAlignment::$halign,
                        libui::controls::GridAlignment::$valign);
        )*
    ];

    // TabGroup
    [ $ui:expr ,
        let $ctl:ident = TabGroup ()
        { $(
            ( $name:expr $( , margined: $margined:expr )? ) :
            let $child:ident = $type:ident ($($opt:tt)*) $({$($body:tt)*})?
        )* }
    ] => [
        #[allow(unused_mut)]
        let mut $ctl = libui::controls::TabGroup::new();
        $(
            libui::layout! { $ui, let $child = $type ($($opt)*) $({$($body)*})? }
            let __tab_n = $ctl.append($name, $child.clone());
            $( $ctl.set_margined(__tab_n - 1, $margined); )?
        )*
    ];

    // VerticalBox
    [ $ui:expr ,
        let $ctl:ident = VerticalBox ( $( padded: $padded:expr )? )
        { $(
            $strategy:ident :
            let $child:ident = $type:ident ($($opt:tt)*) $({$($body:tt)*})?
        )* }
    ] => [
        #[allow(unused_mut)]
        let mut $ctl = libui::controls::VerticalBox::new();
        $( $ctl.set_padded($padded); )?
        $(
            libui::layout! { $ui, let $child = $type ($($opt)*) $({$($body)*})? }
            $ctl.append($child.clone(),
                        libui::controls::LayoutStrategy::$strategy);
        )*
    ];
}



/// Creates menu entries for the applications main menu from a hierarchical description.
/// 
/// # Example
///
/// ```no_run
/// extern crate libui;
/// use libui::prelude::*;
/// 
/// fn main() {
///     let ui = UI::init().unwrap();
/// 
///     libui::menu! { &ui,
///         let menu_file = Menu("File") {
///             let menu_file_open = MenuItem("Open")
///             let menu_file_save = MenuItem("Save")
///             let menu_file_close = MenuItem("Close")
///             Separator()
///             let menu_file_quit = MenuItem("Exit")
///         }
///         let menu_settings = Menu("Settings") {
///             let menu_settings_num = MenuItem("Line Numbers", checked: true)
///         }
///         let menu_help = Menu("Help") {
///             let menu_help_about = MenuItem("About")
///         }
///     }
/// 
///     let mut window = Window::new(&ui, "Title", 300, 200, WindowType::HasMenubar);
///     libui::layout! { &ui,
///         let layout = VerticalBox() { }
///     }
///     window.set_child(layout);
///     window.show();
///     ui.main();
/// }
/// ```
#[macro_export]
macro_rules! menu {

    // End recursion
    [@impl $parent:ident,] => [];

    // MenuItem
    [@impl $parent:ident,
        let $item:ident = MenuItem ( $text:expr )
        $($tail:tt)*
    ] => [
        #[allow(unused_mut)]
        let mut $item = $parent.append_item($text);
        libui::menu! { @impl $parent, $($tail)* }
    ];

    // Checked MenuItem
    [@impl $parent:ident,
        let $item:ident = MenuItem ( $text:expr, checked: $checked:expr )
        $($tail:tt)*
    ] => [
        #[allow(unused_mut)]
        let mut $item = $parent.append_check_item($text);
        $item.set_checked($checked);
        libui::menu! { @impl $parent, $($tail)* }
    ];

    // Separator
    [@impl $parent:ident,
        Separator ( )
        $($tail:tt)*
    ] => [
        $parent.append_separator();
        libui::menu! { @impl $parent, $($tail)* }
    ];

    // Menu
    [ $ui:expr ,
        $( 
            let $menu:ident = Menu ( $name:expr )
            {
                $($tail:tt)*
            }
        )+
    ] => [
        $(
            #[allow(unused_mut)]
            let mut $menu = libui::menus::Menu::new( $name );
            libui::menu! { @impl $menu, $($tail)* }
        )+
    ];
}
