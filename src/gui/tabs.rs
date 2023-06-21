use std::fs::File;
use std::io::Read;
use std::panic;

use druid::im::Vector;
use druid::widget::{
    Label,
    TabInfo,
    TabsPolicy,
    Controller,
    Tabs,
    Either,
    Image
};
use druid::{
    Data,
    Lens,
    Selector,
    WidgetId,
    EventCtx, 
    Event,
    Env,
    Widget,
    ImageBuf,
    WidgetExt,
    Color
};

use crate::uml_parser::UmlParser;

use super::text_editor::TextEditor;
use super::ui_builer::UIBuilder;

pub const ACTIVE_TAB: Selector = Selector::new("tab.active");
pub const SAVE_TAB: Selector = Selector::new("tab.save");
pub const PREVIEW_TAB: Selector = Selector::new("tab.preview");
pub const SET_ACTIVE_TAB: Selector = Selector::new("set.tab.active");
pub const SET_LAST_ACTIVE_TAB: Selector = Selector::new("set.last.tab.active");
pub const TAB_ID: WidgetId = WidgetId::reserved(1);

#[derive(Data, Clone, Lens, Debug)]
pub struct DynamicTabData {
    pub is_svg: bool,
    pub name: String,
    pub content: String,
    pub file_path: String,
    pub error: String
}

#[derive(Data, Clone, Lens)]
pub struct DynamicTabsData {
    text: String,
    last_tab: usize,
    removed_tabs: usize,
    pub current_tab: usize,
    pub tabs: Vector<DynamicTabData>,
}

impl DynamicTabsData {
    pub fn new() -> Self {
        DynamicTabsData {
            text: String::from(""),
            current_tab: usize::MAX,
            last_tab: 0,
            removed_tabs: 0,
            tabs: Vector::new()
        }
    }

    pub fn get_index(&self, name: String) -> Option<usize> {
        for (index, value) in self.tabs.clone().iter().enumerate() {
            if value.name == name {
                return Some(index);
            }
        }

        None
    }
    
    pub fn add_tab(&mut self, tab_data: DynamicTabData) {
        self.last_tab += 1;
        self.tabs.push_back(tab_data);
    }

    fn count_occurrences(&mut self, text: String) -> usize {
        self.tabs.clone().iter()
                .filter(|x| x.name.starts_with(&text))
                .count()
    }

    pub fn add_empty_tab(&mut self) { 
        self.last_tab += 1;
        let empty_count = self.count_occurrences(String::from("New"));
        self.tabs.push_back(
            DynamicTabData {
                is_svg: false,
                name: String::from(format!("New ({})", empty_count+1)),
                content: String::from(""),
                file_path: String::from(""),
                error: String::from(""),
            }
        );
    }

    pub fn add_svg_preview(&mut self, name: String, content: String, error: String) {
        self.last_tab += 1;
        let svg_name = String::from(format!("SVG {}", name.replace(".uml", "")));
        let dynamic_tab_data = DynamicTabData {
            is_svg: true,
            name: svg_name.clone(),
            content,
            file_path: String::from(""),
            error
        };
        match self.get_index(svg_name.clone()) {
            Some(index) => {
                self.current_tab = index;
                self.remove_tab(index);
                self.tabs.insert(index, dynamic_tab_data);
            },
            None => {
                self.current_tab = self.last_tab - 1;
                self.tabs.push_back(dynamic_tab_data);
            },
        };
    }

    fn remove_tab(&mut self, idx: usize) {
        if idx >= self.tabs.len() {
            tracing::warn!("Attempt to remove non existent tab at index {}", idx)
        } else {
            self.last_tab -= 1;
            self.removed_tabs += 1;
            self.tabs.remove(idx);
        }
    }

    // This provides a key that will monotonically increase as interactions occur.
    fn tabs_key(&self) -> (usize, usize) {
        (self.last_tab, self.removed_tabs)
    }
}


#[derive(Clone, Data)]
pub struct DynamicTabs;

impl TabsPolicy for DynamicTabs {
    type Key = String;
    type Build = ();
    type Input = DynamicTabsData;
    type LabelWidget = Label<DynamicTabsData>;
    type BodyWidget = Either<DynamicTabsData>;

    fn tabs_changed(&self, old_data: &DynamicTabsData, data: &DynamicTabsData) -> bool {
        old_data.tabs_key() != data.tabs_key()
    }

    fn tabs(&self, data: &DynamicTabsData) -> Vec<Self::Key> {
        data.tabs.clone().into_iter().map(|v| v.name).collect()
    }

    fn tab_info(&self, key: Self::Key, data: &DynamicTabsData) -> TabInfo<DynamicTabsData> {
        let tab_data = data.tabs.get(data.get_index(key).unwrap()).unwrap();

        TabInfo::new(format!("{}", tab_data.name), true)
    }

    fn tab_body(&self, key: Self::Key, data: &DynamicTabsData) -> Self::BodyWidget {
        let index = data.get_index(key.clone()).unwrap();
        let tab_data = data.tabs.get(index).unwrap();

        let img = match File::open("output.png") {
            Ok(mut file) => {
                let mut buf = Vec::new();
                file.read_to_end(&mut buf).unwrap();
                Image::new(ImageBuf::from_data(&buf).unwrap())
            },
            Err(_) => {
                Image::new(ImageBuf::empty())
            }
        };

        Either::new(
            move |d: &DynamicTabsData, _| {
                let tab = d.tabs.get(index).unwrap();
                tab.is_svg
            },
            Either::new(
                                move |d: &DynamicTabsData, _| {
                                let tab = d.tabs.get(index).unwrap();
                                tab.error.len() > 0
                            },
                            Label::new(tab_data.error.clone())
                                        .with_text_color(Color::RED),
                            img
                        ),
            TextEditor::new(tab_data.content.clone())
                            .expand()
        )
    }

    fn close_tab(&self, key: Self::Key, data: &mut DynamicTabsData) {
        if let Some(idx) = data.get_index(key) {
            data.remove_tab(idx)
        }
    }

    fn tab_label(
        &self,
        _key: Self::Key,
        info: TabInfo<Self::Input>,
        _data: &Self::Input,
    ) -> Self::LabelWidget {
        Self::default_make_label(info)
    }
}

pub struct TabsControler;
impl Controller<DynamicTabsData, Tabs<DynamicTabs>> for TabsControler {
    fn event(
        &mut self,
        child: &mut Tabs<DynamicTabs>,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut DynamicTabsData,
        env: &Env,
    ) {
        match event {
            Event::MouseDown(_mouse_event) => {
                data.current_tab = child.tab_index();
                child.event(ctx, event, data, env);
            }
            Event::Command(cmd) if cmd.is(PREVIEW_TAB) => {
                data.current_tab = child.tab_index();
                let index = data.current_tab;
                let tab_data = data.tabs.get(index).unwrap();
                let result = panic::catch_unwind(|| {
                    UmlParser::parse(tab_data.content.clone().as_str())
                });
                if result.is_ok() {
                    println!("asd");
                    let content = result.ok().unwrap();
                    data.add_svg_preview(tab_data.name.clone(), content, String::from(""));
                } else {
                    tracing::error!("Creating preview failed...");
                    let error = result.err().unwrap();
                    if let Some(err) = error.downcast_ref::<&str>() {
                        // The panic value is a string
                        data.add_svg_preview(tab_data.name.clone(), String::from(""), err.to_string());
                    } else if let Some(err) = error.downcast_ref::<String>() {
                        // The panic value is a String
                        data.add_svg_preview(tab_data.name.clone(), String::from(""), err.to_string());
                    } else {
                        // Unknown panic value type
                        data.add_svg_preview(tab_data.name.clone(), String::from(""), String::from("Unknown error occurred"));
                    }
                }
                child.set_tab_index(data.current_tab);
            },
            Event::Command(cmd) if cmd.is(SAVE_TAB) => {
                let save_dialog_options_svg = UIBuilder::get_save_dialog_options(true);
                let save_dialog_options_uml = UIBuilder::get_save_dialog_options(false);
                data.current_tab = child.tab_index();
                let index = data.current_tab;
                let tab_data = data.tabs.get(index).unwrap();
                ctx.submit_command(druid::commands::SHOW_SAVE_PANEL.with(if tab_data.is_svg {
                    save_dialog_options_svg.clone()
                } else {
                    save_dialog_options_uml.clone()
                }));
            },
            Event::Command(cmd) if cmd.is(SET_ACTIVE_TAB) => {
                child.set_tab_index(data.current_tab);
            },
            Event::Command(cmd) if cmd.is(SET_LAST_ACTIVE_TAB) => {
                data.current_tab = data.last_tab - 1;
                child.set_tab_index(data.current_tab);
            },
            _ => child.event(ctx, event, data, env),
        }
    }
}
