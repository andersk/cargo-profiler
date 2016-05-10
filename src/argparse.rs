extern crate clap;
use clap::ArgMatches;
use profiler::Profiler;
use parse::cachegrind::Metric;
use err::ProfError;
use std::path::Path;

pub fn match_profiler<'a>(matches: &'a ArgMatches)
                          -> Result<(&'a ArgMatches<'a>, Profiler<'a>), ProfError> {
    match matches.subcommand_matches("profiler") {
        Some(matches) => {
            match matches.subcommand_matches("callgrind") {
                Some(matches) => Ok((matches, Profiler::new_callgrind())),
                None => {
                    match matches.subcommand_matches("cachegrind") {
                        Some(matches) => Ok((matches, Profiler::new_cachegrind())),
                        None => return Err(ProfError::InvalidProfiler),
                    }
                }
            }
        }
        None => return Err(ProfError::InvalidProfiler),
    }
}

pub fn match_binary<'a>(matches: &'a ArgMatches) -> Result<&'a str, ProfError> {
    // read binary argument, make sure it exists in the filesystem
    match matches.value_of("binary") {
        Some(z) => {
            if !Path::new(z).exists() {
                return Err(ProfError::InvalidBinary);
            }

            return Ok(z);
        }
        None => return Err(ProfError::InvalidBinary),
    }


}

pub fn parse_num(matches: &ArgMatches) -> Result<usize, ProfError> {

    match matches.value_of("n").map(|x| x.parse::<usize>()) {
        Some(Ok(z)) => Ok(z),
        Some(Err(_)) => return Err(ProfError::InvalidNum),
        None => Ok(10000), // some arbitrarily large number...
    }

}

pub fn get_sort_metric(matches: &ArgMatches) -> Result<Metric, ProfError> {
    match matches.value_of("sort") {
        Some("ir") => Ok(Metric::Ir),
        Some("i1mr") => Ok(Metric::I1mr),
        Some("ilmr") => Ok(Metric::Ilmr),
        Some("dr") => Ok(Metric::Dr),
        Some("d1mr") => Ok(Metric::D1mr),
        Some("dlmr") => Ok(Metric::Dlmr),
        Some("dw") => Ok(Metric::Dw),
        Some("d1mw") => Ok(Metric::D1mw),
        Some("dlmw") => Ok(Metric::Dlmw),
        None => Ok(Metric::NAN),
        _ => return Err(ProfError::InvalidSortMetric),
    }
}