pub trait FnBox {
  fn box_call(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
  fn box_call(self: Box<F>) {
    (*self)();
  }
}

// Job type - function for run in thread
// Needed implement FnBox Trait and Send for channel send
pub type Job = Box<FnBox + Send + 'static>;

pub enum Message {
  NewJob(Job),
  Terminate
}