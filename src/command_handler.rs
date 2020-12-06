pub(crate) struct Cmd<T, E> {
    pub name: String,
    pub invocation: Box<dyn FnMut() -> Result<T, E>>,
}

impl<T, E> Cmd<T, E> {
    pub fn new<F>(invoke_str: &str, invocation: F) -> Cmd<T, E>
        where
            F: FnMut() -> Result<T, E> + 'static,
    {
        Cmd {
            name: String::from(invoke_str),
            invocation: Box::new(invocation),
        }
    }

    pub fn invoke(&mut self) -> Result<T, E> {
        (self.invocation)()
    }
}


pub struct Cmdr<T, E> {
    cmds: Vec<Cmd<T, E>>,
}

impl<T, E> Cmdr<T, E> {
    pub fn new() -> Cmdr<T, E> {
        Cmdr { cmds: Vec::new() }
    }

    pub fn add<F>(&mut self, name: &str, cmd: F)
        where
            F:  FnMut() -> Result<T, E> + 'static,
    {
        self.cmds.push(Cmd::new(name, cmd));
    }

    pub fn invoke(&mut self, cmd_name: &str) -> Option<Result<T, E>> {
        self.cmds.iter_mut()
            .find(|cmd| cmd.name == cmd_name)
            .map(|cmd| cmd.invoke())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    // -> Result<&str, u16>
    fn hhhh() -> Result<String, u16> {
        Ok("dsdf".to_string())
        // println!("!!!!!");
        // Ok("++++")
    }
    #[test]
    fn xx() {
        let mut cmdr = Cmdr::new();
        // cmdr.add("test1", || Ok("test1 executed."));
        // cmdr.add("test2", || Err(42));
        cmdr.add("hhhh", hhhh);
        cmdr.invoke("hhh");
    }

    // #[test]
    // fn aaa() {
    //     let mut context = Context::new();
    //     context.bottom_right = Some(GeoCoords(10.0, 10.0));
    //     context.top_left = Some(GeoCoords(10.0, 10.0));
    //     context.image = Some("aaa".as_bytes())
    // }
    #[test]
    fn cmdr() {
        let mut ddd: Vec<String> = Vec::new();
        ddd.push(String::from("+++"));
    }

    #[test]
    fn ppp() {
        let mut cmdr = Cmdr::new();
        cmdr.add("test1", || Ok("test1 executed."));
        cmdr.add("test2", || Err(42));

        // Good commands.
        let test1_msg = cmdr.invoke("test1");
        assert_eq!(test1_msg, Some(Ok("test1 executed.")), "Incorrect test1 message.");

        let test2_msg = cmdr.invoke("test2");
        assert_eq!(test2_msg, Some(Err(42)));

        // Bad command.
        assert!(cmdr.invoke("ghost").is_none(), "Non-existent command somehow returned Some().");
    }
}

