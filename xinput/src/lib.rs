use crossterm::event;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use rusty_xinput as xi;
use std::{thread::sleep, time::Duration};

use std::io;
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::Terminal;

use tui::backend::Backend;
use tui::widgets::Paragraph;
use tui::widgets::{Block, Borders, Tabs};
use tui::text::{Span, Spans};

const N_CONTROLLERS: u32 = 5;

struct ControllerState {
    state_txt: String,
    connected: bool,
}

fn print_xinput_state(state: xi::XInputState) -> String {
    let mut output = String::new();
    output.push_str("DPAD:");
    output.push_str(format!("\n    --Left: {}", state.arrow_left()).as_str());
    output.push_str(format!("\n    --Right: {}", state.arrow_right()).as_str());
    output.push_str(format!("\n    --Up: {}", state.arrow_up()).as_str());
    output.push_str(format!("\n    --Down: {}", state.arrow_down()).as_str());
    output.push_str("\nButtons:");
    output.push_str(format!("\n    --A: {}", state.south_button()).as_str());
    output.push_str(format!("\n    --B: {}", state.east_button()).as_str());
    output.push_str(format!("\n    --X: {}", state.west_button()).as_str());
    output.push_str(format!("\n    --Y: {}", state.north_button()).as_str());
    output.push_str("\nBack Buttons:");
    output.push_str(format!("\n    --L: {}", state.left_shoulder()).as_str());
    output.push_str(format!("\n    --R: {}", state.right_shoulder()).as_str());
    output.push_str(format!("\n    --LT: {}", state.left_trigger()).as_str());
    output.push_str(format!("\n    --LR: {}", state.right_trigger()).as_str());
    output.push_str("\nLeft Joystick:");
    output.push_str(format!("\n    --Ver: {}", state.left_stick_normalized().1).as_str());
    output.push_str(format!("\n    --Hor: {}", state.left_stick_normalized().0).as_str());
    output.push_str(format!("\n    --Button: {}", state.left_thumb_button()).as_str());
    output.push_str("\nRight Joystick:");
    output.push_str(format!("\n    --Ver: {}", state.right_stick_normalized().1).as_str());
    output.push_str(format!("\n    --Hor: {}", state.right_stick_normalized().0).as_str());
    output.push_str(format!("\n    --Button: {}", state.right_thumb_button()).as_str());
    output.push_str("\nFunction Buttons:");
    output.push_str(format!("\n    -- (+): {}", state.start_button()).as_str());
    output.push_str(format!("\n    -- (-): {}", state.select_button()).as_str());

    output
}

fn get_xinput_state(id: u32, handle: &xi::XInputHandle) -> (String, bool) {
    let mut connected = false;
    let state = match handle.get_state(id) {
        Ok(s) => {
            connected = true;
            format!("CONNECTED\n\n{}", print_xinput_state(s))
        }
        Err(_) => {
            format!("DISCONNECTED")
        }
    };

    (state, connected)
}

fn get_controllers_state(handle: &xi::XInputHandle) -> Vec<ControllerState> {
    let mut states_vec: Vec<ControllerState> = Vec::new();

    for i in 0..N_CONTROLLERS {
        let (state, connected) = get_xinput_state(i, &handle);
        states_vec.push(ControllerState {
            state_txt: state,
            connected: connected,
        });
    }

    states_vec
}

fn get_spans_from_state<'a>(state: &'a ControllerState, color : &Color) -> Vec<Spans<'a>> {
    state.state_txt.lines()
            .map(|f| Spans::from(Span::styled(f, Style::default().fg(color.to_owned()))))
            .collect()
}

fn draw_ui<B: Backend>(terminal: &mut Terminal<B>, states: Vec<ControllerState>, id: u32) {
    terminal
        .draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Length(3), Constraint::Length(100)].as_ref())
                .split(size);

            let state = &states[usize::try_from(id).unwrap()];
            let title = format!("Controller ID ({}) State", id);
            let title_color = if state.connected {
                Color::LightGreen
            } else {
                Color::LightRed
            };


            // let inner_text = vec![
            //     Spans::from(
            //         Span::styled(state.state_txt.to_owned(), Style::default().fg(title_color))
            //     )];

            let inner_text = get_spans_from_state(&state, &title_color);


            let mut titles: Vec<String> = Vec::new();
            for i in 0..N_CONTROLLERS{
                titles.push(format!("Controller {}", i));
            }
            let tabs = Tabs::new(titles.iter().cloned().map(Spans::from).collect())
                .block(Block::default().borders(Borders::ALL).title("Tabs"))
                .select(id.try_into().unwrap())
                .style(Style::default().fg(Color::Cyan))
                .highlight_style(
                    Style::default()
                        .add_modifier(Modifier::BOLD)
                        .bg(title_color)
                        .fg(Color::Black)
                );
            f.render_widget(tabs, chunks[0]);



            let p = Paragraph::new(inner_text)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().fg(Color::White))
                        .title(title.as_str())
                )
                .style(Style::default().fg(Color::LightCyan));
            f.render_widget(p, chunks[1])
        })
        .unwrap();
}

fn increase_index(index: &u32, min: u32, max: u32) -> u32 {
    let mut i = index + 1;
    if i > max {
        i = min;
    }

    i
}
fn decrease_index(index: &u32, min: u32, max: u32) -> u32 {
    let mut i = index + 0;
    if i == min {
        i = max;
    }
    else {
        i = i - 1;
    }

    i
}

pub fn run_app(){
    let handle = xi::XInputHandle::load_default().unwrap();
    let stdout = io::stdout();
    let mut backend = CrosstermBackend::new(stdout);
    backend.hide_cursor().unwrap();
    let mut terminal = Terminal::new(backend).unwrap();
    enable_raw_mode().unwrap();
    terminal.clear().unwrap();
    let mut index: u32 = 0;
    loop {
        if event::poll(Duration::from_millis(10)).unwrap() {
            if let event::Event::Key(key) = event::read().unwrap() {
                match key.code {
                    event::KeyCode::Char('n') => {
                        index = increase_index(&index, 0, N_CONTROLLERS - 1)
                    }
                    event::KeyCode::Char('p') => {
                        index = decrease_index(&index, 0, N_CONTROLLERS - 1)
                    }
                    _ => break,
                }
            }
        }

        let states = get_controllers_state(&handle);
        draw_ui(&mut terminal, states, index);
        sleep(Duration::from_millis(50));
    }

    terminal.clear().unwrap();
    disable_raw_mode().unwrap();
}