
pub trait MessageFor<A> where Self: Send {
    fn run(&self, actor: &mut A);
}

