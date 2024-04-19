use bevy_math::Rect;

pub struct QuadTree<T> {
    pub leaf: QuadLeaf<T>,
    pub depth: usize,
    pub max_depth: usize,
    pub bounds: Rect,
}

impl<T> QuadTree<T>
where
    T: Clone,
{
    pub fn new(bounds: Rect, max_depth: usize) -> Self {
        Self {
            leaf: QuadLeaf::None,
            depth: 0,
            max_depth,
            bounds,
        }
    }

    fn new_child(bounds: Rect, max_depth: usize, depth: usize) -> Self {
        Self {
            bounds,
            depth,
            max_depth,
            leaf: QuadLeaf::None,
        }
    }

    pub fn collapse(&mut self) -> bool {
        if self.depth >= self.max_depth {
            return false;
        }

        if let QuadLeaf::Collapsed(_) = self.leaf {
            return false;
        }

        let rects = subdivide_rect(self.bounds);
        self.leaf = QuadLeaf::Collapsed(Box::new([
            QuadTree::new_child(rects.0, self.max_depth, self.depth + 1),
            QuadTree::new_child(rects.1, self.max_depth, self.depth + 1),
            QuadTree::new_child(rects.2, self.max_depth, self.depth + 1),
            QuadTree::new_child(rects.3, self.max_depth, self.depth + 1),
        ]));

        return true;
    }

    pub fn can_collapse(&self) -> bool {
        if self.depth >= self.max_depth {
            return false;
        }

        if let QuadLeaf::Collapsed(_) = self.leaf {
            return false;
        }

        return true;
    }

    pub fn feed(&mut self, mut things: Vec<T>) {
        if things.is_empty() {
            return;
        }

        if things.len() == 1 {
            self.leaf = QuadLeaf::Leaf(things.pop().unwrap());
            return;
        }

        if !self.collapse() {
            return;
        }

        let QuadLeaf::Collapsed(children) = &mut self.leaf else {
            return;
        };

        let parts = splice(4, &things);

        for i in 0..parts.len() {
            children[i].feed(parts[i].clone());
        }
    }

    pub fn iter<G>(&self, handler: &mut G)
    where
        G: FnMut(Rect, T),
    {
        if let QuadLeaf::Leaf(data) = &self.leaf {
            handler(self.bounds, data.clone());
            return;
        }

        if let QuadLeaf::Collapsed(children) = &self.leaf {
            for child in children.iter() {
                child.iter(handler);
            }
        }
    }
}

pub fn splice<T>(channels: usize, data: &[T]) -> Vec<Vec<T>>
where
    T: Clone,
{
    let each_len = data.len() / channels + if data.len() % channels == 0 { 0 } else { 1 };
    let mut out = vec![Vec::with_capacity(each_len); channels];
    for (i, d) in data.iter().cloned().enumerate() {
        out[i % channels].push(d);
    }
    out
}

pub enum QuadLeaf<T> {
    Leaf(T),
    None,
    Collapsed(Box<[QuadTree<T>; 4]>),
}

fn subdivide_rect(rect: Rect) -> (Rect, Rect, Rect, Rect) {
    (
        Rect::new(
            rect.min.x,
            rect.min.y,
            rect.min.x + (rect.max.x - rect.min.x) / 2.0,
            rect.min.y + (rect.max.y - rect.min.y) / 2.0,
        ),
        Rect::new(
            rect.min.x + (rect.max.x - rect.min.x) / 2.0,
            rect.min.y,
            rect.max.x,
            rect.min.y + (rect.max.y - rect.min.y) / 2.0,
        ),
        Rect::new(
            rect.min.x,
            rect.min.y + (rect.max.y - rect.min.y) / 2.0,
            rect.min.x + (rect.max.x - rect.min.x) / 2.0,
            rect.max.y,
        ),
        Rect::new(
            rect.min.x + (rect.max.x - rect.min.x) / 2.0,
            rect.min.y + (rect.max.y - rect.min.y) / 2.0,
            rect.max.x,
            rect.max.y,
        ),
    )
}
