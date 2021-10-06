use yew::prelude::*;

pub struct Crosshair
{
}

#[derive(Properties, Debug, Clone, PartialEq)]
pub struct Props
{
    #[prop_or_default]
    pub class: Option<&'static str>,
    #[prop_or_default]
    pub size: Option<i64>,
    #[prop_or_default]
    pub color: Option<&'static str>,
    #[prop_or_default]
    pub fill: Option<&'static str>,
    #[prop_or_default]
    pub stroke_width: Option<i64>,
    #[prop_or_default]
    pub stroke_linecap: Option<&'static str>,
    #[prop_or_default]
    pub stroke_linejoin: Option<&'static str>,
}

impl Component for Crosshair
{
    type Properties = Props;
    type Message = ();

    fn create(_ctx: &Context<Self>) -> Self
    {
        Self
        {
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html
    {
        html! {
            <svg
                class={ctx.props().class.unwrap_or("")}
                width={ctx.props().size.unwrap_or(24).to_string()}
                height={ctx.props().size.unwrap_or(24).to_string()}
                viewBox="0 0 24 24"
                fill={ctx.props().fill.unwrap_or("none")}
                stroke={ctx.props().color.unwrap_or("currentColor")}
                stroke-width={ctx.props().stroke_width.unwrap_or(2).to_string()}
                stroke-linecap={ctx.props().stroke_linecap.unwrap_or("round")}
                stroke-linejoin={ctx.props().stroke_linejoin.unwrap_or("round")}
            >
                <circle cx="12" cy="12" r="10"></circle><line x1="22" y1="12" x2="18" y2="12"></line><line x1="6" y1="12" x2="2" y2="12"></line><line x1="12" y1="6" x2="12" y2="2"></line><line x1="12" y1="22" x2="12" y2="18"></line>
            </svg>
        }
    }
}
