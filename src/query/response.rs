use common::validation::{Issue, Validatable};

#[derive(Clone, Serialize, Deserialize)]
pub struct Response {
    pub kind: ResponseKind,
    pub include: Vec<ResponseItem>,
}

#[derive(PartialEq, Debug, Copy, Clone, Serialize, Deserialize)]
pub enum ResponseKind {
    Full,
    Partial,
}

#[derive(PartialEq, Debug, Copy, Clone, Serialize, Deserialize)]
pub enum ResponseItem {
    Excerpt,
    Url,
    Domain,
    Mime,
}

impl Validatable for Response {
    fn validate(&self) -> Option<Vec<Issue>> {
        let mut issues: Vec<Issue> = Vec::new();
        if self.kind == ResponseKind::Partial {
            let disallowed_items = vec![ResponseItem::Excerpt, ResponseItem::Url];
            for item in &self.include {
                if disallowed_items.contains(&item) {
                    issues.push(Issue::Error(format!("include `{:?}` is not allowed in partial responses", item)))
                }
            }
        }
        if issues.len() == 0 {
            return None;
        } else {
            return Some(issues);
        }
    }
}
