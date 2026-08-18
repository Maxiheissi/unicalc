use crate::eval::{self, CalcError};
pub struct HistoryEntry
{
    pub input: String,
    pub result: Result<f64, CalcError>,
}

pub enum OutputBase
{
    Decimal,
    Hex,
    Binary,
}

pub struct App
{
    pub input: String,
    pub history: Vec<HistoryEntry>,
    pub cursor: usize,
    pub selected: Option<usize>,
    pub output_mode: OutputBase,
    pub show_help: bool,
}

impl App
{
    pub fn new() -> Self
    {
        Self {
            input: String::new(),
            history: Vec::new(),
            cursor: 0,
            selected: None,
            output_mode: OutputBase::Decimal,
            show_help: false,
        }
    }

    pub fn evaluate(&mut self)
    {
        let tokens = eval::tokenize_expression(&self.input);
        let mut it = tokens.into_iter().peekable();

        let result = match eval::parse_expression(&mut it)
        {
            Ok(root) => eval::eval_tree(&root),
            Err(e) => Err(e),
        };

        let entry = HistoryEntry {
            input: self.input.clone(),
            result,
        };

        if let Some(i) = self.selected
        {
            self.history[i] = entry;
            self.selected = None;
        }
        else
        {
            self.history.push(entry);
        }
        self.input.clear();
        self.cursor = 0;
        self.selected = None;
    }

    pub fn push_input(&mut self, c: char)
    {
        let byte_index = self
            .input
            .char_indices()
            .nth(self.cursor)
            .map(|(i, _)| i)
            .unwrap_or(self.input.len());
        self.input.insert(byte_index, c);
        self.cursor += 1;
    }

    pub fn pop_input(&mut self)
    {
        if self.cursor > 0 && !self.input.is_empty()
        {
            self.cursor -= 1;
            let byte_index = self
                .input
                .char_indices()
                .nth(self.cursor)
                .map(|(i, _)| i)
                .unwrap_or(self.input.len());

            self.input.remove(byte_index);
        }
    }

    pub fn cursor_left(&mut self)
    {
        if self.cursor > 0
        {
            self.cursor -= 1;
        }
    }

    pub fn cursor_right(&mut self)
    {
        if self.cursor < self.input.len()
        {
            self.cursor += 1;
        }
    }

    pub fn selected_up(&mut self)
    {
        match self.selected
        {
            None =>
            {
                if !self.history.is_empty()
                {
                    self.selected = Some(self.history.len() - 1);
                    self.input = self.history.last().unwrap().input.clone();
                    self.cursor = self.input.len();
                }
            }
            Some(i) =>
            {
                if i > 0
                {
                    self.selected = Some(i - 1);
                    self.input = self.history[i - 1].input.clone();
                    self.cursor = self.input.len();
                }
            }
        }
    }

    pub fn selected_down(&mut self)
    {
        match self.selected
        {
            None =>
            {}
            Some(i) =>
            {
                if i < self.history.len() - 1
                {
                    self.selected = Some(i + 1);
                    self.input = self.history[i + 1].input.clone();
                    self.cursor = self.input.len();
                }
                else if i == self.history.len() - 1
                {
                    self.selected = None;
                    self.input = String::new();
                    self.cursor = self.input.len();
                }
            }
        }
    }

    pub fn cycle_output_mode(&mut self)
    {
        self.output_mode = match self.output_mode
        {
            OutputBase::Decimal => OutputBase::Hex,
            OutputBase::Hex => OutputBase::Binary,
            OutputBase::Binary => OutputBase::Decimal,
        };
    }

    pub fn delete_selected(&mut self)
    {
       
        if let Some(idx) = self.selected {
            if idx < self.history.len() {
                self.history.remove(idx);

                if self.history.is_empty() {
                    self.selected = None;
                    self.input.clear();
                } else {
                    let new_idx = idx.min(self.history.len() - 1);
                    self.selected = Some(new_idx);
                
                    self.input = self.history[new_idx].input.clone();
                }

                self.cursor = self.input.len();
            }
        }
     }

     pub fn toggle_help(&mut self)
     {
         self.show_help = !self.show_help;
     }
}
