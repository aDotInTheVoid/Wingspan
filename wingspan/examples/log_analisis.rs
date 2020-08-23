use std::error::Error;
use std::io::prelude::*;


fn main() -> Result<(), Box<dyn Error>> {
    let resp = ""; //include_str!("../log.txt");
    let mut goods = vec![];
    let mut bads = vec![];

    for i in resp.lines() {
        // Left is good
        if i.contains("   left: `") {
            goods.push(i);
        }
        if i.contains(r"  right: `") {
            bads.push(i);
        }
    }

    assert_eq!(goods.len(), bads.len());
    let mut i = 0;
    for (good, bad) in goods.iter().zip(bads.iter()).map(|(&x, &y)| (x, y)) {
        let good_bytes = &good[good.find('[').unwrap() + 1..good.find(']').unwrap()];
        let bad_bytes = &bad[bad.find('[').unwrap() + 1..bad.find(']').unwrap()];
        let good_vec: Vec<u8> = good_bytes.split(", ").map(|x| x.parse().unwrap()).collect();
        let bad_vec: Vec<u8> = bad_bytes.split(", ").map(|x| x.parse().unwrap()).collect();
        std::fs::File::create(format!("./good{}.png", i))
            .unwrap()
            .write_all(&good_vec)
            .unwrap();
        std::fs::File::create(format!("./bad{}.png", i))
            .unwrap()
            .write_all(&bad_vec)
            .unwrap();
        i += 1;
    }
    Ok(())
}
