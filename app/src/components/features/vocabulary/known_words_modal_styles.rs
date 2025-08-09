use crate::theme::Theme;

pub struct KnownWordsModalStyles<'a> {
    pub theme: &'a Theme,
}

impl<'a> KnownWordsModalStyles<'a> {
    pub fn new(theme: &'a Theme) -> Self {
        Self { theme }
    }

    pub fn header(&self) -> String {
        format!(
            "padding: 20px; \
             border-bottom: 1px solid {}; \
             display: flex; \
             justify-content: space-between; \
             align-items: center;",
            self.theme.border
        )
    }

    pub fn title(&self) -> String {
        format!(
            "margin: 0; \
             color: {}; \
             font-size: 1.5em; \
             font-weight: 600;",
            self.theme.text_primary
        )
    }

    pub fn close_button(&self) -> String {
        format!(
            "background: none; \
             border: none; \
             font-size: 1.5em; \
             cursor: pointer; \
             color: {}; \
             padding: 5px; \
             border-radius: 4px; \
             transition: background-color 0.2s ease;",
            self.theme.text_secondary
        )
    }

    pub fn search_section(&self) -> String {
        format!(
            "padding: 20px; \
             border-bottom: 1px solid {};",
            self.theme.border
        )
    }

    pub fn search_input(&self) -> String {
        format!(
            "width: 100%; \
             padding: 12px; \
             border: 1px solid {}; \
             border-radius: 6px; \
             background: {}; \
             color: {}; \
             font-size: 1em; \
             box-sizing: border-box;",
            self.theme.border, self.theme.background, self.theme.text_primary
        )
    }

    pub fn body(&self) -> String {
        "flex: 1; \
         overflow-y: auto; \
         padding: 20px; \
         max-height: 400px;".to_string()
    }

    pub fn empty_state(&self) -> String {
        format!(
            "text-align: center; \
             color: {}; \
             padding: 40px; \
             font-style: italic;",
            self.theme.text_secondary
        )
    }

    pub fn words_grid(&self) -> String {
        "display: grid; \
         grid-template-columns: repeat(auto-fill, minmax(200px, 1fr)); \
         gap: 12px;".to_string()
    }

    pub fn word_item(&self) -> String {
        format!(
            "background: {}; \
             border: 1px solid {}; \
             border-radius: 8px; \
             padding: 12px; \
             display: flex; \
             justify-content: space-between; \
             align-items: center; \
             transition: all 0.2s ease;",
            self.theme.background, self.theme.border
        )
    }

    pub fn word_text(&self) -> String {
        format!(
            "color: {}; \
             font-weight: 500; \
             flex: 1; \
             text-align: left;",
            self.theme.text_primary
        )
    }

    pub fn remove_button(&self) -> String {
        format!(
            "background: {}; \
             border: none; \
             color: white; \
             border-radius: 4px; \
             padding: 4px 8px; \
             cursor: pointer; \
             font-size: 0.8em; \
             transition: opacity 0.2s ease;",
            self.theme.error
        )
    }

    pub fn footer(&self) -> String {
        format!(
            "padding: 20px; \
             border-top: 1px solid {}; \
             display: flex; \
             justify-content: flex-end; \
             gap: 10px;",
            self.theme.border
        )
    }

    pub fn action_button(&self) -> String {
        format!(
            "background: {}; \
             color: white; \
             border: none; \
             padding: 10px 20px; \
             border-radius: 6px; \
             cursor: pointer; \
             font-size: 1em; \
             transition: opacity 0.2s ease;",
            self.theme.accent
        )
    }
}