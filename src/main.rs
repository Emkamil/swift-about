mod contributors;
mod metadata;
use gettextrs::*;
use gtk4::prelude::*;
use gtk4::{
    Application, Box as GtkBox, Button, Dialog, Grid, Image, Label, Orientation, ResponseType,
    ScrolledWindow, SizeGroup, SizeGroupMode, TextBuffer, TextView, glib,
};
use libadwaita::prelude::*;
use libadwaita::{
    ActionRow, ApplicationWindow as AdwApplicationWindow, HeaderBar, PreferencesGroup,
    PreferencesPage, ViewStack, ViewSwitcherTitle,
};
use std::{env, fs};

fn main() -> glib::ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.iter().any(|arg| arg == "--version" || arg == "-v") {
        println!("Version: {}", metadata::LFBE_VERSION);

        println!("Codename: {}", metadata::LFBE_CODENAME);

        return glib::ExitCode::SUCCESS;
    }

    setlocale(LocaleCategory::LcAll, "");

    let _ = bindtextdomain("lfbe-about", "/usr/share/locale");
    let _ = textdomain("lfbe-about");

    libadwaita::init().expect("Nie udało się zainicjować Libadwaita");
    let app = Application::builder()
        .application_id("org.lfbe.about")
        .build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let window = AdwApplicationWindow::builder()
        .application(app)
        .title(gettext("About"))
        .default_width(820)
        .default_height(620)
        .build();

    let header_bar = HeaderBar::new();
    let view_switcher_title = ViewSwitcherTitle::new();
    header_bar.set_title_widget(Some(&view_switcher_title));

    let view_stack = ViewStack::new();
    view_switcher_title.set_stack(Some(&view_stack));

    let sys_holder = GtkBox::new(Orientation::Vertical, 0);
    let info_holder = GtkBox::new(Orientation::Vertical, 0);
    let credits_holder = GtkBox::new(Orientation::Vertical, 0);
    let copy_holder = GtkBox::new(Orientation::Vertical, 0);

    let sp = view_stack.add_titled(&sys_holder, Some("system"), &gettext("System"));
    sp.set_icon_name(Some("computer-symbolic"));

    let ip = view_stack.add_titled(&info_holder, Some("information"), &gettext("Information"));
    ip.set_icon_name(Some("dialog-information-symbolic"));

    let crp = view_stack.add_titled(&credits_holder, Some("credits"), &gettext("Credits"));
    crp.set_icon_name(Some("contact-new-symbolic"));

    let cop = view_stack.add_titled(&copy_holder, Some("copyright"), &gettext("Copyright"));
    cop.set_icon_name(Some("emblem-readonly-symbolic"));

    sys_holder.append(&create_system_page());

    view_stack.connect_visible_child_name_notify(move |stack| {
        if let (Some(name), Some(child)) = (stack.visible_child_name(), stack.visible_child()) {
            let holder = child.downcast_ref::<GtkBox>().expect("Must be a GtkBox");

            if holder.first_child().is_none() {
                match name.as_str() {
                    "information" => holder.append(&create_information_page()),
                    "credits" => holder.append(&create_credits_page()),
                    "copyright" => holder.append(&create_copyright_page()),
                    _ => {}
                }
                println!("Lazy Loading: Załadowano zasoby dla sekcji {}", name);
            }
        }
    });

    let main_box = GtkBox::new(Orientation::Vertical, 0);
    main_box.append(&header_bar);

    view_stack.set_vexpand(true);
    view_stack.set_hexpand(true);
    main_box.append(&view_stack);

    window.set_content(Some(&main_box));
    window.present();
}

fn create_system_page() -> PreferencesPage {
    let page = PreferencesPage::new();
    let group = PreferencesGroup::new();
    group.set_title(&gettext("System Information"));

    let content_box = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(24)
        .margin_top(32)
        .margin_bottom(32)
        .margin_start(32)
        .margin_end(32)
        .build();

    let content_grid = Grid::new();
    content_grid.set_column_spacing(40);
    content_grid.set_row_spacing(12);

    let logo = Image::builder()
        .icon_name(get_os_logo())
        .pixel_size(128)
        .halign(gtk4::Align::Start)
        .valign(gtk4::Align::Start)
        .build();

    content_grid.attach(&logo, 0, 0, 1, 1);

    let info_grid = Grid::new();
    info_grid.set_column_spacing(20);
    info_grid.set_row_spacing(8);

    let label_size_group = SizeGroup::new(SizeGroupMode::Horizontal);

    struct InfoRow {
        label: String,
        value: String,
        is_header_start: bool,
    }

    let info_rows = vec![
        InfoRow {
            label: gettext("Device"),
            value: get_hostname(),
            is_header_start: false,
        },
        InfoRow {
            label: gettext("Operating System"),
            value: read_os_release().unwrap_or_default(),
            is_header_start: false,
        },
        InfoRow {
            label: gettext("System Type"),
            value: if cfg!(target_pointer_width = "64") {
                "64-bit".into()
            } else {
                "32-bit".into()
            },
            is_header_start: false,
        },
        InfoRow {
            label: gettext("LFBE Version"),
            value: format!("{} ({})", metadata::LFBE_VERSION, metadata::LFBE_CODENAME),
            is_header_start: true,
        },
        InfoRow {
            label: gettext("Kernel Version"),
            value: get_kernel_version(),
            is_header_start: false,
        },
        InfoRow {
            label: gettext("Windowing System"),
            value: detect_windowing_system(),
            is_header_start: false,
        },
        InfoRow {
            label: gettext("Processor"),
            value: get_cpu_model(),
            is_header_start: true,
        },
        InfoRow {
            label: gettext("Memory"),
            value: get_total_memory(),
            is_header_start: false,
        },
        /*InfoRow {
            label: gettext("Graphics"),
            value: detect_gpu()
            is_header_start: false,
        },*/
    ];

    for (i, row) in info_rows.into_iter().enumerate() {
        let key_label = Label::builder()
            .label(&row.label)
            .halign(gtk4::Align::Start)
            .xalign(0.0)
            .css_classes(vec!["dim-label".to_string()])
            .build();

        label_size_group.add_widget(&key_label);

        let value_label = Label::builder()
            .label(&row.value)
            .halign(gtk4::Align::Start)
            .xalign(0.0)
            .selectable(true)
            .build();

        if row.is_header_start {
            key_label.set_margin_top(20);

            value_label.set_margin_top(20);
        }

        info_grid.attach(&key_label, 0, i as i32, 1, 1);

        info_grid.attach(&value_label, 1, i as i32, 1, 1);
    }

    content_grid.attach(&info_grid, 1, 0, 1, 1);

    content_box.append(&content_grid);

    group.add(&content_box);

    page.add(&group);

    page
}

fn read_os_release() -> Option<String> {
    let content = fs::read_to_string("/etc/os-release").ok()?;

    let mut name = None;

    let mut version = None;

    for line in content.lines() {
        if let Some(stripped) = line.strip_prefix("PRETTY_NAME=") {
            return Some(stripped.trim_matches('"').to_string());
        }
        if let Some(stripped) = line.strip_prefix("NAME=") {
            name = Some(stripped.trim_matches('"').to_string());
        }
        if let Some(stripped) = line.strip_prefix("VERSION=") {
            version = Some(stripped.trim_matches('"').to_string());
        }
    }
    match (name, version) {
        (Some(n), Some(v)) => Some(format!("{} {}", n, v)),

        (Some(n), _) => Some(n),

        _ => None,
    }
}

fn detect_windowing_system() -> String {
    let session_type = env::var("XDG_SESSION_TYPE")
        .unwrap_or_else(|_| "Unknown".to_string())
        .to_lowercase();

    match session_type.as_str() {
        "wayland" => "Wayland".to_string(),

        "x11" => "X11".to_string(),

        _ => "Unknown".to_string(),
    }
}

fn get_hostname() -> String {
    fs::read_to_string("/proc/sys/kernel/hostname")
        .ok()
        .map(|v| v.trim().to_string())
        .unwrap_or_else(|| "Unknown".to_string())
}

fn get_kernel_version() -> String {
    fs::read_to_string("/proc/sys/kernel/osrelease")
        .ok()
        .map(|v| v.trim().to_string())
        .unwrap_or_else(|| "Unknown Kernel".to_string())
}

fn get_total_memory() -> String {
    fs::read_to_string("/proc/meminfo")
        .ok()
        .and_then(|content| {
            content
                .lines()
                .find(|l| l.starts_with("MemTotal:"))
                .map(|l| {
                    let kb = l
                        .split_whitespace()
                        .nth(1)
                        .and_then(|v| v.parse::<f64>().ok())
                        .unwrap_or(0.0);
                    format!("{:.1} GB", kb / 1024.0 / 1024.0)
                })
        })
        .unwrap_or_else(|| "N/A".to_string())
}

fn get_cpu_model() -> String {
    fs::read_to_string("/proc/cpuinfo")
        .ok()
        .and_then(|content| {
            content
                .lines()
                .find(|l| l.starts_with("model name"))
                .map(|l| l.split(':').nth(1).unwrap_or("N/A").trim().to_string())
        })
        .unwrap_or_else(|| "Unknown CPU".to_string())
}

fn get_os_logo() -> String {
    if let Ok(content) = fs::read_to_string("/etc/os-release") {
        let mut id = None;

        for line in content.lines() {
            if let Some(stripped) = line.strip_prefix("LOGO=") {
                let value = stripped.trim_matches('"').trim_matches('\'').to_string();

                if !value.is_empty() {
                    return value;
                }
            }

            if let Some(stripped) = line.strip_prefix("ID=") {
                id = Some(stripped.trim_matches('"').trim_matches('\'').to_string());
            }
        }

        if let Some(dist_id) = id {
            return format!("{}-logo", dist_id);
        }
    }

    "dialog-information".to_string()
}

fn create_information_page() -> PreferencesPage {
    let page = PreferencesPage::new();

    let group = PreferencesGroup::new();

    let vbox = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(24)
        .margin_top(32)
        .margin_bottom(32)
        .margin_start(32)
        .margin_end(32)
        .build();

    let logo = Image::builder()
        .icon_name("preferences-system-details")
        .pixel_size(96)
        .halign(gtk4::Align::Center)
        .build();

    vbox.append(&logo);

    let desc = Label::builder()

        .label(gettext("LFBE is a set of programs that together provide a fully functional desktop environment."))

        .wrap(true)

        .halign(gtk4::Align::Start)

        .build();

    vbox.append(&desc);

    let components = vec![
        (
            "preferences-system-windows",
            "Window Manager",
            "labwc",
            "Manages the placement of windows on the screen.",
        ),
        (
            "application-menu",
            "Panel",
            "lfbe-panel",
            "Provides a place for window buttons and menus.",
        ),
        (
            "preferences-other",
            "Desktop Manager",
            "lfbe-desktop",
            "Sets desktop backgrounds and icons.",
        ),
        (
            "folder",
            "File Manager",
            "nemo",
            "Manages files in a modern and fast way.",
        ),
    ];

    for (icon, title, app, desc_text) in components {
        let comp_hbox = GtkBox::new(Orientation::Horizontal, 16);

        let comp_icon = Image::builder().icon_name(icon).pixel_size(48).build();

        let comp_vbox = GtkBox::new(Orientation::Vertical, 4);

        let comp_title = Label::builder()
            .label(format!("{} ({})", gettext(title), app))
            .halign(gtk4::Align::Start)
            .css_classes(vec!["heading".to_string()])
            .build();

        let comp_desc = Label::builder()
            .label(gettext(desc_text))
            .wrap(true)
            .halign(gtk4::Align::Start)
            .css_classes(vec!["dim-label".to_string()])
            .build();

        comp_vbox.append(&comp_title);

        comp_vbox.append(&comp_desc);

        comp_hbox.append(&comp_icon);

        comp_hbox.append(&comp_vbox);

        vbox.append(&comp_hbox);
    }

    group.add(&vbox);

    page.add(&group);

    page
}

fn create_credits_page() -> PreferencesPage {
    let page = PreferencesPage::new();

    for group_data in contributors::LFBE_CONTRIBUTORS {
        let pref_group = PreferencesGroup::new();

        pref_group.set_title(group_data.name);

        for contributor in group_data.contributors {
            let row = ActionRow::builder()
                .title(contributor.name)
                .subtitle(contributor.email)
                .build();

            pref_group.add(&row);
        }

        page.add(&pref_group);
    }

    page
}

fn create_copyright_page() -> PreferencesPage {
    let page = PreferencesPage::new();

    let main_group = PreferencesGroup::new();
    main_group.set_title(&gettext("Legal Information"));

    let info_label = Label::builder()
        .label(gettext(
            "This program is distributed under the following licenses:",
        ))
        .wrap(true)
        .xalign(0.0)
        .margin_bottom(12)
        .build();
    main_group.add(&info_label);
    page.add(&main_group);

    let list_group = PreferencesGroup::new();

    let licenses = [
        ("GNU General Public License v3.0", "gpl-3.0"),
        ("GNU Lesser General Public License", "lgpl-3.0"),
        ("The BSD 3-Clause License", "bsd-3-clause"),
    ];

    for (full_name, file_key) in licenses {
        let row = libadwaita::ActionRow::builder()
            .title(full_name)
            .activatable(true) // Sprawia, że wiersz reaguje na kliknięcie
            .build();

        let arrow = Image::from_icon_name("go-next-symbolic");
        row.add_suffix(&arrow);

        let key = file_key.to_string();

        row.connect_activated(move |r| {
            if let Some(window) = r
                .root()
                .and_then(|root| root.downcast::<AdwApplicationWindow>().ok())
            {
                let license_content = get_license_text(&key);
                show_license_dialog(&window, &license_content);
            }
        });

        list_group.add(&row);
    }

    page.add(&list_group);
    page
}

fn show_license_dialog(parent: &AdwApplicationWindow, license_text: &str) {
    let window = libadwaita::Window::builder()
        .transient_for(parent)
        .modal(true)
        .default_width(600)
        .default_height(700)
        .build();

    let main_vbox = GtkBox::builder().orientation(Orientation::Vertical).build();

    let header_bar = libadwaita::HeaderBar::builder()
        .show_end_title_buttons(true)
        .title_widget(&libadwaita::WindowTitle::new("License Information", ""))
        .build();
    main_vbox.append(&header_bar);

    let scrolled = ScrolledWindow::builder()
        .vexpand(true)
        .margin_top(24)
        .margin_bottom(12)
        .margin_start(24)
        .margin_end(24)
        .build();

    scrolled.add_css_class("card");
    scrolled.add_css_class("view");

    let text_view = TextView::builder()
        .editable(false)
        .cursor_visible(false)
        .wrap_mode(gtk4::WrapMode::Word)
        .margin_top(20)
        .margin_bottom(20)
        .margin_start(20)
        .margin_end(20)
        .build();

    let buffer = TextBuffer::new(None);
    buffer.set_text(license_text);
    text_view.set_buffer(Some(&buffer));
    scrolled.set_child(Some(&text_view));

    let button_container = GtkBox::builder()
        .orientation(Orientation::Horizontal)
        .margin_bottom(24)
        .halign(gtk4::Align::Center)
        .build();

    let close_button = gtk4::Button::builder()
        .label(gettext("Close"))
        .width_request(120)
        .build();

    close_button.add_css_class("pill");

    close_button.connect_clicked(glib::clone!(@weak window => move |_| {
        window.close();
    }));

    button_container.append(&close_button);

    main_vbox.append(&scrolled);
    main_vbox.append(&button_container);

    window.set_content(Some(&main_vbox));
    window.present();
}

fn get_license_text(name: &str) -> String {
    let path = format!("/usr/share/lfbe/licenses/{}.txt", name.to_lowercase());

    fs::read_to_string(&path).unwrap_or_else(|_| {
        format!(
            "{}: {}\n{}",
            gettext("Error"),
            gettext("License file not found"),
            path
        )
    })
}
