use getopts::Options;
use std::str::FromStr;

pub type FetchResult<T> = Result<T, String>;

/// Initializes argument
pub fn add_arg(opts: &mut Options, id: &str, desc: &str) {
    let name = id.to_string();
    opts.optopt(
        name.chars().next().unwrap().to_string().as_str(),
        name.as_str(),
        desc,
        name.to_uppercase().as_str(),
    );
}

/// Returns the result of fetching a commandline argument's value by id.
///
/// # Examples
/// User input:
/// `./app -f ~/filepath`
///
/// ```
/// use getopts::Options;
///
/// let args: Vec<String> = env::args().collect();
/// let mut opts = Options::new();
///
/// opts.optopt("f", "file", "loads an ascii file from path", "FILE");
///
/// let file_path: FetchResult<String> = fetch_argument(&args, &opts, "f");
/// ```
pub fn fetch_argument<T>(args: &Vec<String>, opts: &Options, alias: &str) -> FetchResult<T>
where
    T: FromStr,
    T::Err: std::fmt::Debug,
{
    let provided_args = match opts.parse(&args[1..]) {
        Ok(a) => Some(a),
        Err(_) => None,
    };

    if provided_args.is_none() {
        return FetchResult::Err("Value wasn't provided to argument.".to_owned());
    }
    if let Some(p) = provided_args.unwrap().opt_str(alias) {
        p.parse::<T>()
            .map_err(|err| format!("Failed to parse argument: {:?}", err))
    } else {
        Err(format!("No argument called: \"{}\"", alias))
    }
}
