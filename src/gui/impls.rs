use std::sync::Arc;

use druid::{Data, widget::ListIter};

use crate::recipes::recipe::Ingredient;


impl ListIter<Ingredient> for Arc<[Ingredient]> {
    fn for_each(&self, mut cb: impl FnMut(&Ingredient, usize)) {
        let mut idx = 0;
        for i in self.iter() {
            cb(i, idx);
            idx += 1;
        }
    }

    fn for_each_mut(&mut self, mut cb: impl FnMut(&mut Ingredient, usize)) {
        if let Some(this) = Arc::get_mut(self) {
            let mut idx = 0;
            for i in this.iter_mut() {
                cb(i, idx);
                idx += 1;
            }
        }
    }

    fn data_len(&self) -> usize {
        self.len()
    }
}