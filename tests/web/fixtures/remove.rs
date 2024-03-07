use crate::get;

pub fn receptacle() {
    if let Some(receptacle) = get::receptacle() {
        receptacle.remove();
    }
}

pub fn responses() {
    for div in get::responses() {
        div.remove();
    }
}
