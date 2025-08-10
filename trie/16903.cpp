#include <iostream>
using namespace std;

class Trie {
public:
    void add_num(unsigned int x, int depth) {
        count += 1;
        if(depth == 8) {
            is_terminate = true;
            return;
        }

        unsigned int index = x >> 28;
        if(child[index] == nullptr) {
            child[index] = new Trie();
        }
        child[index]->add_num(x << 4, depth + 1);
    }

    void remove_num(unsigned int x, int depth) {
        count -= 1;
        if(depth == 8) {
            return;
        }

        unsigned int index = x >> 28;
        child[index]->remove_num(x << 4, depth + 1);
    }

    unsigned int query(unsigned int x, int depth) {
        unsigned int max_value = 0;
        unsigned int cur_value = x >> 28;
        for(unsigned int i = 0; i < 16; ++i) {
            if(child[i] == nullptr || child[i]->count == 0) {
                continue;
            }
            unsigned int temp = i ^ cur_value;
            if(temp > max_value) {
                max_value = temp;
            }
        }

        unsigned int max_child_value = 0;
        for(unsigned int i = 0; i < 16; ++i) {
            if(child[i] == nullptr || child[i]->count == 0) {
                continue;
            }
            unsigned int temp = i ^ cur_value;
            if(temp == max_value) {
                unsigned int c = child[i]->query(x << 4, depth + 1);
                if(c > max_child_value) {
                    max_child_value = c;
                }
            }
        }

        return (max_value << (4 * (7 - depth))) + max_child_value;
    }

    bool is_terminate = false;
    int count = 0;
    Trie* child[16] = {};
};

int main() {
    ios::sync_with_stdio(false);
	cin.tie(0);

    int m;
    Trie* root = new Trie();
    root->add_num(0, 0);
    cin >> m;
    for(int i = 0; i < m; ++i) {
        int t;
        unsigned int x;
        cin >> t >> x;
        switch(t) {
            case 1:
                root->add_num(x, 0);
                break;
            case 2:
                root->remove_num(x, 0);
                break;
            case 3:
                cout << root->query(x, 0) << "\n";
                break;
        }
    }

    return 0;
}