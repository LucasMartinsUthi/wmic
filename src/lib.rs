use std::error::Error;
use std::fs;

pub struct Args {
    pub address: String,
    pub wql : String,
}

impl Args {
    pub fn new(args: &[String]) -> Result<Args, &str> {        
        if args.len() != 3 {
            return Err("Wrong number of arguments, expected at least 2");
        }

        //filter args that starts with "-"
        let mut args = args.iter().filter(|arg| arg.chars().next().unwrap() != '-').collect::<Vec<&String>>();

        let address_index = args.iter().position(|arg| arg.starts_with("//"));

        let address = match address_index {
            Some(index) => {
                let address = args[index].clone();
                args.remove(index);

                
                address[2..].to_string() 
            },
            None => return Err("No address specified"),
        };

        let wql = args[1].clone();

    
        Ok(Args { address, wql })
    }
}

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    // let contents = fs::read_to_string(args.wql)?;
    
    println!("Address: {}", args.address);
    println!("WQL: {}", args.wql);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test_args_new() {
        let args = vec!["wmic".to_string(), "//localhost".to_string(), "select * from Win32_ComputerSystem".to_string()];

        let args = Args::new(&args).unwrap();
        
        assert_eq!(args.address, "localhost");
        assert_eq!(args.wql, "select * from Win32_ComputerSystem");
    }

    #[test]
    fn test_args_diferent_order() {
        let args = vec!["wmic".to_string(), "select * from Win32_ComputerSystem".to_string(), "//localhost".to_string() ];

        let args = Args::new(&args).unwrap();
        
        assert_eq!(args.address, "localhost");
        assert_eq!(args.wql, "select * from Win32_ComputerSystem");
    }
}
