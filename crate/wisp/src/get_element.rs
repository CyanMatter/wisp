use wasm_bindgen::JsCast;
use web_sys::Document;

macro_rules! document {
  () => {
    web_sys::window()
      .expect("could not access DOM: global window does not exist.")
      .document()
      .expect("could not access DOM: document does not exist.")
  };
}

// pub fn document() -> Document {
//   window()
//         .expect("could not access DOM: global window does not exist.")
//         .document()
//         .expect("could not access DOM: document does not exist.")
// }

pub fn by_id<T: wasm_bindgen::JsCast>(doc: &Document, id: &str) -> Option<T> {
  let element = doc.get_element_by_id(id);
  element.as_ref()?;
  Some(element.unwrap().dyn_into::<T>().unwrap())
}

pub(crate) use document;