use std::io;

use ratatui::prelude::*;
use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    style::Stylize,
    widgets::Paragraph,
    DefaultTerminal,
};

use cpal::traits::{DeviceTrait, HostTrait};

pub mod app;
pub mod attributes;
pub mod context;
pub mod msg;
mod nodebug;

use nodebug::NoDebug;

const APP_NAME: &str = "panotti";

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let app_result = run(terminal);
    ratatui::restore();
    app_result
}

fn run(mut terminal: DefaultTerminal) -> io::Result<()> {
    let mut app = app::App::default();
    
    update(&mut app, msg::Msg::Started);
    loop {
        // First paint the view or it's very confusing to the user
        view(&mut terminal, &app)?;

        // But of course... you have to decide to quit at some point
        if app.exit {
            break Ok(());
        }

        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                update(&mut app, msg::Msg::Stopping);
                continue;
            } 
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('*') {
                update(&mut app, msg::Msg::ClearMessages);
                continue;
            }
        }
    }
}

fn update(model: &mut app::App, msg: msg::Msg) -> Option<msg::Msg> {
    match msg {
        msg::Msg::ClearMessages => {
            model.messages.clear();
        }
        msg::Msg::Started => {
            model.mode = app::Mode::Idle;
            model.messages.push("App started".to_string());
            init_cpal(model);
        }
        msg::Msg::Stopping => {
            model.exit = true;
        }
        msg::Msg::ContextAdded { attributes } => {
            for attribute in attributes {
                model.current_context.add_attribute(attribute);
            }
        }
        msg::Msg::ContextRemoved { attributes } => {
            for attribute in attributes {
                model.current_context.remove_attribute(attribute);
            }
        }
    }
    None
}

fn view(terminal: &mut DefaultTerminal, _app: &app::App) -> io::Result<()> {
    terminal.draw(|frame| {
        let greeting = Paragraph::new(format!("{name} (press 'q' to quit)", name = APP_NAME))
            .white()
            .on_blue();
        frame.render_widget(greeting, frame.area());
        // for each message append a new paragraph
        for (i, message) in _app.messages.iter().enumerate() {
            let area = Rect::new(0, 1 + i as u16, frame.area().width, 1);
            let paragraph = Paragraph::new(message.clone()).white().on_black();
            frame.render_widget(paragraph, area)
        }

    })?;
    Ok(())
}

fn init_cpal(model: &mut app::App) {
    let audio_host = cpal::default_host();
    model.audio_host = Some(NoDebug::from(audio_host));
    match &model.audio_host {
        Some(host) => {
            match host.default_input_device() {
                Some(device) => {
                    model.messages.push(format!("Default input device: {}", device.name().unwrap()));
                }
                None => {
                    model.messages.push("No audio input device?".into());
                }
            };
        },
        None => {
            println!("No audio host?");
        }
    };
    match &model.audio_host {
        Some(host) => {
            match  host.default_output_device() {
                Some(device) => {
                    model.messages.push(format!("Default output device: {}", device.name().unwrap()));
                }
                None => {
                    model.messages.push("No default output device?".into());
                }
            };
        }
        None => {
            model.messages.push("No audio host?".into());
        }
    }
}
