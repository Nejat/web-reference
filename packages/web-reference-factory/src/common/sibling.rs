use select::node::Node;
use select::predicate::Predicate;

///
pub struct SiblingIter<'a, Find, Stop>
    where Find: Predicate,
          Stop: Predicate
{
    next: Option<Node<'a>>,
    find: Find,
    stop: Stop,
}

impl<'a, Find, Stop> Iterator for SiblingIter<'a, Find, Stop>
    where Find: Predicate,
          Stop: Predicate
{
    type Item = Node<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next = self.next?;

            if self.find.matches(&next) {
                self.next = None;

                return Some(next);
            } else if self.stop.matches(&next) {
                return None;
            }

            self.next = next.next();
        }
    }
}

///
pub trait IterSibling<'a, Find, Stop>
    where Find: Predicate,
          Stop: Predicate
{
    ///
    fn iter_sibling(&self, find: Find, stop: Stop) -> SiblingIter<'a, Find, Stop>;
}

impl<'a, Find, Stop> IterSibling<'a, Find, Stop> for Node<'a>
    where Find: Predicate,
          Stop: Predicate
{
    fn iter_sibling(&self, find: Find, stop: Stop) -> SiblingIter<'a, Find, Stop> {
        SiblingIter {
            next: self.next(),
            find,
            stop,
        }
    }
}
