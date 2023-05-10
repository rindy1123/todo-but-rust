use serde::{Deserialize, Serialize};
use yew::Properties;

#[derive(Serialize, Deserialize, Default, Clone, PartialEq, Properties)]
pub struct ToDo {
    pub id: String,
    pub done: bool,
    pub description: String,
}
