use std::io;

use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    style::Stylize,
    widgets::Paragraph,
    DefaultTerminal,
};

pub mod app;
pub mod attributes;
pub mod context;
pub mod msg;

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
        }
    }
}

fn update(model: &mut app::App, msg: msg::Msg) -> Option<msg::Msg> {
    match msg {
        msg::Msg::Started => {
            model.mode = app::Mode::Idle;
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
        let greeting = Paragraph::new("Hello Ratatui! (press 'q' to quit)")
            .white()
            .on_blue();
        frame.render_widget(greeting, frame.area());
    })?;
    Ok(())
}
