use crate::*;

pub fn draw_main_menu(main_menu_options: &mut Vec<MenuOption>) {
    sprite!("title-screen");
    sprite!(
        "logo",
        x = 112,
        y = 50,
    );

    for menu_option in main_menu_options {
        menu_option.draw();
    }
    
    sprite!(
        "acornr",
        x = 166,
        y = 180,
    );
}

#[turbo::serialize]
pub enum GameFlowState {
    MainMenu,
    Credits,
    InGameSingle,
    InGameCoOp,
    WaitingForPlayer2,
}

impl fmt::Display for GameFlowState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let state_str = match self {
            GameFlowState::MainMenu => "Main Menu",
            GameFlowState::Credits => "Credits",
            GameFlowState::InGameSingle => "In Game Single",
            GameFlowState::InGameCoOp => "In Game CoOp",
            GameFlowState::WaitingForPlayer2 => "Waiting for Player 2",
        };
        write!(f, "{}", state_str)
    }
}

#[turbo::serialize]
pub struct MenuOption {
    text: String,
    position_x: i32,
    position_y: i32,
    pub is_selected: bool,
}

impl MenuOption {
    fn new(text: String, position_x: i32, position_y: i32, is_selected: bool) -> Self {
        Self {
            text,
            position_x,
            position_y,
            is_selected,
        }
    }
    
    fn draw(&self) {
        if self.is_selected {
            sprite!("menu-option-selected", x = self.position_x, y = self.position_y);
        } else {
            sprite!("menu-option", x = self.position_x, y = self.position_y);
        }
        text!(&self.text, x = self.position_x + 110 - self.text.len() as i32 * 4, y = self.position_y + 6, font = "large");
    }
}

pub fn get_main_menu_options() -> Vec<MenuOption> {
    let mut main_menu_options = Vec::new();
    
    let x_coord = 63;
    let y_coord = 90;
    let option_height = 30;
    let options = vec!["START", "Co-Op", "Credits"];
    for (idx, option) in options.iter().enumerate() {
        main_menu_options.push(MenuOption::new(String::from(*option), x_coord, y_coord + option_height * idx as i32, idx == 0));
    }
        
    main_menu_options
}

pub fn handle_input(options: &mut Vec<MenuOption>) -> Option<String> {
    let gp = gamepad::get(0);
    
    if gp.up.just_pressed() {
        cycle_option(options, CycleDirection::Up);
    }

    if gp.down.just_pressed() {
        cycle_option(options, CycleDirection::Down);
    }

    if gp.b.just_pressed() {
        let selected_option = options.iter().position(|x| x.is_selected);
        match selected_option {
            Some(idx) => {
                return Some(options[idx].text.clone());
            },
            None => {},
        }
    }
    
    None
}

enum CycleDirection {
    Up,
    Down
}

fn cycle_option(options: &mut Vec<MenuOption>, direction: CycleDirection) {
    if options.is_empty() {
        return;
    }

    // Find the index of the currently selected option
    let current_index = options.iter().position(|opt| opt.is_selected);

    // Clear all selections first (optional, depending on your logic)
    for opt in options.iter_mut() {
        opt.is_selected = false;
    }

    let next_index = match direction {
        CycleDirection::Down => {
            match current_index {
                Some(i) => (i + 1) % options.len(),
                None => 0,
            }
        },
        CycleDirection::Up => {
            match current_index {
                Some(0) | None => options.len() - 1,
                Some(i) => i - 1,
            }           
        }
    };

    // Set the next option as selected
    options[next_index].is_selected = true;
}
