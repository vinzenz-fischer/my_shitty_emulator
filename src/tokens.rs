pub enum Register {
    RSP
}

impl IsToken for Register {
    fn get_opcode(&self) {
        todo!()
    }
}

fn main() {
    let rsp: Token<Register> = Token { token: Register::RSP };
}

pub struct Token<T>
where T: IsToken {
    token: T
}

impl<T> Token<T> where T: IsToken {
    pub fn new(token: T) -> Self {
        Token { token }
    }
}

pub trait IsToken {
    fn get_opcode(&self);
}
