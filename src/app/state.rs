use crate::scenes::gameplay::Board;
use crate::scenes::menu::level_select::LevelSelection;
use crate::scenes::menu::main_menu::MainMenu;
use crate::scenes::story::research_center::ResearchCenterView;
use crate::scenes::story::runner::StoryRunner;
use crate::systems::audio::AudioManager;
use crate::systems::localization::I18nManager;
use crate::systems::save::SaveManager;

#[derive(PartialEq)]
pub enum AppState {
    Menu,
    LevelSelect,
    Playing,
    StoryIntro,
    LevelPlaying,
    StoryOutro,
    LevelComplete,
    LevelFailed,
    Paused,
    ResearchCenter,
    Exiting,
}

pub struct App {
    pub(crate) state: AppState,
    pub(crate) menu: MainMenu,
    pub(crate) board: Board,
    pub(crate) audio: AudioManager,
    pub(crate) save: SaveManager,
    pub(crate) i18n: I18nManager,
    pub(crate) level_selection: LevelSelection,
    pub(crate) research_center: ResearchCenterView,
    pub(crate) fps_counter: f32,
    pub(crate) show_fps: bool,
    pub(crate) story_runner: Option<StoryRunner>,
    pub(crate) pending_level_id: Option<u32>,
}

impl App {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let save = SaveManager::new();
        let mut audio = AudioManager::new().await;
        audio.set_music_volume(save.data.music_volume);
        audio.set_sfx_volume(save.data.sfx_volume);
        let mut i18n = I18nManager::new("en");
        i18n.set_language(&save.data.language);

        let mut level_selection = LevelSelection::new();
        level_selection.level_manager.max_unlocked_level = save.data.max_unlocked_level;
        level_selection.level_manager.level_stars = save.data.level_stars.clone();

        let mut menu = MainMenu::new(&i18n);
        menu.music_volume = save.data.music_volume;
        menu.sfx_volume = save.data.sfx_volume;

        Ok(App {
            state: AppState::Menu,
            menu,
            level_selection,
            board: Board::new(),
            audio,
            save,
            i18n,
            fps_counter: 0.0,
            show_fps: false,
            story_runner: None,
            pending_level_id: None,
            research_center: ResearchCenterView::new(),
        })
    }
}


