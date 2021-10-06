use yew::prelude::*;

pub struct Slash
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

impl Component for Slash
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
                <circle cx="12" cy="12" r="10"></circle><line x1="4.93" y1="4.93" x2="19.07" y2="19.07"></line>
            </svg>
        }
    }
}
