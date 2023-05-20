use crate::{prelude::*, tool_tips::ToolTip};
use belly::prelude::*;
use bevy::prelude::*;
use strum::IntoEnumIterator;

pub fn just_pressed<T: Copy + Eq + std::hash::Hash + Send + Sync + 'static>(
    key: T,
) -> impl FnMut(Res<Input<T>>) -> bool + Clone {
    move |current_state: Res<Input<T>>| current_state.just_pressed(key)
}

use crate::loading::LoadingProgress;
pub(super) fn setup_loading(mut commands: Commands) {
    commands.add(StyleSheet::load("ui/color-picker.ess"));
    commands.add(eml! {
        <body id="loading">
            <label value="ToInit" with=LoadingProgress/>
        </body>
    });
}

// Resource
pub(super) fn setup_ui(mut commands: Commands, mut is_init: Local<bool>) {
    if *is_init {
        return;
    }
    commands.add(eml! {
            <buttongroup c:left with=Tab>
                <for tab in = Tab::iter()>
                    <button c:tab value={format!("{:?}", tab)} with=tab>
                        <img c:icon src=tab.icon_path()/>
                    </button>
                </for>
            </buttongroup> // end left
    });
    commands.add(eml! {
        <div c:top>
            <label c:tool-tip value="tooltip" with=ToolTip/>
        </div> // end top
    });
    let bgc = BackgroundColor(Color::WHITE);
    commands.add(eml! {
        <div id="hotbar" c:bottom>
            <for id in = Slot::iter_hotbar()>
                <button s:background-color=managed() flat c:slot with=(id, Item, bgc)>
                    <img c:slot-icon/>
                </button>
            </for>
        </div> //end bottom
    });
    commands.add(eml! {
        <buttongroup c:right  with=Tool>
            <for tab in = Tool::iter()>
                <button c:tool with=tab>
                    <img c:icon src=tab.icon_path()/>
                </button>
            </for>
        </buttongroup> //end right
    });
    *is_init = true;
}

pub(super) fn detect_change(
    tab: Res<State<Tab>>,
    tool: Res<State<Tool>>,
    mut tabs: Query<&mut BtnGroup, (With<Tab>, Without<Tool>)>,
    mut tools: Query<&mut BtnGroup, (With<Tool>, Without<Tab>)>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if tab.is_changed() {
        if let Ok(mut btn) = tabs.get_single_mut() {
            btn.value = format!("{:?}", tab.0);
        }
        if tab.0 != Tab::Menu && current_state.0 != GameState::Playing {
            next_state.set(GameState::Playing);
        }
    }
    if tool.is_changed() {
        if let Ok(mut btn) = tools.get_single_mut() {
            btn.value = format!("{:?}", tool.0);
        }
    }
}

pub(super) fn open_menu(
    state: Res<State<GameState>>,
    mut elements: Elements,
    settings: Res<CameraSettings>,
) {
    elements.select(".menu").add_class("hidden");
    match state.0 {
        GameState::Loading => {}
        GameState::Playing => {}
        GameState::MainMenu => {
            let count = elements.select("#main.menu").entities();
            if count.is_empty() {
                spawn_main_menu(elements);
            } else {
                elements.select("#main.menu").remove_class("hidden");
            }
        }
        GameState::Settings => {
            let count = elements.select("#settings.menu").entities();
            if count.is_empty() {
                spawn_settings_menu(elements, &settings);
            } else {
                elements.select("#settings.menu").remove_class("hidden");
            }
        }
    }
}

fn spawn_main_menu(mut elements: Elements) {
    let playing = GameState::Playing;
    let settings = GameState::Settings;
    elements.commands().add(eml! {
        <div id="main" c:menu>
            <button with=playing>"Play"</button>
            <button with=settings>"Settings"</button>
        </div>
    })
}

const MOUSE_BUTTONS: [MouseButton; 3] =
    [MouseButton::Left, MouseButton::Right, MouseButton::Middle];

fn spawn_settings_menu(mut elements: Elements, settings: &CameraSettings) {
    // use belly::widgets
    let main = GameState::MainMenu;
    let sensitivity = SettingsSlider::Sensitivity;
    let speed = SettingsSlider::Speed;
    let move_cam = SettingsButton::MoveCamera;
    let rot_cam = SettingsButton::RotateCamera;
    let current_settings = *settings;
    elements.commands().add(eml!{
        <div id="settings" c:menu>
        <span id="sensitivity">
        "Sensitivity:"
        <slider c:sensitivity
        minimum="0.001"
        maximum="0.010"
                    // bind:value=to!(PlayerSettings:sensitivity)
                    value={current_settings.sensitivity}
                    bind:value=from!(CameraSettings:sensitivity)
                with=sensitivity />
        </span>
        <span id="speed">
        "Speed:"
        <slider c:speed
        minimum="0.0499"
        maximum="3"
                    value={current_settings.speed}
                    bind:value=from!(CameraSettings:speed)
                with=speed />
        </span>
        <span c:mouse-settings>
            "Move Button:"
            <buttongroup with=move_cam value={format!("{:?}", current_settings.move_cam)}>
            <for m_button in=MOUSE_BUTTONS>
                <button group="rotate_button" value={format!("{:?}", m_button)}>{format!("{:?}", m_button)}</button>
            </for>
            </buttongroup>
        </span>
        <span c:mouse-settings>
            "Rotate Button:"
            <buttongroup with=rot_cam value={format!("{:?}", current_settings.rotate_cam)}>
            <for m_button in=MOUSE_BUTTONS>
                <button group="rotate_button" value={format!("{:?}", m_button)}>{format!("{:?}", m_button)}</button>
            </for>
            </buttongroup>
        </span>
        <button with=main>"Back"</button>
        </div>
    })
}

pub(super) fn back_to_menu(mut next: ResMut<NextState<GameState>>) {
    next.set(GameState::MainMenu)
}
