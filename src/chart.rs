use std::time::Duration;
use charts_rs::*;
use leptos::{component, create_effect, create_signal, ev::MouseEvent, leptos_dom::helpers::debounce, logging::log, set_interval, set_timeout, spawn_local, view, IntoView, SignalGet, SignalSet};

#[component]
pub fn Chart() -> impl IntoView {
    let theme = "shadcn";

    let (candlestick_values, set_candlestick_values) = create_signal(vec![
        20.0, 34.0, 10.0, 38.0, 40.0, 35.0, 30.0, 50.0, 31.0, 38.0, 33.0, 44.0, 38.0, 15.0, 5.0, 42.0,
    ]);
    let (svg_content, set_svg_content) = create_signal(String::new());

    let update_value = move || {
        let mut values = candlestick_values.get().clone();
        log!("{:?}", values); 
        values[1] += 1.0;
        log!("{:?}", values);
        set_candlestick_values.set(values);
    };

    let render_chart = move || {
        let mut candlestick = CandlestickChart::new_with_theme(
            vec![("", candlestick_values.get().clone()).into()],
            vec![
                "2017-10-24".to_string(),
                "2017-10-25".to_string(),
                "2017-10-26".to_string(),
                "2017-10-27".to_string(),
            ],
            theme,
        );

        candlestick.height = 500.0;
        candlestick.svg().unwrap()
    };

    create_effect(move |_| {
        let new_svg = render_chart();
        set_svg_content.set(new_svg);

        let interval = Duration::from_secs(5);
        set_timeout(move || {
            update_value();
        }, interval);
    });

    let chart_svg = move || {
        view! {
            <svg 
                width="100%" 
                height="100%" 
                viewBox="0 0 1000 500"
                inner_html={svg_content}
            ></svg>
        }
    };

    view! {
        {
            chart_svg()
        }
    }
}
