use druid::widget::{
    Axis, Tabs, 
    TabsEdge, TabsTransition,
    Flex
};
use druid::{
    AppLauncher, Data, Lens, Widget,
    WidgetExt, WindowDesc, Env,
    FileDialogOptions, FileSpec,
    commands, Handled, Target,
    AppDelegate, Command, DelegateCtx,
    Menu, WindowId, LocalizedString,
    MenuItem, SysMods
};
use druid::platform_menus;
use instant::Duration;
use super::tabs::{
    DynamicTabsData,
    DynamicTabs,
    DynamicTabData,
    TabsControler,
    TAB_ID,
    SET_LAST_ACTIVE_TAB,
    SAVE_TAB,
    PREVIEW_TAB
};

type Nanos = u64;

#[derive(Data, Clone, Lens)]
struct AppState {
    dynamictabdata: DynamicTabsData,
}

pub struct UIBuilder;
impl UIBuilder {
    pub fn new() -> Self {
        UIBuilder {}
    }
    pub fn build(&self) {
        // describe the main window
        let main_window = WindowDesc::new(self.build_root_widget())
        .menu(UIBuilder::make_menu)
        .title("UML - COMPOSER")
        .window_size((700.0, 400.0));

        // create the initial app state
        let initial_state = AppState {
            dynamictabdata: DynamicTabsData::new(),
        };

        // start the application
        AppLauncher::with_window(main_window)
            .delegate(Delegate)
            .log_to_console()
            .launch(initial_state)
            .expect("Failed to launch application");
    }

    fn build_root_widget(&self) -> impl Widget<AppState> {
        Flex::column()
            .with_flex_child(
            Tabs::for_policy(DynamicTabs)
                    .with_axis(Axis::Horizontal)
                    .with_edge(TabsEdge::Leading)
                    .with_transition(TabsTransition::Slide(
                        Duration::from_millis(100).as_nanos() as Nanos //Default::default()
                    ))
                    .controller(TabsControler)
                    .with_id(TAB_ID)
                    .lens(AppState::dynamictabdata),
            1.0)
    }

    #[allow(unused_assignments, unused_mut)]
    fn make_menu(_window_id: Option<WindowId>, _app_state: &AppState, _env: &Env) -> Menu<AppState> {
        let open_dialog_options = UIBuilder::get_open_dialog_options();
        let mut base = Menu::empty();
        #[cfg(target_os = "macos")]
        {
            base = base.entry(
                Menu::new("File")
                    .entry(platform_menus::mac::file::new_file())
                    .entry(
                        MenuItem::new(LocalizedString::new("common-menu-file-open"))
                            .on_activate(|ctx, _data: &mut AppState, _env| {
                                ctx.submit_command(druid::commands::SHOW_OPEN_PANEL.with(open_dialog_options.clone()))
                            }).hotkey(SysMods::Cmd, "o")
                    )
                    .entry(platform_menus::mac::file::save())
                    .entry(platform_menus::mac::file::save_as())
                    .entry(
                        MenuItem::new(LocalizedString::new("common-menu-file-save-as"))
                            .on_activate(move |ctx, _data: &mut AppState, _env| {
                                ctx.submit_command(SAVE_TAB.to(TAB_ID));
                            })
                            .hotkey(SysMods::CmdShift, "S")
                    )
                    .separator()
                    .entry(platform_menus::mac::file::close())
            );
        }
        #[cfg(any(
            target_os = "windows",
            target_os = "freebsd",
            target_os = "linux",
            target_os = "openbsd"
        ))]
        {
            base = base.entry(
                Menu::new(LocalizedString::new("common-menu-file-menu"))
                    .entry(platform_menus::win::file::new())
                    .entry(
                        MenuItem::new(LocalizedString::new("common-menu-file-open"))
                            .on_activate(move |ctx, _data: &mut AppState, _env| {
                                ctx.submit_command(druid::commands::SHOW_OPEN_PANEL.with(open_dialog_options.clone()))
                            }).hotkey(SysMods::Cmd, "o")
                    )
                    .entry(platform_menus::win::file::save())
                    .entry(
                        MenuItem::new(LocalizedString::new("common-menu-file-save-as"))
                            .on_activate(|ctx, _data: &mut AppState, _env| {
                                ctx.submit_command(SAVE_TAB.to(TAB_ID));
                            }).hotkey(SysMods::CmdShift, "S")
                    )
                    .separator()
                    .entry(platform_menus::win::file::exit())
            );
        }
        base.entry(
            MenuItem::new("Preview")
            .on_activate(|ctx, _data: &mut AppState, _env| {
                ctx.submit_command(PREVIEW_TAB.to(TAB_ID));
            })
            .hotkey(SysMods::Cmd, "p")
            .enabled_if(move |data, _env| {
                match data.dynamictabdata.tabs.get(data.dynamictabdata.current_tab) {
                    Some(tab_data) => !tab_data.is_svg,
                    None => false
                }
            })
        )
        .refresh_on(|old_data, data, _env| {
            old_data.dynamictabdata.current_tab != data.dynamictabdata.current_tab
        })
    }
    
    pub fn get_open_dialog_options() -> FileDialogOptions {
        let uml = FileSpec::new("UML composer file", &["uml"]);
        FileDialogOptions::new()
            .allowed_types(vec![uml])
            .default_type(uml)
            .default_name("diagram.uml")
            .name_label("Source")
            .title("Where did you put that file?")
            .button_text("Open")
    }

    pub fn get_save_dialog_options(is_svg: bool) -> FileDialogOptions {
        let uml = FileSpec::new("UML composer file", &["uml"]);
        let svg = FileSpec::new("SVG file", &["svg"]);
        let allowed_types = match is_svg {
            true => vec![svg],
            false => vec![uml],
        };
        let default_name = match is_svg {
            true => "diagram.svg",
            false => "diagram.uml"
        };
        FileDialogOptions::new()
            .allowed_types(allowed_types)
            .default_type(uml)
            .default_name(default_name)
            .name_label("Target")
            .title("Choose a target for this beautiful UML diagram file")
            .button_text("Save")
    }

}

struct Delegate;
impl AppDelegate<AppState> for Delegate {
    fn command(
        &mut self,
        ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut AppState,
        _env: &Env,
    ) -> Handled {
        if let Some(_f) = cmd.get(commands::NEW_FILE) {
            data.dynamictabdata.add_empty_tab();
            ctx.submit_command(SET_LAST_ACTIVE_TAB.to(TAB_ID));
            return Handled::Yes;
        }
        if let Some(_f) = cmd.get(commands::SAVE_FILE) {
            let tab_data = data.dynamictabdata.tabs.get(data.dynamictabdata.current_tab).unwrap();
            if  tab_data.file_path.len() > 0 {
                let file_content = tab_data.content.clone();
                if let Err(e) = std::fs::write(tab_data.file_path.clone(), file_content) {
                    tracing::error!("Error writing file: {e}");
                }
            } else {
                ctx.submit_command(SAVE_TAB.to(TAB_ID));
            }
            return Handled::Yes;
        }
        if let Some(file_info) = cmd.get(commands::SAVE_FILE_AS) {
            let tab_data = data.dynamictabdata.tabs.get(data.dynamictabdata.current_tab).unwrap();
            let file_content = tab_data.content.clone();
            if let Err(e) = std::fs::write(file_info.path(), file_content) {
                tracing::error!("Error writing file: {e}");
            }
            return Handled::Yes;
        }
        if let Some(file_info) = cmd.get(commands::OPEN_FILE) {
            match std::fs::read_to_string(file_info.path()) {
                Ok(s) => {
                    data.dynamictabdata.add_tab(DynamicTabData {
                        is_svg: false,
                        name: file_info.path.file_name().unwrap().to_owned().to_str().unwrap().to_owned(),
                        content: s,
                        file_path: file_info.path.to_str().unwrap().to_string()
                    });
                    ctx.submit_command(SET_LAST_ACTIVE_TAB.to(TAB_ID));
                }
                Err(e) => {
                    tracing::error!("Error opening file: {e}")
                }
            }
            return Handled::Yes;
        }
        Handled::No
    }
}
