use crate::view::button::button;
use crate::view::grid::row;
use crate::view::text::text;
use crate::view::text_field::text_field;
use seed::prelude::Node;

////////////////////////////////////////////////////////////////
// TYPES //
////////////////////////////////////////////////////////////////

pub struct Model {
    game_name_field: String,
    player_name_field: String,
}

#[derive(Clone)]
pub enum Msg {
    StartClicked,
    GameNameFieldUpdated(String),
    PlayerNameFieldUpdated(String),
}

impl Model {
    pub fn update_game_name(&mut self, new_name: String) {
        self.game_name_field = new_name;
    }

    pub fn update_player_name(&mut self, new_name: String) {
        self.player_name_field = new_name;
    }
}

////////////////////////////////////////////////////////////////
// INIT //
////////////////////////////////////////////////////////////////

pub fn init() -> Model {
    Model {
        game_name_field: String::new(),
        player_name_field: String::new(),
    }
}

////////////////////////////////////////////////////////////////
// UPDATE //
////////////////////////////////////////////////////////////////

pub fn update(msg: Msg, model: &mut Model) {
    match msg {
        Msg::StartClicked => {
            // ????? Send http request!!!!
        }
        Msg::GameNameFieldUpdated(new_field) => {
            model.update_game_name(new_field);
        }
        Msg::PlayerNameFieldUpdated(new_field) => {
            model.update_player_name(new_field);
        }
    }
}

////////////////////////////////////////////////////////////////
// VIEW //
////////////////////////////////////////////////////////////////

pub fn view(model: &Model) -> Vec<Node<Msg>> {
    row::many(vec![
        row::single(text("start game")),
        row::single(
            text_field(model.player_name_field.as_str(), |event| {
                Msg::PlayerNameFieldUpdated(event)
            })
            .placeholder("player name")
            .view(),
        ),
        row::single(
            text_field(model.game_name_field.as_str(), |event| {
                Msg::GameNameFieldUpdated(event)
            })
            .placeholder("game name")
            .view(),
        ),
        row::single(button("start", |_| Msg::StartClicked).view()),
    ])
    .map_rows(|row| row.center(true))
    .view()
}
