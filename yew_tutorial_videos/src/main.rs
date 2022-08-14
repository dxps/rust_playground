use reqwasm::http::Request;
use yew::prelude::*;
use yew_tutorial_videos::videos::Video;

#[function_component(App)]
fn app() -> Html {
    let videos = use_state(|| vec![]);
    {
        let videos = videos.clone();
        use_effect_with_deps(
            move |_| {
                let videos = videos.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_videos: Option<Vec<Video>> =
                        match Request::get("/tutorial/data.json").send().await {
                            Ok(resp) => match resp.json().await {
                                Ok(data) => data,
                                Err(e) => {
                                    log::error!("Json deserialization error: {e}");
                                    None
                                }
                            },
                            Err(e) => {
                                log::error!("Request error: {e}");
                                None
                            }
                        };
                    if fetched_videos.is_some() {
                        videos.set(fetched_videos.unwrap());
                    }
                });
                || ()
            },
            (),
        );
    }

    let selected_video = use_state(|| None);
    let on_video_click = {
        let selected_video = selected_video.clone();
        Callback::from(move |video: Video| selected_video.set(Some(video)))
    };
    let select_no_video = {
        let selected_video = selected_video.clone();
        Callback::from(move |_| selected_video.set(None))
    };
    let details = selected_video.as_ref().map(|video| {
        html! {
            <VideoDetails video={video.clone()} />
        }
    });

    html! {
        <>
            <h1>{ "RustConf Explorer" }</h1>
            <div>
                <h3>{"Videos to watch"}</h3>
                <span style="font-size:small" onclick={select_no_video}>{"Clear selection"}</span>
                <VideosList videos={(*videos).clone()} on_click={on_video_click} />
            </div>
            { for details }
        </>
    }
}

#[derive(Clone, PartialEq, Properties)]
struct VideosListProps {
    videos: Vec<Video>,
    on_click: Callback<Video>,
}

#[function_component(VideosList)]
fn videos_list(props: &VideosListProps) -> Html {
    props
        .videos
        .iter()
        .map(|video| {
            let on_video_click = {
                let on_click = props.on_click.clone();
                let video = video.clone();
                Callback::from(move |_| on_click.emit(video.clone()))
            };
            html! {
                <p onclick={on_video_click}>{ format!("{}: {}", video.speaker, video.title) }</p>
            }
        })
        .collect()
}

#[derive(Clone, Properties, PartialEq)]
struct VideosDetailsProps {
    video: Video,
}

#[function_component(VideoDetails)]
fn video_details(props: &VideosDetailsProps) -> Html {
    html! {
        <div>
            <h3>{ props.video.title.clone() }</h3>
            <img src="https://via.placeholder.com/640x360.png?text=Video+Player+Placeholder" alt="video thumbnail" />
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("Starting up ...");
    yew::start_app::<App>();
}
