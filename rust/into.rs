
pub(crate) fn new_with_class<S:Into<String>>(ident: S, class: S) -> HTMLElement {
    HTMLElement {
        ident: ident.into(),
        attributes: HashMap::new(),
        classes: vec![class.into()],
        styles: Vec::new(),
    }
}
