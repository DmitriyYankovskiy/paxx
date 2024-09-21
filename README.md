# PAXX - stress testing manager

---
## Supported Languages:
- ***C++***
- ***Rust***
- ***Python***

---
## command: paxx *command* (*special args*) -*flags*:
- ### init
	initialize

- ### check
	check on ready to stress

- ### stress
	stressing solution
	
	***mode***: `stressing_mode`
	***test count***: `usize`	

- ### catch
	stressing solution up to **mistakes cap**
	
	***mode***: `stress_mode`
	***mistakes cap***: `usize`	
	***test count***: `usize`	

- ### remove
	removing *PAXX*

- ### get
	get test by **number**
	
	***number***: `usize`
	***mode***: `stress_mode`

- ### pat
	create file by pattern
	
	***pattern***:
	- ##### generator 
		test generator .cpp (-set: config.generator_path)
	- ##### edit_cfg
		c++ editconfig for vscode 
	- ##### solution 
		solution .cpp (-set: config.generator_path)

- ### run
	running solution on sample

- ### cfg
	change cfg
	
	***field***:
	- ##### sample
		sample for run
	***path***: path

---

### `stress_mode`
 - ##### check
	check solution result
- ##### comp
	compare solution and reference results
- ##### acomp
	auto compare solution and reference results

---
## Judge verdicts:
 - ***OK*** 
 - ***WA***
---
## Templates:
### generator:
```c++
#include<bits/stdc++.h> 

using namespace std;  

const int SEED = 's' + 'e' + 'e' + 'd';

int main(int argc, char *argv[]) {
	int test_number = atoi(argv[1]);	
	mt19937 rnd(test_number + SEED);	
	cout << rnd() << endl;
}
```

### solution / reference:
```c++
#define PRAGMA
#define _GLIBCXX_DEBUG 
#ifdef PRAGMA
#pragma GCC optimize("O3,unroll-loops")
#endif
#include<bits/stdc++.h>
#ifdef PRAGMA
#pragma GCC target("avx2,bmi,bmi2,lzcnt,popcnt")
#endif

using namespace std;
using ll = long long;
  
#define all(v) v.begin(), v.end()
#ifdef DBG
#define debug(x) std::cout << #x << " = " << x << endl;
#else
#define debug(x);
#endif

void solve() {
	// TODO:
}

int main() {
	ios::sync_with_stdio(false);
	cin.tie(nullptr);  
	ll t = 1;
	
	#ifdef DBG
	cin >> t;
	#endif
	
	for (ll i = 0; i < t; ++i) {
		#ifdef SOLO	
			solve();	
		#endif	
	}
}
```



### checker:
```c++
#define PRAGMA
#define _GLIBCXX_DEBUG 
#ifdef PRAGMA
#pragma GCC optimize("O3,unroll-loops")
#endif
#include<bits/stdc++.h>
#ifdef PRAGMA
#pragma GCC target("avx2,bmi,bmi2,lzcnt,popcnt")
#endif

using namespace std;
using ll = long long;
  
#define all(v) v.begin(), v.end()

void check() {
	int res;
	cin >> res;
	if (true) {
		cout << "OK" << endl
	} else {
		cout << "WA" << " <sth>" << endl;
	}
}

int main() {
	ios::sync_with_stdio(false);
	cin.tie(nullptr);
	check();
}
```


### comparator:
```c++
#define PRAGMA
#define _GLIBCXX_DEBUG 
#ifdef PRAGMA
#pragma GCC optimize("O3,unroll-loops")
#endif
#include<bits/stdc++.h>
#ifdef PRAGMA
#pragma GCC target("avx2,bmi,bmi2,lzcnt,popcnt")
#endif

using namespace std;
using ll = long long;
  
#define all(v) v.begin(), v.end()

int main(int argc, char *argv[]) {
	ios::sync_with_stdio(false);
	cin.tie(nullptr);
	string solution_result_path = argv[1];
	string reference_result_path = argv[2];
	
	ifstream s_in(solution_result_path);
	ifstream r_in(reference_result_path);
	int xs, xr;
	s_in >> xs;
	r_in >> xr;
	if (xs == xr) {
		cout << "OK" << endl;
	} else {
		cout << "WA" << endl;
	}
}
```
