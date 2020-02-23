extern crate clap;
extern crate curl;

fn main() -> std::io::Result<()> {
  let mut app = clap::App::new("crate-name")
                      .version("1.0.0")
                      .author("Carlos Menezes <talk@carlosmenezes.com>")
                      .about("Checks whether a package name is available on crates.io")
                      .usage("crate-name <name>")
                      .settings(&[clap::AppSettings::AllowExternalSubcommands]);
  
  let args: Vec<String> = std::env::args().collect();

  if args.len() < 2 {
    &app.print_help();
    std::process::exit(0);
  }

  for i in &args[1..] {
    let availability = check_availability(&i);
    if availability {
      let output = String::from(format!("✔️  {} is available", &i));
      println!("{}", output);
    } else {
      let output = String::from(format!("❌  {0} is unavailable (https://crates.io/crates/{0})", &i));
      println!("{}", output);
    }
  }

  Ok(())
}

fn check_availability(name: &String) -> bool {
  let url: String = format!("https://crates.io/api/v1/crates/{}", &name);
  let mut req = curl::easy::Easy::new();
  req.useragent("crate-name").unwrap();
  req.url(&url).unwrap();
  
  let mut html = String::new();

  {
    let mut transfer = req.transfer();
    transfer.write_function(|data| {
      html = String::from_utf8(Vec::from(data)).unwrap();
      Ok(data.len())
    }).unwrap();

    transfer.perform().unwrap();
  }

  if html == "{\"errors\":[{\"detail\":\"Not Found\"}]}" {
    return true
  } else {
    return false
  };
}