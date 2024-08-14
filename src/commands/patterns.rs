use std::{fs, io::Write, path::Path};

use colored::Colorize;

use crate::Flags;

const GEN_PATTERN: &str = 
r#"#include<bits/stdc++.h>

using namespace std;

const int SEED = 's' + 'e' + 'e' + 'd';

int main(int argc, char *argv[]) {
    int test_number = atoi(argv[1]);
    mt19937 rnd(test_number + SEED);
    cout << rnd() << endl;
}
"#;

pub fn gen(path: String, flags: Flags) {
    if !Path::new(path.as_str()).exists() || flags.contains("r") {
        let mut file = fs::File::create(&path).unwrap();
        file.write_all(GEN_PATTERN.as_bytes()).unwrap();
        println!("{} {}", path.bold().bright_cyan(), "created".bright_cyan());
    } else {
        println!("{} {}", path.bold().blink(), "already exists".white());
    }
}


const STD_PATTERN: &str =
r#"#define PRAGMA

#ifdef PRAGMA
#pragma GCC optimize("O3,unroll-loops")
#endif

#include<bits/stdc++.h>

#ifdef PRAGMA
#pragma GCC target("avx2,bmi,bmi2,lzcnt,popcnt")
#endif

using namespace std;
using ll = long long;
using dbl = long double;

#define all(v) v.begin(), v.end()

#ifdef LOC 
#define debug(x) std::cout << #x << " = " << x << endl;
#else
#define debug(x);
#endif

const int INF = 1e9 + 10;

void solve() {
    // writing here ........
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int t = 1;
#ifdef LOC
    cin >> t;
#endif
    for (ll i = 0; i < t; ++i) {
        solve();
    }
}
"#;

pub fn std(path: String, flags: Flags) {
    if !Path::new(path.as_str()).exists() || flags.contains("r") {
        let mut file = fs::File::create(&path).unwrap();
        file.write_all(STD_PATTERN.as_bytes()).unwrap();
        println!("{} {}", path.bold().bright_cyan(), "created".bright_cyan());
    } else {
        println!("{} {}", path.bold().blink(), "already exists".white());
    }
}