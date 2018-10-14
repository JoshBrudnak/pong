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
use tui::widgets::{
    BarChart, Block, Borders, Dataset, Gauge, List, Marker, Paragraph, Row, SelectableList,
    Sparkline, Table, Tabs, Text, Widget,
};
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
            let chunks = Layout::default()
                .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
                .split(size);
            Tabs::default()
                .block(Block::default().borders(Borders::ALL).title("Tabs"))
                .titles(&vec!["title1", "title2"])
                .style(Style::default().fg(Color::Green))
                .highlight_style(Style::default().fg(Color::Yellow))
                .select(0)
                .render(&mut f, chunks[0]);

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
                .render(&mut f, chunks[1]);
            /*

            BarChart::default()
                .block(Block::default().borders(Borders::ALL).title("Bar chart"))
                .data(&vec![
                    ("B1", 9),
                    ("B2", 12),
                    ("B3", 5),
                    ("B4", 8),
                    ("B5", 2),
                    ("B6", 4),
                    ("B7", 5),
                    ("B8", 9),
                    ("B9", 14),
                    ("B10", 15),
                    ("B11", 1),
                ]).bar_width(3)
                .bar_gap(2)
                .value_style(
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Green)
                        .modifier(Modifier::Italic),
                ).label_style(Style::default().fg(Color::Yellow))
                .style(Style::default().fg(Color::Green))
                .render(&mut f, chunks[2]);
                */
        })?;

        sleep(Duration::new(2, 0));
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
