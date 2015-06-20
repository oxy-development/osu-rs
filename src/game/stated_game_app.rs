use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub enum GameState {
    Loading,
    Settings,
    Initial,
    SongChoose,
    SongPlaying,
    Pause,
    PlayResult,
    PlayFail,

    //Reserved
    Download,
    ReplayView,
}

pub trait StateMachine {
    fn add_state(&mut self, state: GameState, paths: &Vec<GameState>);
    fn set_state(&mut self, state: GameState);
}

#[derive(Clone)]
pub struct GameApp {
    states: HashMap<GameState, Vec<GameState>>,
    pub current_state: GameState,
}

impl GameApp {
    pub fn new() -> GameApp {
        GameApp {
            current_state: GameState::Loading,
            states: HashMap::new()
        }
    }
}

impl StateMachine for GameApp {

    fn set_state(&mut self, state: GameState) {
        let st:Vec<GameState> = self.states.get(&self.current_state).unwrap().clone();
        match st.binary_search_by(|p| p.cmp(&state) ) {
            Ok(s) => {
                self.current_state = state;
            }
            _ => {}
        };
    }

    fn add_state(&mut self, state: GameState, paths: &Vec<GameState>) {
        self.states.insert( state, paths.clone().to_owned() );
    }
}
