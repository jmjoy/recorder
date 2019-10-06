///! 竖笛简谱可视化

use structopt::StructOpt;
use std::error::Error;
use recorder::tone::{FingerTone, visualize_tone};
use std::io::{stdin, Read};

#[derive(Debug, StructOpt)]
#[structopt(name = "recorder-visualizer", about = "简单的竖笛指法图生成器。")]
struct Opt {
    /// 指法音调，例如`c`，`#c`
    #[structopt(long = "tone", short = "t")]
    tone: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();
    let tone = FingerTone::from_str(&opt.tone).ok_or("tone参数有误")?;

    let mut input = Vec::new();
    stdin().read_to_end(&mut input)?;
    let input = String::from_utf8(input)?;

    let content = visualize_tone(&input, tone)?;
    print!("{}", content);

    Ok(())
}