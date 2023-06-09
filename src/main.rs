use chrono::{DateTime, Duration, Local, Offset, TimeZone, Utc};
use chrono_tz::Tz;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Config {
    #[arg(help = "Remote timezone(s) to print, for example, Europe/London")]
    tz: Vec<Tz>,

    #[arg(
        short,
        long,
        help = "A strftime format specifying how to format times",
        default_value = "%a %F %H:%M"
    )]
    fmt: String,
}

struct RemoteTz {
    tz: Tz,
    offset_from_local: Duration,
}

fn format_offset(duration: &Duration) -> String {
    let hours = duration.num_hours();
    let minutes = duration.num_minutes() % 60;

    if hours == 0 && minutes == 0 {
        "".to_string()
    } else if minutes == 0 {
        format!(" ({hours:+}h)")
    } else {
        format!(" ({hours:+}h{minutes:+02}m)")
    }
}

fn get_remote_tz(local_offset_secs: i32, tz: Tz) -> RemoteTz {
    let offset_from_local_secs = tz
        .offset_from_utc_datetime(&Utc::now().naive_utc())
        .fix()
        .local_minus_utc()
        - local_offset_secs;
    let offset_from_local = Duration::seconds(offset_from_local_secs.into());

    RemoteTz {
        tz,
        offset_from_local,
    }
}

fn main() {
    let cfg = Config::parse();
    let local: DateTime<Local> = Local::now();
    let local_offset = local.offset().fix().local_minus_utc();

    let remote_tzs: Vec<_> = cfg
        .tz
        .into_iter()
        .map(|tz| get_remote_tz(local_offset, tz))
        .collect();

    println!("Local: {}", local.format(&cfg.fmt),);

    for rtz in remote_tzs {
        println!(
            "{}: {}{}",
            rtz.tz,
            local.with_timezone(&rtz.tz).format(&cfg.fmt),
            format_offset(&rtz.offset_from_local),
        );
    }
}
