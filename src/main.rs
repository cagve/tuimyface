mod bib_handler;
mod tuibib;

use std::io;



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

// fn render_bib<'a>() -> Paragraph<'a>{
//     let home = Paragraph::new(vec![
//         Spans::from(vec![Span::raw("")]),
//         Spans::from(vec![Span::raw("Welcome")]),
//         Spans::from(vec![Span::raw("")]),
//         Spans::from(vec![Span::raw("to")]),
//         Spans::from(vec![Span::raw("")]),
//         Spans::from(vec![Span::styled(
//             "pet-CLI",
//             Style::default().fg(Color::LightBlue),
//         )]),
//         Spans::from(vec![Span::raw("")]),
//         Spans::from(vec![Span::raw("Press 'p' to access pets, 'a' to add random new pets and 'd' to delete the currently selected pet.")]),
//     ])
//     .alignment(Alignment::Center)
//     .block(
//         Block::default()
//             .borders(Borders::ALL)
//             .style(Style::default().fg(Color::White))
//             .title("Home")
//             .border_type(BorderType::Plain),
//     );
//     return home;
// }

fn main() -> Result<(), io::Error> {
    tuibib::run_tui();
    return Ok(());
}
