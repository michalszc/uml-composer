use druid::widget::{
    Axis, Tabs, TabsEdge, TabsTransition,
    Flex, Label, Either, Painter,
    ControllerHost, Align, Click
};
use druid::{
    AppLauncher, Data, Lens, Widget,
    WidgetExt, WindowDesc, theme, Color,
    RenderContext, EventCtx, Env, FileDialogOptions,
    FileSpec, commands, Handled, Target,
    AppDelegate, Command, DelegateCtx
};
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
    advanced: DynamicTabsData,
}

pub struct UIBuilder;
impl UIBuilder {
    pub fn new() -> Self {
        UIBuilder {}
    }
    pub fn build(&self) {
        // describe the main window
        let main_window = WindowDesc::new(self.build_root_widget())
        .title("UML - COMPOSER")
        .window_size((700.0, 400.0));

        // create the initial app state
        let initial_state = AppState {
            advanced: DynamicTabsData::new(),
        };

        // start the application
        AppLauncher::with_window(main_window)
            .delegate(Delegate)
            .log_to_console()
            .launch(initial_state)
            .expect("Failed to launch application");
    }

    fn build_root_widget(&self) -> impl Widget<AppState> {

        let open_dialog_options = UIBuilder::get_open_dialog_options();

        Flex::column()
            .with_flex_child(
                Either::new(
                    |data: &AppState, &_| data.advanced.tabs.is_empty(),
                    self.create_default_window(),
                    Flex::column()
                        .with_flex_child(
                            Flex::row()
                            .with_child(
                                self.create_custom_btn(String::from("Open"), move |ctx, _data: &mut AppState, _e| {
                                        ctx.submit_command(druid::commands::SHOW_OPEN_PANEL.with(open_dialog_options.clone()));
                                    }
                                ))
                            .with_child(
                                self.create_custom_btn(String::from("Save"), move|ctx, _data, _e| {
                                    ctx.submit_command(SAVE_TAB.to(TAB_ID));
                                }))
                            .with_child(
                                self.create_custom_btn(String::from("New"), |ctx, data: &mut AppState, _e| {
                                    data.advanced.add_empty_tab();
                                    ctx.submit_command(SET_LAST_ACTIVE_TAB.to(TAB_ID));
                                }))
                            .with_child(
                                self.create_custom_btn(String::from("Preview"),  |ctx, _data: &mut AppState, _e| {
                                    ctx.submit_command(PREVIEW_TAB.to(TAB_ID));
                                }))
                            , 0.1)
                        .with_flex_child(
                            Tabs::for_policy(DynamicTabs)
                                    .with_axis(Axis::Horizontal)
                                    .with_edge(TabsEdge::Leading)
                                    .with_transition(TabsTransition::Slide(
                                        Duration::from_millis(100).as_nanos() as Nanos //Default::default()
                                    ))
                                    .controller(TabsControler)
                                    .with_id(TAB_ID)
                                    .lens(AppState::advanced)
                            , 0.9)
                    ),
                1.0)
    
    }

    fn create_default_window(&self) -> Flex<AppState> {
        let open_dialog_options = UIBuilder::get_open_dialog_options();
        Flex::column()
            .with_flex_child(
                Label::new("Open or create a new file")
                        .with_text_size(24.)
                        .center()
                , 0.5)
            .with_flex_child(
                Flex::row()
                    .with_flex_child(
                        self.create_custom_btn(String::from("Open"), move |ctx, _data: &mut AppState, _e| 
                                ctx.submit_command(druid::commands::SHOW_OPEN_PANEL.with(open_dialog_options.clone()))
                            )
                        , 0.5)
                    .with_flex_child(
                        self.create_custom_btn(String::from("New"), |_ctx, data: &mut AppState, _e| 
                            data.advanced.add_empty_tab()
                        )
                        , 0.5)
            , 0.5)
    }

    fn create_custom_btn(&self, text: String, f: impl Fn(&mut EventCtx, &mut AppState, &Env) + 'static) -> ControllerHost<Align<AppState>, Click<AppState>> {
        Label::new(text)
            .with_text_size(24.)
            .padding(6.)
            .background(Painter::new(|ctx, _, env| {
                let bounds = ctx.size().to_rect();
        
                ctx.fill(bounds, &env.get(theme::BACKGROUND_LIGHT));
    
                if ctx.is_hot() {
                    ctx.stroke(bounds.inset(-0.5), &Color::WHITE, 1.0);
                }
        
                if ctx.is_active() {
                    ctx.fill(bounds, &Color::rgb8(0x71, 0x71, 0x71));
                }
            }))
            .center()
            .on_click(f)
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
        if let Some(file_info) = cmd.get(commands::SAVE_FILE_AS) {
            let tab_data = data.advanced.tabs.get(data.advanced.current_tab).unwrap();
            let file_content = tab_data.content.clone();
            if let Err(e) = std::fs::write(file_info.path(), file_content) {
                tracing::error!("Error writing file: {e}");
            }
            return Handled::Yes;
        }
        if let Some(file_info) = cmd.get(commands::OPEN_FILE) {
            match std::fs::read_to_string(file_info.path()) {
                Ok(s) => {
                    data.advanced.add_tab(DynamicTabData {
                        is_svg: false,
                        name: file_info.path.file_name().unwrap().to_owned().to_str().unwrap().to_owned(),
                        content: s
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
