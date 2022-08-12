use std::fs;

use biblatex::{Bibliography, Entry, ChunksExt};


fn get_authors(entry:&Entry) -> Vec<String>{
    let mut names:Vec<String> = Vec::new();
    entry.author().unwrap().into_iter().for_each(|x| names.push(x.to_string()));    
    return names
}

pub fn get_authors_string(entry:&Entry) -> String{
    return entry.author().unwrap().iter().map(|x| x.to_string()).collect();
}

pub fn get_title(entry:&Entry) -> String{
    return entry.title().unwrap().format_sentence();
}

pub fn get_key(entry:&Entry) -> String{
    return entry.sort_key().unwrap();
}

// TODO implementar más de un grupo
fn get_group(entry:&Entry) -> String{
    let lines = entry.to_bibtex_string().unwrap();
    let lines_vec: Vec<_> = lines.lines().collect::<Vec<_>>();
    let group:Vec<_> = lines_vec.iter()
                                    .filter(|x| x.contains("group"))
                                    .map(|x| x.split('{').nth(1))
                                    .map(|x| x.unwrap().split('}').nth(0))
                                    .collect();
    if group.len() ==0 {
        return "Not group".to_string();
    }
    return group[0].unwrap().to_string();


}

fn get_by_group(bib:Bibliography, group: String) -> Vec<Entry>{
    let filtered = bib.iter().filter(|x| {
                        return get_group(x) != group;
                    }).map(|x| x.to_owned())
                    .collect::<Vec<_>>();
    // let filtered = bib.iter().map(|x| x.to_owned()).collect::<Vec<_>>();
    return filtered;
}

fn get_title_by_group(bib:Bibliography, group: String) -> Vec<String>{
    return get_by_group(bib, group).iter().map(|x| get_title(&x)).collect();
}


fn beautiful_print(titles:Vec<String>){
    titles.iter().for_each(|x| println!("> {}",x));
}

pub fn get_entry(){
    let file = "/home/karu/Documents/Pdfs/Database/karubib.bib";
    let bib = fs::read_to_string(file).unwrap();                // >  Read file
    let src = bib.split_once("@comment END").unwrap().0;        // > Delete comments about groups
    let bibliography = Bibliography::parse(src).unwrap();
}

pub fn get_bibliography() -> Vec<biblatex::Entry>{
    let file = "/home/karu/Documents/Pdfs/Database/karubib.bib";
    let bib = fs::read_to_string(file).unwrap();                // >  Read file
    let src = bib.split_once("@comment END").unwrap().0;        // > Delete comments about groups
    let bibliography = Bibliography::parse(src).unwrap();

    return bibliography.into_vec();
}



fn main()  {
    let file = "/home/karu/Documents/Pdfs/Database/karubib.bib";
    let bib = fs::read_to_string(file).unwrap();                // >  Read file
    let src = bib.split_once("@comment END").unwrap().0;        // > Delete comments about groups
    let bibliography = Bibliography::parse(src).unwrap();

}
