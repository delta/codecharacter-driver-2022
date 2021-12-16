#include <algorithm>
#include <fcntl.h>
#include <fstream>
#include <iostream>
#include <iterator>
#include <map>
#include <numeric>
#include <queue>
#include <set>
#include <stack>
#include <string>
#include <string_view>
#include <unordered_map>
#include <unordered_set>
#include <vector>
using namespace std;

int sz = 1000000;
void use_iostream() {
  ofstream f("cpp_player_out");
  cout.rdbuf()->pubsetbuf(0, 0);
  ios_base::sync_with_stdio(false);
  // cin.tie(NULL);
  // cout.tie(NULL);
  // cin.rdbuf()->pubsetbuf(0, 0);

  for (int i = 0; i < sz; i++)
    cout << "Hi\n";
  char s[10000];
  for (int i = 0; i < sz; i++) {
    cin.getline(s, 10000);
    f << s << '\n';
  }
}

int main() { use_iostream(); }
