use std::error::Error;



pub struct Args {
    pub address: String,
    pub wql : String,
}

impl Args {
    pub fn new(args: &[String]) -> Result<Args, &str> {        
        let address_index = args.iter().position(|arg| arg.starts_with("//"));

        let (address, wql) = match address_index {
            Some(index) => {
                let address = args[index].clone();
                let address = address[2..].to_string();//remove "//"

                let wql = args[index + 1].clone();
                
                (address, wql)
            },
            None => return Err("No address specified"),
        };
    
        Ok(Args { address, wql })
    }
}

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    // let contents = fs::read_to_string(args.wql)?;
    
    println!("Address: {}", args.address);
    println!("WQL: {}", args.wql);

    Ok(())
}