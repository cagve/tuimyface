use std::io;

use biblatex::Entry;
use crossterm::{terminal::{enable_raw_mode, disable_raw_mode, LeaveAlternateScreen}, execute, event::{DisableMouseCapture, Event, read, KeyCode}};
use tui::{
    backend::CrosstermBackend,
    Terminal,
    widgets::{Block, Borders, Tabs, Paragraph, BorderType, List, ListItem, ListState, Cell, Table, Row}, layout::{Layout, Direction, Constraint, Alignment}, text::{Spans, Span}, style::{Style, Color, Modifier}
};


use crate::bib_handler;

#[derive(Copy, Clone, Debug)]
enum MenuItem {
    Home,
    Authors,
}

impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> usize {
        match input {
            MenuItem::Home => 0,
            MenuItem::Authors => 1,
        }
    }
}

fn render_entries<'a>(entry_list_state: &ListState) -> (List<'a>, Paragraph){
    let entries = bib_handler::get_bibliography();
    let items:Vec<_> = entries
        .iter()
        .map(|x| {
            let title = bib_handler::get_title(&x);
              ListItem::new(Spans::from(vec![Span::styled(
                title,
                Style::default(),
            )]))
            // ListItem::new(Spans::from(vec![Span::raw(title)]));
        })
        .collect();

    let selected  = entries
        .get(entry_list_state.selected().expect("Must be a selected entry"))
        .clone();

    let list = List::new(items)
        .highlight_style(Style::default()
            .fg(Color::Black)
            .bg(Color::White))
        .highlight_symbol("  ")
        .block(Block::default()
               .title("Bib entries")
               .borders(Borders::ALL)
               );

    let info = render_info(selected.unwrap().to_owned());

    return (list, info);
}

fn render_info<'a>(entry: Entry) -> Paragraph<'a>{
    let title = bib_handler::get_title(&entry);
    let author = bib_handler::get_authors_string(&entry);
    // let key = bib_handler::get_key(&entry);

    let info = Paragraph::new(vec![
        // Spans::from(vec![Span::raw("Key: ".to_string() + &key)]),
        Spans::from(vec![Span::raw("Title: ".to_string() + &title)]),
        Spans::from(vec![Span::raw("Author: ".to_string() + &author)]),
    ])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Home")
            .border_type(BorderType::Plain),
    );
    return info;
}

pub fn run_tui() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let menu_titles = vec!["Home", "Authors"];
    let mut active_menu_item = MenuItem::Home;

    let mut list_state = ListState::default();
    list_state.select(Some(0));

    loop {
        let menu = menu_titles.iter()
            .cloned()
            .map(Spans::from)
            .collect(); //Must be spam
        terminal.clear()?;
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                    Constraint::Percentage(10),
                    Constraint::Percentage(10),
                    Constraint::Percentage(80),
                    ].as_ref()
                    )
                .split(f.size());
            let tabs = Tabs::new(menu)
                .select(active_menu_item.into())
                .block(Block::default().title("Tabs").borders(Borders::ALL))
                .highlight_style(Style::default().fg(Color::Yellow));
            f.render_widget(tabs,chunks[0]);
            match active_menu_item {
                MenuItem::Home => {
                    let home_chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints(
                            [Constraint::Percentage(50), Constraint::Percentage(50)].as_ref(),
                            )
                        .split(chunks[2]);
                    f.render_stateful_widget(render_entries(&list_state).0, home_chunks[0], &mut list_state);
                    f.render_widget(render_entries(&list_state).1, home_chunks[1]);
                }
                MenuItem::Authors => {}
            }

        })?;
        match read()?{
            Event::Key(key) => match key.code {
                KeyCode::Char('q') =>{
                    disable_raw_mode()?;
                    terminal.show_cursor()?;
                    break;
                }
                KeyCode::Char('1') => active_menu_item = MenuItem::Home,
                KeyCode::Char('2') => active_menu_item = MenuItem::Authors,
                KeyCode::Char('j') => match active_menu_item{
                        MenuItem::Home => {
                            let selected = list_state.selected().unwrap();
                            if selected >= bib_handler::get_bibliography().len()-1{
                                list_state.select(Some(0));
                            } else{
                                list_state.select(Some(selected+1));
                            }
                        }
                        MenuItem::Authors => {
                            break;
                        }
                    }
                KeyCode::Char('k') => match active_menu_item{
                        MenuItem::Home => {
                            let selected = list_state.selected().unwrap();
                            if selected > 0{
                                list_state.select(Some(selected-1));
                            } else{
                                list_state.select(Some(bib_handler::get_bibliography().len() - 1));
                            }
                        }
                        MenuItem::Authors => {
                            break;
                        }
                    }
                _ => {}
            }
            _ =>{}
        }
    }
    return Ok(());
}

// fn render_authors<'a>(author_list_state: &ListState) -> List<'a>{
//     let items = [ListItem::new("Item 1"), ListItem::new("Item 2"), ListItem::new("Item 3")];
//     let selected_author = items
//         .get(
//             author_list_state
//                 .selected()
//                 .expect("there is always a selected pet"),
//         )
//         .expect("exists")
//         .clone();

//     let list =  List::new(items)
//         .block(Block::default().title("List").borders(Borders::ALL))
//         .style(Style::default().fg(Color::White))
//         .highlight_style(Style::default().fg(Color::Red))
//         .highlight_symbol(">>");
//     return list;
// }

