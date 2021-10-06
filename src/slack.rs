use yew::prelude::*;

pub struct Slack
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

impl Component for Slack
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
                <path d="M14.5 10c-.83 0-1.5-.67-1.5-1.5v-5c0-.83.67-1.5 1.5-1.5s1.5.67 1.5 1.5v5c0 .83-.67 1.5-1.5 1.5z"></path><path d="M20.5 10H19V8.5c0-.83.67-1.5 1.5-1.5s1.5.67 1.5 1.5-.67 1.5-1.5 1.5z"></path><path d="M9.5 14c.83 0 1.5.67 1.5 1.5v5c0 .83-.67 1.5-1.5 1.5S8 21.33 8 20.5v-5c0-.83.67-1.5 1.5-1.5z"></path><path d="M3.5 14H5v1.5c0 .83-.67 1.5-1.5 1.5S2 16.33 2 15.5 2.67 14 3.5 14z"></path><path d="M14 14.5c0-.83.67-1.5 1.5-1.5h5c.83 0 1.5.67 1.5 1.5s-.67 1.5-1.5 1.5h-5c-.83 0-1.5-.67-1.5-1.5z"></path><path d="M15.5 19H14v1.5c0 .83.67 1.5 1.5 1.5s1.5-.67 1.5-1.5-.67-1.5-1.5-1.5z"></path><path d="M10 9.5C10 8.67 9.33 8 8.5 8h-5C2.67 8 2 8.67 2 9.5S2.67 11 3.5 11h5c.83 0 1.5-.67 1.5-1.5z"></path><path d="M8.5 5H10V3.5C10 2.67 9.33 2 8.5 2S7 2.67 7 3.5 7.67 5 8.5 5z"></path>
            </svg>
        }
    }
}
