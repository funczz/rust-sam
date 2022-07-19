#[derive(Debug)]
pub enum NextActionData<M> {
    Continue { model: M },
    Terminate { model: M },
}

impl<M> NextActionData<M> {
    pub fn new_continue(model: M) -> NextActionData<M> {
        NextActionData::Continue { model }
    }

    pub fn new_terminate(model: M) -> NextActionData<M> {
        NextActionData::Terminate { model }
    }

    pub fn get(self) -> M {
        match self {
            NextActionData::Continue { model } => model,
            NextActionData::Terminate { model } => model,
        }
    }
}
