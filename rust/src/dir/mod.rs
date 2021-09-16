use std::fs;
use std::ffi::OsString;

fn visit_dirs(dir: String) {
  let paths = fs::read_dir(dir).unwrap();

  let mut import_list: Vec<String> = vec![];

  for path in paths {
    let file = match path {
      Ok(f) => f,
      Err(e) => {
        panic!("{:?}", e)
      },
    };
    
    let tmp: OsString = file.file_name();
    let tmp: String = match tmp.into_string() {
      Ok(s) => s,
      Err(e) => panic!("{:?}", e)
    };
    let tmp: &str = &tmp;
    let t_str = String::from("import [module_name] from '@src/static/images/icons/{replace}'");
    let t_str = t_str.replace("{replace}", tmp);
    import_list.push(t_str);
  }
  println!("{:#?}", import_list);
}

pub fn file_of_dir(dir: String) {
  visit_dirs(dir);
}
