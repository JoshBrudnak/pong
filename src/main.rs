extern crate clap;
extern crate futures;
extern crate http;
extern crate hyper;
extern crate termion;
extern crate tokio_core;
extern crate tui;

use clap::App;
use clap::Arg;
use futures::{Future, Stream};
use hyper::{Client, Uri};
use std::io;
use std::thread::sleep;
use std::time::{Duration, Instant};
use termion::raw::IntoRawMode;
use tokio_core::reactor::Core;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::canvas::{Canvas, Line, Map, MapResolution};
use tui::widgets::{Axis, Block, Borders, Chart, Dataset, List, Marker, Text, Widget};
use tui::Terminal;

fn send(req: String) {
    let uri = req.parse::<Uri>().unwrap();
    let mut core = Core::new().unwrap();
    let client = Client::new();

    let work = client.get(uri).map(|res| {
        //   println!("Response: {:?}", res);
    });
    let resp = core.run(work).unwrap();

    //println!("{:?}", resp);
}

fn ping(url: String) -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let size = terminal.size()?;
    let mut times: Vec<(String, String)> = Vec::new();

    let success_style = Style::default().fg(Color::Green);
    let error_style = Style::default().fg(Color::Magenta);
    let other_style = Style::default().fg(Color::Yellow);

    loop {
        let now = Instant::now();

        send(url.clone());
        let new_now = Instant::now();
        let diff = new_now.duration_since(now);
        let nanos = diff.subsec_nanos() as u64;
        let ms = (1000 * 1000 * 1000 * diff.as_secs() + nanos) / (1000 * 1000);
        times.push((ms.clone().to_string(), String::from("SUCCESS")));

        terminal.draw(|mut f| {
            let constraints = vec![Constraint::Percentage(50), Constraint::Percentage(50)];

            let chunks = Layout::default()
                .constraints(constraints)
                .direction(Direction::Horizontal)
                .split(Rect::default());

            let events = times.iter().map(|(evt, level)| {
                Text::styled(
                    format!("{}: {}", level, evt),
                    match level.as_str() {
                        "SUCCESS" => success_style,
                        "ERROR" => error_style,
                        _ => other_style,
                    },
                )
            });

            List::new(events)
                .block(Block::default().borders(Borders::ALL).title("List"))
                .render(&mut f, chunks[0]);

            Chart::default()
                .block(Block::default().title("Time chart"))
                .x_axis(
                    Axis::default()
                        .title("Time")
                        .title_style(Style::default().fg(Color::Red))
                        .style(Style::default().fg(Color::Gray))
                        .bounds([0.0, 10.0])
                        .labels(&["0.0", "5.0", "10.0"]),
                ).y_axis(
                    Axis::default()
                        .title("Ping time")
                        .title_style(Style::default().fg(Color::Red))
                        .style(Style::default().fg(Color::Gray))
                        .bounds([0.0, 10.0])
                        .labels(&["0.0", "5.0", "10.0"]),
                ).datasets(&[
                    Dataset::default()
                        .name("data1")
                        .marker(Marker::Dot)
                        .style(Style::default().fg(Color::Cyan))
                        .data(&[(0.0, 5.0), (1.0, 6.0), (1.5, 6.434)]),
                    Dataset::default()
                        .name("data2")
                        .marker(Marker::Braille)
                        .style(Style::default().fg(Color::Magenta))
                        .data(&[(4.0, 5.0), (5.0, 8.0), (7.66, 13.5)]),
                ]).render(&mut f, chunks[1]);
        })?;

        sleep(Duration::new(1, 0));
    }
}

fn draw_tui() -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let size = terminal.size()?;
    terminal.draw(|mut f| {
        Block::default()
            .title("Block")
            .borders(Borders::ALL)
            .render(&mut f, size);
    })
}

fn main() {
    let matches = App::new("pong")
        .version("0.1.0")
        .author("Josh Brudnak <jobrud314@gmail.com>")
        .about("A for pinging a url")
        .arg(
            Arg::with_name("INPUT")
                .value_name("URL")
                .required(true)
                .help("The URL to use")
                .index(1),
        ).get_matches();

    let url = matches
        .value_of("INPUT")
        .expect("Url not chosen")
        .to_string();

    ping(url);
}
