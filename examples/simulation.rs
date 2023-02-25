/*
 * Simulates a long-term adversarial attack on threshold signatures with and
 * without proactive refresh. Done for demo purposes.
 */
use proactive_refresh::bls::ECPoint;
use proactive_refresh::pr::ProactiveRefresh;

use curv::{arithmetic::Converter, elliptic::curves::traits::ECScalar};
use serde_json::json;

use std::collections::HashMap;
use std::fs::File;

const T: usize = 5;
const N: usize = 7;
const QUORUM: [usize; 5] = [0, 1, 2, 3, 4];
const BREACHES: [(usize, usize); 5] = [(2, 1), (5, 6), (7, 5), (10, 3), (14, 2)];
const MAX_TIME: usize = 15;

const OUT_FILE: &str = "./out/sim.json";

fn format_pkx(committee: &ProactiveRefresh) -> Vec<String> {
    Vec::from_iter(
        committee
            .threshold_keys()
            .get_quorum_keys(&Vec::from_iter(0..N))
            .into_iter()
            .map(|kp| "0x".to_string() + &kp.priv_key().to_big_int().to_str_radix(16)),
    )
}

fn main() {
    let committee: ProactiveRefresh = ProactiveRefresh::new(N, T);
    let mut committee_pr: ProactiveRefresh = ProactiveRefresh::new(N, T);

    let mut secure: Vec<bool> = vec![true; N];
    let mut secure_pr: Vec<bool> = vec![true; N];

    let mut is_breached = false;
    let is_breached_pr = false;

    let mut breach_ctr = 0;
    let mut epochs = Vec::new();
    for i in 0..MAX_TIME {
        if breach_ctr < BREACHES.len() && BREACHES[breach_ctr].0 == i {
            secure[BREACHES[breach_ctr].1] = false;
            secure_pr[BREACHES[breach_ctr].1] = false;
        }
        if breach_ctr == BREACHES.len() - 1 {
            is_breached = true;
        }

        let mut pk_status = Vec::new();
        for (j, pkx) in format_pkx(&committee).iter().enumerate() {
            let mut hm = HashMap::new();
            hm.insert("key", pkx.clone());
            hm.insert("secure", secure[j].to_string());
            pk_status.push(hm);
        }
        let cx = &committee
            .threshold_keys()
            .collective_pub(&QUORUM.to_vec())
            .bytes_compressed_to_big_int();

        let mut pk_status_pr = Vec::new();
        for (j, pkx) in format_pkx(&committee_pr).iter().enumerate() {
            let mut hm = HashMap::new();
            hm.insert("key", pkx.clone());
            hm.insert("secure", secure_pr[j].to_string());
            pk_status_pr.push(hm);
        }
        let cx_pr = &committee_pr
            .threshold_keys()
            .collective_pub(&QUORUM.to_vec())
            .bytes_compressed_to_big_int();

        let epoch_json = json!({
            "time": i,
            "ats": {
                "breached": is_breached.to_string(),
                "collective_pk": "0x".to_string() + &cx.to_str_radix(16),
                "pks": pk_status
            },
            "ats_pr": {
                "breached": is_breached_pr.to_string(),
                "collective_pk": "0x".to_string() + &cx_pr.to_str_radix(16),
                "pks": pk_status_pr
            }
        });
        epochs.push(epoch_json);

        if BREACHES[breach_ctr].0 == i {
            secure_pr[BREACHES[breach_ctr].1] = true;
            breach_ctr += 1;
        }
        committee_pr.refresh_all();
    }
    serde_json::to_writer(&File::create(OUT_FILE).unwrap(), &epochs).unwrap();
}
