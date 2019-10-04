///! 音调转换器

use structopt::StructOpt;

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

fn main() {
    let opt = Opt::from_args();
    dbg!(opt);
}