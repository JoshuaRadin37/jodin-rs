pub fn with_indent<S: AsRef<str>>(s: S, indent: u32) -> String {
    format!("{}{}", "  ".repeat(indent as usize), s.as_ref())
}

pub trait Tree {
    fn direct_children(&self) -> Vec<&Self>;

    fn children_prefix(&self) -> Vec<&Self> {
        let mut ret = vec![self];
        for child in self.direct_children() {
            ret.extend(child.children_prefix());
        }
        ret
    }

    fn children_postfix(&self) -> Vec<&Self> {
        let mut ret = vec![];
        for child in self.direct_children() {
            ret.extend(child.children_postfix());
        }
        ret.push(self);
        ret
    }
}
