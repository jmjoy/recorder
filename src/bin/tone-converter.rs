///! 音调转换器

use structopt::StructOpt;
use std::io::{stdin, Read};
use std::error::Error;
use recorder::tone::{convert_tone, FingerTone};

#[derive(Debug, StructOpt)]
#[structopt(name = "tone-converter", about = "音调转换器，标准输入接受数字简谱，标准输出转换后的简谱。")]
struct Opt {
    /// 输入的调，例如`c`，`#c`
    #[structopt(long = "from")]
    from: String,

    /// 输出的调，例如`c`，`#c`
    #[structopt(long = "to")]
    to: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();
    let from = FingerTone::from_str(&opt.from).ok_or("from参数有误")?;
    let to = FingerTone::from_str(&opt.to).ok_or("to参数有误")?;

    let mut input = Vec::new();
    stdin().read_to_end(&mut input)?;
    let input = String::from_utf8(input)?;

    let content = convert_tone(&input, from, to)?;
    print!("{}", content);

    Ok(())
}