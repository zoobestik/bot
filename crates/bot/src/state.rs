use serde::Serialize;

#[derive(Clone, Default, Serialize, serde::Deserialize)]
pub enum BotState {
    #[default]
    DialogState,
}
