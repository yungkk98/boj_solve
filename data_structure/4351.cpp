#include <iostream>
#include <unordered_map>
#include <string>
using namespace std;

int main() {
    ios::sync_with_stdio(false);
	cin.tie(0);

    int n, m;
    cin >> n >> m;
    unordered_map<string, int> dic;
    for(int i = 0; i < n; ++i) {
        string word;
        int value;
        cin >> word >> value;
        dic[word] = value;
    }

    for(int i = 0; i < m; ++i) {
        int value = 0;
        string word;
        while(true) {
            cin >> word;
            if(word == ".") {
                break;
            }
            if(dic.find(word) != dic.end()) {
                value += dic[word];
            }
        }
        cout << value << "\n";
    }

    return 0;
}