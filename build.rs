extern crate prost_build;
// Code copy from backend
// If it must fail > it must panics > no output to concole without panic

fn main() {
    // build_pb();
}

fn build_pb() {
    println!("======================= INSIDE THE BUILD PROJECT =========================");

    let dir = std::fs::read_dir("src/ctrader/oapi/").unwrap();

    let mut vec_protos = vec![];
    for fl in dir {
        let path = format!("{:}", fl.unwrap().path().to_str().unwrap());
        vec_protos.push(path);
    }
    // println!("{:#?}", &vec_protos);

    let mut config = prost_build::Config::default();

    config.out_dir("src/");

    // config.type_attribute(".","#[derive(Smg)]");
    // config.message_attribute(".", "#[derive(Default)]");
    // config.att(".", "#[derive(Smg)]");
    let v = config.compile_protos(&vec_protos, &["src/ctrader/oapi".to_string()]);
    println!("{:?}", v);
    v.unwrap();

    // run_format_codes();
}
