use yew::prelude::*;

pub struct ShieldOff
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

impl Component for ShieldOff
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
                <path d="M19.69 14a6.9 6.9 0 0 0 .31-2V5l-8-3-3.16 1.18"></path><path d="M4.73 4.73L4 5v7c0 6 8 10 8 10a20.29 20.29 0 0 0 5.62-4.38"></path><line x1="1" y1="1" x2="23" y2="23"></line>
            </svg>
        }
    }
}
