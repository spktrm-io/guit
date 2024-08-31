use gpui::*;
#[derive(IntoElement)]

pub struct HeaderButton {
    pub selected_tab: Model<String>,
    text: String,
    id: ElementId,
}

impl RenderOnce for HeaderButton {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        return div()
            .id(self.id)
            .w_full()
            .flex()
            .text_sm()
            .px_4()
            .items_center()
            .justify_center()
            .rounded_xl()
            .h_10()
            .bg(rgb(0x202020))
            .child(self.text.clone())
            .hover(|style| style.bg(rgb(0x181818)).cursor_pointer())
            .border_2()
            .focusable()
            .focus(|style| style.bg(rgb(0x161616)).cursor_pointer())
            .on_click({
                let model = self.selected_tab.clone();
                move |_, cx: &mut WindowContext| {
                    cx.update_model(&model, |tab, _| {
                        *tab = self.text.clone();
                    });
                }
            });
    }
}

#[derive(IntoElement)]
pub struct Header {
    pub selected_tab: Model<String>,
}

impl RenderOnce for Header {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        let commit_button = HeaderButton {
            selected_tab: self.selected_tab.clone(),
            text: "commit".to_string(),
            id: ElementId::Name("commit_button".into()),
        };

        let branches_button = HeaderButton {
            selected_tab: self.selected_tab.clone(),
            text: "branches".to_string(),
            id: ElementId::Name("branches_button".into()),
        };
        let history_button = HeaderButton {
            selected_tab: self.selected_tab.clone(),
            text: "history".to_string(),
            id: ElementId::Name("history_button".into()),
        };

        return div()
            .w_full()
            .flex()
            .bg(rgb(0x191919))
            .justify_between()
            .items_center()
            .p_2()
            .gap_2()
            .child(div().font_weight(FontWeight(900.0)).child("guit").mx_4())
            .children([commit_button, branches_button, history_button]);
    }
}
