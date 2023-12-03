pub fn read_input() -> &'static str {
    include_str!("input")
}

#[derive(Clone)]
pub struct GameLine<'a> {
    line: &'a str,
    previous: Option<&'a str>,
    next: Option<&'a str>
}

impl<'a> GameLine<'a> {
    pub fn new(line: &'a str, previous: Option<&'a str>) -> Self {
        Self {
            line,
            previous,
            next: None
        }
    }

    pub fn add_next(&'a mut self, next: &'a str) {
        self.next = Some(next);
    }
}

pub struct Game<'a> {
    lines: Vec<GameLine<'a>>,
}

impl<'a> Game<'a> {
    pub fn from_input(game: &'a str) -> Self {
        let mut lines: Vec<GameLine<'a>> = Vec::with_capacity(game.lines().count());
        let mut prev_line: Option<&str> = None;
        let mut prev_game: Option<GameLine<'a>> = None;
        for line in game.lines() {
            if let Some(mut g) = prev_game {
                g.add_next(line);
                lines.push(g.clone());
            }

            prev_game = Some(GameLine::new(line, prev_line));
            prev_line = Some(line);
        }

        Self { lines }
    }
}
