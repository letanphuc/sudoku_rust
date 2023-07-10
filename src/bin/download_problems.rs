extern crate reqwest;
use std::{fs::File, io::Write};

use reqwest::header;
use serde_json::Value;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert("authority", "sudoku.com".parse().unwrap());
    headers.insert("accept", "*/*".parse().unwrap());
    headers.insert(
        "accept-language",
        "en-US,en;q=0.9,vi-VN;q=0.8,vi;q=0.7".parse().unwrap(),
    );
    headers.insert(header::COOKIE, "ab_test=aa_test_des%3Daa_test_des_default2%26dt%3D1688977153; first_visit=fv%3D1688977153%26dt%3D1688977153; __cflb=02DiuE7hKpaqvCsoqtT41sbucqM5JAhhDRDXTk8iuVUHS; _ga=GA1.1.1515033702.1688977154; _fbp=fb.1.1688977154320.1077057364; mode=classic; _pbjs_userid_consent_data=3524755945110770; _pubcid=6d43fe7a-e4ab-4690-a0a5-3ece10698ad2; panoramaId_expiry=1689063568126; _cc_id=e2d8f603ab863d25ba593ee085715e14; panoramaId=3182753cba70e4e700804eddf2e4a9fb927a30e47a87ee4c994335cce56c17f7; __gads=ID=09e5782f39886b02:T=1688977154:RT=1688977471:S=ALNI_MaozknPSc8h5421uKcK6Frt6g6Xfw; __gpi=UID=00000c1f65e87532:T=1688977154:RT=1688977471:S=ALNI_MZVFyY43OSr7JM91iYmlmM1zpHZKg; cto_bundle=oqgBmV9ZdDg1WkI4NmF6NGkxUTZTNGR0dk5ldm8yRUlEM3JtNkJ3TzVzQTcyamtwSHhscWslMkJvT0FSUlZkdzQxT3BuNnYwOVR2TzNCJTJCUnA5eDFRT040UUVHSkJKVEtnenE5RDlwaDRoa0tMUW5iV1BEMXpEZGlzS0dVcXI4RzVockI3M1BmNkVDWnZLTkRpbXdMR0NKY01hJTJGNVElM0QlM0Q; cto_bidid=vdTSNV9VYlFHbFAlMkJsZjc1bzlXT1FvVkQ0JTJCZ1glMkZKcVg2V0hLTGllQiUyRnRqdExvZ2xhUGg3JTJCM2FhczZ1azJxcmVqQUEwOEZ1SVdHbnZ2cDhRUGpPQ1FrY08lMkZmWXM0ZSUyRmxVQ3ZnNTRzb2V5VElacFlZJTNE; _ga_LKCCSV4WGG=GS1.1.1688977154.1.1.1688977811.0.0.0".parse().unwrap());
    headers.insert("referer", "https://sudoku.com/evil/".parse().unwrap());
    headers.insert(
        "sec-ch-ua",
        "\"Not.A/Brand\";v=\"8\", \"Chromium\";v=\"114\", \"Google Chrome\";v=\"114\""
            .parse()
            .unwrap(),
    );
    headers.insert("sec-ch-ua-mobile", "?0".parse().unwrap());
    headers.insert("sec-ch-ua-platform", "\"Linux\"".parse().unwrap());
    headers.insert("sec-fetch-dest", "empty".parse().unwrap());
    headers.insert("sec-fetch-mode", "cors".parse().unwrap());
    headers.insert("sec-fetch-site", "same-origin".parse().unwrap());
    headers.insert("user-agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36".parse().unwrap());
    headers.insert("x-easy-locale", "en".parse().unwrap());
    headers.insert("x-requested-with", "XMLHttpRequest".parse().unwrap());

    let client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    for _ in 0..20 {
        let res = client
            .get("https://sudoku.com/api/level/evil")
            .headers(headers.clone())
            .send()?
            .text()?;

        let v: Value = serde_json::from_str(res.as_str())?;

        let file_name = format!("src/data/problem_{}.json", v["id"]);
        let mut file = File::create(file_name.as_str())?;
        file.write_all(res.as_bytes()).unwrap();

        println!("{} {}", res, v["id"]);
    }

    Ok(())
}
