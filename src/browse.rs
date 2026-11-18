use crate::{index, pack::Pack};
use kobo_sdk::{action_id, ActionId, ScreenBuilder};

const ROWS: usize = 3;

#[derive(Clone, Copy, Default)]
pub struct Filter {
    rating: usize,
    theme: usize,
}

impl Filter {
    fn matches(self, pack: &Pack, i: usize) -> bool {
        let (rating, themes) = pack.facets(i);
        let (min, max, _) = index::RATINGS[self.rating];
        (min..=max).contains(&rating) && (self.theme == 0 || themes & (1 << (self.theme - 1)) != 0)
    }
    pub fn select(self, pack: &Pack) -> Vec<usize> {
        (pack.demo_len()..pack.len())
            .filter(|&i| self.matches(pack, i))
            .collect()
    }
}

#[derive(Default)]
pub struct Browser {
    pub filter: Filter,
    page: usize,
    matches: Vec<usize>,
}

impl Browser {
    pub fn refresh(&mut self, pack: &Pack) {
        self.page = 0;
        self.matches = self.filter.select(pack);
    }
    fn rows(&self) -> impl Iterator<Item = usize> + '_ {
        self.matches
            .iter()
            .copied()
            .skip(self.page * ROWS)
            .take(ROWS)
    }
    pub fn action(&mut self, a: ActionId, pack: &Pack) -> Option<usize> {
        let is = |name| a == action_id(name);
        if is("rating-filter") {
            self.filter.rating = (self.filter.rating + 1) % index::RATINGS.len();
            self.refresh(pack);
        } else if is("theme-filter") {
            self.filter.theme = (self.filter.theme + 1) % (index::THEMES.len() + 1);
            self.refresh(pack);
        } else if is("browse-next") && (self.page + 1) * ROWS < self.matches.len() {
            self.page += 1;
        } else if is("browse-prev") {
            self.page = self.page.saturating_sub(1);
        } else {
            return self
                .rows()
                .zip(["pick-0", "pick-1", "pick-2"])
                .find_map(|(i, name)| is(name).then_some(i));
        }
        None
    }
    pub fn screen(&self, pack: &Pack, b: ScreenBuilder) -> ScreenBuilder {
        let rating = index::RATINGS[self.filter.rating].2;
        let theme = self
            .filter
            .theme
            .checked_sub(1)
            .map_or("Any theme", |i| index::THEMES[i].1);
        let mut b = b
            .heading("Browse Lichess")
            .buttons([("rating-filter", rating), ("theme-filter", theme)])
            .secondary(format!("{} matches", self.matches.len()));
        for (i, action) in self.rows().zip(["pick-0", "pick-1", "pick-2"]) {
            b = b.button(action, format!("{} · {}", pack.id(i), pack.facets(i).0));
        }
        let prev = self.page > 0;
        let next = (self.page + 1) * ROWS < self.matches.len();
        b = match (prev, next) {
            (true, true) => b.buttons([("browse-prev", "Earlier"), ("browse-next", "More")]),
            (true, false) => b.button("browse-prev", "Earlier"),
            (false, true) => b.button("browse-next", "More"),
            (false, false) => b,
        };
        b.button("collections", "Collections")
    }
}
