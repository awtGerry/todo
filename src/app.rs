#[derive(Debug, Default)]
pub struct App
{
    pub is_running: bool,
    pub counter: i32,
}

impl App
{
    pub fn new() -> Self
    {
        Self::default()
    }

    pub fn tick(&self) {}

    pub fn quit(&mut self)
    {
        self.is_running = false;
    }

    pub fn inc(&mut self)
    {
        if let Some(n) = self.counter.checked_add(1)
        {
            self.counter = n;
        }
    }

    pub fn dec(&mut self)
    {
        if let Some(n) = self.counter.checked_sub(1)
        {
            self.counter = n;
        }
    }
}
