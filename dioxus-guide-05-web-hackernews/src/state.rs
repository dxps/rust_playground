use crate::model::StoryPageData;

#[derive(Clone, Debug)]
pub enum PreviewState {
    Unset,
    Loading,
    Loaded(StoryPageData),
}
