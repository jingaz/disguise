use clap::{App, Arg};
use data_encoding::HEXUPPER;
use std::fs::rename;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;
fn main() {
    let matches = App::new("disguise")
        .version("0.1.0")
        .author("jingaz <jingaz@163.com>")
        .about("阿里云盘分享文件类型伪装，最好先备份，防止源文件丢失，仅支持转换到zip、rar、png三种类型（且需要显式指定）")
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .takes_value(true)
                .help("input file path"),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .takes_value(true)
                .help("output file name"),
        )
        .arg(
            Arg::with_name("too")
                .short("t")
                .long("type_of_output")
                .takes_value(true)
                .help("type of the output file"),
        )
        .get_matches();

    let input = matches.value_of("input").expect("源文件必须要指定");
    let path = Path::new(input);
    let parent = path.parent().unwrap();
    let file_stem = path.file_stem().unwrap();
    println!("The inputfile is: {}", input);
    let outputfile = matches.value_of("output").unwrap_or(input);
    let typeofoutput = matches.value_of("too").expect("目标文件格式必须显示指定");
    let mut newfile = String::from(outputfile);
    match typeofoutput {
        "png" | "zip" | "rar" => {
            newfile = if outputfile == input {
                format!(
                    "{}.{}",
                    file_stem.to_owned().into_string().unwrap(),
                    typeofoutput
                )
            } else {
                newfile
            };
            println!("从 {} 伪装成 {}", input, newfile)
        }
        _ => panic!("{} 不是png、zip、rar其中之一", typeofoutput),
    }

    let mut file = openfile(path).unwrap();

    disguisefile(&mut file, typeofoutput);
    let outputfilepath = parent.join(newfile);
    savefile(path, &outputfilepath).unwrap();
}
fn openfile(inputfilepath: &Path) -> std::io::Result<File> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(false)
        .append(false)
        .truncate(false)
        .open(inputfilepath);
    Ok(file.unwrap())
}
fn disguisefile(file: &mut File, typeofoutput: &str) {
    match typeofoutput {
        "png" => file.write(&HEXUPPER.decode(b"89504E47").unwrap()).unwrap(),
        "zip" => file.write(&HEXUPPER.decode(b"504B0304").unwrap()).unwrap(),
        "rar" => file.write(&HEXUPPER.decode(b"52617221").unwrap()).unwrap(),
        _ => panic!("not a supported type."),
    };
}
fn savefile(inputfile: &Path, outputfile: &Path) -> std::io::Result<()> {
    rename(inputfile, outputfile)
}
