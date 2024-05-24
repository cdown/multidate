use chrono::{DateTime, Duration, Local, Offset, TimeZone};
use chrono_tz::Tz;
use clap::Parser;
use std::str::FromStr;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Config {
    /// Remote timezone(s) to print, for example, Europe/London.
    ///
    /// A custom name can be provided after an @ symbol, for example, Europe/London@Home.
    #[arg(required = true)]
    tz: Vec<TzWithName>,

    #[arg(
        short,
        long,
        help = "A strftime format specifying how to format times",
        default_value = "%a %F %H:%M"
    )]
    fmt: String,

    #[arg(
        long,
        help = "Do not right align timezone names",
        default_value = "false"
    )]
    no_align: bool,

    #[arg(long, help = "Do not print the local time", default_value = "false")]
    no_local: bool,
}

#[derive(Clone, Debug)]
struct TzWithName {
    name: String,
    tz: Tz,
}

impl FromStr for TzWithName {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((tz, name)) = s.split_once('@') {
            Ok(Self {
                name: name.to_string(),
                tz: Tz::from_str(tz).map_err(|e| e.to_string())?,
            })
        } else {
            Ok(Self {
                name: s.to_string(),
                tz: Tz::from_str(s).map_err(|e| e.to_string())?,
            })
        }
    }
}

struct ExplicitTz {
    name: String,
    offset_from_local: Duration,
    dt: DateTime<Tz>,
}

impl ExplicitTz {
    fn new(local: &DateTime<Local>, local_offset_secs: i32, twn: TzWithName) -> Self {
        let offset_from_local_secs = twn
            .tz
            .offset_from_utc_datetime(&local.naive_utc())
            .fix()
            .local_minus_utc()
            - local_offset_secs;
        let offset_from_local = Duration::seconds(offset_from_local_secs.into());
        let dt = local.with_timezone(&twn.tz);

        Self {
            name: twn.name,
            offset_from_local,
            dt,
        }
    }
}

fn format_offset(duration: &Duration) -> String {
    let hours = duration.num_hours();
    let minutes = duration.num_minutes() % 60;

    if minutes == 0 {
        format!("{hours:+}h")
    } else {
        format!("{hours:+}h{}m", minutes.abs())
    }
}

fn main() {
    let cfg = Config::parse();
    let local: DateTime<Local> = Local::now();
    let local_offset = local.offset().fix().local_minus_utc();

    let mut remote_tzs: Vec<_> = cfg
        .tz
        .into_iter()
        .map(|tz| ExplicitTz::new(&local, local_offset, tz))
        .collect();

    remote_tzs.sort_by_key(|rtz| rtz.dt.naive_local());

    let tz_width = if cfg.no_align {
        0
    } else {
        remote_tzs
            .iter()
            .map(|rtz| rtz.name.len())
            .max()
            .unwrap_or(0)
    };
    if !cfg.no_local {
        println!("{: >tz_width$}: {}\n", "Local", local.format(&cfg.fmt));
    }

    for rtz in remote_tzs {
        println!(
            "{: >tz_width$}: {} ({})",
            rtz.name,
            rtz.dt.format(&cfg.fmt),
            format_offset(&rtz.offset_from_local),
        );
    }
}
