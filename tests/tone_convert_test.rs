use std::error::Error;
use std::fs::File;
use std::io::Read;
use recorder::tone::{convert_tone, FingerTone};

#[test]
fn test_convert_tone() -> Result<(), Box<dyn Error>> {
    let mut content = Vec::new();
    File::open("tests/孟姜女.txt")?.read_to_end(&mut content)?;
    let content = String::from_utf8(content)?;
    let content = convert_tone(&content, FingerTone::SA, FingerTone::C)?;
    assert_eq!(content.trim(), r##"4 6 #6 #6 [1] #6 4 4
4 4 4 1 2 #2 2
2 6 #6 #6 [1] [2] 5 5
[2] [2] [1] [2] [1] #6 6 #6

4 6 #6 #6 [1] #6 4 4
4 4 4 [1] [#2] [2] [#2] [2] #6 #6
4 4 #6 #6 #6 [1] [2] [1] #6 6 #6
4 [2] #6 [1]  [1] [2] [1] #6 #6
4 #6 [1] #6  [2] [2] [#2] [4] [5] #6  #6 [4] #6 [1] [2]

[4] #6 [1] [2] [2] [5] [4] [5] [4]  #6 [1] [2] [#2] [2] #6
5 6 #6 #6 [2] [1]
#6 [1] [1] [4] [2]
#6 [1] [2]  [2] [2] [5] [4] [5] [4]  #6 [1] [2] [#2] [2] [2]
5 6 #6 [1] #6  6 #6 [1] [2] [1] [#2] [2]  [#2] [2] [1] #6 6 #6 #6"##);
    Ok(())
}