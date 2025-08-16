#include <iostream>
#include <vector>
#include <algorithm>
#include <cmath>
using namespace std;

int sqrt_n;

struct Query {
    int index;
    int start;
    int end;

    bool operator<(const Query& other) const {
        int block_size = sqrt_n;
        int a_block = start / block_size;
        int b_block = other.start / block_size;
        if (a_block != b_block) return a_block < b_block;
        return end < other.end;
    }
};

int main() {
    ios::sync_with_stdio(false);
	cin.tie(0);

    int n;
    cin >> n;
    vector<int> numbers(n);
    for(int i = 0; i < n; ++i) {
        cin >> numbers[i];
    }

    vector<int> comp = numbers;
    sort(comp.begin(), comp.end());
    comp.erase(unique(comp.begin(), comp.end()), comp.end());
    
    vector<int> v(n);
    for(int i = 0; i < n; ++i) {
        v[i] = lower_bound(comp.begin(), comp.end(), numbers[i]) - comp.begin();
    }

    int q;
    cin >> q;
    vector<Query> query_list(q);
    for(int i = 0; i < q; ++i) {
        int l, r;
        cin >> l >> r;
        query_list[i] = {i, --l, --r};
    }

    sqrt_n = static_cast<int>(sqrt(n));
    sort(query_list.begin(), query_list.end());

    vector<int> count(comp.size(), 0);
    vector<int> result(q);

    int start = 0;
    int end = 0;
    count[v[0]] = 1;
    int answer = 1;

    for(auto query: query_list) {
        while (end < query.end) {
            ++end;
            int val = v[end];
            if(count[val] == 0) {
                ++answer;
            }
            ++count[val];
        }
        while (end > query.end) {
            int val = v[end];
            --count[val];
            if(count[val] == 0) {
                --answer;
            }
            --end;
        }
        while (start < query.start) {
            int val = v[start];
            --count[val];
            if(count[val] == 0) {
                --answer;
            }
            ++start;
        }
        while (start > query.start) {
            --start;
            int val = v[start];
            if(count[val] == 0) {
                ++answer;
            }
            ++count[val];
        }
        result[query.index] = answer;
    }

    for(auto r: result) {
        cout << r << "\n";
    }

    return 0;
}