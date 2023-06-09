use chrono::{DateTime, Duration, Local, Offset, TimeZone};
use chrono_tz::Tz;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Config {
    #[arg(
        help = "Remote timezone(s) to print, for example, Europe/London",
        required = true
    )]
    tz: Vec<Tz>,

    #[arg(
        short,
        long,
        help = "A strftime format specifying how to format times",
        default_value = "%a %F %H:%M"
    )]
    fmt: String,
}

struct ExplicitTz {
    tz: Tz,
    offset_from_local: Duration,
    dt: DateTime<Tz>,
}

impl ExplicitTz {
    fn new(local: &DateTime<Local>, local_offset_secs: i32, tz: Tz) -> Self {
        let offset_from_local_secs = tz
            .offset_from_utc_datetime(&local.naive_utc())
            .fix()
            .local_minus_utc()
            - local_offset_secs;
        let offset_from_local = Duration::seconds(offset_from_local_secs.into());
        let dt = local.with_timezone(&tz);

        Self {
            tz,
            offset_from_local,
            dt,
        }
    }
}

fn format_offset(duration: &Duration) -> String {
    let hours = duration.num_hours();
    let minutes = duration.num_minutes() % 60;

    if minutes == 0 {
        format!(" ({hours:+}h)")
    } else {
        format!(" ({hours:+}h{}m)", minutes.abs())
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

    let tz_width = remote_tzs
        .iter()
        .map(|rtz| rtz.tz.to_string().len())
        .max()
        .unwrap_or(0);

    println!("{: >tz_width$}: {}\n", "Local", local.format(&cfg.fmt));

    for rtz in remote_tzs {
        println!(
            "{: >tz_width$}: {}{}",
            rtz.tz.to_string(),
            rtz.dt.format(&cfg.fmt),
            format_offset(&rtz.offset_from_local),
        );
    }
}
