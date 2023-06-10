use druid::widget::{
    SvgData,
    Svg
};

pub struct SvgWidget {
    content: String
}
impl SvgWidget {
    pub fn new(content: String) -> SvgWidget {
        SvgWidget {
            content
        }
    }
    pub fn build(&self) -> Svg { //IdentityWrapper<ControllerHost<Svg, SVGControler>>
        let svg_content = match self.content.parse() {
            Ok(svg) => svg,
            Err(err) => {
                tracing::error!("{}", err);
                tracing::error!("Using an empty SVG instead.");
                SvgData::default()
            }
        };
        Svg::new(svg_content)
    }
}

// pub struct SVGControler;
// impl Controller<DynamicTabsData, Svg> for SVGControler {
//     fn event(
//         &mut self,
//         child:&mut Svg,
//         ctx: &mut EventCtx,
//         event: &Event,
//         data: &mut DynamicTabsData,
//         env: &Env,
//     ) {
//         match event {
//             Event::Command(cmd) if cmd.is(REDRAW) => {
//                 // ctx.request_paint();
//             },
//             _ => child.event(ctx, event, data, env),
//         }
//     }
// }
