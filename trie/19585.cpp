#include <iostream>
#include <string>
#include <vector>
#include <bitset>
#include <unordered_set>
#include <map>
using namespace std;

unordered_set<string> nickname_set;

class Trie {
public:
    Trie() : is_terminate(false) {
    }

    void add_string(const string& s, int i) {
        if(s.length() == i) {
            is_terminate = true;
            return;
        }

        if(child[s[i]] == nullptr) {
            child[s[i]] = new Trie();
        }
        child[s[i]]->add_string(s, i + 1);
    }

    bool is_terminate;
    map<char, Trie*> child;
};

bool query(Trie* color_trie, const string& team) {
    for(int i = 0; i < team.length(); ++i) {
        if(color_trie->child[team[i]] == nullptr) {
            break;
        }
        color_trie = color_trie->child[team[i]];
        if(color_trie->is_terminate) {
            if(nickname_set.count(team.substr(i + 1))) {
                return true;
            }
        }
    }
    return false;
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    Trie color_trie;

    int c, n;
    cin >> c >> n;
    for(int i = 0; i < c; ++i) {
        string color;
        cin >> color;
        color_trie.add_string(color, 0);
    }
    for(int i = 0; i < n; ++i) {
        string nickname;
        cin >> nickname;
        nickname_set.insert(nickname);
    }

    int q;
    cin >> q;
    for(int i = 0; i < q; ++i) {
        string team;
        cin >> team;
        if(query(&color_trie, team)) {
            cout << "Yes\n";
        } else {
            cout << "No\n";
        }
    }

    return 0;
}
