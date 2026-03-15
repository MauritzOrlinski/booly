pub fn remove_element<T: PartialEq>(vs: &mut Vec<T>, e: T) -> Option<T> {
    match vs.iter().position(|e_| *e_ == e) {
        Some(index) => Some(vs.swap_remove(index)),
        _ => None,
    }
}
